use super::*;
use crate::export_surface_line::SurfaceLine;
use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};

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

#[test]
fn append_line_link_metadata_uses_explicit_line_anchor_before_link_spans() {
    let line = SurfaceLine::body_spans(
        vec![
            SurfaceTextSpan::plain("1. note ".to_string()),
            SurfaceTextSpan::linked("↩", "#fnref-1", SurfaceTextStyle::default().link()),
        ],
        0,
    )
    .with_anchor_id("fn-1".to_string());
    let mut annotations = Vec::new();
    let mut anchors = Vec::new();

    SurfacePainter::append_line_link_metadata(&mut annotations, &mut anchors, &line, 0, 24);

    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].target, "#fnref-1");
    assert_eq!(anchors.len(), 1);
    assert_eq!(anchors[0].id, "fn-1");
    assert!(anchors[0].x < annotations[0].x);
}

#[test]
fn append_line_link_metadata_keeps_reference_anchor_at_reference_span() {
    let line = SurfaceLine::body_spans(
        vec![
            SurfaceTextSpan::plain("Body ".to_string()),
            SurfaceTextSpan::linked("[1]", "#fn-1", SurfaceTextStyle::default().link()),
        ],
        0,
    );
    let mut annotations = Vec::new();
    let mut anchors = Vec::new();

    SurfacePainter::append_line_link_metadata(&mut annotations, &mut anchors, &line, 0, 24);

    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].target, "#fn-1");
    assert_eq!(anchors.len(), 1);
    assert_eq!(anchors[0].id, "fnref-1");
    assert_eq!(anchors[0].x, annotations[0].x);
}
