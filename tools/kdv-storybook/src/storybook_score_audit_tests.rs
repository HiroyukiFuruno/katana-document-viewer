use super::frame::FrameRenderRequest;
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::frame::StorybookFrameRenderer;
use crate::layout::{
    HEADER_HEIGHT, PREVIEW_CONTENT_INSET, SIDEBAR_WIDTH as SIDEBAR_WIDTH_LAYOUT,
    preview_content_width,
};
use crate::media_host_action::StorybookMediaHostAction;
use crate::mouse::StorybookHostActionHits;
use crate::palette::StorybookPalette;
use crate::preview::PreviewBuilder as SceneBuilder;
use crate::preview::PreviewScene;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::UiNode;
use katana_ui_core_storybook::UiTreeHostActionHit;
use std::path::PathBuf;

const FRAME_WIDTH: usize = 1_280;
const FRAME_HEIGHT: usize = 900;
const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 600.0;
const MIN_SIDEBAR_SIGNAL_PIXELS: usize = 300;
const MEDIA_CONTROL_ACTIONS: [&str; 2] = ["fullscreen", "open"];
const RAW_FIXTURE: &str = "katana/sample_basic.md";
const MEDIA_FIXTURE: &str = "direct/sample.md";
const PENDING_FIXTURE: &str = "katana/sample_diagrams.md";

#[test]
fn storybook_score_fails_when_sidebar_or_controls_broken() -> Result<(), Box<dyn std::error::Error>>
{
    let scene = render_scene(MEDIA_FIXTURE)?;
    let canvas = render_frame(MEDIA_FIXTURE, &scene);
    let palette = StorybookPalette::new(true);
    let sidebar_pixels = visible_sidebar_pixels(&canvas, palette);
    assert!(
        sidebar_pixels >= MIN_SIDEBAR_SIGNAL_PIXELS,
        "sidebar has too little visible pixels: {sidebar_pixels}"
    );

    let mut expected_controls = 0usize;
    for action in MEDIA_CONTROL_ACTIONS {
        if action_count(scene.tree.root(), action) > 0 {
            expected_controls += 1;
            let hit = frame_action_hit(&scene, action)?;
            let (x, y) = frame_action_center(&hit);
            assert!(
                rendered_around_point(&canvas, x as i32, y as i32, 8, palette.preview_background()),
                "media control action `{action}` has no rendered pixels near point ({x}, {y})"
            );
        }
    }
    assert!(
        expected_controls > 0,
        "no media controls found in Storybook scene"
    );
    Ok(())
}

fn frame_action_hit(
    scene: &PreviewScene,
    action: &str,
) -> Result<UiTreeHostActionHit, std::io::Error> {
    StorybookHostActionHits::hits_for_preview_width(scene, preview_content_width(FRAME_WIDTH))
        .into_iter()
        .find(|hit| {
            StorybookMediaHostAction::from_host_action_plan(&hit.action)
                .is_some_and(|media_action| media_action.into_viewer_action().command == action)
        })
        .ok_or_else(|| std::io::Error::other(format!("missing media control action: {action}")))
}

fn frame_action_center(hit: &UiTreeHostActionHit) -> (usize, usize) {
    (
        SIDEBAR_WIDTH_LAYOUT
            + PREVIEW_CONTENT_INSET
            + hit.rect.x
            + hit.rect.width.saturating_div(2),
        HEADER_HEIGHT + PREVIEW_CONTENT_INSET + hit.rect.y + hit.rect.height.saturating_div(2),
    )
}

#[test]
fn storybook_score_audit_rejects_raw_markers() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(RAW_FIXTURE)?;
    let text = visible_text(scene.tree.root());

    for marker in raw_markers() {
        assert!(
            !text.contains(marker),
            "raw marker `{marker}` leaked into KUC visible text"
        );
    }
    Ok(())
}

