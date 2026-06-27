use super::window_sidebar_frame_cache::{
    StorybookSidebarFrameCacheKey, StorybookSidebarFrameCacheKeyInput,
};
use super::{StorybookFrameCache, StorybookWindow};
use crate::frame::{FrameRenderRequest, StorybookFrameRenderer};

impl StorybookWindow {
    pub(super) fn render_canvas(&mut self, width: usize, height: usize) -> crate::canvas::Canvas {
        let sidebar_key = self.sidebar_frame_cache_key(width, height);
        if self.sidebar_frame_cache.canvas(&sidebar_key).is_none() {
            let sidebar = {
                let request = self.frame_render_request(width, height);
                StorybookFrameRenderer::render_sidebar(&request)
            };
            #[cfg(test)]
            {
                self.sidebar_frame_cache_misses += 1;
            }
            self.sidebar_frame_cache
                .insert(sidebar_key.clone(), sidebar);
        }
        let request = self.frame_render_request(width, height);
        let Some(sidebar) = self.sidebar_frame_cache.canvas(&sidebar_key) else {
            let sidebar = StorybookFrameRenderer::render_sidebar(&request);
            let mut canvas = StorybookFrameRenderer::render_with_sidebar(request, &sidebar);
            self.draw_text_selection(&mut canvas);
            self.start_deferred_asset_job_for_current_viewport(width, height);
            return canvas;
        };
        let mut canvas = StorybookFrameRenderer::render_with_sidebar(request, sidebar);
        self.draw_text_selection(&mut canvas);
        self.start_deferred_asset_job_for_current_viewport(width, height);
        canvas
    }

    pub(super) fn render_canvas_scaled(
        &mut self,
        width: usize,
        height: usize,
        scale: f32,
    ) -> crate::canvas::Canvas {
        if scale <= 1.0 {
            return self.render_canvas(width, height);
        }
        let sidebar_key = self.sidebar_frame_cache_key(width, height);
        if self
            .sidebar_frame_cache
            .canvas_scaled(&sidebar_key, scale)
            .is_none()
        {
            let sidebar = {
                let request = self.frame_render_request(width, height);
                StorybookFrameRenderer::render_sidebar_scaled(&request, scale)
            };
            #[cfg(test)]
            {
                self.sidebar_frame_cache_misses += 1;
            }
            self.sidebar_frame_cache
                .insert(sidebar_key.clone(), sidebar);
        }
        let request = self.frame_render_request(width, height);
        let Some(sidebar) = self.sidebar_frame_cache.canvas_scaled(&sidebar_key, scale) else {
            let sidebar = StorybookFrameRenderer::render_sidebar_scaled(&request, scale);
            let mut canvas =
                StorybookFrameRenderer::render_scaled_with_sidebar(request, &sidebar, scale);
            self.draw_text_selection(&mut canvas);
            self.start_deferred_asset_job_for_current_viewport(width, height);
            return canvas;
        };
        let mut canvas =
            StorybookFrameRenderer::render_scaled_with_sidebar(request, sidebar, scale);
        self.draw_text_selection(&mut canvas);
        self.start_deferred_asset_job_for_current_viewport(width, height);
        canvas
    }

    pub(super) fn render_frame_cache_scaled(
        &mut self,
        width: usize,
        height: usize,
        scale: f32,
    ) -> StorybookFrameCache {
        let canvas = self.render_canvas_scaled(width, height, scale);
        StorybookFrameCache::new_for_scroll(canvas, self.scroll_y)
    }

    pub(super) fn draw_text_selection(&self, canvas: &mut crate::canvas::Canvas) {
        if self
            .scene
            .as_ref()
            .is_some_and(|scene| scene.fullscreen_diagram_active())
        {
            return;
        }
        canvas.draw_text_selection_highlight(
            self.text_selection_start,
            self.text_selection_end,
            text_selection_color(self.dark),
        );
    }

    pub(super) fn frame_render_request(
        &self,
        width: usize,
        height: usize,
    ) -> FrameRenderRequest<'_> {
        FrameRenderRequest {
            width,
            height,
            fixtures: &self.catalog.fixtures,
            selected_index: self.selected_index,
            scene: self.scene.as_ref(),
            scroll_y: self.scroll_y,
            sidebar_scroll: self.sidebar_scroll,
            file_tree_state: self.file_tree_state.clone(),
            settings_state: &self.settings_state,
            dark: self.dark,
            interaction: &self.interaction,
            typography: self.typography,
            last_command_label: &self.last_command_label,
            task_context_menu: self.task_context_menu.as_ref(),
            hovered_node_id: self.hovered_node_id.as_deref(),
            hovered_action_node_id: self.hovered_action_node_id.as_ref(),
            animation_phase: self.animation_phase,
        }
    }

    pub(super) fn sidebar_frame_cache_key(
        &self,
        width: usize,
        height: usize,
    ) -> StorybookSidebarFrameCacheKey {
        StorybookSidebarFrameCacheKey::new(StorybookSidebarFrameCacheKeyInput {
            width,
            height,
            selected_index: self.selected_index,
            scroll: self.sidebar_scroll,
            file_tree_state: self.file_tree_state.clone(),
            settings_state: self.settings_state.clone(),
            dark: self.dark,
            interaction: self.interaction.clone(),
            typography: self.typography,
            scene: self.scene.as_ref(),
        })
    }
}

fn text_selection_color(dark: bool) -> u32 {
    if dark { 0x4C9BFF } else { 0x006FD6 }
}
