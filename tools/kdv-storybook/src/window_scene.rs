use super::{
    MAX_LOADED_ASSET_SCENES, StorybookError, StorybookLoadedAssetScene, StorybookSceneRefreshMode,
    StorybookWindow,
};
use crate::catalog::StorybookFixture;
use crate::frame::StorybookFrameRenderer;
use crate::layout::{StorybookPreviewArea, preview_content_width};
use crate::mouse::StorybookHostActionHits;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use crate::window_asset_job::{StorybookAssetJob, StorybookAssetJobKey, StorybookAssetJobKeyInput};
use katana_document_viewer::ViewerViewport;

impl StorybookWindow {
    pub(super) fn update_scene_for_refresh(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        let mode = self.refresh_mode_for_current_scene();
        self.scene_refresh = StorybookSceneRefreshMode::Lazy;
        match mode {
            StorybookSceneRefreshMode::Lazy => self.update_scene_deferred_asset_job(width, height),
            StorybookSceneRefreshMode::Loaded => self.update_scene_loaded(width, height),
        }
    }

    pub(super) fn update_scene(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        self.update_scene_with_asset_job_mode(width, height, true)
    }

    pub(super) fn update_scene_deferred_asset_job(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        self.update_scene_with_asset_job_mode(width, height, false)
    }

    fn update_scene_with_asset_job_mode(
        &mut self,
        width: usize,
        height: usize,
        start_asset_job_now: bool,
    ) -> Result<(), StorybookError> {
        let preserve_bottom_anchor = self.is_scroll_y_at_current_scene_bottom();
        self.update_frame_size(width, height);
        let fixture = self.catalog.fixtures[self.selected_index].clone();
        let viewport = self.viewport(width, height);
        let mut scene = self.preview.build_scene(PreviewBuildRequest {
            fixture: &fixture,
            viewport,
            dark: self.dark,
            theme: None,
            interaction: self.interaction.clone(),
            mode: self.mode.clone(),
            typography: self.typography,
            search: self.search.clone(),
            diagram_viewports: self.diagram_viewports.clone(),
            image_viewports: self.image_viewports.clone(),
            task_state_overrides: self.task_state_overrides.clone(),
            accordion_open_overrides: self.accordion_open_overrides.clone(),
            copied_code_node_ids: self.copied_code_node_ids.clone(),
            asset_mode: PreviewBuildAssetMode::Lazy,
            attach_surface: false,
            export_surface: false,
        })?;
        if self.apply_scene_scroll_bounds(
            scene.content_height,
            viewport.height,
            preserve_bottom_anchor,
        ) {
            scene = self.preview.build_scene(PreviewBuildRequest {
                fixture: &fixture,
                viewport,
                dark: self.dark,
                theme: None,
                interaction: self.interaction.clone(),
                mode: self.mode.clone(),
                typography: self.typography,
                search: self.search.clone(),
                diagram_viewports: self.diagram_viewports.clone(),
                image_viewports: self.image_viewports.clone(),
                task_state_overrides: self.task_state_overrides.clone(),
                accordion_open_overrides: self.accordion_open_overrides.clone(),
                copied_code_node_ids: self.copied_code_node_ids.clone(),
                asset_mode: PreviewBuildAssetMode::Lazy,
                attach_surface: false,
                export_surface: false,
            })?;
        }
        self.replace_scene_preserving_sidebar_frame_cache(scene);
        if start_asset_job_now {
            self.start_asset_job(fixture, viewport);
        } else {
            self.deferred_asset_job = self
                .scene
                .as_ref()
                .is_some_and(|scene| scene.asset_request_count > 0);
            if !self.deferred_asset_job {
                self.asset_job = None;
            }
        }
        Ok(())
    }

