use super::super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH, StorybookPreviewArea};
use crate::preview::{PreviewBuilder, PreviewScene};
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::UiNodeId;
use katana_ui_core::theme::{Rgba, ThemeSnapshot};
use katana_ui_core_storybook::UiTreeHostActionHit;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub(super) const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 720;
const PREVIEW_HEIGHT: f32 = 600.0;
pub(super) const FRAME_PREVIEW_LEFT: usize = SIDEBAR_WIDTH + 16;
pub(super) const FRAME_PREVIEW_TOP: usize = HEADER_HEIGHT + 16;

#[derive(Clone)]
pub(super) struct MediaActionHit {
    pub(super) command: String,
    pub(super) hit: UiTreeHostActionHit,
}

pub(super) type DiagramActionHit = MediaActionHit;

pub(super) struct MediaControlFrameSupport;

impl MediaControlFrameSupport {
    pub(super) fn build_scene(dark: bool) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        let cache = if dark {
            &DIAGRAM_DARK_SCENE
        } else {
            &DIAGRAM_LIGHT_SCENE
        };
        cached_scene(cache, || {
            Self::build_fixture_scene("direct/sample.md", dark, ViewerInteractionConfig::default())
        })
    }

    pub(super) fn build_image_fixture_scene(
        label: &str,
        dark: bool,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        if label == "direct/kdv-icon.png" {
            let cache = if dark {
                &IMAGE_DARK_SCENE
            } else {
                &IMAGE_LIGHT_SCENE
            };
            return cached_scene(cache, || {
                Self::build_fixture_scene(
                    label,
                    dark,
                    ViewerInteractionConfig {
                        image_controls_enabled: true,
                        ..ViewerInteractionConfig::default()
                    },
                )
            });
        }
        Self::build_fixture_scene(
            label,
            dark,
            ViewerInteractionConfig {
                image_controls_enabled: true,
                ..ViewerInteractionConfig::default()
            },
        )
    }

    pub(super) fn build_code_scene(dark: bool) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        let cache = if dark {
            &CODE_DARK_SCENE
        } else {
            &CODE_LIGHT_SCENE
        };
        cached_scene(cache, || {
            Self::build_fixture_scene(
                "katana/sample_basic.md",
                dark,
                ViewerInteractionConfig {
                    code_controls_enabled: true,
                    ..ViewerInteractionConfig::default()
                },
            )
        })
    }

    pub(super) fn render_scene(scene: &PreviewScene, dark: bool) -> Canvas {
        Self::render_scene_with_action_hover(scene, dark, None)
    }

    pub(super) fn render_scene_with_action_hover(
        scene: &PreviewScene,
        dark: bool,
        hovered_action_node_id: Option<&UiNodeId>,
    ) -> Canvas {
        Self::render_fixture_scene_with_action_hover(
            "direct/sample.md",
            scene,
            dark,
            0.0,
            hovered_action_node_id,
        )
    }

    pub(super) fn render_scene_sidebar(scene: &PreviewScene, dark: bool) -> Canvas {
        let cache = if dark {
            &DIAGRAM_DARK_SIDEBAR
        } else {
            &DIAGRAM_LIGHT_SIDEBAR
        };
        cached_canvas(cache, || {
            Self::render_fixture_sidebar("direct/sample.md", scene, dark)
        })
    }

    pub(super) fn render_scene_with_action_hover_and_sidebar(
        scene: &PreviewScene,
        dark: bool,
        hovered_action_node_id: Option<&UiNodeId>,
        sidebar: &Canvas,
    ) -> Canvas {
        Self::render_fixture_scene_with_action_hover_and_sidebar(
            "direct/sample.md",
            scene,
            dark,
            0.0,
            hovered_action_node_id,
            sidebar,
        )
    }

    pub(super) fn render_image_fixture_scene_with_action_hover(
        label: &str,
        scene: &PreviewScene,
        dark: bool,
        hovered_action_node_id: Option<&UiNodeId>,
    ) -> Canvas {
        Self::render_fixture_scene_with_action_hover(
            label,
            scene,
            dark,
            0.0,
            hovered_action_node_id,
        )
    }

    pub(super) fn render_image_fixture_sidebar(
        label: &str,
        scene: &PreviewScene,
        dark: bool,
    ) -> Canvas {
        if label == "direct/kdv-icon.png" {
            let cache = if dark {
                &IMAGE_DARK_SIDEBAR
            } else {
                &IMAGE_LIGHT_SIDEBAR
            };
            return cached_canvas(cache, || Self::render_fixture_sidebar(label, scene, dark));
        }
        Self::render_fixture_sidebar(label, scene, dark)
    }

    pub(super) fn render_image_fixture_scene_with_action_hover_and_sidebar(
        label: &str,
        scene: &PreviewScene,
        dark: bool,
        hovered_action_node_id: Option<&UiNodeId>,
        sidebar: &Canvas,
    ) -> Canvas {
        Self::render_fixture_scene_with_action_hover_and_sidebar(
            label,
            scene,
            dark,
            0.0,
            hovered_action_node_id,
            sidebar,
        )
    }

    pub(super) fn render_code_scene_sidebar(scene: &PreviewScene, dark: bool) -> Canvas {
        let cache = if dark {
            &CODE_DARK_SIDEBAR
        } else {
            &CODE_LIGHT_SIDEBAR
        };
        cached_canvas(cache, || {
            Self::render_fixture_sidebar("katana/sample_basic.md", scene, dark)
        })
    }

    pub(super) fn render_code_scene_with_scroll_action_hover_and_sidebar(
        scene: &PreviewScene,
        dark: bool,
        scroll_y: f32,
        hovered_action_node_id: Option<&UiNodeId>,
        sidebar: &Canvas,
    ) -> Canvas {
        Self::render_fixture_scene_with_action_hover_and_sidebar(
            "katana/sample_basic.md",
            scene,
            dark,
            scroll_y,
            hovered_action_node_id,
            sidebar,
        )
    }

    fn build_fixture_scene(
        label: &str,
        dark: bool,
        interaction: ViewerInteractionConfig,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        shared_builder().build(&fixture(label), viewport(), dark, interaction)
    }

    fn render_fixture_sidebar(label: &str, scene: &PreviewScene, dark: bool) -> Canvas {
        let selected_fixture = fixture(label);
        let fixtures = [selected_fixture];
        let settings_state = Default::default();
        let interaction = ViewerInteractionConfig::default();
        let request = FrameRenderRequest {
            width: FRAME_WIDTH,
            height: FRAME_HEIGHT,
            fixtures: &fixtures,
            selected_index: 0,
            scene: Some(scene),
            scroll_y: 0.0,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &settings_state,
            dark,
            interaction: &interaction,
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: None,
            hovered_action_node_id: None,
            animation_phase: 0,
        };
        StorybookFrameRenderer::render_sidebar(&request)
    }

    fn render_fixture_scene_with_action_hover(
        label: &str,
        scene: &PreviewScene,
        dark: bool,
        scroll_y: f32,
        hovered_action_node_id: Option<&UiNodeId>,
    ) -> Canvas {
        let selected_fixture = fixture(label);
        let fixtures = [selected_fixture];
        let settings_state = Default::default();
        let interaction = ViewerInteractionConfig::default();
        StorybookFrameRenderer::render(FrameRenderRequest {
            width: FRAME_WIDTH,
            height: FRAME_HEIGHT,
            fixtures: &fixtures,
            selected_index: 0,
            scene: Some(scene),
            scroll_y,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &settings_state,
            dark,
            interaction: &interaction,
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: None,
            hovered_action_node_id,
            animation_phase: 0,
        })
    }

    fn render_fixture_scene_with_action_hover_and_sidebar(
        label: &str,
        scene: &PreviewScene,
        dark: bool,
        scroll_y: f32,
        hovered_action_node_id: Option<&UiNodeId>,
        sidebar: &Canvas,
    ) -> Canvas {
        let selected_fixture = fixture(label);
        let fixtures = [selected_fixture];
        let settings_state = Default::default();
        let interaction = ViewerInteractionConfig::default();
        StorybookFrameRenderer::render_with_sidebar(
            FrameRenderRequest {
                width: FRAME_WIDTH,
                height: FRAME_HEIGHT,
                fixtures: &fixtures,
                selected_index: 0,
                scene: Some(scene),
                scroll_y,
                sidebar_scroll: Default::default(),
                file_tree_state: Default::default(),
                settings_state: &settings_state,
                dark,
                interaction: &interaction,
                typography: Default::default(),
                last_command_label: "none",
                task_context_menu: None,
                hovered_node_id: None,
                hovered_action_node_id,
                animation_phase: 0,
            },
            sidebar,
        )
    }
}

