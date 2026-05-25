#[path = "export_surface_font_rendering_attrs.rs"]
mod attrs;
#[path = "export_surface_font_rendering_math.rs"]
mod math;
#[path = "export_surface_font_rendering_pixels.rs"]
mod pixels;
#[path = "export_surface_font_rendering_ranges.rs"]
mod ranges;
#[path = "export_surface_font_rendering_shapes.rs"]
mod shapes;

#[derive(Clone, Copy, Debug)]
pub(super) struct SpanVisualRange {
    pub(super) start_x: u32,
    end_x: u32,
}

impl SpanVisualRange {
    pub(super) fn new(start_x: f32, end_x: f32) -> Self {
        Self {
            start_x: start_x.floor().max(0.0) as u32,
            end_x: end_x.ceil().max(0.0) as u32,
        }
    }

    pub(super) fn width(self) -> u32 {
        self.end_x.saturating_sub(self.start_x).max(1)
    }

    fn extend(self, start_x: f32, end_x: f32) -> Self {
        Self {
            start_x: (self.start_x as f32).min(start_x) as u32,
            end_x: (self.end_x as f32).max(end_x) as u32,
        }
    }
}

pub(super) struct SurfaceGlyphPixel {
    pub(super) origin_x: u32,
    pub(super) origin_y: u32,
    pub(super) glyph_x: i32,
    pub(super) glyph_y: i32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) color: cosmic_text::Color,
}

pub(super) fn attrs_for_span_with_metadata(
    span: &crate::export_surface_span::SurfaceTextSpan,
    metadata: usize,
) -> cosmic_text::Attrs<'static> {
    attrs::attrs_for_span_with_metadata(span, metadata)
}

pub(super) fn draw_glyph_pixel(image: &mut image::RgbaImage, glyph: SurfaceGlyphPixel) {
    pixels::draw_glyph_pixel(image, glyph);
}

pub(super) fn span_visual_ranges(
    buffer: &cosmic_text::Buffer,
    span_count: usize,
) -> Vec<Option<SpanVisualRange>> {
    ranges::span_visual_ranges(buffer, span_count)
}

pub(super) fn draw_span_backgrounds(
    image: &mut image::RgbaImage,
    spans: &[crate::export_surface_span::SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    shapes::draw_span_backgrounds(image, spans, ranges, x, y, size);
}

pub(super) fn draw_inline_images(
    image: &mut image::RgbaImage,
    spans: &[crate::export_surface_span::SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    shapes::draw_inline_images(image, spans, ranges, x, y, size);
}

pub(super) fn draw_span_decorations(
    image: &mut image::RgbaImage,
    spans: &[crate::export_surface_span::SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    shapes::draw_span_decorations(image, spans, ranges, x, y, size);
}

#[cfg(test)]
pub(super) fn is_half_width_math_symbol(character: char) -> bool {
    math::is_half_width_math_symbol(character)
}
