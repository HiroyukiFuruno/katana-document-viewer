use super::window_cursor;
use super::window_loop::render_size_for_window;
use super::window_sidebar_frame_cache::{
    StorybookSidebarFrameCacheKey, StorybookSidebarFrameCacheKeyInput,
    StorybookSidebarInteractionSurfaceCache,
};
use super::{
    StorybookDocumentInteractionSurfaceCache, StorybookPointerPosition,
    StorybookSidebarInteractionCache, StorybookWindow,
};
use crate::canvas::Canvas;
use crate::mouse::DocumentPoint;
use crate::mouse::StorybookHostActionHits;
use crate::mouse::task_context_menu::StorybookTaskContextMenu;
use crate::mouse::{
    StorybookHoverState, StorybookMouse, StorybookMouseButton, StorybookMouseCursor,
    StorybookPointer,
};
use crate::sidebar_hit::{SidebarHit, SidebarHitRequest, SidebarHitResult, SidebarInteraction};
use crate::window_coordinates;
use katana_ui_core::molecule::FileTreeAction;
use katana_ui_core::render_model::{UiCursor, UiNodeId};
use minifb::{MouseButton, Window};

impl StorybookWindow {
    pub(super) fn update_pointer_position(&mut self, window: &Window) -> bool {
        let raw = self.current_canvas_mouse_position(window);
        let (width, height) = self.current_canvas_size(window);
        let next = raw.map(|(x, y)| StorybookPointerPosition::new(x, y));
        let pointer_changed = self.pointer_position != next;
        self.pointer_position = next;
        let sidebar_hover_changed = self.update_sidebar_hover(raw, width, height);
        pointer_changed || sidebar_hover_changed
    }

    pub(super) fn apply_cursor(&self, window: &mut Window) {
        let (width, height) = self.current_canvas_size(window);
        let cursor = self.cursor_for_window(window, width, height);
        window_cursor::apply_cursor_style(window, cursor);
    }

    pub(super) fn apply_hover(&mut self, window: &Window) -> bool {
        let (width, height) = self.current_canvas_size(window);
        let Some((x, y)) = self.current_canvas_mouse_position(window) else {
            return self.clear_document_hover_state();
        };
        let next = self.cached_document_hover_state_for_canvas_point(x, y, width, height);
        self.apply_document_hover_state(next)
    }

    pub(super) fn apply_mouse(
        &mut self,
        window: &Window,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let left_down = window.get_mouse_down(MouseButton::Left);
        let raw_position = self.current_canvas_mouse_position(window);
        let pressed = self
            .mouse
            .pressed(left_down, window.get_mouse_down(MouseButton::Right));
        let fullscreen_drag_changed =
            self.update_fullscreen_diagram_drag(left_down, raw_position, pressed);
        let (width, height) = self.current_canvas_size(window);
        let document_diagram_drag_consumed =
            self.update_document_diagram_drag(left_down, raw_position, pressed, width, height);
        let selection_changed =
            if self.fullscreen_diagram_active() || document_diagram_drag_consumed {
                false
            } else {
                self.update_text_selection(left_down, raw_position)
            };
        let Some(button) = pressed else {
            return Ok(selection_changed
                || fullscreen_drag_changed
                || document_diagram_drag_consumed);
        };
        let Some((x, y)) = raw_position else {
            return Ok(selection_changed
                || fullscreen_drag_changed
                || document_diagram_drag_consumed);
        };
        let pointer = StorybookPointer::new(x, y, button);
        Ok(self.apply_canvas_click(pointer, width, height)?
            || selection_changed
            || fullscreen_drag_changed
            || document_diagram_drag_consumed)
    }

    fn update_text_selection(&mut self, left_down: bool, pointer: Option<(f32, f32)>) -> bool {
        if !self.interaction.selection_enabled {
            if self.text_selection_start.is_none()
                && self.text_selection_end.is_none()
                && !self.text_selection_active
            {
                return false;
            }
            self.text_selection_start = None;
            self.text_selection_end = None;
            self.text_selection_active = false;
            self.frame_cache = None;
            return true;
        }
        if !left_down {
            if self.text_selection_active {
                self.text_selection_active = false;
                if self.text_selection_start == self.text_selection_end {
                    self.text_selection_start = None;
                    self.text_selection_end = None;
                }
                return true;
            }
            return false;
        }
        let Some((x, y)) = pointer else {
            return false;
        };
        let point = (x.round().max(0.0) as usize, y.round().max(0.0) as usize);
        let mut changed = false;
        if !self.text_selection_active {
            self.text_selection_start = Some(point);
            self.text_selection_active = true;
            changed = true;
        }
        if self.text_selection_end != Some(point) {
            self.text_selection_end = Some(point);
            changed = true;
        }
        if changed {
            self.frame_cache = None;
        }
        changed
    }

