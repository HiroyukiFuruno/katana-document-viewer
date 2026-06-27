use super::StorybookWindow;
use super::window_loop::render_size_for_window;
use crate::interaction_keys::StorybookInteractionKeys;
use crate::search_keys::StorybookSearchKeys as Search;
use crate::settings_action::{
    StorybookSettingsAction, StorybookSettingsField, StorybookSettingsRequest,
};
use crate::slideshow_keys::StorybookSlideshowKeys as Slides;
use katana_document_viewer::{ViewerMode, ViewerStateEngine};
use katana_ui_core::molecule::SettingsListAction;
use minifb::{Key, KeyRepeat, Window};

impl StorybookWindow {
    pub(super) fn apply_keyboard(
        &mut self,
        window: &Window,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut changed = false;
        changed |= self.apply_text_selection_copy(window);
        changed |= self.move_selection(window, Key::Down, 1);
        changed |= self.move_selection(window, Key::Up, -1);
        changed |= self.apply_settings_key(window, Key::D, StorybookSettingsField::Dark)?;
        let (_, height) = render_size_for_window_size(window);
        let viewport_height = Slides::viewport_height_for_window(height);
        let previous_mode = self.mode.clone();
        let slides_changed = {
            let scene = self.scene.as_ref();
            Slides::apply(
                window,
                &mut self.mode,
                &mut self.scroll_y,
                scene,
                viewport_height,
            )
        };
        changed |= slides_changed;
        let search_changed = {
            let scene = self.scene.as_ref();
            Search::apply(
                window,
                &mut self.search,
                scene,
                &mut self.scroll_y,
                viewport_height,
            )
        };
        let mode_changed = slides_changed && previous_mode != self.mode;
        if slides_changed {
            if mode_changed {
                self.invalidate_lazy_scene(false);
            } else {
                self.apply_slideshow_page_scroll(viewport_height);
            }
        }
        if search_changed && !mode_changed {
            self.invalidate_loaded_scene();
        }
        changed |= search_changed;
        let (width, height) = render_size_for_window_size(window);
        changed |= StorybookInteractionKeys::apply(window, |field| {
            self.apply_settings_field(field, width, height)
        })?;
        Ok(changed)
    }

    fn apply_text_selection_copy(&mut self, window: &Window) -> bool {
        if !window.is_key_pressed(Key::C, KeyRepeat::No) || !Self::copy_modifier_down(window) {
            return false;
        }
        let (width, height) = render_size_for_window_size(window);
        self.copy_selected_text_to_clipboard(width, height)
    }

    pub(super) fn copy_selected_text_to_clipboard(&mut self, width: usize, height: usize) -> bool {
        let payload = self
            .selected_text_payload(width, height)
            .filter(|value| !value.trim().is_empty());
        let Some(payload) = payload else {
            return false;
        };
        if let Err(error) = super::window_command::write_clipboard_text(&payload) {
            eprintln!("[kdv-storybook] clipboard write failed: {error}");
            return false;
        }
        self.last_command_label = "copy-selection".to_string();
        true
    }

    fn copy_modifier_down(window: &Window) -> bool {
        window.is_key_down(Key::LeftSuper)
            || window.is_key_down(Key::RightSuper)
            || window.is_key_down(Key::LeftCtrl)
            || window.is_key_down(Key::RightCtrl)
    }

    pub(super) fn selected_text_payload(&mut self, width: usize, height: usize) -> Option<String> {
        if !self.interaction.selection_enabled {
            return None;
        }
        if self.text_selection_start == self.text_selection_end {
            return None;
        }
        if !self.frame_cache_matches(width, height) {
            let canvas = self.render_canvas(width, height);
            self.frame_cache = Some(super::StorybookFrameCache::new(canvas));
        }
        self.frame_cache.as_ref().and_then(|frame| {
            frame
                .canvas()
                .copy_text_in_selection(self.text_selection_start, self.text_selection_end)
        })
    }

