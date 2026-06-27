use crate::canvas::Canvas;
use crate::frame::PreviewScrollRedraw;
use crate::preview::PreviewScene;
use crate::sidebar_hit::SidebarInteraction;
use crate::window::window_sidebar_frame_cache::StorybookSidebarFrameCacheKey;
use crate::window_asset_job::StorybookAssetJobKey;
use katana_ui_core_storybook::{StorybookPresentation, UiTreeHostActionHit, UiTreeNodeHit};
use std::sync::Arc;

pub(super) const MAX_LOADED_ASSET_SCENES: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct StorybookFrameCache {
    pub(super) width: usize,
    pub(super) height: usize,
    rendered_scroll_y: f32,
    canvas: Canvas,
    presented: Option<Canvas>,
}

impl StorybookFrameCache {
    pub(super) fn new(canvas: Canvas) -> Self {
        Self::new_for_scroll(canvas, 0.0)
    }

    pub(super) fn new_for_scroll(canvas: Canvas, rendered_scroll_y: f32) -> Self {
        let width = canvas.logical_width();
        let height = canvas.logical_height();
        Self {
            width,
            height,
            rendered_scroll_y,
            canvas,
            presented: None,
        }
    }

    pub(super) fn matches(&self, width: usize, height: usize) -> bool {
        self.width == width && self.height == height
    }

    pub(super) fn matches_scaled(&self, width: usize, height: usize, scale: f32) -> bool {
        self.matches(width, height) && (self.canvas.scale_factor() - scale).abs() < f32::EPSILON
    }

    pub(super) fn pixels(&self) -> &[u32] {
        self.canvas.pixels()
    }

    pub(super) fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub(super) fn canvas_mut(&mut self) -> &mut Canvas {
        self.presented = None;
        &mut self.canvas
    }

