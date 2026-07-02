use crate::export_surface_font::SurfaceTextBackgroundPalette;
use crate::export_surface_span::SurfaceTextSpan;
use image::{Rgba, RgbaImage};

use super::super::SpanVisualRange;

const HIGHLIGHT_Y_OFFSET_SCALE: f32 = 0.12;
const HIGHLIGHT_HEIGHT_SCALE: f32 = 1.18;
const INLINE_CODE_Y_OFFSET_SCALE: f32 = 0.08;
const INLINE_CODE_LEFT_PADDING: u32 = 4;
const INLINE_CODE_EXTRA_WIDTH: u32 = 8;
const INLINE_CODE_HEIGHT_SCALE: f32 = 1.24;

pub(super) fn draw_span_backgrounds(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
    palette: SurfaceTextBackgroundPalette,
) {
    for (index, span) in spans.iter().enumerate() {
        SpanBackgroundPainter::paint(image, span, ranges.get(index), x, y, size, palette);
    }
}

struct SpanBackgroundPainter;

impl SpanBackgroundPainter {
    fn paint(
        image: &mut RgbaImage,
        span: &SurfaceTextSpan,
        range: Option<&Option<SpanVisualRange>>,
        x: u32,
        y: u32,
        size: f32,
        palette: SurfaceTextBackgroundPalette,
    ) {
        let Some(range) = range.and_then(|range| *range) else {
            return;
        };
        if span.style.highlight {
            Self::paint_highlight(image, range, x, y, size, palette.highlight);
            return;
        }
        if span.style.inline_code {
            Self::paint_inline_code(image, range, x, y, size, palette.inline_code);
        }
    }

    fn paint_highlight(
        image: &mut RgbaImage,
        range: SpanVisualRange,
        x: u32,
        y: u32,
        size: f32,
        color: Rgba<u8>,
    ) {
        draw_fill_rect(
            image,
            x.saturating_add(range.start_x),
            y + (size * HIGHLIGHT_Y_OFFSET_SCALE) as u32,
            range.width(),
            (size * HIGHLIGHT_HEIGHT_SCALE) as u32,
            color,
        );
    }

    fn paint_inline_code(
        image: &mut RgbaImage,
        range: SpanVisualRange,
        x: u32,
        y: u32,
        size: f32,
        color: Rgba<u8>,
    ) {
        draw_fill_rect(
            image,
            x.saturating_add(range.start_x)
                .saturating_sub(INLINE_CODE_LEFT_PADDING),
            y + (size * INLINE_CODE_Y_OFFSET_SCALE) as u32,
            range.width() + INLINE_CODE_EXTRA_WIDTH,
            (size * INLINE_CODE_HEIGHT_SCALE) as u32,
            color,
        );
    }
}

fn draw_fill_rect(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, color: Rgba<u8>) {
    for dy in 0..height {
        for dx in 0..width {
            PixelWriter::put(image, x + dx, y + dy, color);
        }
    }
}

struct PixelWriter;

impl PixelWriter {
    fn put(image: &mut RgbaImage, target_x: u32, target_y: u32, color: Rgba<u8>) {
        if target_x < image.width() && target_y < image.height() {
            image.put_pixel(target_x, target_y, color);
        }
    }
}

#[cfg(test)]
#[path = "export_surface_font_rendering_backgrounds_tests.rs"]
mod tests;
