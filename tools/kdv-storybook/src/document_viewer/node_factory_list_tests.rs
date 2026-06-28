use super::KucNodeFactory;
use crate::test_assert::KucTestExpect;
use katana_document_viewer::{ViewerTaskState, ViewerTextSpan, ViewerTextStyle};
use katana_ui_core::render_model::{
    UI_TASK_SET_STATE_ACTION_ID, UI_TASK_TOGGLE_ACTION_ID, UiCursor, UiDimension,
    UiHostActionPayload, UiHostActionPlan, UiNodeKind,
};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea, UiTreeSurfaceHost};
use std::collections::BTreeMap;

#[path = "node_factory_list_tests_support.rs"]
mod support;

use support::{
    assert_margin_left_px, assert_no_kdv_list_class, theme_rgb,
    vertical_bounds_for_color_in_x_range, viewer_node,
};

#[test]
fn list_node_preserves_katana_task_states_and_context_menu() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("[ ] todo\n[x] done\n[/] doing\n[-] hold");

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiNodeKind::Column, ui_node.kind());
    assert_eq!(4, ui_node.children().len());
    let progress_checkbox = &ui_node.children()[2].children()[0];
    assert_eq!("[/]", progress_checkbox.props().interaction.value);
    assert_eq!(
        "ui-task-state:list:2",
        progress_checkbox.props().state_id.as_str()
    );
    assert_eq!(4, progress_checkbox.props().context_menu.items.len());
}

#[test]
fn list_node_emits_typed_task_host_action_without_string_payload() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("[/] doing");

    let ui_node = factory.viewer_node(&node);
    let action = UiHostActionPlan::collect_from_root(&ui_node)
        .into_iter()
        .find(|action| action.action_id == UI_TASK_TOGGLE_ACTION_ID)
        .kuc_expect("task host action");

    assert!(action.payload.is_empty());
    let target = action
        .task_control_target()
        .kuc_expect("typed task target should be available");
    assert_eq!("list", target.node_id);
    assert_eq!(0, target.row_index);
    assert_eq!("ui-task-state:list:0", target.state_id);
}

#[test]
fn list_task_context_menu_items_carry_typed_requested_state_actions() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("[/] doing");

    let ui_node = factory.viewer_node(&node);
    let checkbox = &ui_node.children()[0].children()[0];
    let done = checkbox
        .props()
        .context_menu
        .items
        .iter()
        .find(|item| item.label == "完了")
        .kuc_expect("done context menu item");
    let action = done
        .host_action
        .as_ref()
        .kuc_expect("context menu item must carry typed action");

    assert_eq!(UI_TASK_SET_STATE_ACTION_ID, action.action_id);
    assert!(action.payload.is_empty());
    let UiHostActionPayload::TaskControlState(payload) = &action.typed_payload else {
        assert!(
            matches!(
                action.typed_payload,
                UiHostActionPayload::TaskControlState(_)
            ),
            "expected typed task state payload: {:?}",
            action.typed_payload
        );
        return;
    };
    assert_eq!("list", payload.node_id);
    assert_eq!(0, payload.row_index);
    assert_eq!("ui-task-state:list:0", payload.state_id);
    assert_eq!("[x]", payload.marker);
}

#[test]
fn list_node_treats_uppercase_done_marker_as_task_checkbox() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("[X] done");

    let ui_node = factory.viewer_node(&node);

    let checkbox = &ui_node.children()[0].children()[0];
    assert_eq!("[x]", checkbox.props().interaction.value);
    assert!(
        checkbox
            .props()
            .style_classes
            .contains(&"kdv-task-done".to_string())
    );
    assert_eq!(4, checkbox.props().context_menu.items.len());
}

#[test]
fn list_node_uses_task_state_override_for_checkbox() {
    let mut overrides = BTreeMap::new();
    overrides.insert("ui-task-state:list:0".to_string(), ViewerTaskState::Blocked);
    let factory = KucNodeFactory::new(&[], 120).task_state_overrides(&overrides);
    let node = viewer_node("[ ] todo");

    let ui_node = factory.viewer_node(&node);

    let checkbox = &ui_node.children()[0].children()[0];
    assert_eq!("[-]", checkbox.props().interaction.value);
    assert!(
        checkbox
            .props()
            .style_classes
            .contains(&"kdv-task-blocked".to_string())
    );
}

