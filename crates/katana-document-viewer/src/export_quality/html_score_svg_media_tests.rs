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
