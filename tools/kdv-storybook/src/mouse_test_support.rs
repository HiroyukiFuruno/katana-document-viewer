use super::mouse_host_action::{StorybookHostActionHits, StorybookHostActionRouter};
use super::mouse_media::StorybookMediaMouse;
use super::{StorybookMouseButton, StorybookPointer};
use crate::layout::{StorybookPreviewArea, preview_content_width};
use crate::preview::PreviewScene;
use crate::preview_interaction_command_support::build_scene;
use katana_document_viewer::ViewerInteractionConfig;
use katana_ui_core::render_model::UiTextSpanAction;
use katana_ui_core_storybook::UiTreeHostActionHit;
use std::sync::OnceLock;

pub(crate) const WINDOW_WIDTH: usize = 1_000;
pub(crate) const WINDOW_HEIGHT: usize = 900;

static SAMPLE_BASIC_SCENE: OnceLock<Result<PreviewScene, String>> = OnceLock::new();
static SAMPLE_DIAGRAM_CONTROLS_SCENE: OnceLock<Result<PreviewScene, String>> = OnceLock::new();
static DIRECT_IMAGE_CONTROLS_SCENE: OnceLock<Result<PreviewScene, String>> = OnceLock::new();

pub(crate) fn sample_basic_scene() -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cached_scene(
        &SAMPLE_BASIC_SCENE,
        "katana/sample_basic.md",
        ViewerInteractionConfig::default(),
    )
}

pub(crate) fn sample_diagram_controls_scene() -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cached_scene(
        &SAMPLE_DIAGRAM_CONTROLS_SCENE,
        "katana/sample_diagrams.md",
        ViewerInteractionConfig {
            diagram_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )
}

pub(crate) fn direct_image_controls_scene() -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cached_scene(
        &DIRECT_IMAGE_CONTROLS_SCENE,
        "direct/kdv-icon.png",
        ViewerInteractionConfig {
            image_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )
}

pub(crate) fn pointer_for_task(
    scene: &PreviewScene,
    marker: &str,
) -> Result<PointerHit, Box<dyn std::error::Error>> {
    pointer_for_host_action(scene, |hit| {
        hit.action
            .task_control_action_from_root(scene.tree.root())
            .is_some_and(|action| action.current_marker.marker() == marker)
    })
}

pub(crate) fn pointer_for_link(
    scene: &PreviewScene,
    label: &str,
    uri: &str,
) -> Result<PointerHit, Box<dyn std::error::Error>> {
    pointer_for_host_action(scene, |hit| {
        let Some(UiTextSpanAction::OpenLink { target }) = hit.action.text_span_action() else {
            return false;
        };
        hit.action.label == label && target == uri
    })
}

pub(crate) fn pointer_for_media_action(
    scene: &PreviewScene,
    action: &str,
) -> Result<PointerHit, Box<dyn std::error::Error>> {
    let preview_width = preview_content_width(WINDOW_WIDTH);
    let (document_x, document_y) =
        StorybookMediaMouse::test_point_for_action(scene, action, preview_width)
            .ok_or_else(|| std::io::Error::other("missing media hit"))?;
    Ok(pointer_for_document_point(document_x, document_y))
}

pub(crate) fn pointer_for_internal_diagram_action(
    scene: &PreviewScene,
    action: &str,
) -> Result<PointerHit, Box<dyn std::error::Error>> {
    let preview_width = preview_content_width(WINDOW_WIDTH);
    let router = StorybookHostActionRouter::for_preview_width(scene, preview_width);
    let (document_x, document_y) = router
        .internal_diagram_point_for_action(action)
        .ok_or_else(|| std::io::Error::other("missing internal diagram hit"))?;
    Ok(pointer_for_document_point(document_x, document_y))
}

pub(crate) fn pointer_for_accordion(
    scene: &PreviewScene,
) -> Result<PointerHit, Box<dyn std::error::Error>> {
    pointer_for_host_action(scene, |hit| {
        matches!(
            hit.action.text_span_action(),
            Some(UiTextSpanAction::ToggleAccordion { .. })
        )
    })
}

pub(crate) struct PointerHit {
    pub(crate) pointer: StorybookPointer,
    pub(crate) scroll_y: f32,
}

fn pointer_for_document_point(document_x: f32, document_y: f32) -> PointerHit {
    let scroll_y = (document_y - 120.0).max(0.0);
    let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
    let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
    PointerHit {
        pointer: StorybookPointer::new(x, y, StorybookMouseButton::Left),
        scroll_y,
    }
}

fn cached_scene(
    cache: &'static OnceLock<Result<PreviewScene, String>>,
    path: &'static str,
    interaction: ViewerInteractionConfig,
) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cache
        .get_or_init(|| build_scene(path, interaction).map_err(|error| error.to_string()))
        .clone()
        .map_err(|error| std::io::Error::other(error).into())
}

fn pointer_for_host_action(
    scene: &PreviewScene,
    predicate: impl Fn(&UiTreeHostActionHit) -> bool,
) -> Result<PointerHit, Box<dyn std::error::Error>> {
    let hits = StorybookHostActionHits::hits(scene, WINDOW_WIDTH);
    let hit = hits
        .into_iter()
        .filter(predicate)
        .min_by_key(|hit| hit.rect.area())
        .ok_or_else(|| std::io::Error::other("missing host action hit"))?;
    let (center_x, center_y) = hit.center_point();
    Ok(pointer_for_document_point(center_x, center_y))
}
