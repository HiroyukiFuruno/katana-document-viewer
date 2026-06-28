use super::super::metrics::ViewerNodeMetrics;
use super::{ViewerNodeKind, ViewerTextSpan, ViewerTextStyle};

#[test]
fn linked_spans_keep_target_and_link_style() {
    let span = ViewerTextSpan::linked("label", "https://example.com", ViewerTextStyle::default());

    assert_eq!("label", span.text);
    assert_eq!("https://example.com", span.link_target);
}

#[test]
fn text_style_remembers_color_channels() {
    let style = ViewerTextStyle::default().color_rgba([1, 2, 3, 4]);

    assert_eq!([1, 2, 3, 4], style.color_rgba);
}

#[test]
fn heading_heights_match_export_surface_levels() {
    let typography = ViewerNodeMetrics::default_typography();

    assert_eq!(
        92.0,
        ViewerNodeMetrics::block_height(&ViewerNodeKind::Heading { level: 1 }, "h1", typography)
    );
    assert_eq!(
        78.0,
        ViewerNodeMetrics::block_height(&ViewerNodeKind::Heading { level: 2 }, "h2", typography)
    );
    assert_eq!(
        66.0,
        ViewerNodeMetrics::block_height(&ViewerNodeKind::Heading { level: 3 }, "h3", typography)
    );
}

#[test]
fn body_height_counts_kuc_document_body_wrapped_lines() {
    let text = "a".repeat(59);
    let typography = ViewerNodeMetrics::default_typography();

    assert_eq!(
        92.0,
        ViewerNodeMetrics::block_height(&ViewerNodeKind::Paragraph, &text, typography)
    );
}

#[test]
fn alert_height_keeps_export_surface_vertical_padding() {
    let typography = ViewerNodeMetrics::default_typography();

    assert_eq!(
        124.0,
        ViewerNodeMetrics::block_height(
            &ViewerNodeKind::Alert {
                label: "TIP".to_string()
            },
            "TIP: body",
            typography
        )
    );
}

#[test]
fn rule_height_matches_export_surface_rule_geometry() {
    let typography = ViewerNodeMetrics::default_typography();

    assert_eq!(
        34.0,
        ViewerNodeMetrics::block_height(&ViewerNodeKind::Rule, "", typography)
    );
}

#[test]
fn code_height_matches_export_surface_box_model() {
    let typography = ViewerNodeMetrics::default_typography();

    assert_eq!(
        84.0,
        ViewerNodeMetrics::block_height(
            &ViewerNodeKind::Code {
                language: Some("rust".to_string())
            },
            "",
            typography
        )
    );
    assert_eq!(
        142.0,
        ViewerNodeMetrics::block_height(
            &ViewerNodeKind::Code {
                language: Some("rust".to_string())
            },
            "one\ntwo\nthree",
            typography
        )
    );
}
