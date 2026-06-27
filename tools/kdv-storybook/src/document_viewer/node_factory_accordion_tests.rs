use super::KucNodeFactory;
use katana_document_viewer::{ViewerHtmlRole, ViewerNode, ViewerNodeKind, ViewerRect};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::UiNodeKind;
use std::collections::BTreeMap;

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 32.0;
const MAX_MEDIA_WIDTH: u32 = 120;

#[test]
fn details_html_without_open_attribute_renders_closed_accordion() {
    let ui_node = accordion("<details><summary>Title</summary><div>Body</div></details>");

    assert_eq!(UiNodeKind::Accordion, ui_node.kind());
    assert_eq!("Title", ui_node.props().label);
    assert!(!ui_node.props().interaction.open);
    assert_eq!("Body", accordion_body_label(&ui_node));
}

#[test]
fn details_html_with_open_attribute_preserves_open_state() {
    let ui_node =
        accordion(r#"<details open><summary class="lead">Title</summary><p>Body</p></details>"#);

    assert_eq!(UiNodeKind::Accordion, ui_node.kind());
    assert!(ui_node.props().interaction.open);
    assert_eq!("Body", accordion_body_label(&ui_node));
}

#[test]
fn accordion_open_override_toggles_rendered_state() {
    let ui_node = accordion_with_override(
        "<details><summary>Title</summary><div>Body</div></details>",
        true,
    );

    assert!(ui_node.props().interaction.open);
    assert_eq!("accordion", ui_node.id().as_str());
    assert_eq!("accordion", ui_node.props().state_id.as_str());
}

#[test]
fn details_html_body_markdown_is_rendered_as_structured_kdv_nodes() {
    let ui_node = accordion(
        "<details open><summary>Show details</summary><div>\n\n- Swords\n  - Muramasa\n  - Masamune\n  - Kotetsu\n\n</div></details>",
    );

    assert_eq!(UiNodeKind::Accordion, ui_node.kind());
    let body_column = &ui_node.children()[0];
    assert_eq!(UiNodeKind::Column, body_column.kind());
    let list_node = &body_column.children()[0];
    assert_eq!(UiNodeKind::Column, list_node.kind());
    assert!(
        nested_labels(list_node)
            .iter()
            .any(|label| label == "Swords"),
        "accordion body should use KDV list projection instead of raw markdown text"
    );
    assert!(
        !nested_labels(list_node)
            .iter()
            .any(|label| label == "- Swords"),
        "accordion body must not keep raw markdown markers as paragraph text"
    );
}

fn accordion(raw: &str) -> katana_ui_core::render_model::UiNode {
    let factory = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH);
    factory.viewer_node(&viewer_node(raw))
}

fn accordion_with_override(raw: &str, open: bool) -> katana_ui_core::render_model::UiNode {
    let mut overrides = BTreeMap::new();
    overrides.insert("accordion".to_string(), open);
    let factory = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH).accordion_open_overrides(&overrides);
    factory.viewer_node(&viewer_node(raw))
}

fn accordion_body_label(ui_node: &katana_ui_core::render_model::UiNode) -> String {
    let body_column = &ui_node.children()[0];
    body_column.children()[0].props().label.clone()
}

fn nested_labels(ui_node: &katana_ui_core::render_model::UiNode) -> Vec<String> {
    let mut labels = vec![ui_node.props().label.clone()];
    for child in ui_node.children() {
        labels.extend(nested_labels(child));
    }
    labels
}

fn viewer_node(raw: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("accordion".to_string()),
        kind: ViewerNodeKind::Html {
            role: ViewerHtmlRole::Accordion,
        },
        source: source(raw),
        text: raw.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: NODE_WIDTH,
            height: NODE_HEIGHT,
        },
        artifact_id: None,
    }
}

#[test]
fn details_html_without_details_markup_falls_back_to_text_node() {
    let factory = KucNodeFactory::new(&[], 120);
    let ui_node = factory.viewer_node(&viewer_node("not a details block"));

    assert_eq!(UiNodeKind::Text, ui_node.kind());
    assert_eq!("not a details block", ui_node.props().label);
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
