#[path = "export_surface_font_rendering_backgrounds.rs"]
mod backgrounds;
#[path = "export_surface_font_rendering_decorations.rs"]
mod decorations;
#[path = "export_surface_font_rendering_inline_images.rs"]
mod inline_images;

use crate::export_surface_font::SurfaceTextBackgroundPalette;
use crate::export_surface_span::SurfaceTextSpan;
use image::RgbaImage;

use super::SpanVisualRange;

pub(super) fn draw_span_backgrounds(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
    palette: SurfaceTextBackgroundPalette,
) {
    backgrounds::draw_span_backgrounds(image, spans, ranges, x, y, size, palette);
}

pub(super) fn draw_inline_images(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    inline_images::draw_inline_images(image, spans, ranges, x, y, size);
}

pub(super) fn draw_span_decorations(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    decorations::draw_span_decorations(image, spans, ranges, x, y, size);
}