#[test]
fn storybook_score_audit_rejects_persistent_pending() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = storybook_fixture(PENDING_FIXTURE);
    let lazy = SceneBuilder::default().build_lazy_with_mode_and_search(
        &fixture,
        viewport(),
        true,
        ViewerInteractionConfig::default(),
        katana_document_viewer::ViewerMode::Document,
        katana_document_viewer::ViewerSearchState::default(),
    )?;
    assert!(
        lazy.asset_request_count >= 1,
        "lazy build should expose pending assets for `katana/sample_diagrams.md`"
    );
    let loaded = SceneBuilder::default().build(
        &fixture,
        viewport(),
        true,
        ViewerInteractionConfig::default(),
    )?;
    assert_eq!(
        0, loaded.asset_request_count,
        "loaded scene must not keep visible-diagram assets pending"
    );
    assert!(
        loaded.failed_asset_count <= loaded.loaded_asset_count,
        "loaded scene should not report more failures than loaded artifacts"
    );
    Ok(())
}

fn render_scene(path: &str) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    SceneBuilder::default().build(
        &storybook_fixture(path),
        viewport(),
        true,
        ViewerInteractionConfig::default(),
    )
}

fn render_frame(path: &str, scene: &PreviewScene) -> Canvas {
    StorybookFrameRenderer::render(FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: &[storybook_fixture(path)],
        selected_index: 0,
        scene: Some(scene),
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
    })
}

fn visible_sidebar_pixels(canvas: &Canvas, palette: StorybookPalette) -> usize {
    let bg = palette.sidebar_background();
    let mut count = 0usize;

    for y in 0..canvas.height() {
        let row = y.saturating_mul(canvas.width());
        for x in 0..SIDEBAR_WIDTH_LAYOUT {
            if x < SIDEBAR_WIDTH_LAYOUT && canvas.pixels()[row + x] != bg {
                count += 1;
            }
        }
    }
    count
}

fn rendered_around_point(
    canvas: &Canvas,
    center_x: i32,
    center_y: i32,
    radius: i32,
    background: u32,
) -> bool {
    let min_x = (center_x - radius).max(0);
    let max_x = (center_x + radius).min(canvas.width() as i32 - 1);
    let min_y = (center_y - radius).max(0);
    let max_y = (center_y + radius).min(canvas.height() as i32 - 1);

    for y in min_y..=max_y {
        let row = (y as usize).saturating_mul(canvas.width());
        for x in min_x..=max_x {
            let pixel = canvas.pixels()[row + x as usize];
            if pixel != background {
                return true;
            }
        }
    }
    false
}

fn visible_text(node: &UiNode) -> String {
    let mut text = String::new();
    push_visible_text(&mut text, node);
    text
}

fn push_visible_text(text: &mut String, node: &UiNode) {
    if !node.props().label.is_empty() {
        text.push_str(&node.props().label);
        text.push('\n');
    }
    for span in &node.props().text.spans {
        text.push_str(&span.text);
        text.push('\n');
    }
    for child in node.children() {
        push_visible_text(text, child);
    }
}

fn raw_markers() -> &'static [&'static str] {
    &[
        "```",
        "| --- |",
        "| :--- |",
        "| :---: |",
        "[!NOTE]",
        "[!TIP]",
        "[!IMPORTANT]",
        "[!WARNING]",
        "[!CAUTION]",
        "<details",
        "</details>",
        "<summary",
        "</summary>",
    ]
}

fn build_scene(path: &str) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    SceneBuilder::default().build(
        &storybook_fixture(path),
        viewport(),
        true,
        ViewerInteractionConfig::default(),
    )
}

fn storybook_fixture(path: &str) -> StorybookFixture {
    StorybookFixture {
        label: path.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{path}")),
    }
}

fn viewport() -> ViewerViewport {
    ViewerViewport {
        width: PREVIEW_WIDTH,
        height: PREVIEW_HEIGHT,
    }
}

fn action_count(node: &UiNode, action: &str) -> usize {
    usize::from(node.props().interaction.value == action)
        + node
            .children()
            .iter()
            .map(|child| action_count(child, action))
            .sum::<usize>()
}