    pub(super) fn update_scene_loaded(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        let preserve_bottom_anchor = self.is_scroll_y_at_current_scene_bottom();
        self.update_frame_size(width, height);
        let fixture = self.catalog.fixtures[self.selected_index].clone();
        let viewport = self.viewport(width, height);
        let mut scene = self.preview.build_scene(PreviewBuildRequest {
            fixture: &fixture,
            viewport,
            dark: self.dark,
            theme: None,
            interaction: self.interaction.clone(),
            mode: self.mode.clone(),
            typography: self.typography,
            search: self.search.clone(),
            diagram_viewports: self.diagram_viewports.clone(),
            image_viewports: self.image_viewports.clone(),
            task_state_overrides: self.task_state_overrides.clone(),
            accordion_open_overrides: self.accordion_open_overrides.clone(),
            copied_code_node_ids: self.copied_code_node_ids.clone(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface: false,
            export_surface: false,
        })?;
        if self.apply_scene_scroll_bounds(
            scene.content_height,
            viewport.height,
            preserve_bottom_anchor,
        ) {
            scene = self.preview.build_scene(PreviewBuildRequest {
                fixture: &fixture,
                viewport,
                dark: self.dark,
                theme: None,
                interaction: self.interaction.clone(),
                mode: self.mode.clone(),
                typography: self.typography,
                search: self.search.clone(),
                diagram_viewports: self.diagram_viewports.clone(),
                image_viewports: self.image_viewports.clone(),
                task_state_overrides: self.task_state_overrides.clone(),
                accordion_open_overrides: self.accordion_open_overrides.clone(),
                copied_code_node_ids: self.copied_code_node_ids.clone(),
                asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
                attach_surface: false,
                export_surface: false,
            })?;
        }
        let key = self.asset_job_key(&fixture.label, viewport);
        let scope_key = scene.asset_request_key.clone();
        self.replace_scene_preserving_sidebar_frame_cache(scene.clone());
        self.remember_loaded_asset_scene(key, scope_key, scene);
        self.asset_job = None;
        self.deferred_asset_job = false;
        Ok(())
    }

    fn refresh_mode_for_current_scene(&self) -> StorybookSceneRefreshMode {
        if self.scene_refresh == StorybookSceneRefreshMode::Loaded {
            return StorybookSceneRefreshMode::Loaded;
        }
        if self.current_scene_has_resolved_assets() {
            return StorybookSceneRefreshMode::Loaded;
        }
        StorybookSceneRefreshMode::Lazy
    }

    pub(super) fn current_scene_has_resolved_assets(&self) -> bool {
        self.scene.as_ref().is_some_and(|scene| {
            scene.asset_request_count == 0
                && (scene.loaded_asset_count > 0
                    || scene.failed_asset_count > 0
                    || scene.image_surface_count > 0)
        })
    }

    fn viewport(&self, width: usize, height: usize) -> ViewerViewport {
        if self
            .diagram_viewports
            .values()
            .any(|state| state.fullscreen_open)
        {
            return ViewerViewport {
                width: width as f32,
                height: height as f32,
            };
        }
        let area = StorybookPreviewArea::for_window(width, height, self.scroll_y);
        ViewerViewport {
            width: area.width as f32,
            height: area.height as f32,
        }
    }

    fn start_asset_job(&mut self, fixture: StorybookFixture, viewport: ViewerViewport) {
        let Some(scene) = self.scene.as_ref() else {
            return;
        };
        if scene.asset_request_count == 0 {
            self.asset_job = None;
            return;
        }
        let key = self.asset_job_key(&fixture.label, viewport);
        let scope_key = scene.asset_request_key.clone();
        if self
            .loaded_asset_job_keys
            .iter()
            .any(|loaded| loaded == &key)
            && let Some(scene) = self.loaded_asset_scene(&key, &scope_key)
        {
            self.replace_scene_preserving_sidebar_frame_cache(scene.clone());
            self.asset_job = None;
            return;
        }
        if let Some(job) = self.asset_job.as_ref() {
            if job.key() == &key && job.covers_scope(&scope_key) {
                return;
            }
            job.cancel();
        }
        self.asset_job = Some(StorybookAssetJob::spawn(
            self.preview.clone(),
            crate::window_asset_job::StorybookAssetJobRequest {
                key,
                scope_key,
                fixture,
                viewport,
                dark: self.dark,
                interaction: self.interaction.clone(),
                mode: self.mode.clone(),
                typography: self.typography,
                search: self.search.clone(),
                diagram_viewports: self.diagram_viewports.clone(),
                image_viewports: self.image_viewports.clone(),
                task_state_overrides: self.task_state_overrides.clone(),
                accordion_open_overrides: self.accordion_open_overrides.clone(),
                copied_code_node_ids: self.copied_code_node_ids.clone(),
            },
        ));
    }

    pub(super) fn start_asset_job_for_current_viewport(&mut self, width: usize, height: usize) {
        let fixture = self.catalog.fixtures[self.selected_index].clone();
        let viewport = self.viewport(width, height);
        self.deferred_asset_job = false;
        self.start_asset_job(fixture, viewport);
    }

    pub(super) fn start_deferred_asset_job_for_current_viewport(
        &mut self,
        width: usize,
        height: usize,
    ) {
        if !self.deferred_asset_job {
            return;
        }
        self.start_asset_job_for_current_viewport(width, height);
    }

    pub(super) fn invalidate_lazy_scene(&mut self, clear_loaded_asset_keys: bool) {
        self.scene = None;
        self.asset_job = None;
        self.deferred_asset_job = false;
        self.hovered_node_id = None;
        self.hovered_action_node_id = None;
        self.frame_cache = None;
        self.sidebar_interaction_cache = None;
        self.sidebar_interaction_surface_cache = None;
        self.scene_refresh = StorybookSceneRefreshMode::Lazy;
        if clear_loaded_asset_keys {
            self.loaded_asset_job_keys.clear();
            self.loaded_asset_scenes.clear();
        }
    }

    pub(super) fn invalidate_lazy_scene_preserving_asset_job(&mut self) {
        self.scene = None;
        self.deferred_asset_job = false;
        self.hovered_node_id = None;
        self.hovered_action_node_id = None;
        self.frame_cache = None;
        self.sidebar_interaction_cache = None;
        self.sidebar_interaction_surface_cache = None;
        self.scene_refresh = StorybookSceneRefreshMode::Lazy;
    }

    pub(super) fn invalidate_loaded_scene(&mut self) {
        self.scene = None;
        self.asset_job = None;
        self.deferred_asset_job = false;
        self.hovered_node_id = None;
        self.hovered_action_node_id = None;
        self.frame_cache = None;
        self.sidebar_interaction_cache = None;
        self.sidebar_interaction_surface_cache = None;
        self.scene_refresh = StorybookSceneRefreshMode::Loaded;
    }

    pub(super) fn apply_asset_job(&mut self) -> Result<bool, StorybookError> {
        let Some((expected_key, expected_scope_key, result)) =
            self.asset_job.as_ref().and_then(|job| {
                job.try_recv()
                    .map(|result| (job.key().clone(), job.scope_key().to_string(), result))
            })
        else {
            return Ok(false);
        };
        if expected_key != self.current_asset_job_key() {
            if let Some(job) = self.asset_job.as_ref() {
                job.cancel();
            }
            self.asset_job = None;
            let (width, height) = self
                .frame_size
                .unwrap_or((self.args.width, self.args.height));
            self.start_asset_job_for_current_viewport(width, height);
            return Ok(false);
        }
        let event = match result {
            Ok(event) => event,
            Err(error) => {
                return self.apply_asset_job_error(expected_key, expected_scope_key, error);
            }
        };
        self.replace_scene(event.scene.clone());
        if event.complete {
            self.asset_job = None;
            self.remember_loaded_asset_scene(expected_key, expected_scope_key, event.scene);
        }
        Ok(true)
    }

    fn apply_asset_job_error(
        &mut self,
        expected_key: StorybookAssetJobKey,
        expected_scope_key: String,
        error: String,
    ) -> Result<bool, StorybookError> {
        eprintln!("[kdv-storybook] asset job failed: {error}");
        if let Some(job) = self.asset_job.as_ref() {
            job.cancel();
        }
        self.asset_job = None;
        self.deferred_asset_job = false;
        let Some(scene) = self.scene.as_mut() else {
            return Ok(false);
        };
        let failed_count = scene.asset_request_count.max(1);
        scene.failed_asset_count += failed_count;
        scene.asset_request_count = 0;
        scene.asset_request_key.clear();
        let failed_scene = scene.clone();
        self.remember_loaded_asset_scene(expected_key, expected_scope_key, failed_scene);
        self.invalidate_scene_dependent_render_caches(true);
        Ok(true)
    }

    fn replace_scene(&mut self, scene: crate::preview::PreviewScene) {
        StorybookFrameRenderer::prewarm_theme(&scene.theme);
        self.scene = Some(scene);
        self.invalidate_scene_dependent_render_caches(true);
    }

    fn replace_scene_preserving_sidebar_frame_cache(
        &mut self,
        scene: crate::preview::PreviewScene,
    ) {
        StorybookFrameRenderer::prewarm_theme(&scene.theme);
        self.scene = Some(scene);
        self.invalidate_scene_dependent_render_caches(false);
    }

    fn invalidate_scene_dependent_render_caches(&mut self, clear_sidebar_frame_cache: bool) {
        self.hovered_node_id = None;
        self.hovered_action_node_id = None;
        self.document_cursor = katana_ui_core::render_model::UiCursor::Default;
        self.frame_cache = None;
        self.document_interaction_surface_cache = None;
        self.sidebar_interaction_cache = None;
        self.sidebar_interaction_surface_cache = None;
        if clear_sidebar_frame_cache {
            self.sidebar_frame_cache.clear();
        }
    }

    fn loaded_asset_scene(
        &self,
        key: &StorybookAssetJobKey,
        scope_key: &str,
    ) -> Option<&crate::preview::PreviewScene> {
        self.loaded_asset_scenes
            .iter()
            .find(|loaded| &loaded.key == key && loaded.scope_key == scope_key)
            .map(|loaded| &loaded.scene)
    }

    pub(super) fn remember_loaded_asset_scene(
        &mut self,
        key: StorybookAssetJobKey,
        scope_key: String,
        scene: crate::preview::PreviewScene,
    ) {
        self.prewarm_loaded_scene_interaction_cache(&scene);
        if !self
            .loaded_asset_job_keys
            .iter()
            .any(|loaded| loaded == &key)
        {
            self.loaded_asset_job_keys.push(key.clone());
        }
        if let Some(loaded) = self
            .loaded_asset_scenes
            .iter_mut()
            .find(|loaded| loaded.key == key && loaded.scope_key == scope_key)
        {
            loaded.scene = scene;
            self.prune_loaded_asset_scenes();
            return;
        }
        self.loaded_asset_scenes.push(StorybookLoadedAssetScene {
            key,
            scope_key,
            scene,
        });
        self.prune_loaded_asset_scenes();
    }

    fn prune_loaded_asset_scenes(&mut self) {
        while self.loaded_asset_job_keys.len() > MAX_LOADED_ASSET_SCENES {
            let removed = self.loaded_asset_job_keys.remove(0);
            self.loaded_asset_scenes
                .retain(|loaded| loaded.key != removed);
        }
        while self.loaded_asset_scenes.len() > MAX_LOADED_ASSET_SCENES {
            let removed = self.loaded_asset_scenes.remove(0);
            self.loaded_asset_job_keys.retain(|key| key != &removed.key);
        }
    }

    fn prewarm_loaded_scene_interaction_cache(&self, scene: &crate::preview::PreviewScene) {
        if scene.mode != katana_document_viewer::ViewerMode::Document {
            return;
        }
        let (width, _) = self
            .frame_size
            .unwrap_or((self.args.width, self.args.height));
        let preview_width = preview_content_width(width);
        let _ = StorybookHostActionHits::hits_arc_for_preview_width(scene, preview_width);
        let _ = StorybookHostActionHits::node_hits_arc_for_preview_width(scene, preview_width);
    }

    fn current_asset_job_key(&self) -> StorybookAssetJobKey {
        let fixture_label = self.catalog.fixtures[self.selected_index].label.clone();
        let (width, height) = self
            .frame_size
            .unwrap_or((self.args.width, self.args.height));
        self.asset_job_key(&fixture_label, self.viewport(width, height))
    }

    fn asset_job_key(&self, fixture_label: &str, viewport: ViewerViewport) -> StorybookAssetJobKey {
        StorybookAssetJobKey::new(StorybookAssetJobKeyInput {
            fixture_label: fixture_label.to_string(),
            dark: self.dark,
            mode: self.mode.clone(),
            typography: self.typography,
            search: &self.search,
            diagram_viewports: &self.diagram_viewports,
            image_viewports: &self.image_viewports,
            task_state_overrides: &self.task_state_overrides,
            accordion_open_overrides: &self.accordion_open_overrides,
            viewport,
        })
    }
}
