use super::media_control_tests_support as support;
use crate::document_viewer::KucDiagramControlResolver;
use crate::test_assert::KucTestExpect;
use katana_document_viewer::{
    DiagramViewportState, ViewerDiagramKind, ViewerMediaControlAction, ViewerMediaControlKind,
    ViewerNodeKind,
};
use katana_ui_core::render_model::{UiCursor, UiDimension, UiNodeId};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{UiTreeHitRect, UiTreeNodeHit, UiTreeRenderArea, UiTreeSurfaceHost};
use std::collections::BTreeMap;
use support::{
    assert_absolute_overlay, assert_diagram_fullscreen_button, assert_diagram_gap,
    assert_diagram_grid_buttons, assert_diagram_spacer, assert_image_button,
    assert_no_kdv_media_style_classes, collect_host_actions, diagram_controls_factory,
    image_controls_factory, math_controls_factory, viewer_node,
};

#[test]
fn diagram_controls_use_katana_controller_frame_without_style_contract() {
    let factory = diagram_controls_factory();
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD",
    );

    let ui_node = factory.viewer_node(&node);
    assert_no_kdv_media_style_classes(&ui_node);

    let frame = &ui_node;
    assert_eq!(
        katana_ui_core::render_model::UiVisualRole::MediaFrame,
        frame.props().visual_role
    );
    let grid = &frame.children()[1];
    let top_controls = &frame.children()[2];

    assert_diagram_grid_buttons(grid);
    assert_absolute_overlay(grid, 0, 8, 8, 0);
    assert_absolute_overlay(top_controls, 8, 8, 0, 0);
    assert_eq!(1, top_controls.children().len());
    assert_diagram_fullscreen_button(&top_controls.children()[0]);
    assert_diagram_spacer(&grid.children()[0].children()[0]);
    assert_diagram_gap(&grid.children()[0].children()[1]);
}

#[test]
fn diagram_internal_control_resolves_to_kuc_owned_typed_action() {
    let factory = diagram_controls_factory();
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD",
    );
    let ui_node = factory.viewer_node(&node);
    let grid = &ui_node.children()[1];
    let pan_up = &grid.children()[0].children()[2];
    assert!(
        !pan_up
            .props()
            .state_id
            .as_str()
            .starts_with("viewer-media-control:"),
        "internal diagram controls must not depend on host control state_id parsing"
    );

    let action = KucDiagramControlResolver::internal_action_for_node(
        &ui_node,
        &UiNodeId::new(pan_up.id().as_str()),
    )
    .kuc_expect("pan-up must resolve through KUC internal control contract");

    assert_eq!(ViewerMediaControlKind::Diagram, action.kind);
    assert_eq!("node", action.node_id);
    assert_eq!("pan-up", action.command);
}

#[test]
fn diagram_internal_control_hover_resolves_kuc_control_node_id() {
    let factory = diagram_controls_factory();
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD",
    );
    let ui_node = factory.viewer_node(&node);
    let grid = &ui_node.children()[1];
    let pan_up = &grid.children()[0].children()[2];
    let pan_up_id = UiNodeId::new(pan_up.id().as_str());
    let hits = vec![
        UiTreeNodeHit {
            node_id: UiNodeId::new("media-frame"),
            semantic_node_id: None,
            rect: UiTreeHitRect {
                x: 0,
                y: 0,
                width: 80,
                height: 80,
            },
            cursor: UiCursor::Default,
        },
        UiTreeNodeHit {
            node_id: pan_up_id.clone(),
            semantic_node_id: Some(UiNodeId::new("viewer-diagram-node")),
            rect: UiTreeHitRect {
                x: 10,
                y: 10,
                width: 18,
                height: 18,
            },
            cursor: UiCursor::Pointer,
        },
    ];

    assert_eq!(
        Some(pan_up_id),
        KucDiagramControlResolver::internal_control_node_id_at(&ui_node, &hits, 12.0, 12.0)
    );
}

#[test]
fn diagram_fullscreen_control_stays_host_propagated_only() {
    let factory = diagram_controls_factory();
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD",
    );
    let ui_node = factory.viewer_node(&node);
    let top_controls = &ui_node.children()[2];
    let fullscreen = &top_controls.children()[0];

    let action = KucDiagramControlResolver::internal_action_for_node(
        &ui_node,
        &UiNodeId::new(fullscreen.id().as_str()),
    );

    assert!(action.is_none());
}

