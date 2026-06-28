use crate::canvas::Canvas;
use crate::preview::PreviewScene;
use crate::sidebar::StorybookSidebarScroll;
use crate::sidebar_hit::{SidebarInteraction, SidebarInteractionSurface};
use crate::sidebar_settings_state::StorybookSettingsState;
use katana_document_viewer::{ViewerInteractionConfig, ViewerMode, ViewerTypographyConfig};
use katana_ui_core::molecule::FileTreeState;

const MAX_SIDEBAR_FRAME_CACHE_ENTRIES: usize = 4;

#[derive(Debug, Clone)]
pub(super) struct StorybookSidebarFrameCache {
    key: StorybookSidebarFrameCacheKey,
    canvas: Canvas,
}

impl StorybookSidebarFrameCache {
    pub(super) fn new(key: StorybookSidebarFrameCacheKey, canvas: Canvas) -> Self {
        Self { key, canvas }
    }

    pub(super) fn matches(&self, key: &StorybookSidebarFrameCacheKey) -> bool {
        &self.key == key
    }

    pub(super) fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub(super) fn scale_factor(&self) -> f32 {
        self.canvas.scale_factor()
    }
}

#[derive(Debug, Default)]
pub(super) struct StorybookSidebarFrameCacheStore {
    entries: Vec<StorybookSidebarFrameCache>,
}

impl StorybookSidebarFrameCacheStore {
    pub(super) fn canvas(&self, key: &StorybookSidebarFrameCacheKey) -> Option<&Canvas> {
        self.canvas_scaled(key, 1.0)
    }

    pub(super) fn canvas_scaled(
        &self,
        key: &StorybookSidebarFrameCacheKey,
        scale: f32,
    ) -> Option<&Canvas> {
        self.entries
            .iter()
            .find(|entry| entry.matches(key) && entry.scale_factor() == scale)
            .map(StorybookSidebarFrameCache::canvas)
    }

    pub(super) fn insert(&mut self, key: StorybookSidebarFrameCacheKey, canvas: Canvas) {
        self.entries.retain(|entry| !entry.matches(&key));
        self.entries
            .push(StorybookSidebarFrameCache::new(key, canvas));
        while self.entries.len() > MAX_SIDEBAR_FRAME_CACHE_ENTRIES {
            self.entries.remove(0);
        }
    }

    pub(super) fn clear(&mut self) {
        self.entries.clear();
    }

    #[cfg(test)]
    pub(super) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[derive(Clone)]
pub(super) struct StorybookSidebarInteractionSurfaceCache {
    key: StorybookSidebarFrameCacheKey,
    surface: SidebarInteractionSurface,
}

impl StorybookSidebarInteractionSurfaceCache {
    pub(super) fn new(
        key: StorybookSidebarFrameCacheKey,
        surface: SidebarInteractionSurface,
    ) -> Self {
        Self { key, surface }
    }

    pub(super) fn matches(&self, key: &StorybookSidebarFrameCacheKey) -> bool {
        &self.key == key
    }

    pub(super) fn canvas_interaction_at(
        &self,
        x: f32,
        y: f32,
        height: usize,
    ) -> SidebarInteraction {
        self.surface.canvas_interaction_at(x, y, height)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StorybookSidebarFrameCacheKey {
    width: usize,
    height: usize,
    selected_index: usize,
    scroll: StorybookSidebarScroll,
    file_tree_state: FileTreeState,
    settings_state: StorybookSettingsState,
    dark: bool,
    interaction: ViewerInteractionConfig,
    typography: ViewerTypographyConfig,
    scene: Option<StorybookSidebarSceneKey>,
}

impl StorybookSidebarFrameCacheKey {
    pub(super) fn new(input: StorybookSidebarFrameCacheKeyInput<'_>) -> Self {
        Self {
            width: input.width,
            height: input.height,
            selected_index: input.selected_index,
            scroll: input.scroll,
            file_tree_state: input.file_tree_state,
            settings_state: input.settings_state,
            dark: input.dark,
            interaction: input.interaction,
            typography: input.typography,
            scene: input.scene.map(StorybookSidebarSceneKey::from),
        }
    }
}

pub(super) struct StorybookSidebarFrameCacheKeyInput<'a> {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) selected_index: usize,
    pub(super) scroll: StorybookSidebarScroll,
    pub(super) file_tree_state: FileTreeState,
    pub(super) settings_state: StorybookSettingsState,
    pub(super) dark: bool,
    pub(super) interaction: ViewerInteractionConfig,
    pub(super) typography: ViewerTypographyConfig,
    pub(super) scene: Option<&'a PreviewScene>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StorybookSidebarSceneKey {
    mode: ViewerMode,
    slide_current: usize,
    slide_max: usize,
    nodes: usize,
    loaded_assets: usize,
    failed_assets: usize,
    image_surfaces: usize,
    surface: Option<StorybookSurfaceKey>,
}

impl From<&PreviewScene> for StorybookSidebarSceneKey {
    fn from(scene: &PreviewScene) -> Self {
        Self {
            mode: scene.mode.clone(),
            slide_current: scene.slideshow_current_page,
            slide_max: scene.slideshow_max_page,
            nodes: scene.node_count,
            loaded_assets: scene.loaded_asset_count,
            failed_assets: scene.failed_asset_count,
            image_surfaces: scene.image_surface_count,
            surface: scene.surface.as_ref().map(StorybookSurfaceKey::from),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StorybookSurfaceKey {
    width: u32,
    height: u32,
}

impl From<&katana_document_viewer::PreviewSurfaceImage> for StorybookSurfaceKey {
    fn from(surface: &katana_document_viewer::PreviewSurfaceImage) -> Self {
        Self {
            width: surface.width,
            height: surface.height,
        }
    }
}
