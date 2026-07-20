use super::RenderedSvgHtmlQuality;

#[test]
fn rendered_svg_rejects_empty_placeholder_svg() {
    assert!(!RenderedSvgHtmlQuality::has_rendered_svg("<svg></svg>"));
}

#[test]
fn rendered_svg_accepts_visual_body_elements() {
    assert!(RenderedSvgHtmlQuality::has_rendered_svg(
        r#"<svg><g><rect width="10" height="10"></rect></g></svg>"#
    ));
}

#[test]
fn rendered_svg_ignores_empty_svg_before_rendered_svg() {
    assert!(RenderedSvgHtmlQuality::has_rendered_svg(
        r#"<svg></svg><svg><text>Rendered</text></svg>"#
    ));
}

#[test]
fn rendered_svg_rejects_style_only_svg() {
    assert!(!RenderedSvgHtmlQuality::has_rendered_svg(
        r#"<svg><style>.a { fill: red; }</style></svg>"#
    ));
}

#[test]
fn rendered_svg_rejects_empty_group_only_svg() {
    assert!(!RenderedSvgHtmlQuality::has_rendered_svg(
        r#"<svg><g id="placeholder"></g></svg>"#
    ));
}

#[test]
fn rendered_svg_rejects_svg_with_no_close_angle_bracket() {
    assert!(!RenderedSvgHtmlQuality::has_rendered_svg("<svg"));
}

#[test]
fn svg_at_has_visual_body_rejects_tag_without_close_angle_bracket() {
    assert!(!RenderedSvgHtmlQuality::svg_at_has_visual_body(
        "<svg", "<svg", 0,
    ));
}

#[test]
fn svg_at_has_visual_body_rejects_svg_without_closing_tag() {
    assert!(!RenderedSvgHtmlQuality::svg_at_has_visual_body(
        "<svg><rect",
        "<svg><rect",
        0,
    ));
}

#[test]
fn next_svg_cursor_falls_back_to_body_start_when_close_tag_missing() {
    assert_eq!(
        Some(5),
        RenderedSvgHtmlQuality::next_svg_cursor("<svg><rect", "<svg><rect", 0)
    );
}
