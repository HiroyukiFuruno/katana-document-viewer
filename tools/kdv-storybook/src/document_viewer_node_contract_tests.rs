use super::KucNodeFactory;
use katana_document_viewer::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
    ViewerInteractionConfig, ViewerNode, ViewerNodeKind, ViewerRect, ViewerTextSpan,
};
use katana_ui_core::render_model::{UiDimension, UiNodeKind};

const CONTRACT_MEDIA_WIDTH: u32 = 120;
const VIEWER_NODE_WIDTH: f32 = 120.0;
const VIEWER_NODE_HEIGHT: f32 = 64.0;
const OVERRIDE_CODE_HEIGHT: f32 = 84.0;
const OVERRIDE_CODE_HEIGHT_PX: u16 = 84;
const CODE_PADDING_X: f32 = 36.0;
const CODE_PADDING_X_PX: u16 = 36;
const TASK_LIST_HEIGHT: f32 = 72.0;
const TASK_CHECKBOX_HEIGHT_PX: u16 = 23;
const QUOTED_CODE_SOURCE_HEIGHT: f32 = 12.0;
const QUOTED_CODE_HEIGHT_PX: u16 = 43;
const FOOTNOTE_MARKER_WIDTH_PX: u16 = 36;
const FIRST_CHILD: usize = 0;
const SECOND_CHILD: usize = 1;

#[test]
fn code_block_height_comes_from_viewer_rect_not_kuc_text_metrics() {
    let mut node = viewer_node(
        ViewerNodeKind::Code {
            language: Some("rust".to_string()),
        },
        "fn main() {\n    println!(\"hello\");\n}",
    );
    node.rect.height = OVERRIDE_CODE_HEIGHT;

    let ui_node = KucNodeFactory::new(&[], CONTRACT_MEDIA_WIDTH)
        .interaction(interaction(true))
        .viewer_node(&node);

    assert_eq!(UiNodeKind::Stack, ui_node.kind());
    assert_eq!(
        UiDimension::Px(OVERRIDE_CODE_HEIGHT_PX),
        ui_node.props().common.height
    );
    assert_eq!(
        UiDimension::Px(OVERRIDE_CODE_HEIGHT_PX),
        ui_node.children()[FIRST_CHILD].props().common.height
    );
}

#[test]
fn code_block_uses_viewer_rect_x_as_container_padding() {
    let mut node = viewer_node(
        ViewerNodeKind::Code {
            language: Some("rust".to_string()),
        },
        "let x = 42;",
    );
    node.rect.x = CODE_PADDING_X;

    let ui_node = KucNodeFactory::new(&[], CONTRACT_MEDIA_WIDTH)
        .interaction(interaction(true))
        .viewer_node(&node);

    assert_eq!(UiNodeKind::Column, ui_node.kind());
    assert_eq!(
        UiDimension::Px(CODE_PADDING_X_PX),
        ui_node.props().common.padding.left
    );
    assert_eq!(UiNodeKind::Stack, ui_node.children()[FIRST_CHILD].kind());
    assert_eq!(
        UiDimension::Px(VIEWER_NODE_HEIGHT as u16),
        ui_node.children()[FIRST_CHILD].props().common.height
    );
}

#[test]
fn list_task_checkbox_height_comes_from_kuc_text_metrics_not_viewer_rect() {
    let mut node = viewer_node(ViewerNodeKind::List, "[x] done\n[/] doing");
    node.rect.height = TASK_LIST_HEIGHT;

    let ui_node = KucNodeFactory::new(&[], CONTRACT_MEDIA_WIDTH).viewer_node(&node);

    let first_checkbox = &ui_node.children()[FIRST_CHILD].children()[FIRST_CHILD];
    let second_checkbox = &ui_node.children()[SECOND_CHILD].children()[FIRST_CHILD];
    assert_eq!(
        UiDimension::Px(TASK_CHECKBOX_HEIGHT_PX),
        first_checkbox.props().common.height
    );
    assert_eq!(
        UiDimension::Px(TASK_CHECKBOX_HEIGHT_PX),
        second_checkbox.props().common.height
    );
}

#[test]
fn blockquote_code_height_uses_compact_quoted_code_metrics() {
    let mut node = viewer_node(
        ViewerNodeKind::BlockQuote,
        "> ```rust\n> let quoted_code = true;\n> ```",
    );
    node.rect.height = QUOTED_CODE_SOURCE_HEIGHT;

    let ui_node = KucNodeFactory::new(&[], CONTRACT_MEDIA_WIDTH).viewer_node(&node);

    assert_eq!(UiNodeKind::Column, ui_node.kind());
    assert_eq!(
        UiDimension::Px(QUOTED_CODE_HEIGHT_PX),
        ui_node.children()[FIRST_CHILD].props().common.height
    );
}

#[test]
fn footnote_definition_uses_marker_column_and_body_spans() {
    let mut node = viewer_node(
        ViewerNodeKind::FootnoteDefinition {
            label: "1".to_string(),
        },
        "1. First footnote content. ↩",
    );
    node.spans = vec![
        ViewerTextSpan::plain("1. "),
        ViewerTextSpan::plain("First footnote content."),
        ViewerTextSpan::plain(" "),
        ViewerTextSpan::linked(
            "↩",
            "#fnref-1",
            katana_document_viewer::ViewerTextStyle::default().link(),
        ),
    ];

    let ui_node = KucNodeFactory::new(&[], CONTRACT_MEDIA_WIDTH).viewer_node(&node);

    assert_eq!(UiNodeKind::Row, ui_node.kind());
    assert_eq!("1.", ui_node.children()[FIRST_CHILD].props().label);
    assert_eq!(
        "list-marker",
        ui_node.children()[FIRST_CHILD].props().text.role
    );
    assert_eq!(
        UiDimension::Px(FOOTNOTE_MARKER_WIDTH_PX),
        ui_node.children()[FIRST_CHILD].props().common.width
    );
    assert_eq!(
        "First footnote content. ↩",
        ui_node.children()[SECOND_CHILD].props().label
    );
    assert_eq!(
        "footnote",
        ui_node.children()[SECOND_CHILD].props().text.role
    );
    assert_eq!(
        "#fnref-1",
        ui_node.children()[1].props().text.spans[2].link_target
    );
}

fn interaction(code_controls_enabled: bool) -> ViewerInteractionConfig {
    ViewerInteractionConfig {
        hover_highlight_enabled: false,
        selection_enabled: false,
        image_controls_enabled: false,
        diagram_controls_enabled: false,
        code_controls_enabled,
    }
}

fn viewer_node(kind: ViewerNodeKind, raw: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("contract-node".to_string()),
        kind,
        source: source(raw),
        text: raw.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: VIEWER_NODE_WIDTH,
            height: VIEWER_NODE_HEIGHT,
        },
        artifact_id: None,
    }
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
