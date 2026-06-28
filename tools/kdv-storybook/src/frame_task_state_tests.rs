use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::frame_pixel_guard::StorybookFramePixelGuard;
use crate::preview::PreviewBuilder;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use std::collections::BTreeSet;
use std::path::PathBuf;

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 12000;
const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 11800.0;

#[test]
fn katana_task_checkbox_states_reach_scene_and_frame() -> Result<(), Box<dyn std::error::Error>> {
    let rendered = render_fixture("katana/sample_basic.md")?;
    let states = task_checkbox_states(rendered.scene.tree.root());

    assert!(states.contains("[ ]"), "missing empty task checkbox");
    assert!(states.contains("[x]"), "missing done task checkbox");
    assert!(states.contains("[/]"), "missing progress task checkbox");
    assert!(states.contains("[-]"), "missing blocked task checkbox");
    assert!(task_style_exists(
        rendered.scene.tree.root(),
        "kdv-task-empty"
    ));
    assert!(task_style_exists(
        rendered.scene.tree.root(),
        "kdv-task-done"
    ));
    assert!(task_style_exists(
        rendered.scene.tree.root(),
        "kdv-task-progress"
    ));
    assert!(task_style_exists(
        rendered.scene.tree.root(),
        "kdv-task-blocked"
    ));
    assert!(StorybookFramePixelGuard::preview_content_pixel_count(&rendered.canvas, true) > 1024);
    Ok(())
}

fn render_fixture(path: &str) -> Result<RenderedFixture, Box<dyn std::error::Error>> {
    let fixture = fixture(path);
    let scene = PreviewBuilder::default().build(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;
    let canvas = StorybookFrameRenderer::render(FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: &[fixture],
        selected_index: 0,
        scene: Some(&scene),
        scroll_y: 0.0,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &Default::default(),
        dark: true,
        interaction: &ViewerInteractionConfig::default(),
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 0,
    });
    Ok(RenderedFixture { canvas, scene })
}

fn task_checkbox_states(node: &UiNode) -> BTreeSet<&str> {
    let mut states = BTreeSet::new();
    collect_task_checkbox_states(node, &mut states);
    states
}

fn collect_task_checkbox_states<'a>(node: &'a UiNode, states: &mut BTreeSet<&'a str>) {
    if node.kind() == UiNodeKind::Checkbox && has_style(node, "kdv-task-checkbox") {
        states.insert(node.props().interaction.value.as_str());
    }
    for child in node.children() {
        collect_task_checkbox_states(child, states);
    }
}

fn task_style_exists(node: &UiNode, style: &str) -> bool {
    has_style(node, style)
        || node
            .children()
            .iter()
            .any(|child| task_style_exists(child, style))
}

fn has_style(node: &UiNode, style: &str) -> bool {
    node.props()
        .style_classes
        .iter()
        .any(|value| value == style)
}

fn fixture(path: &str) -> StorybookFixture {
    StorybookFixture {
        label: path.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{path}")),
    }
}

struct RenderedFixture {
    canvas: Canvas,
    scene: crate::preview::PreviewScene,
}