pub(super) fn frame_pixel_for_hit(hit: &UiTreeHostActionHit) -> (usize, usize) {
    frame_pixel_for_scrolled_hit(hit, 0.0)
}

pub(super) fn frame_rect_for_hit(hit: &UiTreeHostActionHit) -> (usize, usize, usize, usize) {
    frame_rect_for_scrolled_hit(hit, 0.0)
}

pub(super) fn preview_frame_bounds() -> (usize, usize, usize, usize) {
    let area = StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, 0.0);
    (area.x, area.y, area.x + area.width, area.y + area.height)
}

pub(super) fn frame_pixel_for_scrolled_hit(
    hit: &UiTreeHostActionHit,
    scroll_y: f32,
) -> (usize, usize) {
    let area = StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, scroll_y);
    let x = hit.rect.x.saturating_add(hit.rect.width.saturating_div(2));
    (
        area.x + x,
        area.y + (hit.rect.y as f32 - scroll_y).round().max(0.0) as usize,
    )
}

pub(super) fn frame_rect_for_scrolled_hit(
    hit: &UiTreeHostActionHit,
    scroll_y: f32,
) -> (usize, usize, usize, usize) {
    let area = StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, scroll_y);
    let left = area.x + hit.rect.x;
    let top = area.y + (hit.rect.y as f32 - scroll_y).round().max(0.0) as usize;
    (left, top, left + hit.rect.width, top + hit.rect.height)
}

