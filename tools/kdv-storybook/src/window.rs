use self::window_sidebar_frame_cache::{
    StorybookSidebarFrameCacheStore, StorybookSidebarInteractionSurfaceCache,
};
use self::window_types::{
    MAX_LOADED_ASSET_SCENES, StorybookDocumentInteractionSurfaceCache, StorybookFrameCache,
    StorybookLoadedAssetScene, StorybookPointerPosition, StorybookSceneRefreshMode,
    StorybookSidebarInteractionCache,
};
use crate::args::StorybookArgs;
use crate::catalog::FixtureCatalog;
use crate::frame::StorybookFrameRenderer;
use crate::mouse::StorybookMouseState;
use crate::mouse::task_context_menu::StorybookTaskContextMenu;
use crate::preview::{PreviewBuilder, PreviewScene};
use crate::sidebar::StorybookSidebarScroll;
use crate::sidebar_settings_state::StorybookSettingsState;
use crate::window_asset_job::{StorybookAssetJob, StorybookAssetJobKey};
use crate::window_host_event::StorybookHostEvent;
use katana_document_viewer::{
    DiagramViewportState, ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerTaskState,
    ViewerTypographyConfig,
};
use katana_ui_core::molecule::FileTreeState;
use katana_ui_core::render_model::UiNodeId;
use std::collections::{BTreeMap, BTreeSet};
#[cfg(test)]
use std::time::{Duration, Instant};
type StorybookError = Box<dyn std::error::Error>;
#[cfg(test)]
type TextSelectionEndpoints = (Option<(usize, usize)>, Option<(usize, usize)>);

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct StorybookScrollFramePhaseTimes {
    pub(crate) apply: Duration,
    pub(crate) ensure_presented: Duration,
    pub(crate) redraw_band: Duration,
    pub(crate) update_presented: Duration,
    pub(crate) asset_defer: Duration,
    pub(crate) full_preview_redraw_fallback: bool,
}

pub struct StorybookWindow {
    args: StorybookArgs,
    catalog: FixtureCatalog,
    preview: PreviewBuilder,
    selected_index: usize,
    scroll_y: f32,
    preview_scroll_pixel_residual: f32,
    sidebar_scroll: StorybookSidebarScroll,
    file_tree_state: FileTreeState,
    settings_state: StorybookSettingsState,
    dark: bool,
    mode: ViewerMode,
    typography: ViewerTypographyConfig,
    search: ViewerSearchState,
    interaction: ViewerInteractionConfig,
    diagram_viewports: BTreeMap<String, DiagramViewportState>,
    image_viewports: BTreeMap<String, DiagramViewportState>,
    task_state_overrides: BTreeMap<String, ViewerTaskState>,
    copied_code_node_ids: BTreeSet<String>,
    scene: Option<PreviewScene>,
    hovered_node_id: Option<String>,
    hovered_action_node_id: Option<UiNodeId>,
    document_cursor: katana_ui_core::render_model::UiCursor,
    accordion_open_overrides: BTreeMap<String, bool>,
    frame_size: Option<(usize, usize)>,
    frame_cache: Option<StorybookFrameCache>,
    sidebar_frame_cache: StorybookSidebarFrameCacheStore,
    sidebar_interaction_surface_cache: Option<StorybookSidebarInteractionSurfaceCache>,
    document_interaction_surface_cache: Option<StorybookDocumentInteractionSurfaceCache>,
    #[cfg(test)]
    sidebar_frame_cache_misses: usize,
    #[cfg(test)]
    sidebar_interaction_surface_cache_misses: usize,
    scene_refresh: StorybookSceneRefreshMode,
    asset_job: Option<StorybookAssetJob>,
    deferred_asset_job: bool,
    loaded_asset_job_keys: Vec<StorybookAssetJobKey>,
    loaded_asset_scenes: Vec<StorybookLoadedAssetScene>,
    mouse: StorybookMouseState,
    pointer_position: Option<StorybookPointerPosition>,
    text_selection_start: Option<(usize, usize)>,
    text_selection_end: Option<(usize, usize)>,
    text_selection_active: bool,
    fullscreen_diagram_drag_previous: Option<(f32, f32)>,
    document_diagram_drag_previous: Option<(String, (f32, f32))>,
    sidebar_interaction_cache: Option<StorybookSidebarInteractionCache>,
    animation_phase: u16,
    last_command_label: String,
    host_events: Vec<StorybookHostEvent>,
    task_context_menu: Option<StorybookTaskContextMenu>,
}