    pub(super) fn apply_text_selection_drag_for_smoke(
        &mut self,
        start: (f32, f32),
        end: (f32, f32),
    ) -> bool {
        let mut changed = self.update_text_selection(true, Some(start));
        changed |= self.update_text_selection(true, Some(end));
        changed |= self.update_text_selection(false, Some(end));
        changed
    }

    pub(super) fn apply_canvas_click(
        &mut self,
        pointer: StorybookPointer,
        width: usize,
        height: usize,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let fullscreen_diagram_active = self
            .scene
            .as_ref()
            .is_some_and(|scene| scene.fullscreen_diagram_active());
        if !fullscreen_diagram_active
            && pointer.button == StorybookMouseButton::Left
            && let Some(hit) = self.sidebar_hit(pointer, width, height)
        {
            return self.apply_sidebar_hit(hit, width, height);
        }
        if !fullscreen_diagram_active
            && pointer.button == StorybookMouseButton::Left
            && let Some(command) = self.context_menu_command(pointer)
        {
            self.apply_viewer_command(&command);
            self.last_command_label = StorybookMouse::command_label(&command).to_string();
            return Ok(true);
        }
        if !fullscreen_diagram_active
            && pointer.button == StorybookMouseButton::Left
            && self.apply_accordion(pointer, width, height)
        {
            return Ok(true);
        }
        let Some(scene) = self.scene.as_ref() else {
            return Ok(false);
        };
        if !fullscreen_diagram_active
            && pointer.button == StorybookMouseButton::Right
            && let Some(menu) =
                StorybookTaskContextMenu::open(scene, self.scroll_y, pointer, width, height)
        {
            self.task_context_menu = Some(menu);
            self.last_command_label = "task-context".to_string();
            return Ok(true);
        }
        let Some(command) =
            StorybookMouse::command_for_click(scene, self.scroll_y, pointer, width, height)
        else {
            return Ok(false);
        };
        self.apply_viewer_command(&command);
        self.last_command_label = StorybookMouse::command_label(&command).to_string();
        Ok(true)
    }

    fn cursor_for_window(&self, window: &Window, width: usize, height: usize) -> UiCursor {
        let Some((x, y)) = self.current_canvas_mouse_position(window) else {
            return UiCursor::Default;
        };
        let sidebar_cursor = self.sidebar_cursor_for_canvas_point(x, y, width, height);
        if sidebar_cursor != UiCursor::Default {
            return sidebar_cursor;
        }
        if let Some(scene) = self.scene.as_ref()
            && DocumentPoint::from_scene_position(scene, x, y, self.scroll_y, width, height)
                .is_some()
        {
            self.document_cursor
        } else {
            UiCursor::Default
        }
    }

