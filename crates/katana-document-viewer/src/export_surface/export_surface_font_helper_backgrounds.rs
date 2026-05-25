use super::{
    BACKDROP_PADDING, DEFAULT_ALPHA, HIGHLIGHT_COLOR_BLUE, HIGHLIGHT_COLOR_GREEN,
    HIGHLIGHT_COLOR_RED, HIGHLIGHT_HEIGHT_SCALE, HIGHLIGHT_LINE_ALPHA, HIGHLIGHT_Y_OFFSET_SCALE,
    IMAGE_X_PADDING, INLINE_CODE_BG_BLUE, INLINE_CODE_BG_GREEN, INLINE_CODE_BG_RED,
    INLINE_CODE_HEIGHT_SCALE, INLINE_CODE_Y_OFFSET_SCALE, SpanVisualRange, fill_rect,
};
use crate::export_surface_span::SurfaceTextSpan;
use image::{Rgba, RgbaImage};

pub(super) fn draw_span_backgrounds(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    for (index, span) in spans.iter().enumerate() {
        draw_span_background(image, span, ranges.get(index).and_then(|range| *range), x, y, size);
    }
}

fn draw_span_background(
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
    if span.style.highlight {
        draw_highlight_background(image, cursor_x, y, range.width(), size);
    }
    if span.style.inline_code {
        draw_inline_code_background(image, cursor_x, y, range.width(), size);
    }
}

fn draw_highlight_background(image: &mut RgbaImage, x: u32, y: u32, width: u32, size: f32) {
    fill_rect(
        image,
        x,
        y + (size * HIGHLIGHT_Y_OFFSET_SCALE) as u32,
        width,
        (size * HIGHLIGHT_HEIGHT_SCALE) as u32,
        Rgba([
            HIGHLIGHT_COLOR_RED,
            HIGHLIGHT_COLOR_GREEN,
            HIGHLIGHT_COLOR_BLUE,
            HIGHLIGHT_LINE_ALPHA,
        ]),
    );
}

fn draw_inline_code_background(image: &mut RgbaImage, x: u32, y: u32, width: u32, size: f32) {
    fill_rect(
        image,
        x.saturating_sub(IMAGE_X_PADDING),
        y + (size * INLINE_CODE_Y_OFFSET_SCALE) as u32,
        width + BACKDROP_PADDING,
        (size * INLINE_CODE_HEIGHT_SCALE) as u32,
        Rgba([
            INLINE_CODE_BG_RED,
            INLINE_CODE_BG_GREEN,
            INLINE_CODE_BG_BLUE,
            DEFAULT_ALPHA,
        ]),
    );
}