    pub(super) fn apply_settings_field(
        &mut self,
        field: StorybookSettingsField,
        width: usize,
        height: usize,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        StorybookSettingsAction::apply_field(
            field,
            StorybookSettingsRequest {
                scene: self.scene.as_ref(),
                dark: &mut self.dark,
                mode: &mut self.mode,
                interaction: &mut self.interaction,
                typography: &mut self.typography,
                settings_state: &mut self.settings_state,
                width,
                height,
            },
        )?;
        self.sidebar_interaction_cache = None;
        self.sidebar_interaction_surface_cache = None;
        match field {
            StorybookSettingsField::Dark | StorybookSettingsField::Theme => {
                self.invalidate_lazy_scene(true);
            }
            StorybookSettingsField::Mode | StorybookSettingsField::PreviewFontSize => {
                self.invalidate_lazy_scene(false);
            }
            StorybookSettingsField::Hover => {
                self.hovered_node_id = None;
            }
            StorybookSettingsField::Selection
            | StorybookSettingsField::ImageControls
            | StorybookSettingsField::DiagramControls
            | StorybookSettingsField::CodeControls => {
                self.invalidate_lazy_scene_preserving_asset_job();
            }
        }
        Ok(true)
    }

    pub(super) fn apply_settings_action(
        &mut self,
        action: SettingsListAction,
        width: usize,
        height: usize,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let result = StorybookSettingsAction::apply_action(
            action,
            StorybookSettingsRequest {
                scene: self.scene.as_ref(),
                dark: &mut self.dark,
                mode: &mut self.mode,
                interaction: &mut self.interaction,
                typography: &mut self.typography,
                settings_state: &mut self.settings_state,
                width,
                height,
            },
        )?;
        if let Some(field) = result.field {
            self.sidebar_interaction_cache = None;
            self.sidebar_interaction_surface_cache = None;
            self.apply_settings_side_effect(field);
        } else {
            self.sidebar_frame_cache.clear();
            self.sidebar_interaction_cache = None;
            self.sidebar_interaction_surface_cache = None;
            self.last_command_label = "toggle-settings-section".to_string();
        }
        Ok(result.changed)
    }

    fn apply_settings_side_effect(&mut self, field: StorybookSettingsField) {
        match field {
            StorybookSettingsField::Dark | StorybookSettingsField::Theme => {
                self.invalidate_lazy_scene(true);
            }
            StorybookSettingsField::Mode | StorybookSettingsField::PreviewFontSize => {
                self.invalidate_lazy_scene(false);
            }
            StorybookSettingsField::Hover => {
                self.hovered_node_id = None;
            }
            StorybookSettingsField::Selection
            | StorybookSettingsField::ImageControls
            | StorybookSettingsField::DiagramControls
            | StorybookSettingsField::CodeControls => {
                self.invalidate_lazy_scene_preserving_asset_job();
            }
        }
    }

    pub(super) fn apply_slideshow_page_scroll(&mut self, viewport_height: f32) {
        if self.mode != ViewerMode::Slideshow {
            return;
        }
        let Some(scene) = self.scene.as_mut() else {
            return;
        };
        let current_page = ViewerStateEngine::page_index_for_scroll(self.scroll_y, viewport_height)
            .min(scene.slideshow_max_page);
        if scene.slideshow_current_page == current_page {
            return;
        }
        scene.slideshow_current_page = current_page;
        self.frame_cache = None;
        self.sidebar_frame_cache.clear();
        self.sidebar_interaction_cache = None;
        self.sidebar_interaction_surface_cache = None;
    }

    fn apply_settings_key(
        &mut self,
        window: &Window,
        key: Key,
        field: StorybookSettingsField,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if !window.is_key_pressed(key, KeyRepeat::No) {
            return Ok(false);
        }
        let (width, height) = render_size_for_window_size(window);
        self.apply_settings_field(field, width, height)
    }

    fn move_selection(&mut self, window: &Window, key: Key, delta: isize) -> bool {
        if !window.is_key_pressed(key, KeyRepeat::Yes) {
            return false;
        }
        let last = self.catalog.fixtures.len().saturating_sub(1) as isize;
        let next = (self.selected_index as isize + delta).clamp(0, last) as usize;
        if next == self.selected_index {
            return false;
        }
        self.selected_index = next;
        self.reset_fixture_state();
        true
    }
}

fn render_size_for_window_size(window: &Window) -> (usize, usize) {
    let (width, height) = window.get_size();
    render_size_for_window(width, height)
}
