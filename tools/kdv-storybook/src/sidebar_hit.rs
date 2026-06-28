use crate::canvas::SurfaceArea;
use crate::catalog::StorybookFixture;
use crate::layout::{
    SIDEBAR_CONTENT_INSET, preview_content_width, preview_viewport_height,
    sidebar_content_contains, sidebar_content_height, sidebar_content_local_x,
    sidebar_content_local_y, sidebar_content_width, sidebar_content_x,
};
use crate::sidebar::{StorybookSidebar, StorybookSidebarScroll};
use crate::sidebar_settings_state::StorybookSettingsState;
use katana_document_viewer::{ViewerInteractionConfig, ViewerTypographyConfig};
use katana_ui_core::molecule::{
    FileTree, FileTreeAction, FileTreeState, SettingsList, SettingsListAction,
};
use katana_ui_core::render_model::{UiCursor, UiNodeId};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{UiTreeHostActionHit, UiTreeSurfaceHost};

pub(crate) struct SidebarHit;

impl SidebarHit {
    pub(crate) fn interaction(
        pointer_x: f32,
        pointer_y: f32,
        request: SidebarHitRequest<'_>,
    ) -> SidebarInteraction {
        if !sidebar_content_contains(pointer_x, pointer_y, request.height) {
            return SidebarInteraction::default();
        }
        let Some(local_x) = sidebar_content_local_x(pointer_x) else {
            return SidebarInteraction::default();
        };
        let Some(local_y) = sidebar_content_local_y(pointer_y, request.height) else {
            return SidebarInteraction::default();
        };
        Self::interaction_surface(&request).interaction_at(local_x as f32, local_y as f32)
    }

    #[cfg(test)]
    pub(crate) fn hit(
        pointer_x: f32,
        pointer_y: f32,
        request: SidebarHitRequest<'_>,
    ) -> Option<SidebarHitResult> {
        Self::interaction(pointer_x, pointer_y, request).action
    }

    pub(crate) fn interaction_surface(
        request: &SidebarHitRequest<'_>,
    ) -> SidebarInteractionSurface {
        let list = StorybookSidebar::settings_list_for_hit_contract(
            crate::sidebar::SettingsListHitContractRequest {
                scene: request.scene,
                dark: request.dark,
                interaction: request.interaction,
                typography: request.typography,
                settings_state: &request.settings_state,
                preview_width: preview_content_width(request.width),
                preview_height: preview_viewport_height(request.height),
            },
        );
        let tree = StorybookSidebar::render(crate::sidebar::StorybookSidebarRequest {
            fixtures: request.fixtures,
            selected_index: request.selected_index,
            scene: request.scene,
            dark: request.dark,
            interaction: request.interaction,
            typography: request.typography,
            file_tree_state: request.file_tree_state.clone(),
            settings_state: &request.settings_state,
            width: sidebar_content_width(),
            height: sidebar_content_height(request.height),
            preview_width: preview_content_width(request.width),
            preview_height: preview_viewport_height(request.height),
            scroll: request.scroll,
        });
        let area = SurfaceArea {
            x: 0,
            y: 0,
            width: sidebar_content_width(),
            height: sidebar_content_height(request.height),
            scroll_y: 0.0,
        };
        let host = UiTreeSurfaceHost::new(theme_for(request.dark));
        let hits = host.host_action_hits(tree.root(), area);
        SidebarInteractionSurface::new(list, hits)
    }
}

#[derive(Clone)]
pub(crate) struct SidebarInteractionSurface {
    settings_list: SettingsList,
    hits: Vec<UiTreeHostActionHit>,
}

impl SidebarInteractionSurface {
    fn new(settings_list: SettingsList, hits: Vec<UiTreeHostActionHit>) -> Self {
        Self {
            settings_list,
            hits,
        }
    }

    pub(crate) fn interaction_at(&self, pointer_x: f32, pointer_y: f32) -> SidebarInteraction {
        let hits = UiTreeSurfaceHost::hits_at(&self.hits, pointer_x, pointer_y);
        let Some(hit) = hits.first() else {
            return SidebarInteraction::default();
        };
        if let Some(action) = FileTree::action_from_host_plan(&hit.action) {
            return SidebarInteraction {
                action: Some(SidebarHitResult::FileTree(action.clone())),
                cursor: UiTreeSurfaceHost::cursor_at(&self.hits, pointer_x, pointer_y),
                hovered_file_item_id: file_tree_action_item_id(&action),
                hovered_settings_node_id: None,
            };
        }
        if let Some(action) = self.settings_list.action_from_host_plan(&hit.action) {
            return SidebarInteraction {
                action: Some(SidebarHitResult::SettingsAction(action)),
                cursor: UiTreeSurfaceHost::cursor_at(&self.hits, pointer_x, pointer_y),
                hovered_file_item_id: None,
                hovered_settings_node_id: UiTreeSurfaceHost::hovered_action_node_id_at(
                    &self.hits, pointer_x, pointer_y,
                ),
            };
        }
        SidebarInteraction::default()
    }

