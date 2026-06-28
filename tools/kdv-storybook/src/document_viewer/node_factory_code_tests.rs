use super::KucNodeFactory;
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMediaControlAction, ViewerMediaControlKind, ViewerNode,
    ViewerNodeKind, ViewerRect,
};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::{
    UiDimension, UiHostActionPayload, UiHostActionPlan, UiNodeKind, UiPosition, UiVariant,
};
use std::collections::BTreeSet;

const CONTROL_SIZE_PX: u16 = 28;
const MAX_MEDIA_WIDTH: u32 = 120;
const CODE_NODE_WIDTH: f32 = 120.0;
const CODE_NODE_HEIGHT: f32 = 64.0;

#[test]
fn code_block_has_copy_control_without_losing_code_body() {
    let node = viewer_node("let value = 1;");
    let ui_node = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH)
        .interaction(interaction(true))
        .viewer_node(&node);

    assert_eq!(UiNodeKind::Stack, ui_node.kind());
    assert_eq!("code", ui_node.children()[0].props().text.role);
    assert!(!has_style_class(
        &ui_node.children()[0],
        "kdv-document-code"
    ));
    assert!(ui_node.children()[0].props().common.border.visible);
    assert_eq!(
        UiDimension::Px(24),
        ui_node.children()[0].props().common.padding.left
    );
    assert_eq!(
        UiDimension::Px(52),
        ui_node.children()[0].props().common.padding.right
    );
    assert_eq!(
        UiDimension::Px(20),
        ui_node.children()[0].props().common.padding.top
    );
    assert_eq!(
        UiDimension::Px(64),
        ui_node.children()[0].props().common.height
    );
    assert_eq!(
        UiPosition::Absolute,
        ui_node.children()[1].props().common.position
    );
    assert_eq!(
        UiDimension::Px(12),
        ui_node.children()[1].props().common.margin.top
    );
    assert_eq!(
        UiDimension::Px(12),
        ui_node.children()[1].props().common.margin.right
    );
    let copy = &ui_node.children()[1].children()[0];
    let copy_plans = UiHostActionPlan::collect_from_node(copy);
    assert_eq!(1, copy_plans.len());
    assert_eq!(
        copy.id().as_str(),
        copy_plans[0].target.as_str(),
        "copy action target must use the rendered button node id"
    );
    assert_eq!(
        copy.id().as_str(),
        copy.props().state_id.as_str(),
        "copy button state and hover target must share the same stable id"
    );
    assert_no_kdv_media_style(copy);
    assert_eq!(UiVariant::Icon, copy.props().variant);
    assert_eq!(UiDimension::Px(CONTROL_SIZE_PX), copy.props().common.width);
    assert_eq!(UiDimension::Px(CONTROL_SIZE_PX), copy.props().common.height);
    assert_eq!("copy-code", copy.props().interaction.value);
    assert!(!copy_plans.is_empty(), "missing code copy host action");
    let action = &copy_plans[0];
    assert_eq!(
        ViewerMediaControlAction::host_action_id_for(ViewerMediaControlKind::Code, "copy-code"),
        action.action_id
    );
    assert!(action.payload.is_empty());
    let UiHostActionPayload::SurfaceControl(payload) = &action.typed_payload else {
        assert!(
            matches!(action.typed_payload, UiHostActionPayload::SurfaceControl(_)),
            "expected typed surface control payload: {:?}",
            action.typed_payload
        );
        return;
    };
    assert_eq!("code-node", payload.node_id);
}

#[test]
fn code_block_hides_copy_control_when_interaction_is_disabled() {
    let node = viewer_node("let value = 1;");
    let ui_node = KucNodeFactory::new(&[], 120)
        .interaction(interaction(false))
        .viewer_node(&node);

    assert_eq!(UiNodeKind::Text, ui_node.kind());
    assert_eq!("code", ui_node.props().text.role);
    assert!(ui_node.props().common.border.visible);
}

#[test]
fn copied_code_block_keeps_copy_action_and_shows_check_mark() {
    let node = viewer_node("let value = 1;");
    let copied = BTreeSet::from(["code-node".to_string()]);
    let ui_node = KucNodeFactory::new(&[], 120)
        .interaction(interaction(true))
        .copied_code_node_ids(&copied)
        .viewer_node(&node);

    let copy = &ui_node.children()[1].children()[0];
    assert_eq!("✓", copy.props().label);
    assert_eq!("copy-code", copy.props().interaction.value);
    let copy_plans = UiHostActionPlan::collect_from_node(copy);
    assert!(
        !copy_plans.is_empty(),
        "copied button must keep the copy host action"
    );
}

#[test]
fn code_block_height_comes_from_viewer_rect_not_kuc_text_metrics() {
    let mut node = viewer_node("fn main() {\n    println!(\"hello\");\n}");
    node.rect.height = 84.0;

    let ui_node = KucNodeFactory::new(&[], 120)
        .interaction(interaction(true))
        .viewer_node(&node);

    assert_eq!(UiDimension::Px(84), ui_node.props().common.height);
    assert_eq!(
        UiDimension::Px(84),
        ui_node.children()[0].props().common.height
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

fn assert_no_kdv_media_style(node: &katana_ui_core::render_model::UiNode) {
    assert!(
        !has_kdv_media_style(node),
        "unexpected media style class in {:?}",
        node.props().style_classes
    );
}

fn has_kdv_media_style(node: &katana_ui_core::render_model::UiNode) -> bool {
    node.props().style_classes.iter().any(|value| {
        value.starts_with("kdv-diagram-")
            || value == "kdv-image-control"
            || value == "kdv-code-control"
    })
}

fn has_style_class(node: &katana_ui_core::render_model::UiNode, expected: &str) -> bool {
    node.props()
        .style_classes
        .iter()
        .any(|value| value == expected)
}

fn viewer_node(raw: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("code-node".to_string()),
        kind: ViewerNodeKind::Code {
            language: Some("rust".to_string()),
        },
        source: source(raw),
        text: raw.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: CODE_NODE_WIDTH,
            height: CODE_NODE_HEIGHT,
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
