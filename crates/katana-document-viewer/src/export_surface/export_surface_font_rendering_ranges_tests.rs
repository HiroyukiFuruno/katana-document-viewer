use super::*;
use crate::export_surface_span::SurfaceTextSpan;
use cosmic_text::{Attrs, FontSystem, Metrics, Shaping};

#[test]
fn ignores_zero_metadata_glyphs() {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(16.0, 16.0 * 1.45);
    let mut buffer = cosmic_text::Buffer::new(&mut font_system, metrics);
    buffer.set_size(Some(128.0), Some(16.0 * 1.8));
    buffer.set_rich_text(
        vec![("x", Attrs::new().metadata(0))],
        &Attrs::new(),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(&mut font_system, false);

    let ranges = span_visual_ranges(&buffer, 1);

    assert!(ranges[0].is_none());
}

#[test]
fn ignores_glyphs_outside_span_count() {
    let span = SurfaceTextSpan::plain("text");
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(16.0, 16.0 * 1.45);
    let mut buffer = cosmic_text::Buffer::new(&mut font_system, metrics);
    buffer.set_size(Some(128.0), Some(16.0 * 1.8));
    buffer.set_rich_text(
        vec![(
            span.text.as_str(),
            super::super::attrs_for_span_with_metadata(&span, 1),
        )],
        &Attrs::new(),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(&mut font_system, false);

    assert_eq!(span_visual_ranges(&buffer, 0).len(), 0);
}