    pub(super) fn canvas_mut_preserving_presented(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub(super) fn invalidate_presented(&mut self) {
        self.presented = None;
    }

    pub(super) fn ensure_presented_frame(&mut self, width: usize, height: usize) {
        let _ = self.presented_frame(width, height);
    }

    pub(super) fn update_presented_scroll_region(
        &mut self,
        width: usize,
        height: usize,
        redraw: PreviewScrollRedraw,
    ) -> bool {
        self.ensure_presented_frame(width, height);
        let Some(presented) = self.presented.as_mut() else {
            return false;
        };
        if !presented.scroll_rect_vertically(
            redraw.area_x,
            redraw.area_y,
            redraw.area_width,
            redraw.content_height,
            redraw.logical_delta_y,
        ) {
            return false;
        }
        if redraw.band_height == 0 {
            return true;
        }
        let source = &self.canvas;
        StorybookPresentation::present_frame_region_for_window_into(
            source,
            presented,
            redraw.area_x,
            redraw.area_y.saturating_add(redraw.band_y),
            redraw.area_width,
            redraw.band_height,
        )
    }

    pub(super) fn presented_frame(&mut self, width: usize, height: usize) -> &Canvas {
        if !self.presented.as_ref().is_some_and(|canvas| {
            canvas.logical_width() == width && canvas.logical_height() == height
        }) {
            let fill = self.pixels().first().copied().unwrap_or_default();
            let mut target = self
                .presented
                .take()
                .unwrap_or_else(|| Canvas::new(width.max(1), height.max(1), fill));
            StorybookPresentation::present_frame_for_window_into(
                self.canvas(),
                &mut target,
                width,
                height,
                fill,
            );
            self.presented = Some(target);
        }
        match self.presented.as_ref() {
            Some(presented) => presented,
            None => self.canvas(),
        }
    }

    pub(super) fn rendered_scroll_y(&self) -> f32 {
        self.rendered_scroll_y
    }

    pub(super) fn set_rendered_scroll_y(&mut self, value: f32) {
        self.rendered_scroll_y = value;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StorybookPointerPosition {
    pub(super) x: i32,
    pub(super) y: i32,
}

impl StorybookPointerPosition {
    pub(super) fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.round() as i32,
            y: y.round() as i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StorybookSidebarInteractionCache {
    pointer: StorybookPointerPosition,
    width: usize,
    height: usize,
    key: StorybookSidebarFrameCacheKey,
    interaction: SidebarInteraction,
}

#[derive(Debug, Clone)]
pub(super) struct StorybookDocumentInteractionSurfaceCache {
    width: usize,
    height: usize,
    scroll_y_bits: u32,
    scene_key: String,
    pub(super) hits: Arc<Vec<UiTreeHostActionHit>>,
    pub(super) node_hits: Vec<UiTreeNodeHit>,
}

impl StorybookDocumentInteractionSurfaceCache {
    pub(super) fn new(
        width: usize,
        height: usize,
        scroll_y: f32,
        scene: &PreviewScene,
        hits: Arc<Vec<UiTreeHostActionHit>>,
        node_hits: Vec<UiTreeNodeHit>,
    ) -> Self {
        Self {
            width,
            height,
            scroll_y_bits: scroll_y.round().max(0.0).to_bits(),
            scene_key: Self::scene_key(scene),
            hits,
            node_hits,
        }
    }

    pub(super) fn matches(
        &self,
        width: usize,
        height: usize,
        scroll_y: f32,
        scene: &PreviewScene,
    ) -> bool {
        self.width == width
            && self.height == height
            && self.scroll_y_bits == scroll_y.round().max(0.0).to_bits()
            && self.scene_key == Self::scene_key(scene)
    }

    fn scene_key(scene: &PreviewScene) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            scene.document_id,
            scene.node_count,
            scene.content_height.to_bits(),
            scene.loaded_asset_count,
            scene.failed_asset_count,
            scene.image_surface_count,
            scene.asset_request_count,
            scene.slideshow_current_page
        )
    }
}

impl StorybookSidebarInteractionCache {
    pub(super) fn new(
        pointer: StorybookPointerPosition,
        width: usize,
        height: usize,
        key: StorybookSidebarFrameCacheKey,
        interaction: SidebarInteraction,
    ) -> Self {
        Self {
            pointer,
            width,
            height,
            key,
            interaction,
        }
    }

    pub(super) fn interaction_for(
        &self,
        pointer: StorybookPointerPosition,
        width: usize,
        height: usize,
        key: &StorybookSidebarFrameCacheKey,
    ) -> Option<SidebarInteraction> {
        if self.pointer == pointer
            && self.width == width
            && self.height == height
            && self.key == *key
        {
            return Some(self.interaction.clone());
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StorybookSceneRefreshMode {
    Lazy,
    Loaded,
}

#[derive(Clone)]
pub(super) struct StorybookLoadedAssetScene {
    pub(super) key: StorybookAssetJobKey,
    pub(super) scope_key: String,
    pub(super) scene: PreviewScene,
}

#[cfg(test)]
mod tests {
    use super::{StorybookFrameCache, StorybookPointerPosition, StorybookSidebarInteractionCache};
    use crate::canvas::Canvas;
    use crate::sidebar::StorybookSidebarScroll;
    use crate::sidebar_hit::SidebarInteraction;
    use crate::sidebar_settings_state::StorybookSettingsState;
    use crate::window::window_sidebar_frame_cache::{
        StorybookSidebarFrameCacheKey, StorybookSidebarFrameCacheKeyInput,
    };
    use katana_document_viewer::{ViewerInteractionConfig, ViewerTypographyConfig};
    use katana_ui_core::molecule::FileTreeState;

    #[test]
    fn frame_cache_matches_logical_size_for_scaled_canvas() {
        let canvas = Canvas::new_scaled(320, 240, 2.0, 0);
        let cache = StorybookFrameCache::new(canvas);

        assert!(cache.matches(320, 240));
        assert!(!cache.matches(640, 480));
        assert_eq!(640, cache.canvas.width());
        assert_eq!(480, cache.canvas.height());
    }

    #[test]
    fn frame_cache_scaled_match_rejects_same_logical_size_with_different_scale() {
        let cache = StorybookFrameCache::new(Canvas::new(320, 240, 0));

        assert!(cache.matches(320, 240));
        assert!(cache.matches_scaled(320, 240, 1.0));
        assert!(!cache.matches_scaled(320, 240, 2.0));
    }

    #[test]
    fn sidebar_interaction_cache_rejects_stale_surface_key_and_width() {
        let pointer = StorybookPointerPosition::new(18.0, 42.0);
        let key = sidebar_cache_key(260, 900, true);
        let cache = StorybookSidebarInteractionCache::new(
            pointer,
            260,
            900,
            key.clone(),
            SidebarInteraction::default(),
        );

        assert!(
            cache.interaction_for(pointer, 260, 900, &key).is_some(),
            "same rendered surface must reuse the last KUC interaction"
        );
        assert!(
            cache
                .interaction_for(pointer, 260, 900, &sidebar_cache_key(260, 900, false))
                .is_none(),
            "theme/state changes must not reuse a stale KUC interaction"
        );
        assert!(
            cache.interaction_for(pointer, 320, 900, &key).is_none(),
            "window width changes must not reuse a stale KUC interaction"
        );
    }

    fn sidebar_cache_key(width: usize, height: usize, dark: bool) -> StorybookSidebarFrameCacheKey {
        StorybookSidebarFrameCacheKey::new(StorybookSidebarFrameCacheKeyInput {
            width,
            height,
            selected_index: 0,
            scroll: StorybookSidebarScroll::default(),
            file_tree_state: FileTreeState::default(),
            settings_state: StorybookSettingsState::default(),
            dark,
            interaction: ViewerInteractionConfig::default(),
            typography: ViewerTypographyConfig::default(),
            scene: None,
        })
    }
}
