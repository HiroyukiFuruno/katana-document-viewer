pub const KATANA_ACTIVITY_RAIL_WIDTH: usize = 32;
pub const STORYBOOK_FILE_TREE_PANEL_WIDTH: usize = 468;
pub const SIDEBAR_WIDTH: usize = KATANA_ACTIVITY_RAIL_WIDTH + STORYBOOK_FILE_TREE_PANEL_WIDTH;
pub const HEADER_HEIGHT: usize = 44;
pub const SIDEBAR_CONTENT_INSET: usize = 8;
pub const PREVIEW_CONTENT_INSET: usize = 16;
pub const STATUS_BAR_HEIGHT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct StorybookPreviewArea {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) scroll_y: f32,
}

impl StorybookPreviewArea {
    pub(crate) fn for_window(window_width: usize, window_height: usize, scroll_y: f32) -> Self {
        Self {
            x: SIDEBAR_WIDTH + PREVIEW_CONTENT_INSET,
            y: HEADER_HEIGHT + PREVIEW_CONTENT_INSET,
            width: preview_content_width(window_width),
            height: preview_viewport_height(window_height),
            scroll_y,
        }
    }

    pub(crate) fn document_point(self, x: f32, y: f32) -> Option<(f32, f32)> {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        let left = self.x as f32;
        let top = self.y as f32;
        let right = left + self.width as f32;
        let bottom = top + self.height as f32;
        if x < left || y < top || x >= right || y >= bottom {
            return None;
        }
        Some((x - left, y - top + self.scroll_y))
    }

    pub(crate) fn canvas_point_for_document_point(self, x: f32, y: f32) -> (f32, f32) {
        (self.x as f32 + x, self.y as f32 + y - self.scroll_y)
    }
}

pub(crate) fn sidebar_content_width() -> usize {
    STORYBOOK_FILE_TREE_PANEL_WIDTH.saturating_sub(SIDEBAR_CONTENT_INSET.saturating_mul(2))
}

pub(crate) fn sidebar_content_x() -> usize {
    KATANA_ACTIVITY_RAIL_WIDTH.saturating_add(SIDEBAR_CONTENT_INSET)
}

pub(crate) fn sidebar_content_height(window_height: usize) -> usize {
    window_height.saturating_sub(SIDEBAR_CONTENT_INSET.saturating_mul(2))
}

pub(crate) fn sidebar_content_local_y(pointer_y: f32, window_height: usize) -> Option<u32> {
    if !pointer_y.is_finite() {
        return None;
    }
    let top = SIDEBAR_CONTENT_INSET as f32;
    let bottom = window_height.saturating_sub(SIDEBAR_CONTENT_INSET) as f32;
    if pointer_y < top || pointer_y >= bottom {
        return None;
    }
    Some((pointer_y - top).floor() as u32)
}

pub(crate) fn sidebar_content_local_x(pointer_x: f32) -> Option<u32> {
    if !pointer_x.is_finite() {
        return None;
    }
    let left = sidebar_content_x() as f32;
    let right = SIDEBAR_WIDTH.saturating_sub(SIDEBAR_CONTENT_INSET) as f32;
    if pointer_x < left || pointer_x >= right {
        return None;
    }
    Some((pointer_x - left).floor() as u32)
}

pub(crate) fn sidebar_content_contains(
    pointer_x: f32,
    pointer_y: f32,
    window_height: usize,
) -> bool {
    sidebar_content_local_x(pointer_x).is_some()
        && sidebar_content_local_y(pointer_y, window_height).is_some()
}

pub(crate) fn preview_content_width(window_width: usize) -> usize {
    window_width
        .saturating_sub(SIDEBAR_WIDTH + PREVIEW_CONTENT_INSET * 2)
        .max(1)
}

pub(crate) fn preview_viewport_height(window_height: usize) -> usize {
    window_height.saturating_sub(HEADER_HEIGHT + PREVIEW_CONTENT_INSET * 2 + STATUS_BAR_HEIGHT)
}

