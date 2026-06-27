use super::StorybookWindow;
use crate::layout::preview_viewport_height;
use crate::scroll::StorybookScroll;

impl StorybookWindow {
    pub(super) fn is_scroll_y_at_current_scene_bottom(&self) -> bool {
        let Some(scene) = self.scene.as_ref() else {
            return false;
        };
        let window_height = self
            .frame_size
            .map_or(self.args.height, |(_, height)| height);
        let viewport_height = preview_viewport_height(window_height) as f32;
        let max_scroll = StorybookScroll::max_offset(scene.content_height, viewport_height);
        (self.scroll_y - max_scroll).abs() <= 0.5
    }

    pub(super) fn apply_scene_scroll_bounds(
        &mut self,
        content_height: f32,
        viewport_height: f32,
        preserve_bottom_anchor: bool,
    ) -> bool {
        let next = if preserve_bottom_anchor {
            StorybookScroll::max_offset(content_height, viewport_height)
        } else {
            StorybookScroll::clamp_offset(self.scroll_y, content_height, viewport_height)
        };
        let changed = (next - self.scroll_y).abs() > f32::EPSILON;
        self.scroll_y = next;
        changed
    }
}