    #[cfg(test)]
    pub(super) fn cursor_for_canvas_point(
        &self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> UiCursor {
        let sidebar_cursor = self.sidebar_cursor_for_canvas_point(x, y, width, height);
        if sidebar_cursor != UiCursor::Default {
            return sidebar_cursor;
        }
        let Some(scene) = self.scene.as_ref() else {
            return UiCursor::Default;
        };
        self.document_hover_state_for_scene(scene, x, y, width, height)
            .cursor
    }

    fn sidebar_cursor_for_canvas_point(
        &self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> UiCursor {
        self.sidebar_interaction_for_canvas_point(x, y, width, height)
            .cursor
    }

    pub(crate) fn update_document_hover_for_canvas_point(
        &mut self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> bool {
        let next = self.cached_document_hover_state_for_canvas_point(x, y, width, height);
        self.apply_document_hover_state(next)
    }

    fn cached_document_hover_state_for_canvas_point(
        &mut self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> StorybookHoverState {
        let Some(point) = self.document_point_for_canvas_point(x, y, width, height) else {
            return StorybookHoverState::default();
        };
        self.ensure_document_interaction_surface_cache(width, height);
        let Some(scene) = self.scene.as_ref() else {
            return StorybookHoverState::default();
        };
        let Some(cache) = self.document_interaction_surface_cache.as_ref() else {
            return self.document_hover_state_for_scene(scene, x, y, width, height);
        };
        let mut state = StorybookMouseCursor::hover_state_for_cached_hits(
            scene,
            point,
            cache.hits.as_slice(),
            &cache.node_hits,
        );
        if !self.interaction.hover_highlight_enabled {
            state.hovered_node_id = None;
        }
        state
    }

    fn document_point_for_canvas_point(
        &self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> Option<DocumentPoint> {
        DocumentPoint::from_scene_position(self.scene.as_ref()?, x, y, self.scroll_y, width, height)
    }

    fn ensure_document_interaction_surface_cache(&mut self, width: usize, height: usize) {
        let Some(scene) = self.scene.as_ref() else {
            self.document_interaction_surface_cache = None;
            return;
        };
        if self
            .document_interaction_surface_cache
            .as_ref()
            .is_some_and(|cache| cache.matches(width, height, self.scroll_y, scene))
        {
            return;
        }
        let preview_width = StorybookHostActionHits::interaction_width(scene, width);
        let preview_height = StorybookHostActionHits::interaction_height(scene, height);
        let (hits, node_hits) =
            StorybookHostActionHits::viewport_interaction_hits_for_preview_width(
                scene,
                preview_width,
                preview_height,
                self.scroll_y,
            );
        self.document_interaction_surface_cache =
            Some(StorybookDocumentInteractionSurfaceCache::new(
                width,
                height,
                self.scroll_y,
                scene,
                hits,
                node_hits,
            ));
    }

    fn document_hover_state_for_scene(
        &self,
        scene: &crate::preview::PreviewScene,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> StorybookHoverState {
        let mut state =
            StorybookMouseCursor::hover_state_for_hover(scene, self.scroll_y, x, y, width, height);
        if !self.interaction.hover_highlight_enabled {
            state.hovered_node_id = None;
        }
        state
    }

    fn apply_document_hover_state(&mut self, next: StorybookHoverState) -> bool {
        self.document_cursor = next.cursor;
        if self.hovered_node_id == next.hovered_node_id
            && self.hovered_action_node_id == next.hovered_action_node_id
        {
            return false;
        }
        self.hovered_node_id = next.hovered_node_id;
        self.hovered_action_node_id = next.hovered_action_node_id;
        true
    }

    pub(crate) fn clear_document_hover_state(&mut self) -> bool {
        self.document_cursor = UiCursor::Default;
        if self.hovered_node_id.is_none() && self.hovered_action_node_id.is_none() {
            return false;
        }
        self.hovered_node_id = None;
        self.hovered_action_node_id = None;
        true
    }

    #[cfg(test)]
    pub(super) fn update_sidebar_tree_hover_for_canvas_point(
        &mut self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> bool {
        self.update_sidebar_hover(Some((x, y)), width, height)
    }

    #[cfg(test)]
    pub(super) fn update_sidebar_settings_hover_for_canvas_point(
        &mut self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> bool {
        self.update_sidebar_hover(Some((x, y)), width, height)
    }

    pub(super) fn update_sidebar_hover(
        &mut self,
        pointer: Option<(f32, f32)>,
        width: usize,
        height: usize,
    ) -> bool {
        let interaction = pointer
            .map(|(x, y)| self.cache_sidebar_interaction_for_canvas_point(x, y, width, height))
            .unwrap_or_default();
        let tree_hover_changed =
            self.update_sidebar_tree_hover_state(interaction.hovered_file_item_id);
        let settings_hover_changed =
            self.update_sidebar_settings_hover_state(interaction.hovered_settings_node_id);
        let changed = tree_hover_changed || settings_hover_changed;
        if changed {
            self.sidebar_frame_cache.clear();
        }
        changed
    }

    fn update_sidebar_tree_hover_state(&mut self, next: Option<String>) -> bool {
        if self.file_tree_state.hovered_item_id() == next.as_deref() {
            return false;
        }
        self.file_tree_state.set_hovered_item(next);
        true
    }

    fn update_sidebar_settings_hover_state(&mut self, next: Option<UiNodeId>) -> bool {
        self.settings_state.set_hovered_node_id(next)
    }

    fn context_menu_command(
        &mut self,
        pointer: StorybookPointer,
    ) -> Option<katana_document_viewer::ViewerCommand> {
        let menu = self.task_context_menu.take()?;
        menu.command_for_pointer(pointer)
    }

    fn sidebar_hit(
        &mut self,
        pointer: StorybookPointer,
        width: usize,
        height: usize,
    ) -> Option<SidebarHitResult> {
        self.cache_sidebar_interaction_for_canvas_point(pointer.x, pointer.y, width, height)
            .action
    }

    fn sidebar_interaction_for_canvas_point(
        &self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> SidebarInteraction {
        let pointer = StorybookPointerPosition::new(x, y);
        let key = self.sidebar_interaction_surface_cache_key(width, height);
        if let Some(interaction) = self
            .sidebar_interaction_cache
            .as_ref()
            .and_then(|cache| cache.interaction_for(pointer, width, height, &key))
        {
            return interaction;
        }
        if let Some(cache) = self
            .sidebar_interaction_surface_cache
            .as_ref()
            .filter(|cache| cache.matches(&key))
        {
            return cache.canvas_interaction_at(x, y, height);
        }
        self.compute_sidebar_interaction_for_canvas_point(x, y, width, height)
    }

    fn cache_sidebar_interaction_for_canvas_point(
        &mut self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> SidebarInteraction {
        let pointer = StorybookPointerPosition::new(x, y);
        let key = self.sidebar_interaction_surface_cache_key(width, height);
        if let Some(interaction) = self
            .sidebar_interaction_cache
            .as_ref()
            .and_then(|cache| cache.interaction_for(pointer, width, height, &key))
        {
            return interaction;
        }
        let interaction =
            self.cached_sidebar_interaction_for_canvas_point_with_key(x, y, width, height, &key);
        self.sidebar_interaction_cache = Some(StorybookSidebarInteractionCache::new(
            pointer,
            width,
            height,
            key,
            interaction.clone(),
        ));
        interaction
    }

    fn cached_sidebar_interaction_for_canvas_point_with_key(
        &mut self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
        key: &StorybookSidebarFrameCacheKey,
    ) -> SidebarInteraction {
        if !self
            .sidebar_interaction_surface_cache
            .as_ref()
            .is_some_and(|cache| cache.matches(key))
        {
            let surface = SidebarHit::interaction_surface(&SidebarHitRequest {
                fixtures: &self.catalog.fixtures,
                selected_index: self.selected_index,
                scene: self.scene.as_ref(),
                dark: self.dark,
                interaction: &self.interaction,
                typography: self.typography,
                settings_state: self.settings_state.clone(),
                file_tree_state: self.file_tree_state.clone(),
                scroll: self.sidebar_scroll,
                width,
                height,
            });
            #[cfg(test)]
            {
                self.sidebar_interaction_surface_cache_misses += 1;
            }
            self.sidebar_interaction_surface_cache = Some(
                StorybookSidebarInteractionSurfaceCache::new(key.clone(), surface),
            );
        }
        self.sidebar_interaction_surface_cache.as_ref().map_or_else(
            || self.compute_sidebar_interaction_for_canvas_point(x, y, width, height),
            |cache| cache.canvas_interaction_at(x, y, height),
        )
    }

    fn sidebar_interaction_surface_cache_key(
        &self,
        width: usize,
        height: usize,
    ) -> StorybookSidebarFrameCacheKey {
        let mut file_tree_state = self.file_tree_state.clone();
        file_tree_state.set_hovered_item(None);
        let mut settings_state = self.settings_state.clone();
        settings_state.set_hovered_node_id(None);
        StorybookSidebarFrameCacheKey::new(StorybookSidebarFrameCacheKeyInput {
            width,
            height,
            selected_index: self.selected_index,
            scroll: self.sidebar_scroll,
            file_tree_state,
            settings_state,
            dark: self.dark,
            interaction: self.interaction.clone(),
            typography: self.typography,
            scene: self.scene.as_ref(),
        })
    }

    pub(super) fn compute_sidebar_interaction_for_canvas_point(
        &self,
        x: f32,
        y: f32,
        width: usize,
        height: usize,
    ) -> SidebarInteraction {
        SidebarHit::interaction(
            x,
            y,
            SidebarHitRequest {
                fixtures: &self.catalog.fixtures,
                selected_index: self.selected_index,
                scene: self.scene.as_ref(),
                dark: self.dark,
                interaction: &self.interaction,
                typography: self.typography,
                settings_state: self.settings_state.clone(),
                file_tree_state: self.file_tree_state.clone(),
                scroll: self.sidebar_scroll,
                width,
                height,
            },
        )
    }

    pub(super) fn apply_sidebar_hit(
        &mut self,
        hit: SidebarHitResult,
        width: usize,
        height: usize,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match hit {
            SidebarHitResult::FileTree(action) => {
                self.apply_file_tree_action(action, width, height)
            }
            SidebarHitResult::SettingsAction(action) => {
                self.apply_settings_action(action, width, height)
            }
        }
    }

    fn apply_file_tree_action(
        &mut self,
        action: FileTreeAction,
        width: usize,
        height: usize,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match action {
            FileTreeAction::SelectFile { file_id } => {
                if !self.select_fixture_by_id(&file_id) {
                    return Ok(false);
                }
                if self.catalog.fixtures[self.selected_index].path.is_file() {
                    self.update_scene_deferred_asset_job(width, height)?;
                }
                Ok(true)
            }
            FileTreeAction::ToggleDirectory { directory_id } => {
                self.file_tree_state.toggle_directory(directory_id);
                self.sidebar_interaction_cache = None;
                self.sidebar_interaction_surface_cache = None;
                self.last_command_label = "toggle-directory".to_string();
                Ok(true)
            }
            FileTreeAction::FocusItem { item_id } => {
                self.sidebar_interaction_cache = None;
                self.sidebar_interaction_surface_cache = None;
                self.last_command_label = format!("focus:{item_id}");
                Ok(true)
            }
            FileTreeAction::None => Ok(false),
        }
    }

    fn select_fixture(&mut self, index: usize) {
        if self.selected_index == index {
            return;
        }
        self.selected_index = index;
        self.reset_fixture_state();
        self.last_command_label = "select-file".to_string();
    }

    fn select_fixture_by_id(&mut self, file_id: &str) -> bool {
        let Some(index) = self
            .catalog
            .fixtures
            .iter()
            .position(|fixture| fixture.label == file_id)
        else {
            return false;
        };
        self.select_fixture(index);
        true
    }

    pub(crate) fn current_canvas_mouse_position(&self, window: &Window) -> Option<(f32, f32)> {
        let (surface_width, surface_height) = self.current_surface_size(window);
        let (canvas_width, canvas_height) = self.current_canvas_size(window);
        window_coordinates::canvas_mouse_position(
            window,
            surface_width,
            surface_height,
            canvas_width,
            canvas_height,
        )
    }

    pub(crate) fn current_canvas_size(&self, window: &Window) -> (usize, usize) {
        self.frame_cache
            .as_ref()
            .map(|frame| (frame.width, frame.height))
            .unwrap_or_else(|| {
                let (width, height) = window.get_size();
                render_size_for_window(width, height)
            })
    }

    fn current_surface_size(&self, window: &Window) -> (usize, usize) {
        let window_size = window.get_size();
        input_surface_size_for_window_point(
            window_size,
            self.frame_cache.as_ref().map(|frame| frame.canvas()),
        )
    }
}

fn input_surface_size_for_window_point(
    window_size: (usize, usize),
    _frame_canvas: Option<&Canvas>,
) -> (usize, usize) {
    window_size
}

#[cfg(test)]
mod tests {
    use super::input_surface_size_for_window_point;
    use crate::canvas::Canvas;

    #[test]
    fn scaled_presented_frame_keeps_mouse_surface_in_window_coordinates() {
        let canvas = Canvas::new_scaled(1440, 920, 2.0, 0);

        assert_eq!(
            (1440, 920),
            input_surface_size_for_window_point((1440, 920), Some(&canvas))
        );
    }
}
