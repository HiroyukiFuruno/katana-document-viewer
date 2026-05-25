use super::{
    DEFAULT_ALPHA, DEFAULT_TEXT_COLOR_BLUE, DEFAULT_TEXT_COLOR_GREEN, DEFAULT_TEXT_COLOR_RED,
    STRIKE_THROUGH_OFFSET_SCALE, SpanVisualRange, UNDERLINE_OFFSET_SCALE, draw_horizontal_line,
};
use crate::export_surface_span::SurfaceTextSpan;
use image::{Rgba, RgbaImage};

pub(super) fn draw_span_decorations(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    for (index, span) in spans.iter().enumerate() {
        draw_span_decoration(image, span, ranges.get(index).and_then(|range| *range), x, y, size);
    }
}

fn draw_span_decoration(
    image: &mut RgbaImage,
    span: &SurfaceTextSpan,
    range: Option<SpanVisualRange>,
    x: u32,
    y: u32,
    size: f32,
) {
    let Some(range) = range else {
        return;
    };
    let cursor_x = x.saturating_add(range.start_x());
    let width = range.width();
    if span.style.underline {
        draw_decoration_line(image, span, cursor_x, y, width, size * UNDERLINE_OFFSET_SCALE);
    }
    if span.style.strikethrough {
        draw_decoration_line(
            image,
            span,
            cursor_x,
            y,
            width,
            size * STRIKE_THROUGH_OFFSET_SCALE,
        );
    }
}

fn draw_decoration_line(
    image: &mut RgbaImage,
    span: &SurfaceTextSpan,
    x: u32,
    y: u32,
    width: u32,
    offset: f32,
) {
    let color = span.style.color.unwrap_or(default_text_color());
    draw_horizontal_line(image, x, y + offset as u32, width, color);
}

fn default_text_color() -> Rgba<u8> {
    Rgba([
        DEFAULT_TEXT_COLOR_RED,
        DEFAULT_TEXT_COLOR_GREEN,
        DEFAULT_TEXT_COLOR_BLUE,
        DEFAULT_ALPHA,
    ])
}
