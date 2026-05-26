use crate::export_surface_span::SurfaceTextSpan;
use image::{Rgba, RgbaImage};

use super::super::SpanVisualRange;

const DEFAULT_UNDERLINE_COLOR: Rgba<u8> = Rgba([36, 41, 47, 255]);
const DEFAULT_LINE_THICKNESS: u32 = 2;
const UNDERLINE_OFFSET_SCALE: f32 = 1.22;
const STRIKE_THROUGH_OFFSET_SCALE: f32 = 0.72;

pub(super) fn draw_span_decorations(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    for (index, span) in spans.iter().enumerate() {
        SpanDecorationPainter::paint(image, span, ranges.get(index), x, y, size);
    }
}

struct SpanDecorationPainter;

impl SpanDecorationPainter {
    fn paint(
        image: &mut RgbaImage,
        span: &SurfaceTextSpan,
        range: Option<&Option<SpanVisualRange>>,
        x: u32,
        y: u32,
        size: f32,
    ) {
        let Some(range) = range.and_then(|range| *range) else {
            return;
        };
        if !span.style.underline && !span.style.strikethrough {
            return;
        }
        let cursor_x = x.saturating_add(range.start_x);
        let width = range.width();
        let underline_color = span.style.color.unwrap_or(DEFAULT_UNDERLINE_COLOR);
        Self::paint_lines(image, span, cursor_x, width, y, size, underline_color);
    }

    fn paint_lines(
        image: &mut RgbaImage,
        span: &SurfaceTextSpan,
        cursor_x: u32,
        width: u32,
        y: u32,
        size: f32,
        color: Rgba<u8>,
    ) {
        if span.style.underline {
            let underline_y = y + (size * UNDERLINE_OFFSET_SCALE) as u32;
            draw_horizontal_line(image, cursor_x, underline_y, width, color);
        }
        if span.style.strikethrough {
            let strike_y = y + (size * STRIKE_THROUGH_OFFSET_SCALE) as u32;
            draw_horizontal_line(image, cursor_x, strike_y, width, color);
        }
    }
}

fn draw_horizontal_line(image: &mut RgbaImage, x: u32, y: u32, width: u32, color: Rgba<u8>) {
    for dy in 0..DEFAULT_LINE_THICKNESS {
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
#[path = "export_surface_font_rendering_decorations_tests.rs"]
mod tests;
