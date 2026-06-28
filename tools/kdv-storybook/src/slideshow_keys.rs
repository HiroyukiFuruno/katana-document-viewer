use crate::layout::preview_viewport_height;
use crate::preview::PreviewScene;
use katana_document_viewer::ViewerMode;
use minifb::{Key, KeyRepeat, Window};

pub struct StorybookSlideshowKeys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlideshowKeyPress {
    ToggleMode,
    NextPage,
    PreviousPage,
    Close,
}

#[derive(Debug, Clone, Copy)]
struct SlideshowSceneState {
    current_page: usize,
    max_page: usize,
}

impl StorybookSlideshowKeys {
    pub(crate) fn viewport_height_for_window(window_height: usize) -> f32 {
        preview_viewport_height(window_height) as f32
    }

    pub fn apply(
        window: &Window,
        mode: &mut ViewerMode,
        scroll_y: &mut f32,
        scene: Option<&PreviewScene>,
        viewport_height: f32,
    ) -> bool {
        let Some(pressed) = Self::pressed_key(window) else {
            return false;
        };
        Self::apply_pressed(
            pressed,
            mode,
            scroll_y,
            scene.map(SlideshowSceneState::from),
            viewport_height,
        )
    }

    fn pressed_key(window: &Window) -> Option<SlideshowKeyPress> {
        if window.is_key_pressed(Key::M, KeyRepeat::No) {
            return Some(SlideshowKeyPress::ToggleMode);
        }
        if Self::has_pressed(window, Self::close_keys()) {
            return Some(SlideshowKeyPress::Close);
        }
        if Self::has_pressed(window, Self::next_keys()) {
            return Some(SlideshowKeyPress::NextPage);
        }
        if Self::has_pressed(window, Self::previous_keys()) {
            return Some(SlideshowKeyPress::PreviousPage);
        }
        None
    }

    fn has_pressed(window: &Window, keys: &[Key]) -> bool {
        keys.iter()
            .any(|key| window.is_key_pressed(*key, KeyRepeat::No))
    }

    fn close_keys() -> &'static [Key] {
        &[Key::Escape]
    }

    fn next_keys() -> &'static [Key] {
        &[Key::N, Key::PageDown, Key::Right, Key::Space]
    }

    fn previous_keys() -> &'static [Key] {
        &[Key::P, Key::PageUp, Key::Left]
    }

    fn apply_pressed(
        pressed: SlideshowKeyPress,
        mode: &mut ViewerMode,
        scroll_y: &mut f32,
        scene: Option<SlideshowSceneState>,
        viewport_height: f32,
    ) -> bool {
        match pressed {
            SlideshowKeyPress::ToggleMode => Self::toggle_mode(mode, scroll_y),
            SlideshowKeyPress::NextPage => {
                Self::move_page(mode, scroll_y, scene, viewport_height, PageDelta::Next)
            }
            SlideshowKeyPress::PreviousPage => {
                Self::move_page(mode, scroll_y, scene, viewport_height, PageDelta::Previous)
            }
            SlideshowKeyPress::Close => Self::close_mode(mode, scroll_y),
        }
    }

    fn toggle_mode(mode: &mut ViewerMode, scroll_y: &mut f32) -> bool {
        *mode = match mode {
            ViewerMode::Document => ViewerMode::Slideshow,
            ViewerMode::Slideshow => ViewerMode::Document,
        };
        *scroll_y = 0.0;
        true
    }

    fn close_mode(mode: &mut ViewerMode, scroll_y: &mut f32) -> bool {
        if *mode != ViewerMode::Slideshow {
            return false;
        }
        *mode = ViewerMode::Document;
        *scroll_y = 0.0;
        true
    }

    fn move_page(
        mode: &ViewerMode,
        scroll_y: &mut f32,
        scene: Option<SlideshowSceneState>,
        viewport_height: f32,
        delta: PageDelta,
    ) -> bool {
        if *mode != ViewerMode::Slideshow || viewport_height <= 0.0 {
            return false;
        }
        let Some(scene) = scene else {
            return false;
        };
        let target = match delta {
            PageDelta::Next => (scene.current_page + 1).min(scene.max_page),
            PageDelta::Previous => scene.current_page.saturating_sub(1),
        };
        if target == scene.current_page {
            return false;
        }
        *scroll_y = target as f32 * viewport_height;
        true
    }
}

impl From<&PreviewScene> for SlideshowSceneState {
    fn from(scene: &PreviewScene) -> Self {
        Self {
            current_page: scene.slideshow_current_page,
            max_page: scene.slideshow_max_page,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PageDelta {
    Next,
    Previous,
}

#[cfg(test)]
#[path = "slideshow_keys_tests.rs"]
mod tests;
