use super::StorybookPointer;
use crate::layout::StorybookPreviewArea;
use crate::preview::PreviewScene;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DocumentPoint {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl DocumentPoint {
    pub(super) fn from_scene_pointer(
        scene: &PreviewScene,
        pointer: StorybookPointer,
        scroll_y: f32,
        window_width: usize,
        window_height: usize,
    ) -> Option<Self> {
        Self::from_scene_position(
            scene,
            pointer.x,
            pointer.y,
            scroll_y,
            window_width,
            window_height,
        )
    }

    pub(crate) fn from_scene_position(
        scene: &PreviewScene,
        x: f32,
        y: f32,
        scroll_y: f32,
        window_width: usize,
        window_height: usize,
    ) -> Option<Self> {
        if scene.fullscreen_diagram_active() {
            return Self::from_fullscreen_position(x, y, window_width, window_height);
        }
        Self::from_position(
            x,
            y,
            Self::document_scroll_y(scene, scroll_y),
            window_width,
            window_height,
        )
    }

    fn from_fullscreen_position(
        x: f32,
        y: f32,
        window_width: usize,
        window_height: usize,
    ) -> Option<Self> {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        if x < 0.0 || y < 0.0 || x >= window_width as f32 || y >= window_height as f32 {
            return None;
        }
        Some(Self { x, y })
    }

    fn from_position(
        x: f32,
        y: f32,
        scroll_y: f32,
        window_width: usize,
        window_height: usize,
    ) -> Option<Self> {
        let area = StorybookPreviewArea::for_window(window_width, window_height, scroll_y);
        let (x, y) = area.document_point(x, y)?;
        Some(Self { x, y })
    }

    pub(crate) fn effective_scroll_y(scene: &PreviewScene, requested_scroll_y: f32) -> f32 {
        if scene
            .diagram_viewports
            .values()
            .any(|state| state.fullscreen_open)
        {
            return 0.0;
        }
        let tree_offset = scene.tree.root().props().scroll_area.offset_y as f32;
        requested_scroll_y.max(tree_offset)
    }

    fn document_scroll_y(scene: &PreviewScene, requested_scroll_y: f32) -> f32 {
        Self::effective_scroll_y(scene, requested_scroll_y)
    }
}
