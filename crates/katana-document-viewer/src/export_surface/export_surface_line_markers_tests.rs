use super::*;
use crate::export_surface_span::SurfaceTextSpan;

#[test]
fn list_marker_detects_bullet_and_basic_tasks() {
    let bullet = SurfaceLine::body_spans(
        vec![SurfaceTextSpan::plain("•"), SurfaceTextSpan::plain("x")],
        0,
    );
    let done = SurfaceLine::body_spans(vec![SurfaceTextSpan::plain("☑")], 0);
    let empty = SurfaceLine::body_spans(vec![SurfaceTextSpan::plain("☐")], 0);

    assert!(matches!(
        bullet.list_marker(),
        Some(SurfaceLineMarker::Bullet)
    ));
    assert!(matches!(
        done.list_marker(),
        Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Done))
    ));
    assert!(matches!(
        empty.list_marker(),
        Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Empty))
    ));
}

#[test]
fn list_marker_detects_katana_task_styles() {
    let blocked = SurfaceLine::body_spans(vec![SurfaceTextSpan::plain("⊟")], 0);
    let in_progress = SurfaceLine::body_spans(vec![SurfaceTextSpan::plain("◩")], 0);

    assert!(matches!(
        blocked.list_marker(),
        Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Blocked))
    ));
    assert!(matches!(
        in_progress.list_marker(),
        Some(SurfaceLineMarker::Task(SurfaceTaskMarker::InProgress))
    ));
}

#[test]
fn list_marker_detects_ordered_markers() {
    let ordered = SurfaceLine::body_spans(
        vec![SurfaceTextSpan::plain("1."), SurfaceTextSpan::plain("item")],
        0,
    );

    assert!(matches!(
        ordered.list_marker(),
        Some(SurfaceLineMarker::Ordered(marker)) if marker == "1."
    ));
}

#[test]
fn content_spans_skip_leading_marker_and_keep_original_without_marker() {
    let with_marker = SurfaceLine::body_spans(
        vec![
            SurfaceTextSpan::plain("•"),
            SurfaceTextSpan::plain("content"),
        ],
        0,
    );
    let without_marker = SurfaceLine::body_spans(vec![SurfaceTextSpan::plain("content")], 0);

    assert_eq!(with_marker.content_spans().len(), 1);
    assert_eq!(with_marker.content_spans()[0].text, "content");
    assert_eq!(without_marker.content_spans().len(), 1);
}

#[test]
fn content_spans_are_empty_for_marker_only_line() {
    let marker_only = SurfaceLine::body_spans(vec![SurfaceTextSpan::plain("•")], 0);

    assert!(marker_only.content_spans().is_empty());
}

#[test]
fn list_marker_returns_none_for_empty_or_plain_line() {
    let empty = SurfaceLine::body_spans(Vec::new(), 0);
    let plain = SurfaceLine::body_spans(vec![SurfaceTextSpan::plain("text")], 0);

    assert!(empty.list_marker().is_none());
    assert!(plain.list_marker().is_none());
}

#[test]
fn top_margin_follows_line_level() {
    assert_eq!(SurfaceLine::heading(1, "h1".to_string()).top_margin(), 16);
    assert_eq!(SurfaceLine::heading(2, "h2".to_string()).top_margin(), 14);
    assert_eq!(SurfaceLine::heading(3, "h3".to_string()).top_margin(), 12);
    assert_eq!(SurfaceLine::body("body".to_string()).top_margin(), 5);
    let code = SurfaceLine::code_spans(vec![SurfaceTextSpan::plain("code")]);
    assert_eq!(code.top_margin(), 0);
}
