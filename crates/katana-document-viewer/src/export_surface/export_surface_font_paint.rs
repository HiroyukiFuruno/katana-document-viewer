use super::{SurfaceSpansLayout, SurfaceTextLayout, SurfaceTextPainter, rendering};
use crate::export_surface_span::SurfaceTextSpan;
use cosmic_text::{Attrs, Shaping};
#[cfg(test)]
use image::Rgba;
use image::RgbaImage;

#[cfg(test)]
use super::SurfaceTextBackgroundPalette;

impl SurfaceTextPainter {
    pub(crate) fn draw_text(
        &mut self,
        image: &mut RgbaImage,
        text: &str,
        layout: SurfaceTextLayout,
    ) {
        let mut draw_buffer = self.create_text_buffer(
            layout.size * rendering::TEXT_SUPERSAMPLE_SCALE,
            layout
                .max_width
                .unwrap_or_else(|| image.width().saturating_sub(layout.x) as f32)
                * rendering::TEXT_SUPERSAMPLE_SCALE,
        );
        draw_buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);
        self.draw_buffer(image, &mut draw_buffer, layout.x, layout.y, layout.color);
    }

    #[cfg(test)]
    pub(crate) fn draw_spans(
        &mut self,
        image: &mut RgbaImage,
        spans: &[SurfaceTextSpan],
        x: u32,
        y: u32,
        size: f32,
        color: Rgba<u8>,
    ) {
        self.draw_spans_with_backgrounds(
            image,
            spans,
            SurfaceSpansLayout {
                x,
                y,
                size,
                color,
                backgrounds: SurfaceTextBackgroundPalette::default(),
            },
        );
    }

    pub(crate) fn draw_spans_with_backgrounds(
        &mut self,
        image: &mut RgbaImage,
        spans: &[SurfaceTextSpan],
        layout: SurfaceSpansLayout,
    ) {
        let (_layout_buffer, ranges) =
            self.create_spans_buffer(image, spans, layout.x, layout.y, layout.size);
        let mut draw_buffer =
            self.create_spans_draw_buffer(image, spans, layout.x, layout.y, layout.size);
        rendering::draw_span_backgrounds(
            image,
            spans,
            &ranges,
            layout.x,
            layout.y,
            layout.size,
            layout.backgrounds,
        );
        self.draw_buffer(image, &mut draw_buffer, layout.x, layout.y, layout.color);
        rendering::draw_inline_images(image, spans, &ranges, layout.x, layout.y, layout.size);
        rendering::draw_span_decorations(image, spans, &ranges, layout.x, layout.y, layout.size);
    }
}