#[cfg(test)]
pub(crate) fn preview_content_height(window_height: usize) -> usize {
    preview_viewport_height(window_height)
}

pub(crate) fn preview_status_y(window_height: usize) -> usize {
    window_height.saturating_sub(STATUS_BAR_HEIGHT.saturating_sub(8))
}

#[cfg(test)]
mod tests {
    use super::{
        KATANA_ACTIVITY_RAIL_WIDTH, PREVIEW_CONTENT_INSET, SIDEBAR_CONTENT_INSET, SIDEBAR_WIDTH,
        STATUS_BAR_HEIGHT, STORYBOOK_FILE_TREE_PANEL_WIDTH, StorybookPreviewArea, preview_status_y,
        sidebar_content_local_x, sidebar_content_local_y, sidebar_content_width, sidebar_content_x,
    };

    #[test]
    fn sidebar_local_coordinates_floor_like_kuc_window_normalizer() {
        assert_eq!(
            Some(0),
            sidebar_content_local_x(sidebar_content_x() as f32 + 0.99)
        );
        assert_eq!(
            Some(0),
            sidebar_content_local_y(SIDEBAR_CONTENT_INSET as f32 + 0.99, 900)
        );
    }

    #[test]
    fn preview_area_maps_canvas_and_document_coordinates_from_one_contract() {
        let area = StorybookPreviewArea::for_window(1000, 900, 120.0);
        let (canvas_x, canvas_y) = area.canvas_point_for_document_point(24.0, 180.0);

        assert_eq!(
            SIDEBAR_WIDTH as f32 + PREVIEW_CONTENT_INSET as f32 + 24.0,
            canvas_x
        );
        assert_eq!(44.0 + PREVIEW_CONTENT_INSET as f32 + 60.0, canvas_y);
        assert_eq!(Some((24.0, 180.0)), area.document_point(canvas_x, canvas_y));
    }

    #[test]
    fn preview_area_reserves_status_bar_at_bottom() {
        let area = StorybookPreviewArea::for_window(1280, 900, 0.0);
        let status_y = preview_status_y(900);

        assert_eq!(
            900 - 44 - PREVIEW_CONTENT_INSET * 2 - STATUS_BAR_HEIGHT,
            area.height
        );
        assert!(
            area.y + area.height < status_y,
            "status line must be outside the scrollable preview area"
        );
    }

    #[test]
    fn sidebar_width_keeps_large_window_menu_readable() {
        let window_width = 2048;

        assert_eq!(
            500, SIDEBAR_WIDTH,
            "Storybook sidebar must keep FileTree and settings readable on desktop windows"
        );
        assert!(
            SIDEBAR_WIDTH * 100 >= window_width * 24,
            "sidebar must not collapse below the readable desktop menu ratio"
        );
    }

    #[test]
    fn sidebar_content_starts_after_activity_rail() {
        assert_eq!(
            KATANA_ACTIVITY_RAIL_WIDTH + SIDEBAR_CONTENT_INSET,
            sidebar_content_x(),
            "FileTree/Settings content must not occupy the Katana activity rail"
        );
        assert_eq!(
            STORYBOOK_FILE_TREE_PANEL_WIDTH - SIDEBAR_CONTENT_INSET * 2,
            sidebar_content_width()
        );
        assert_eq!(
            None,
            sidebar_content_local_x((KATANA_ACTIVITY_RAIL_WIDTH / 2) as f32),
            "activity rail clicks must not be treated as FileTree/Settings clicks"
        );
    }

    #[test]
    fn preview_content_width_tracks_window_host_width() {
        assert_eq!(
            1516,
            super::preview_content_width(2048),
            "Storybook must pass the full preview host width to KUC so all Markdown block rows can track resize"
        );
        assert_eq!(
            748,
            super::preview_content_width(1280),
            "Storybook must not keep Markdown content at a fixed split-panel width after resize"
        );
    }
}