impl StorybookWindow {
    pub fn new(args: StorybookArgs, catalog: FixtureCatalog, preview: PreviewBuilder) -> Self {
        StorybookFrameRenderer::prewarm();
        Self {
            args,
            catalog,
            preview,
            selected_index: 0,
            scroll_y: 0.0,
            preview_scroll_pixel_residual: 0.0,
            sidebar_scroll: StorybookSidebarScroll::default(),
            file_tree_state: FileTreeState::default(),
            settings_state: StorybookSettingsState::default(),
            dark: true,
            mode: ViewerMode::Document,
            typography: ViewerTypographyConfig::default(),
            search: ViewerSearchState::default(),
            interaction: ViewerInteractionConfig::default(),
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            copied_code_node_ids: BTreeSet::new(),
            scene: None,
            hovered_node_id: None,
            hovered_action_node_id: None,
            document_cursor: katana_ui_core::render_model::UiCursor::Default,
            accordion_open_overrides: BTreeMap::new(),
            frame_size: None,
            frame_cache: None,
            sidebar_frame_cache: StorybookSidebarFrameCacheStore::default(),
            sidebar_interaction_surface_cache: None,
            document_interaction_surface_cache: None,
            #[cfg(test)]
            sidebar_frame_cache_misses: 0,
            #[cfg(test)]
            sidebar_interaction_surface_cache_misses: 0,
            scene_refresh: StorybookSceneRefreshMode::Lazy,
            asset_job: None,
            deferred_asset_job: false,
            loaded_asset_job_keys: Vec::new(),
            loaded_asset_scenes: Vec::new(),
            mouse: StorybookMouseState::default(),
            pointer_position: None,
            text_selection_start: None,
            text_selection_end: None,
            text_selection_active: false,
            fullscreen_diagram_drag_previous: None,
            document_diagram_drag_previous: None,
            sidebar_interaction_cache: None,
            animation_phase: 0,
            last_command_label: "none".to_string(),
            host_events: Vec::new(),
            task_context_menu: None,
        }
    }

    fn reset_fixture_state(&mut self) {
        self.scroll_y = 0.0;
        self.search = ViewerSearchState::default();
        self.scene = None;
        self.hovered_node_id = None;
        self.hovered_action_node_id = None;
        self.frame_cache = None;
        self.sidebar_interaction_cache = None;
        self.sidebar_interaction_surface_cache = None;
        self.diagram_viewports.clear();
        self.image_viewports.clear();
        self.task_state_overrides.clear();
        self.copied_code_node_ids.clear();
        self.settings_state.clear_task_changes();
        self.accordion_open_overrides.clear();
        self.task_context_menu = None;
        self.scene_refresh = StorybookSceneRefreshMode::Lazy;
        self.asset_job = None;
        self.deferred_asset_job = false;
        self.loaded_asset_job_keys.clear();
        self.loaded_asset_scenes.clear();
        self.animation_phase = 0;
        self.host_events.clear();
        self.text_selection_start = None;
        self.text_selection_end = None;
        self.text_selection_active = false;
        self.fullscreen_diagram_drag_previous = None;
        self.document_diagram_drag_previous = None;
    }

    #[cfg(test)]
    pub(crate) fn select_fixture_index_for_tests(&mut self, index: usize) {
        self.selected_index = index;
        self.reset_fixture_state();
    }

    #[cfg(test)]
    pub(crate) fn sidebar_frame_cache_misses_for_tests(&self) -> usize {
        self.sidebar_frame_cache_misses
    }

    #[cfg(test)]
    pub(crate) fn update_scene_for_tests(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        self.update_scene(width, height)
    }

    #[cfg(test)]
    pub(crate) fn render_canvas_for_tests(
        &mut self,
        width: usize,
        height: usize,
    ) -> crate::canvas::Canvas {
        self.render_canvas(width, height)
    }

