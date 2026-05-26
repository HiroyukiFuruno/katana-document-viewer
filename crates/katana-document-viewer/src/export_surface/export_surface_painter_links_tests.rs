use super::*;
use crate::export_surface_span::SurfaceTextSpan;

#[test]
fn append_line_span_metadata_advances_empty_targets_without_annotation() {
    let span = SurfaceTextSpan::linked("empty", "", Default::default());
    let mut annotations = Vec::new();
    let mut anchors = Vec::new();
    let mut x = 12;

    SurfacePainter::append_line_span_metadata(
        &mut annotations,
        &mut anchors,
        SurfaceSpanMetadataRequest {
            span: &span,
            font_size: 16.0,
            line_height: 24,
            page_index: 0,
            text_y: 4,
        },
        &mut x,
    );

    assert!(annotations.is_empty());
    assert!(anchors.is_empty());
    assert!(x > 12);
}
