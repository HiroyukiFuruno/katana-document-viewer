mod export_surface_font_fallbacks;
mod export_surface_font_metrics;
mod export_surface_font_paint;
mod export_surface_font_rendering;
use self::export_surface_font_metrics::{buffer_text_color, span_ranges_width};
use self::export_surface_font_rendering as rendering;
use crate::export_surface_span::SurfaceTextSpan;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use image::{Rgba, RgbaImage};
use std::cell::RefCell;

const FONT_LINE_HEIGHT_MULTIPLIER: f32 = 1.45;
const FONT_BUFFER_HEIGHT_SCALE: f32 = 1.8;
const DEFAULT_HIGHLIGHT_BACKGROUND: Rgba<u8> = Rgba([255, 235, 59, 255]);
const DEFAULT_INLINE_CODE_BACKGROUND: Rgba<u8> = Rgba([239, 242, 246, 255]);
thread_local! {
    static CACHED_TEXT_PAINTER: RefCell<SurfaceTextPainter> =
        RefCell::new(SurfaceTextPainter::from_system_fonts());
}

#[derive(Clone, Copy)]
pub(crate) struct SurfaceTextBackgroundPalette {
    pub(crate) highlight: Rgba<u8>,
    pub(crate) inline_code: Rgba<u8>,
}

impl Default for SurfaceTextBackgroundPalette {
    fn default() -> Self {
        Self {
            highlight: DEFAULT_HIGHLIGHT_BACKGROUND,
            inline_code: DEFAULT_INLINE_CODE_BACKGROUND,
        }
    }
}

pub(crate) struct SurfaceTextLayout {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) size: f32,
    pub(crate) color: Rgba<u8>,
    pub(crate) max_width: Option<f32>,
}

pub(crate) struct SurfaceSpansLayout {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) size: f32,
    pub(crate) color: Rgba<u8>,
    pub(crate) backgrounds: SurfaceTextBackgroundPalette,
}

pub(crate) struct SurfaceTextPainter {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl SurfaceTextPainter {
    pub(crate) fn from_system_fonts() -> Self {
        Self {
            font_system: export_surface_font_fallbacks::font_system_with_embedded_fallbacks(),
            swash_cache: SwashCache::new(),
        }
    }

    pub(crate) fn with_system_fonts<T>(render: impl FnOnce(&mut SurfaceTextPainter) -> T) -> T {
        CACHED_TEXT_PAINTER.with(|cell| {
            let mut painter = cell.borrow_mut();
            render(&mut painter)
        })
    }

    pub(crate) fn measure_spans_width(
        &mut self,
        spans: &[SurfaceTextSpan],
        size: f32,
        max_width: f32,
    ) -> u32 {
        let (_buffer, ranges) = self.create_spans_buffer_with_width(spans, size, max_width);
        span_ranges_width(&ranges)
    }

    fn draw_buffer(
        &mut self,
        image: &mut RgbaImage,
        buffer: &mut Buffer,
        x: u32,
        y: u32,
        color: Rgba<u8>,
    ) {
        let default_color = buffer_text_color(color);
        let mut samples = rendering::SurfaceTextSupersamples::new();
        buffer.draw(
            &mut self.font_system,
            &mut self.swash_cache,
            default_color,
            |glyph_x, glyph_y, width, height, pixel| {
                samples.push_glyph(glyph_x, glyph_y, width, height, pixel);
            },
        );
        samples.draw(image, x, y);
    }

    fn create_text_buffer(&mut self, size: f32, max_width: f32) -> Buffer {
        let metrics = self.metrics(size);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(Some(max_width), Some(size * FONT_BUFFER_HEIGHT_SCALE));
        buffer
    }

    fn create_spans_buffer(
        &mut self,
        image: &RgbaImage,
        spans: &[SurfaceTextSpan],
        x: u32,
        _y: u32,
        size: f32,
    ) -> (Buffer, Vec<Option<rendering::SpanVisualRange>>) {
        self.create_spans_buffer_with_width(spans, size, image.width().saturating_sub(x) as f32)
    }

    fn create_spans_draw_buffer(
        &mut self,
        image: &RgbaImage,
        spans: &[SurfaceTextSpan],
        x: u32,
        _y: u32,
        size: f32,
    ) -> Buffer {
        let mut buffer = self.create_text_buffer(
            size * rendering::TEXT_SUPERSAMPLE_SCALE,
            image.width().saturating_sub(x) as f32 * rendering::TEXT_SUPERSAMPLE_SCALE,
        );
        let rich = self.prepare_span_rich_text(spans, size);
        let rich = rich
            .iter()
            .map(|(text, attrs)| (text.as_str(), attrs.clone()))
            .collect::<Vec<_>>();
        buffer.set_rich_text(rich, &Attrs::new(), Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.font_system, false);
        buffer
    }

    fn create_spans_buffer_with_width(
        &mut self,
        spans: &[SurfaceTextSpan],
        size: f32,
        max_width: f32,
    ) -> (Buffer, Vec<Option<rendering::SpanVisualRange>>) {
        let mut buffer = self.create_text_buffer(size, max_width);
        let rich = self.prepare_span_rich_text(spans, size);
        let rich = rich
            .iter()
            .map(|(text, attrs)| (text.as_str(), attrs.clone()))
            .collect::<Vec<_>>();
        buffer.set_rich_text(rich, &Attrs::new(), Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.font_system, false);
        let ranges = rendering::span_visual_ranges(&buffer, spans.len());
        (buffer, ranges)
    }

    fn prepare_span_rich_text(
        &self,
        spans: &[SurfaceTextSpan],
        size: f32,
    ) -> Vec<(String, Attrs<'static>)> {
        let layout_texts = spans
            .iter()
            .map(|span| span.layout_text(size))
            .collect::<Vec<_>>();
        spans
            .iter()
            .zip(layout_texts.iter())
            .enumerate()
            .map(|(index, (span, text))| {
                (
                    text.to_string(),
                    rendering::attrs_for_span_with_metadata(span, index + 1),
                )
            })
            .collect()
    }

    fn metrics(&self, size: f32) -> Metrics {
        Metrics::new(size, size * FONT_LINE_HEIGHT_MULTIPLIER)
    }
}

#[cfg(test)]
mod export_surface_font_emoji_tests;
#[cfg(test)]
mod export_surface_font_test_cases;
#[cfg(test)]
mod export_surface_font_test_support;
