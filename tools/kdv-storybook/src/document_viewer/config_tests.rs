use super::{KucViewerConfig, media_control_icons::KucMediaControlIconSet};
use katana_document_viewer::{ViewerInteractionConfig, ViewerTaskState, ViewerViewport};
use katana_ui_core::render_model::UiIconProps;
use std::collections::BTreeMap;

#[test]
fn interaction_changes_render_config() {
    let expected = ViewerInteractionConfig {
        hover_highlight_enabled: true,
        selection_enabled: false,
        image_controls_enabled: true,
        diagram_controls_enabled: false,
        code_controls_enabled: true,
    };

    let config = KucViewerConfig::new(
        "viewer",
        ViewerViewport {
            width: 640.0,
            height: 480.0,
        },
    )
    .interaction(expected.clone());

    assert_eq!(expected, config.interaction);
}

#[test]
fn diagram_viewports_assigns_map() {
    let mut diagram_viewports = BTreeMap::new();
    diagram_viewports.insert("node-1".to_string(), Default::default());

    let config = KucViewerConfig::new(
        "viewer",
        ViewerViewport {
            width: 640.0,
            height: 480.0,
        },
    )
    .diagram_viewports(diagram_viewports.clone());

    assert_eq!(diagram_viewports, config.diagram_viewports);
}

#[test]
fn task_state_overrides_assigns_map() {
    let mut task_state_overrides = BTreeMap::new();
    task_state_overrides.insert("ui-task-state:list:0".to_string(), ViewerTaskState::Done);

    let config = KucViewerConfig::new(
        "viewer",
        ViewerViewport {
            width: 640.0,
            height: 480.0,
        },
    )
    .task_state_overrides(task_state_overrides.clone());

    assert_eq!(task_state_overrides, config.task_state_overrides);
}

#[test]
fn media_control_icons_defaults_to_katana_preset_and_accepts_override() {
    let default_config = KucViewerConfig::new(
        "viewer",
        ViewerViewport {
            width: 640.0,
            height: 480.0,
        },
    );
    assert!(
        default_config
            .media_control_icons
            .icon_for("pan-up", "")
            .svg_source
            .contains(r#"polyline points="4 10 8 4 12 10""#)
    );

    let icon = UiIconProps::new("<svg><path d=\"M1 1\"/></svg>")
        .role("surface.pan-up")
        .view_box("0 0 24 24");
    let config = KucViewerConfig::new(
        "viewer",
        ViewerViewport {
            width: 640.0,
            height: 480.0,
        },
    )
    .media_control_icons(
        KucMediaControlIconSet::katana_default().with_icon("pan-up", icon.clone()),
    );

    assert_eq!(
        icon,
        config.media_control_icons.icon_for("pan-up", "fallback")
    );
}
