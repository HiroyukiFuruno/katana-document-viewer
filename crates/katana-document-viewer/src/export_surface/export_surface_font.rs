mod export_surface_font_rendering;

use crate::export_surface_span::SurfaceTextSpan;
use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};
use image::{Rgba, RgbaImage};

use self::export_surface_font_rendering as rendering;

const FONT_LINE_HEIGHT_MULTIPLIER: f32 = 1.35;
const FONT_BUFFER_HEIGHT_SCALE: f32 = 1.8;
const FONT_COLOR_RED_CHANNEL: usize = 0;
const FONT_COLOR_GREEN_CHANNEL: usize = 1;
const FONT_COLOR_BLUE_CHANNEL: usize = 2;
const FONT_COLOR_ALPHA_CHANNEL: usize = 3;

pub(crate) struct SurfaceTextLayout {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) size: f32,
    pub(crate) color: Rgba<u8>,
    pub(crate) max_width: Option<f32>,
}

pub(crate) struct SurfaceTextPainter {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl SurfaceTextPainter {
    pub(crate) fn from_system_fonts() -> Option<Self> {
        Some(Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        })
    }

    pub(crate) fn draw_text(
        &mut self,
        image: &mut RgbaImage,
        text: &str,
        layout: SurfaceTextLayout,
    ) {
        let mut buffer = self.create_text_buffer(
            layout.size,
            layout
                .max_width
                .unwrap_or_else(|| image.width().saturating_sub(layout.x) as f32),
        );
        buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);
        self.draw_buffer(image, &mut buffer, layout.x, layout.y, layout.color);
    }

    pub(crate) fn draw_spans(
        &mut self,
        image: &mut RgbaImage,
        spans: &[SurfaceTextSpan],
        x: u32,
        y: u32,
        size: f32,
        color: Rgba<u8>,
    ) {
        let (mut buffer, ranges) = self.create_spans_buffer(image, spans, x, y, size);
        rendering::draw_span_backgrounds(image, spans, &ranges, x, y, size);
        self.draw_buffer(image, &mut buffer, x, y, color);
        rendering::draw_inline_images(image, spans, &ranges, x, y, size);
        rendering::draw_span_decorations(image, spans, &ranges, x, y, size);
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
        buffer.draw(
            &mut self.font_system,
            &mut self.swash_cache,
            default_color,
            |glyph_x, glyph_y, width, height, pixel| {
                rendering::draw_glyph_pixel(
                    image,
                    rendering::SurfaceGlyphPixel {
                        origin_x: x,
                        origin_y: y,
                        glyph_x,
                        glyph_y,
                        width,
                        height,
                        color: pixel,
                    },
                );
            },
        );
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
        let mut buffer = self.create_text_buffer(size, image.width().saturating_sub(x) as f32);
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

fn buffer_text_color(color: Rgba<u8>) -> Color {
    Color::rgba(
        color[FONT_COLOR_RED_CHANNEL],
        color[FONT_COLOR_GREEN_CHANNEL],
        color[FONT_COLOR_BLUE_CHANNEL],
        color[FONT_COLOR_ALPHA_CHANNEL],
    )
}

#[cfg(test)]
mod export_surface_font_test_cases;
#[cfg(test)]
mod export_surface_font_test_support;
