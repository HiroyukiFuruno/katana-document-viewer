use super::super::KucNodeFactory;
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMediaControlAction, ViewerMediaControlKind, ViewerNode,
    ViewerNodeKind, ViewerRect,
};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::{
    UiDimension, UiHostActionPayload, UiHostActionPlan, UiNode, UiPosition, UiVariant, UiZIndex,
};

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 32.0;
const CONTROL_SIZE_PX: u16 = 28;
const DIAGRAM_RIGHT_COLUMN_INDEX: usize = 4;

pub(super) fn diagram_controls_factory() -> KucNodeFactory<'static> {
    KucNodeFactory::new(&[], NODE_WIDTH as u32).interaction(ViewerInteractionConfig {
        hover_highlight_enabled: true,
        selection_enabled: true,
        image_controls_enabled: false,
        diagram_controls_enabled: true,
        code_controls_enabled: true,
    })
}

pub(super) fn image_controls_factory() -> KucNodeFactory<'static> {
    KucNodeFactory::new(&[], NODE_WIDTH as u32).interaction(ViewerInteractionConfig {
        hover_highlight_enabled: true,
        selection_enabled: true,
        image_controls_enabled: true,
        diagram_controls_enabled: false,
        code_controls_enabled: true,
    })
}

pub(super) fn math_controls_factory() -> KucNodeFactory<'static> {
    KucNodeFactory::new(&[], NODE_WIDTH as u32).interaction(ViewerInteractionConfig {
        hover_highlight_enabled: true,
        selection_enabled: true,
        image_controls_enabled: true,
        diagram_controls_enabled: true,
        code_controls_enabled: true,
    })
}

pub(super) fn assert_absolute_overlay(node: &UiNode, top: u16, right: u16, bottom: u16, left: u16) {
    let common = &node.props().common;
    assert_eq!(UiPosition::Absolute, common.position);
    assert_eq!(UiZIndex::Value(2), common.z_index);
    assert_eq!(UiDimension::Px(top), common.margin.top);
    assert_eq!(UiDimension::Px(right), common.margin.right);
    assert_eq!(UiDimension::Px(bottom), common.margin.bottom);
    assert_eq!(UiDimension::Px(left), common.margin.left);
}

pub(super) fn assert_diagram_grid_buttons(grid: &UiNode) {
    assert_diagram_internal_button(&grid.children()[0].children()[2], "pan-up");
    assert_diagram_internal_button(
        &grid.children()[0].children()[DIAGRAM_RIGHT_COLUMN_INDEX],
        "zoom-in",
    );
    assert_diagram_internal_button(&grid.children()[1].children()[0], "pan-left");
    assert_diagram_internal_button(&grid.children()[1].children()[2], "reset-view");
    assert_diagram_internal_button(
        &grid.children()[1].children()[DIAGRAM_RIGHT_COLUMN_INDEX],
        "pan-right",
    );
    assert_diagram_internal_button(&grid.children()[2].children()[0], "trackpad-help");
    assert_diagram_internal_button(&grid.children()[2].children()[2], "pan-down");
    assert_diagram_internal_button(
        &grid.children()[2].children()[DIAGRAM_RIGHT_COLUMN_INDEX],
        "zoom-out",
    );
}

pub(super) fn assert_diagram_fullscreen_button(node: &UiNode) {
    assert_diagram_host_button(node, "fullscreen");
}

fn assert_diagram_host_button(node: &UiNode, value: &str) {
    assert_button(node, value, ViewerMediaControlKind::Diagram);
    assert_diagram_button_shape(node, value);
}

fn assert_diagram_internal_button(node: &UiNode, value: &str) {
    assert_eq!(value, node.props().interaction.value);
    assert_eq!("node", node.props().interaction.surface_control_target_id);
    assert!(
        UiHostActionPlan::collect_from_node(node).is_empty(),
        "diagram internal control must not emit host action: {value}"
    );
    assert_diagram_button_shape(node, value);
}

fn assert_diagram_button_shape(node: &UiNode, value: &str) {
    assert_eq!(value, node.props().interaction.value);
    assert_eq!(UiVariant::Icon, node.props().variant);
    assert_square_control(node);
    assert_eq!(
        format!("surface.{value}"),
        node.props().icon.role,
        "diagram controls must expose a typed surface icon role"
    );
    assert!(
        !node.props().icon.svg_source.is_empty(),
        "diagram controls must carry renderer-owned SVG source"
    );
}

pub(super) fn assert_image_button(node: &UiNode, value: &str) {
    assert_button(node, value, ViewerMediaControlKind::Image);
    assert_eq!(UiVariant::Icon, node.props().variant);
    assert_square_control(node);
}

pub(super) fn assert_diagram_spacer(node: &UiNode) {
    assert_square_control(node);
}

pub(super) fn assert_diagram_gap(_node: &UiNode) {}

pub(super) fn collect_host_actions(node: &UiNode, actions: &mut Vec<String>) {
    actions.extend(
        node.props()
            .common
            .host_actions
            .iter()
            .map(|action| action.action_id.clone()),
    );
    for child in node.children() {
        collect_host_actions(child, actions);
    }
}

pub(super) fn assert_no_kdv_media_style_classes(node: &UiNode) {
    assert!(
        !node
            .props()
            .style_classes
            .iter()
            .any(|style_class| style_class.starts_with("kdv-diagram-")
                || style_class == "kdv-image-control")
    );
    for child in node.children() {
        assert_no_kdv_media_style_classes(child);
    }
}

pub(super) fn viewer_node(kind: ViewerNodeKind, text: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("node".to_string()),
        kind,
        source: source(text),
        text: text.to_string(),
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

fn assert_square_control(node: &UiNode) {
    assert_eq!(UiDimension::Px(CONTROL_SIZE_PX), node.props().common.width);
    assert_eq!(UiDimension::Px(CONTROL_SIZE_PX), node.props().common.height);
}

fn assert_button(node: &UiNode, value: &str, kind: ViewerMediaControlKind) {
    assert_eq!(value, node.props().interaction.value);
    let actions = UiHostActionPlan::collect_from_node(node);
    assert!(!actions.is_empty(), "missing host action for {value}");
    let action = &actions[0];
    assert!(action.payload.is_empty());
    let UiHostActionPayload::SurfaceControl(payload) = &action.typed_payload else {
        assert!(
            matches!(action.typed_payload, UiHostActionPayload::SurfaceControl(_)),
            "expected typed surface control payload: {:?}",
            action.typed_payload
        );
        return;
    };
    assert_eq!("node", payload.node_id);
    assert_eq!(
        ViewerMediaControlAction::host_action_id_for(kind, value),
        action.action_id
    );
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
