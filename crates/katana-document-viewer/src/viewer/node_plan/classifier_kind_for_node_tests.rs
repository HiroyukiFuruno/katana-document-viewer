use super::super::{ViewerHtmlRole, ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::node;
use crate::ArtifactFormat;
use katana_markdown_model::{HtmlBlockRole, KmmNodeKind};

#[test]
fn node_kind_for_node_promotes_standalone_image_paragraph() {
    let image_paragraph = node(
        KmmNodeKind::Paragraph,
        "![diagram](diagram.svg)",
        vec![node(image_kind(), "![diagram](diagram.svg)", Vec::new())],
    );

    assert_eq!(
        Some(ViewerNodeKind::Image),
        ViewerNodeClassifier::node_kind_for_node(&image_paragraph)
    );
    assert_eq!(
        Some(ArtifactFormat::Svg),
        ViewerNodeClassifier::asset_format(&image_paragraph, &ViewerNodeKind::Image)
    );
}

#[test]
fn node_kind_for_node_treats_legacy_note_alert_as_blockquote() {
    let legacy = alert_node("> **Note**\n> body");
    let gfm = alert_node("> [!NOTE]\n> body");

    assert_eq!(
        Some(ViewerNodeKind::BlockQuote),
        ViewerNodeClassifier::node_kind_for_node(&legacy)
    );
    assert_eq!(
        Some(ViewerNodeKind::Alert {
            label: "NOTE".to_string()
        }),
        ViewerNodeClassifier::node_kind_for_node(&gfm)
    );
}

#[test]
fn node_kind_for_node_promotes_details_html_to_accordion() {
    let details = node(
        html_kind(HtmlBlockRole::Generic),
        "<details><summary>Title</summary><div>Body</div></details>",
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::Accordion
        }),
        ViewerNodeClassifier::node_kind_for_node(&details)
    );
}

#[test]
fn node_kind_for_node_preserves_html_alignment_roles() {
    assert_html_role(
        r#"<p style="text-align: right">Right</p>"#,
        ViewerHtmlRole::Right,
    );
    assert_html_role(r#"<p align="left">Left</p>"#, ViewerHtmlRole::Left);
}

#[test]
fn node_kind_for_node_promotes_html_image_paragraph() {
    let image = node(
        KmmNodeKind::Paragraph,
        r#"<img src="data:image/svg+xml,%3Csvg%3E%3C%2Fsvg%3E" width="128" alt="icon">"#,
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Html {
            role: ViewerHtmlRole::Generic
        }),
        ViewerNodeClassifier::node_kind_for_node(&image)
    );
}

fn assert_html_role(raw: &str, expected: ViewerHtmlRole) {
    let current = node(html_kind(HtmlBlockRole::Generic), raw, Vec::new());
    assert_eq!(
        Some(ViewerNodeKind::Html { role: expected }),
        ViewerNodeClassifier::node_kind_for_node(&current)
    );
}

fn image_kind() -> KmmNodeKind {
    KmmNodeKind::Image(katana_markdown_model::ImageNode {
        alt: "diagram".to_string(),
        src: "diagram.svg".to_string(),
        title: None,
    })
}

fn alert_node(raw: &str) -> katana_markdown_model::KmmNode {
    node(
        KmmNodeKind::Alert {
            label: "NOTE".to_string(),
        },
        raw,
        Vec::new(),
    )
}

fn html_kind(role: HtmlBlockRole) -> KmmNodeKind {
    KmmNodeKind::HtmlBlock(role)
}
