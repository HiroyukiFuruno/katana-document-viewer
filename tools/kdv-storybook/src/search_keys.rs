use crate::preview::PreviewScene;
use crate::scroll::StorybookScroll;
use katana_document_viewer::{ViewerSearchDirection, ViewerSearchEngine, ViewerSearchState};
use minifb::{Key, KeyRepeat, Window};

const STORYBOOK_SEARCH_QUERY: &str = "Direct";

pub struct StorybookSearchKeys;

impl StorybookSearchKeys {
    pub fn apply(
        window: &Window,
        search: &mut ViewerSearchState,
        scene: Option<&PreviewScene>,
        scroll_y: &mut f32,
        viewport_height: f32,
    ) -> bool {
        let Some(pressed) = Self::pressed_key(window) else {
            return false;
        };
        Self::apply_pressed(pressed, search, scene, scroll_y, viewport_height)
    }

    fn pressed_key(window: &Window) -> Option<SearchKeyPress> {
        if window.is_key_pressed(Key::F, KeyRepeat::No) {
            return Some(SearchKeyPress::Toggle);
        }
        if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            return Some(SearchKeyPress::Next);
        }
        if window.is_key_pressed(Key::Backspace, KeyRepeat::No) {
            return Some(SearchKeyPress::Previous);
        }
        None
    }

    fn apply_pressed(
        pressed: SearchKeyPress,
        search: &mut ViewerSearchState,
        scene: Option<&PreviewScene>,
        scroll_y: &mut f32,
        viewport_height: f32,
    ) -> bool {
        match pressed {
            SearchKeyPress::Toggle => Self::toggle_search(search),
            SearchKeyPress::Next => Self::navigate(
                search,
                scene,
                scroll_y,
                viewport_height,
                ViewerSearchDirection::Next,
            ),
            SearchKeyPress::Previous => Self::navigate(
                search,
                scene,
                scroll_y,
                viewport_height,
                ViewerSearchDirection::Previous,
            ),
        }
    }

    fn toggle_search(search: &mut ViewerSearchState) -> bool {
        if search.query.is_empty() {
            *search = ViewerSearchEngine::state(STORYBOOK_SEARCH_QUERY, Vec::new(), None);
            return true;
        }
        *search = ViewerSearchState::default();
        true
    }

    fn navigate(
        search: &mut ViewerSearchState,
        scene: Option<&PreviewScene>,
        scroll_y: &mut f32,
        viewport_height: f32,
        direction: ViewerSearchDirection,
    ) -> bool {
        if search.query.is_empty() {
            return false;
        }
        let Some(scene) = scene else {
            return false;
        };
        if scene.search_targets.is_empty() {
            return false;
        }
        let state = ViewerSearchEngine::state(
            search.query.clone(),
            scene.search_targets.clone(),
            search.current_index,
        );
        let Some(command) = ViewerSearchEngine::navigate(&state, direction) else {
            return false;
        };
        let scrollable_content_height = scene
            .content_height
            .max(command.scroll.target.rect.y + viewport_height);
        *scroll_y = StorybookScroll::clamp_offset(
            command.scroll.target.rect.y,
            scrollable_content_height,
            viewport_height,
        );
        *search = ViewerSearchEngine::state(
            search.query.clone(),
            scene.search_targets.clone(),
            Some(command.target.index),
        );
        true
    }
}

#[derive(Debug, Clone, Copy)]
enum SearchKeyPress {
    Toggle,
    Next,
    Previous,
}

#[cfg(test)]
#[path = "search_keys_tests.rs"]
mod tests;
