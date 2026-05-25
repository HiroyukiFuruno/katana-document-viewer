use crate::export_surface_span::SurfaceTextSpan;
use image::{Rgba, RgbaImage};

#[path = "export_surface_font_helper_backgrounds.rs"]
mod backgrounds;
#[path = "export_surface_font_helper_decorations.rs"]
mod decorations;
#[path = "export_surface_font_helper_pixels.rs"]
mod pixels;
#[path = "export_surface_font_helper_ranges.rs"]
mod ranges;
#[cfg(test)]
#[path = "export_surface_font_helper_test_metrics.rs"]
mod test_metrics;

pub(super) use self::pixels::{
    blend_pixel, draw_horizontal_line, fill_glyph_pixel, fill_rect, paste_inline_rgba,
    SurfaceGlyphPixel,
};
pub(super) use self::backgrounds::draw_span_backgrounds;
pub(super) use self::decorations::draw_span_decorations;
pub(super) use self::ranges::{span_visual_ranges, SpanVisualRange};
#[cfg(test)]
pub(super) use self::test_metrics::estimated_text_width;

pub(super) const DEFAULT_ALPHA: u8 = 255;
pub(super) const HIGHLIGHT_LINE_ALPHA: u8 = DEFAULT_ALPHA;
pub(super) const HIGHLIGHT_COLOR_RED: u8 = 255;
pub(super) const HIGHLIGHT_COLOR_GREEN: u8 = 235;
pub(super) const HIGHLIGHT_COLOR_BLUE: u8 = 59;
pub(super) const INLINE_CODE_BG_RED: u8 = 239;
pub(super) const INLINE_CODE_BG_GREEN: u8 = 242;
pub(super) const INLINE_CODE_BG_BLUE: u8 = 246;
pub(super) const STROKE_PADDING: u32 = 4;
pub(super) const UNDERLINE_STROKE_THICKNESS: u32 = 2;
pub(super) const DEFAULT_TEXT_COLOR_RED: u8 = 36;
pub(super) const DEFAULT_TEXT_COLOR_GREEN: u8 = 41;
pub(super) const DEFAULT_TEXT_COLOR_BLUE: u8 = 47;
pub(super) const BASELINE_ADJUSTMENT_RATIO: f32 = 0.22;
pub(super) const IMAGE_X_PADDING: u32 = 4;
pub(super) const BACKDROP_PADDING: u32 = 8;
pub(super) const UNDERLINE_THICKNESS: u32 = 2;
pub(super) const HALF_ALPHA_MASK: u16 = u16::MAX / u8::MAX as u16;

pub(super) const FALLBACK_CHAR_WIDTH_FACTOR: u32 = 10;
pub(super) const FALLBACK_LINE_WIDTH_LIMIT: u32 = 720;
pub(super) const FALLBACK_MAX_HEIGHT: u32 = 18;
pub(super) const HALF_WIDTH_SPACE_FACTOR: f32 = 0.35;
pub(super) const HALF_WIDTH_PUNCTUATION_FACTOR: f32 = 0.43;
pub(super) const HALF_WIDTH_TEXT_FACTOR: f32 = 0.54;
pub(super) const HALF_WIDTH_MATH_FACTOR: f32 = 0.65;
pub(super) const DEFAULT_WIDTH_FACTOR: f32 = 0.92;
pub(super) const DEFAULT_ALPHA_FLOAT: f32 = 255.0;
pub(super) const MAX_CHANNEL: f32 = 255.0;
pub(super) const HIGHLIGHT_Y_OFFSET_SCALE: f32 = 0.12;
pub(super) const HIGHLIGHT_HEIGHT_SCALE: f32 = 1.18;
pub(super) const INLINE_CODE_Y_OFFSET_SCALE: f32 = 0.08;
pub(super) const INLINE_CODE_HEIGHT_SCALE: f32 = 1.24;
pub(super) const UNDERLINE_OFFSET_SCALE: f32 = 1.22;
pub(super) const STRIKE_THROUGH_OFFSET_SCALE: f32 = 0.72;
pub(super) const SURFACE_TEXT_LAYOUT_HEIGHT_RATIO: f32 = 1.8;
pub(super) const COLOR_CHANNEL_COUNT: usize = 3;

pub(super) fn draw_inline_images(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    for (index, span) in spans.iter().enumerate() {
        let Some(inline_image) = &span.inline_image else {
            continue;
        };
        let Some(range) = ranges.get(index).and_then(|range| *range) else {
            continue;
        };
        let image_x = x.saturating_add(range.start_x());
        let baseline_adjustment = size * BASELINE_ADJUSTMENT_RATIO;
        let image_y = y
            .saturating_add(baseline_adjustment as u32)
            .saturating_sub(inline_image.height().saturating_sub(size as u32) / 2);
        paste_inline_rgba(image, inline_image.image(), image_x, image_y);
    }
}

pub(super) fn surface_text_layout_height(size: f32) -> f32 {
    size * SURFACE_TEXT_LAYOUT_HEIGHT_RATIO
}

pub(super) const SURFACE_TEXT_FONT_SIZE_RATIO: f32 = 1.35;