    pub(crate) fn canvas_interaction_at(
        &self,
        pointer_x: f32,
        pointer_y: f32,
        height: usize,
    ) -> SidebarInteraction {
        if !sidebar_content_contains(pointer_x, pointer_y, height) {
            return SidebarInteraction::default();
        }
        let Some(local_x) = sidebar_content_local_x(pointer_x) else {
            return SidebarInteraction::default();
        };
        let Some(local_y) = sidebar_content_local_y(pointer_y, height) else {
            return SidebarInteraction::default();
        };
        self.interaction_at(local_x as f32, local_y as f32)
    }

    pub(crate) fn file_tree_canvas_point(
        &self,
        accepts: impl Fn(&FileTreeAction) -> bool,
    ) -> Option<(FileTreeAction, f32, f32)> {
        self.hits.iter().find_map(|hit| {
            let action = FileTree::action_from_host_plan(&hit.action)?;
            if !accepts(&action) {
                return None;
            }
            let (x, y) = hit.center_point();
            Some((
                action,
                sidebar_canvas_x_coordinate(x),
                sidebar_canvas_y_coordinate(y),
            ))
        })
    }

    pub(crate) fn settings_field_canvas_point(&self, field_id: &str) -> Option<(f32, f32)> {
        self.hits.iter().find_map(|hit| {
            let action = self.settings_list.action_from_host_plan(&hit.action)?;
            match action {
                SettingsListAction::UpdateField {
                    field_id: ref candidate,
                    ..
                } if candidate == field_id => {
                    let (x, y) = hit.center_point();
                    Some((
                        sidebar_canvas_x_coordinate(x),
                        sidebar_canvas_y_coordinate(y),
                    ))
                }
                _ => None,
            }
        })
    }
}

fn sidebar_canvas_x_coordinate(value: f32) -> f32 {
    sidebar_content_x() as f32 + value
}

fn sidebar_canvas_y_coordinate(value: f32) -> f32 {
    SIDEBAR_CONTENT_INSET as f32 + value
}

fn theme_for(dark: bool) -> ThemeSnapshot {
    if dark {
        return ThemeSnapshot::dark();
    }
    ThemeSnapshot::light()
}

fn file_tree_action_item_id(action: &FileTreeAction) -> Option<String> {
    match action {
        FileTreeAction::SelectFile { file_id } => Some(file_id.clone()),
        FileTreeAction::ToggleDirectory { directory_id } => Some(directory_id.clone()),
        FileTreeAction::FocusItem { item_id } => Some(item_id.clone()),
        FileTreeAction::None => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SidebarInteraction {
    pub(crate) action: Option<SidebarHitResult>,
    pub(crate) cursor: UiCursor,
    pub(crate) hovered_file_item_id: Option<String>,
    pub(crate) hovered_settings_node_id: Option<UiNodeId>,
}

impl Default for SidebarInteraction {
    fn default() -> Self {
        Self {
            action: None,
            cursor: UiCursor::Default,
            hovered_file_item_id: None,
            hovered_settings_node_id: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SidebarHitResult {
    FileTree(FileTreeAction),
    SettingsAction(SettingsListAction),
}

pub(crate) struct SidebarHitRequest<'a> {
    pub(crate) fixtures: &'a [StorybookFixture],
    pub(crate) selected_index: usize,
    pub(crate) scene: Option<&'a crate::preview::PreviewScene>,
    pub(crate) dark: bool,
    pub(crate) interaction: &'a ViewerInteractionConfig,
    pub(crate) typography: ViewerTypographyConfig,
    pub(crate) settings_state: StorybookSettingsState,
    pub(crate) file_tree_state: FileTreeState,
    pub(crate) scroll: StorybookSidebarScroll,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

#[cfg(test)]
#[path = "sidebar_hit_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "sidebar_hit_bounds_tests.rs"]
mod bounds_tests;
