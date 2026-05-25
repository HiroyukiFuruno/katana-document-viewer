use crate::export_surface_span::SurfaceTextSpan;
use cosmic_text::{Attrs, Color, Family, Style, Weight};

const HIDE_IMAGE_COLOR: u8 = 0;
const RED_CHANNEL: usize = 0;
const GREEN_CHANNEL: usize = 1;
const BLUE_CHANNEL: usize = 2;
const ALPHA_CHANNEL: usize = 3;

pub(super) fn attrs_for_span_with_metadata(
    span: &SurfaceTextSpan,
    metadata: usize,
) -> Attrs<'static> {
    let mut attrs = Attrs::new();
    let style = span.style;
    if style.bold {
        attrs = attrs.weight(Weight::BOLD);
    }
    if style.italic {
        attrs = attrs.style(Style::Italic);
    }
    if style.monospace && span.text.is_ascii() {
        attrs = attrs.family(Family::Monospace);
    }
    if let Some(color) = style.color {
        attrs = attrs.color(rgba_text_color(color));
    }
    if span.inline_image.is_some() {
        attrs = attrs.color(hidden_inline_image_color());
    }
    attrs.metadata(metadata)
}

fn rgba_text_color(color: image::Rgba<u8>) -> Color {
    Color::rgba(
        color[RED_CHANNEL],
        color[GREEN_CHANNEL],
        color[BLUE_CHANNEL],
        color[ALPHA_CHANNEL],
    )
}

fn hidden_inline_image_color() -> Color {
    Color::rgba(
        HIDE_IMAGE_COLOR,
        HIDE_IMAGE_COLOR,
        HIDE_IMAGE_COLOR,
        HIDE_IMAGE_COLOR,
    )
}