#[test]
fn fullscreen_diagram_top_control_uses_katana_close_modal_contract() {
    let mut viewports = BTreeMap::new();
    viewports.insert(
        "node".to_string(),
        DiagramViewportState {
            fullscreen_open: true,
            ..DiagramViewportState::default()
        },
    );
    let factory = diagram_controls_factory().diagram_viewports(&viewports);
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD",
    );
    let ui_node = factory.viewer_node(&node);
    let top_controls = &ui_node.children()[3];
    let close = &top_controls.children()[0];

    assert_absolute_overlay(top_controls, 20, 20, 0, 0);
    assert_eq!("fullscreen", close.props().interaction.value);
    assert_eq!(UiDimension::Px(32), close.props().common.width);
    assert_eq!(UiDimension::Px(32), close.props().common.height);
    assert_eq!("surface.close-modal", close.props().icon.role);
    assert_eq!("katana.ui.close_modal", close.props().icon.path_summary);
    assert!(
        close.props().icon.svg_source.contains(r#"x1="3" y1="3""#),
        "fullscreen close control must use KatanA close modal SVG source"
    );
    let action = KucDiagramControlResolver::internal_action_for_node(
        &ui_node,
        &UiNodeId::new(close.id().as_str()),
    );
    assert!(
        action.is_none(),
        "fullscreen close remains a host-propagated diagram fullscreen toggle"
    );
}

#[test]
fn fullscreen_diagram_backdrop_exposes_full_viewport_close_action() {
    let mut viewports = BTreeMap::new();
    viewports.insert(
        "node".to_string(),
        DiagramViewportState {
            fullscreen_open: true,
            ..DiagramViewportState::default()
        },
    );
    let factory = diagram_controls_factory()
        .diagram_viewports(&viewports)
        .fullscreen_viewport_size(640, 360);
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD",
    );
    let ui_node = factory.viewer_node(&node);
    let action_id =
        ViewerMediaControlAction::host_action_id_for(ViewerMediaControlKind::Diagram, "fullscreen");

    let hits = UiTreeSurfaceHost::new(ThemeSnapshot::dark()).host_action_hits(
        &ui_node,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 640,
            height: 360,
            scroll_y: 0.0,
        },
    );
    let fullscreen_hits = hits
        .iter()
        .filter(|hit| hit.action.action_id == action_id)
        .collect::<Vec<_>>();

    assert_eq!(
        2,
        fullscreen_hits.len(),
        "fullscreen must expose both KatanA backdrop close and top-right close button"
    );
    let backdrop = fullscreen_hits
        .iter()
        .max_by_key(|hit| hit.rect.area())
        .kuc_expect("fullscreen backdrop hit");
    assert_eq!(0, backdrop.rect.x);
    assert_eq!(0, backdrop.rect.y);
    assert_eq!(640, backdrop.rect.width);
    assert_eq!(360, backdrop.rect.height);
    let close = fullscreen_hits
        .iter()
        .min_by_key(|hit| hit.rect.area())
        .kuc_expect("fullscreen close hit");
    assert_eq!(32, close.rect.width);
    assert_eq!(32, close.rect.height);
    assert_eq!(588, close.rect.x);
    assert_eq!(20, close.rect.y);
}

#[test]
fn image_controls_scope_each_button_to_viewer_node_action() {
    let factory = image_controls_factory();
    let node = viewer_node(ViewerNodeKind::Image, "image");

    let ui_node = factory.viewer_node(&node);
    assert_no_kdv_media_style_classes(&ui_node);
    assert_eq!(
        katana_ui_core::render_model::UiVisualRole::MediaFrame,
        ui_node.props().visual_role
    );
    let grid = &ui_node.children()[1];
    let top_controls = &ui_node.children()[2];

    assert_absolute_overlay(grid, 0, 8, 8, 0);
    assert_absolute_overlay(top_controls, 8, 8, 0, 0);
    assert_image_button(&top_controls.children()[0], "open");
    assert_image_button(&top_controls.children()[2], "copy");
    assert_image_button(&top_controls.children()[4], "reveal-in-os");
    assert_image_button(&grid.children()[0].children()[2], "zoom-in");
    assert_image_button(&grid.children()[1].children()[2], "fit");
    assert_image_button(&grid.children()[2].children()[2], "zoom-out");
}

#[test]
fn math_node_has_no_diagram_or_image_viewer_control_action() {
    let factory = math_controls_factory();
    let node = viewer_node(ViewerNodeKind::Math, "math");

    let ui_node = factory.viewer_node(&node);
    let mut actions = Vec::new();
    collect_host_actions(&ui_node, &mut actions);

    assert!(
        !actions
            .iter()
            .filter_map(|action| ViewerMediaControlAction::from_host_action(action, "node"))
            .any(|action| matches!(
                action.kind,
                ViewerMediaControlKind::Diagram | ViewerMediaControlKind::Image
            ))
    );
}
