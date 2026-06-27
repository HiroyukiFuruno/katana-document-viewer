use crate::catalog::StorybookFixture;
use crate::preview::PreviewBuilder;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::{UiNode, UiTextWrapMode, UiTone};
use std::path::PathBuf;

const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 11800.0;

#[test]
fn katana_alert_scene_keeps_title_body_and_kind_contract() -> Result<(), Box<dyn std::error::Error>>
{
    let fixture = StorybookFixture {
        label: "katana/sample_basic.md".to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/fixtures/katana/sample_basic.md"),
    };
    let scene = PreviewBuilder::default().build(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;

    let alerts = collect_alert_nodes(scene.tree.root());
    assert!(alerts.len() >= 5, "sample_basic alert nodes missing");
    for expected in AlertContract::all() {
        let alert = alerts
            .iter()
            .find(|node| node.props().common.border.color_token == expected.color_token)
            .ok_or_else(|| format!("missing alert kind: {}", expected.title))?;
        assert_alert_contract(alert, *expected);
    }
    Ok(())
}

fn assert_alert_contract(alert: &UiNode, expected: AlertContract) {
    let props = alert.props();
    let lines = props.label.lines().collect::<Vec<_>>();
    assert_eq!(Some(expected.title), lines.first().copied());
    assert!(
        lines.get(1).is_some_and(|line| !line.trim().is_empty()),
        "{} alert body must stay separate from the title",
        expected.title
    );
    assert!(
        !props.label.contains("[!") && !props.label.starts_with(expected.raw_prefix),
        "{} alert must not leak raw GFM marker or raw prefix: {}",
        expected.title,
        props.label
    );
    assert_eq!(UiTextWrapMode::Wrap, props.text.wrap);
    assert_eq!("alert", props.text.role);
    assert_eq!(expected.tone, props.severity);
    assert!(props.common.border.visible);
    assert_eq!(4, props.common.border.width_px);
    assert_eq!(expected.color_token, props.common.border.color_token);
}

fn collect_alert_nodes(node: &UiNode) -> Vec<&UiNode> {
    let mut alerts = Vec::new();
    collect_alert_nodes_into(node, &mut alerts);
    alerts
}

fn collect_alert_nodes_into<'a>(node: &'a UiNode, alerts: &mut Vec<&'a UiNode>) {
    if node.props().text.role == "alert" {
        alerts.push(node);
    }
    for child in node.children() {
        collect_alert_nodes_into(child, alerts);
    }
}

#[derive(Clone, Copy)]
struct AlertContract {
    title: &'static str,
    raw_prefix: &'static str,
    color_token: &'static str,
    tone: UiTone,
}

impl AlertContract {
    fn all() -> &'static [Self] {
        &[
            Self {
                title: "Note",
                raw_prefix: "NOTE:",
                color_token: "alert-note",
                tone: UiTone::Accent,
            },
            Self {
                title: "Tip",
                raw_prefix: "TIP:",
                color_token: "alert-tip",
                tone: UiTone::Success,
            },
            Self {
                title: "Important",
                raw_prefix: "IMPORTANT:",
                color_token: "alert-important",
                tone: UiTone::Accent,
            },
            Self {
                title: "Warning",
                raw_prefix: "WARNING:",
                color_token: "alert-warning",
                tone: UiTone::Warning,
            },
            Self {
                title: "Caution",
                raw_prefix: "CAUTION:",
                color_token: "alert-caution",
                tone: UiTone::Danger,
            },
        ]
    }
}