pub(super) fn color(theme: &ThemeSnapshot, token: &str) -> Result<u32, std::io::Error> {
    theme
        .color(token)
        .map(rgb)
        .ok_or_else(|| std::io::Error::other(format!("missing theme token: {token}")))
}

fn rgb(rgba: Rgba) -> u32 {
    ((u32::from(rgba[0])) << 16) | ((u32::from(rgba[1])) << 8) | u32::from(rgba[2])
}

fn fixture(label: &str) -> StorybookFixture {
    StorybookFixture {
        label: label.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{label}")),
    }
}

fn viewport() -> ViewerViewport {
    ViewerViewport {
        width: StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, 0.0).width as f32,
        height: PREVIEW_HEIGHT,
    }
}

fn shared_builder() -> &'static PreviewBuilder {
    static BUILDER: OnceLock<PreviewBuilder> = OnceLock::new();
    BUILDER.get_or_init(PreviewBuilder::default)
}

static DIAGRAM_DARK_SCENE: Mutex<Option<PreviewScene>> = Mutex::new(None);
static DIAGRAM_LIGHT_SCENE: Mutex<Option<PreviewScene>> = Mutex::new(None);
static IMAGE_DARK_SCENE: Mutex<Option<PreviewScene>> = Mutex::new(None);
static IMAGE_LIGHT_SCENE: Mutex<Option<PreviewScene>> = Mutex::new(None);
static CODE_DARK_SCENE: Mutex<Option<PreviewScene>> = Mutex::new(None);
static CODE_LIGHT_SCENE: Mutex<Option<PreviewScene>> = Mutex::new(None);
static DIAGRAM_DARK_SIDEBAR: Mutex<Option<Canvas>> = Mutex::new(None);
static DIAGRAM_LIGHT_SIDEBAR: Mutex<Option<Canvas>> = Mutex::new(None);
static IMAGE_DARK_SIDEBAR: Mutex<Option<Canvas>> = Mutex::new(None);
static IMAGE_LIGHT_SIDEBAR: Mutex<Option<Canvas>> = Mutex::new(None);
static CODE_DARK_SIDEBAR: Mutex<Option<Canvas>> = Mutex::new(None);
static CODE_LIGHT_SIDEBAR: Mutex<Option<Canvas>> = Mutex::new(None);

fn cached_scene(
    cache: &'static Mutex<Option<PreviewScene>>,
    build: impl FnOnce() -> Result<PreviewScene, Box<dyn std::error::Error>>,
) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    let mut guard = cache
        .lock()
        .map_err(|_| std::io::Error::other("media control scene cache lock poisoned"))?;
    if let Some(scene) = guard.as_ref() {
        return Ok(scene.clone());
    }
    let scene = build()?;
    *guard = Some(scene.clone());
    Ok(scene)
}

fn cached_canvas(cache: &'static Mutex<Option<Canvas>>, build: impl FnOnce() -> Canvas) -> Canvas {
    let Ok(mut guard) = cache.lock() else {
        return build();
    };
    if let Some(canvas) = guard.as_ref() {
        return canvas.clone();
    }
    let canvas = build();
    *guard = Some(canvas.clone());
    canvas
}