#[test]
fn list_node_treats_ordered_task_markers_as_task_checkboxes() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("1.\t[/] doing\n2)  [-] hold");

    let ui_node = factory.viewer_node(&node);

    let progress_checkbox = &ui_node.children()[0].children()[0];
    let blocked_checkbox = &ui_node.children()[1].children()[0];
    assert_eq!("[/]", progress_checkbox.props().interaction.value);
    assert_eq!("[-]", blocked_checkbox.props().interaction.value);
    assert_eq!("doing", ui_node.children()[0].children()[1].props().label);
    assert_eq!("hold", ui_node.children()[1].children()[1].props().label);
}

#[test]
fn list_node_uses_kuc_row_height_for_task_checkbox() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node("[x] done\n[/] doing");
    node.rect.height = 72.0;

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiDimension::Px(72), ui_node.props().common.height);
    let first_checkbox = &ui_node.children()[0].children()[0];
    let second_checkbox = &ui_node.children()[1].children()[0];
    assert_eq!(
        UiDimension::Px(23),
        first_checkbox.props().common.height,
        "{:#?}",
        first_checkbox.props().common
    );
    assert_eq!(
        UiDimension::Px(23),
        second_checkbox.props().common.height,
        "{:#?}",
        second_checkbox.props().common
    );
}

#[test]
fn list_node_preserves_nested_depth_as_row_common_margin() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("- parent\n  - child\n    [/] doing");

    let ui_node = factory.viewer_node(&node);

    assert_margin_left_px(&ui_node.children()[0], 0);
    assert_margin_left_px(&ui_node.children()[1], 40);
    assert_margin_left_px(&ui_node.children()[2], 80);
    assert_no_kdv_list_class(&ui_node);
}

#[test]
fn list_node_body_hits_keep_viewer_semantic_node_id() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("- parent\n  - Nested item 2-1");
    let ui_node = factory.viewer_node(&node);

    let hits = UiTreeSurfaceHost::new(ThemeSnapshot::light()).document_node_hits(
        &ui_node,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert!(
        hits.iter().any(|hit| {
            hit.node_id.as_str() != "list"
                && hit
                    .semantic_node_id
                    .as_ref()
                    .is_some_and(|node_id| node_id.as_str() == "list")
        }),
        "list body visual hits must keep the source viewer node id: {hits:#?}"
    );
}

#[test]
fn unordered_list_marker_uses_bullet_canvas_marker_without_dash_label() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("- item");

    let ui_node = factory.viewer_node(&node);

    let marker = &ui_node.children()[0].children()[0];
    assert_eq!("  ", marker.props().label);
    assert_margin_left_px(marker, 0);
    assert!(
        !marker
            .props()
            .style_classes
            .iter()
            .any(|style_class| style_class.starts_with("kdv-list")),
        "{:#?}",
        marker.props().style_classes
    );
}

#[test]
fn list_node_renders_marker_center_aligned_with_body_text() {
    let theme = ThemeSnapshot::light();
    let text_color = theme_rgb(&theme, "text");
    let background = theme_rgb(&theme, "background");
    let mut canvas = Canvas::new(240, 60, background);
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node("- item");
    let ui_node = factory.viewer_node(&node);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &ui_node,
        UiTreeRenderArea {
            x: 16,
            y: 0,
            width: 200,
            height: 60,
            scroll_y: 0.0,
        },
    );

    let marker_bounds = vertical_bounds_for_color_in_x_range(&canvas, text_color, 24, 38)
        .kuc_expect("KDV list marker should render through KUC canvas primitive");
    let body_bounds = vertical_bounds_for_color_in_x_range(&canvas, text_color, 52, 160)
        .kuc_expect("KDV list body should render through KUC text node");

    assert!(
        marker_bounds
            .center_twice()
            .abs_diff(body_bounds.center_twice())
            <= 4,
        "KDV list marker must align with body text ink center: marker={marker_bounds:?}, body={body_bounds:?}"
    );
}

#[test]
fn list_node_preserves_link_spans_as_kuc_host_targets() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node("- Normal link");
    node.spans = vec![
        ViewerTextSpan::plain("- "),
        ViewerTextSpan::linked(
            "Normal link",
            "https://github.com",
            ViewerTextStyle::default().link(),
        ),
    ];

    let ui_node = factory.viewer_node(&node);

    let body = &ui_node.children()[0].children()[1];
    assert_eq!("list", body.id().as_str());
    assert_eq!(UiCursor::Pointer, body.props().common.cursor);
    assert_eq!("https://github.com", body.props().text.spans[0].link_target);
}
