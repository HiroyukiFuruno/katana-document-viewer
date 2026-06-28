use super::media_control_tests_support as support;
use crate::document_viewer::media_control_icons::KucMediaControlIconSet;
use katana_document_viewer::{ViewerDiagramKind, ViewerNodeKind};
use katana_ui_core::render_model::{UiIconProps, UiSvgPaintPolicy};
use support::{diagram_controls_factory, viewer_node};

#[test]
fn diagram_controls_use_katana_icon_preset_by_default() {
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
    let icon = &pan_up.props().icon;

    assert_eq!("surface.pan-up", icon.role);
    assert_eq!("0 0 16 16", icon.view_box);
    assert_eq!(UiSvgPaintPolicy::CurrentColor, icon.paint_policy);
    assert_eq!("katana.view.pan_up", icon.path_summary);
    assert!(
        icon.svg_source
            .contains(r#"polyline points="4 10 8 4 12 10""#),
        "diagram controls must default to the KatanA icon pack preset"
    );
}

#[test]
fn every_diagram_control_uses_katana_icon_asset_source() {
    let icons = KucMediaControlIconSet::katana_default();
    for (command, view_box, summary, source_fragment) in [
        (
            "close-modal",
            "0 0 16 16",
            "katana.ui.close_modal",
            r#"line x1="3" y1="3" x2="13" y2="13""#,
        ),
        (
            "fullscreen",
            "0 0 16 16",
            "katana.view.fullscreen",
            r#"polyline points="2 6 2 2 6 2""#,
        ),
        (
            "pan-up",
            "0 0 16 16",
            "katana.view.pan_up",
            r#"polyline points="4 10 8 4 12 10""#,
        ),
        (
            "zoom-in",
            "0 0 16 16",
            "katana.view.zoom_in",
            r#"line x1="7" y1="5" x2="7" y2="9""#,
        ),
        (
            "pan-left",
            "0 0 16 16",
            "katana.view.pan_left",
            r#"polyline points="10 4 4 8 10 12""#,
        ),
        (
            "reset-view",
            "0 -960 960 960",
            "katana.view.reset_view",
            "M480-80",
        ),
        (
            "pan-right",
            "0 0 16 16",
            "katana.view.pan_right",
            r#"polyline points="6 4 12 8 6 12""#,
        ),
        (
            "trackpad-help",
            "0 0 16 16",
            "katana.status.info",
            r#"circle cx="8" cy="8" r="6""#,
        ),
        (
            "pan-down",
            "0 0 16 16",
            "katana.view.pan_down",
            r#"polyline points="4 6 8 12 12 6""#,
        ),
        (
            "zoom-out",
            "0 0 16 16",
            "katana.view.zoom_out",
            r#"line x1="5" y1="7" x2="9" y2="7""#,
        ),
    ] {
        let icon = icons.icon_for(command, "");
        assert_eq!(view_box, icon.view_box, "{command}");
        assert_eq!(summary, icon.path_summary, "{command}");
        assert!(
            icon.svg_source.contains(source_fragment),
            "{command} must use KatanA icon pack source"
        );
    }
}

#[test]
fn diagram_control_icons_can_be_overridden_from_host_config() {
    let icon = UiIconProps::new("<svg viewBox=\"0 0 24 24\"><path d=\"M1 2\"/></svg>")
        .role("surface.pan-up")
        .view_box("0 0 24 24")
        .path_summary("external-pan-up")
        .paint_policy(UiSvgPaintPolicy::FillOnly)
        .color_token("accent")
        .theme_token("accent");
    let icons = KucMediaControlIconSet::katana_default().with_icon("pan-up", icon.clone());
    let factory = diagram_controls_factory().media_control_icons(&icons);
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD",
    );
    let ui_node = factory.viewer_node(&node);
    let grid = &ui_node.children()[1];
    let pan_up = &grid.children()[0].children()[2];

    assert_eq!(icon, pan_up.props().icon);
}