    #[cfg(test)]
    pub(crate) fn render_cached_scroll_canvas_scaled_for_tests(
        &mut self,
        width: usize,
        height: usize,
        scale: f32,
    ) -> Result<(), StorybookError> {
        if self.frame_cache_matches_scaled(width, height, scale) {
            self.redraw_cached_preview(width, height)?;
        } else {
            self.frame_cache = Some(self.render_frame_cache_scaled(width, height, scale));
            if let Some(frame) = self.frame_cache.as_mut() {
                frame.ensure_presented_frame(width, height);
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn render_wheel_scroll_presented_frame_for_tests(
        &mut self,
        delta_y: f32,
        width: usize,
        height: usize,
        scale: f32,
    ) -> Result<(), StorybookError> {
        self.apply_preview_scroll(delta_y, height);
        self.clear_document_hover_state();
        self.render_cached_scroll_presented_frame_for_tests(width, height, scale)?;
        let Some(frame) = self.frame_cache.as_mut() else {
            return Err("frame cache missing after wheel scroll render".into());
        };
        let presented = frame.presented_frame(width, height);
        if presented.pixels().is_empty() {
            return Err("presented wheel scroll frame is empty".into());
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn render_wheel_scroll_cached_frame_phase_times_for_tests(
        &mut self,
        delta_y: f32,
        width: usize,
        height: usize,
        scale: f32,
    ) -> Result<StorybookScrollFramePhaseTimes, StorybookError> {
        let apply_started = Instant::now();
        self.apply_preview_scroll(delta_y, height);
        self.clear_document_hover_state();
        let apply = apply_started.elapsed();
        if !self.frame_cache_matches_scaled(width, height, scale) {
            self.frame_cache = Some(self.render_frame_cache_scaled(width, height, scale));
            if let Some(frame) = self.frame_cache.as_mut() {
                let ensure_started = Instant::now();
                frame.ensure_presented_frame(width, height);
                return Ok(StorybookScrollFramePhaseTimes {
                    apply,
                    ensure_presented: ensure_started.elapsed(),
                    redraw_band: Duration::ZERO,
                    update_presented: Duration::ZERO,
                    asset_defer: Duration::ZERO,
                    full_preview_redraw_fallback: true,
                });
            }
        }
        let Some(mut frame) = self.frame_cache.take() else {
            return Err("frame cache missing before phase-timed scroll render".into());
        };
        let request = self.frame_render_request(width, height);
        let rendered_scroll_y = frame.rendered_scroll_y();
        let ensure_started = Instant::now();
        frame.ensure_presented_frame(width, height);
        let ensure_presented = ensure_started.elapsed();
        let redraw_started = Instant::now();
        let scroll_redraw = StorybookFrameRenderer::redraw_preview_scroll_delta_with_result(
            frame.canvas_mut_preserving_presented(),
            &request,
            rendered_scroll_y,
        );
        let redraw_band = redraw_started.elapsed();
        let update_started = Instant::now();
        let can_scroll_redraw = scroll_redraw
            .is_some_and(|redraw| frame.update_presented_scroll_region(width, height, redraw));
        let update_presented = update_started.elapsed();
        let full_preview_redraw_fallback = scroll_redraw.is_none() || !can_scroll_redraw;
        if full_preview_redraw_fallback {
            frame.invalidate_presented();
            StorybookFrameRenderer::redraw_preview(frame.canvas_mut(), &request);
        }
        frame.set_rendered_scroll_y(self.scroll_y);
        self.frame_cache = Some(frame);
        let asset_started = Instant::now();
        self.start_deferred_asset_job_for_current_viewport(width, height);
        let asset_defer = asset_started.elapsed();
        Ok(StorybookScrollFramePhaseTimes {
            apply,
            ensure_presented,
            redraw_band,
            update_presented,
            asset_defer,
            full_preview_redraw_fallback,
        })
    }

    #[cfg(test)]
    pub(crate) fn render_cached_scroll_presented_frame_for_tests(
        &mut self,
        width: usize,
        height: usize,
        scale: f32,
    ) -> Result<(), StorybookError> {
        if self.frame_cache_matches_scaled(width, height, scale) {
            self.redraw_cached_preview_for_presented_scroll(width, height)?;
        } else {
            self.frame_cache = Some(self.render_frame_cache_scaled(width, height, scale));
            if let Some(frame) = self.frame_cache.as_mut() {
                frame.ensure_presented_frame(width, height);
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn present_cached_frame_for_tests(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<crate::canvas::Canvas, StorybookError> {
        let Some(frame) = self.frame_cache.as_mut() else {
            return Err("frame cache missing before presentation".into());
        };
        Ok(frame.presented_frame(width, height).clone())
    }

    #[cfg(test)]
    pub(crate) fn cached_source_frame_for_tests(
        &self,
    ) -> Result<crate::canvas::Canvas, StorybookError> {
        let Some(frame) = self.frame_cache.as_ref() else {
            return Err("frame cache missing before source frame inspection".into());
        };
        Ok(frame.canvas().clone())
    }

    #[cfg(test)]
    pub(crate) fn scroll_y_value_for_tests(&self) -> f32 {
        self.scroll_y
    }

    #[cfg(test)]
    pub(crate) fn render_pending_wheel_scroll_presented_frame_for_tests(
        &mut self,
        delta_y: f32,
        width: usize,
        height: usize,
        scale: f32,
    ) -> Result<(), StorybookError> {
        self.update_loading_animation(true);
        self.render_wheel_scroll_presented_frame_for_tests(delta_y, width, height, scale)
    }

    #[cfg(test)]
    pub(crate) fn scene_for_tests(&self) -> Option<&PreviewScene> {
        self.scene.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn scroll_y_for_tests(&mut self, value: f32) {
        self.scroll_y = value;
    }

    #[cfg(test)]
    pub(crate) fn wait_loaded_asset_scene_for_tests(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        self.start_asset_job_for_current_viewport(width, height);
        let deadline = Instant::now() + Duration::from_secs(8);
        while Instant::now() <= deadline {
            if self.apply_asset_job()? && self.asset_job.is_none() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(8));
        }
        Err("asset job did not complete before loaded scroll performance test".into())
    }

    #[cfg(test)]
    pub(crate) fn set_text_selection_for_tests(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
    ) {
        self.text_selection_start = Some(start);
        self.text_selection_end = Some(end);
        self.frame_cache = None;
    }

    #[cfg(test)]
    pub(crate) fn text_selection_for_tests(&self) -> TextSelectionEndpoints {
        (self.text_selection_start, self.text_selection_end)
    }

    #[cfg(test)]
    pub(crate) fn selected_text_payload_for_tests(
        &mut self,
        width: usize,
        height: usize,
    ) -> Option<String> {
        self.selected_text_payload(width, height)
    }
}

#[path = "window_types.rs"]
mod window_types;

#[path = "window_headless.rs"]
mod window_headless;

#[path = "window_keyboard.rs"]
mod window_keyboard;

#[path = "window_loop.rs"]
mod window_loop;

#[path = "window_render.rs"]
mod window_render;

#[path = "window_scene.rs"]
mod window_scene;

#[path = "window_scene_scroll.rs"]
mod window_scene_scroll;

#[path = "window_sidebar_frame_cache.rs"]
mod window_sidebar_frame_cache;

#[path = "window_scroll.rs"]
mod window_scroll;

#[path = "window_accordion.rs"]
mod window_accordion;

#[path = "window_cursor.rs"]
mod window_cursor;

#[path = "window_diagram_gesture.rs"]
mod window_diagram_gesture;

#[path = "window_mouse.rs"]
mod window_mouse;

#[cfg(test)]
#[path = "window_mouse_source_tests.rs"]
mod window_mouse_source_tests;

#[cfg(test)]
#[path = "window_cursor_tests.rs"]
mod window_cursor_tests;

#[cfg(test)]
#[path = "window/interaction_matrix_support.rs"]
mod interaction_matrix_support;

#[cfg(test)]
#[path = "window/interaction_matrix_tests.rs"]
mod interaction_matrix_tests;

#[cfg(test)]
#[path = "window/accordion_window_tests.rs"]
mod accordion_window_tests;

#[cfg(test)]
#[path = "window/task_checkbox_component_tests.rs"]
mod task_checkbox_component_tests;

#[cfg(test)]
#[path = "window/emoji_component_tests.rs"]
mod emoji_component_tests;

#[cfg(test)]
#[path = "window/scroll_lazy_scene_tests.rs"]
mod scroll_lazy_scene_tests;

#[cfg(test)]
#[path = "window/scroll_bottom_window_tests.rs"]
mod scroll_bottom_window_tests;

#[cfg(test)]
#[path = "window/image_fixture_window_tests.rs"]
mod image_fixture_window_tests;

#[cfg(test)]
#[path = "window/diagram_asset_tests.rs"]
mod diagram_asset_tests;

#[cfg(test)]
#[path = "window/diagram_asset_scroll_tests.rs"]
mod diagram_asset_scroll_tests;

#[cfg(test)]
#[path = "window/asset_job_scope_tests.rs"]
mod asset_job_scope_tests;

#[cfg(test)]
#[path = "window/sidebar_row_contract_tests.rs"]
mod sidebar_row_contract_tests;

#[cfg(test)]
#[path = "window/sidebar_interaction_surface_cache_tests.rs"]
mod sidebar_interaction_surface_cache_tests;

#[path = "window_command.rs"]
mod window_command;

#[path = "window_clipboard_smoke.rs"]
mod window_clipboard_smoke;

#[cfg(test)]
#[path = "window_tests.rs"]
mod tests;
