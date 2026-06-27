use crate::canvas::SurfaceArea;
use crate::catalog::StorybookFixture;
use crate::layout::{SIDEBAR_CONTENT_INSET, sidebar_content_width, sidebar_content_x};
use crate::preview::PreviewScene;
use crate::sidebar::{
    StorybookSidebar, StorybookSidebarScroll, file_tree, settings_list_tree, sidebar_settings,
    split_height,
};
use crate::sidebar_settings_state::StorybookSettingsState;
use katana_document_viewer::{ViewerInteractionConfig, ViewerTypographyConfig};
use katana_ui_core::molecule::{FileTree, FileTreeAction, FileTreeState, SettingsListAction};
use katana_ui_core::render_model::{UiHostActionPlan, UiNode};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{UiTreeHostActionHit, UiTreeStorybookHost};

pub(crate) struct StorybookFileTreeItemPointRequest<'a> {
    pub(crate) fixtures: &'a [StorybookFixture],
    pub(crate) selected_index: usize,
    pub(crate) state: &'a FileTreeState,
    pub(crate) item_id: &'a str,
    pub(crate) height: usize,
    pub(crate) scroll: StorybookSidebarScroll,
}

pub(crate) struct StorybookSettingsFieldHitRequest<'a> {
    pub(crate) scene: Option<&'a PreviewScene>,
    pub(crate) dark: bool,
    pub(crate) interaction: &'a ViewerInteractionConfig,
    pub(crate) typography: ViewerTypographyConfig,
    pub(crate) settings_state: &'a StorybookSettingsState,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) scroll: StorybookSidebarScroll,
    pub(crate) field_id: &'a str,
}

pub(crate) struct StorybookSettingsSectionHitRequest<'a> {
    pub(crate) scene: Option<&'a PreviewScene>,
    pub(crate) dark: bool,
    pub(crate) interaction: &'a ViewerInteractionConfig,
    pub(crate) typography: ViewerTypographyConfig,
    pub(crate) settings_state: &'a StorybookSettingsState,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) scroll: StorybookSidebarScroll,
    pub(crate) section_id: &'a str,
}

pub(crate) struct StorybookSidebarCanvasPoint {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub(crate) struct StorybookSettingsHitTarget {
    pub(crate) left: f32,
    pub(crate) top: f32,
    pub(crate) right: f32,
    pub(crate) center_x: f32,
    pub(crate) center_y: f32,
    pub(crate) action: Option<SettingsListAction>,
}

impl StorybookSidebar {
    pub(crate) fn fixture_hit_target_for_item_id(
        fixtures: &[StorybookFixture],
        selected_index: usize,
        state: &FileTreeState,
        item_id: &str,
        height: usize,
        scroll: StorybookSidebarScroll,
    ) -> Option<StorybookSidebarCanvasPoint> {
        let tree_height = split_height(height);
        let tree = file_tree(
            fixtures,
            selected_index,
            state,
            sidebar_content_width(),
            tree_height,
            scroll.tree_y,
        );
        host_action_hit(
            tree.root(),
            sidebar_content_width(),
            tree_height,
            true,
            |plan| file_tree_plan_matches(plan, item_id),
        )
        .map(|hit| StorybookSidebarCanvasPoint::from_host_action_hit(&hit))
    }

    pub(crate) fn fixture_canvas_point_for_item_id(
        request: StorybookFileTreeItemPointRequest<'_>,
    ) -> Option<StorybookSidebarCanvasPoint> {
        Self::fixture_hit_target_for_item_id(
            request.fixtures,
            request.selected_index,
            request.state,
            request.item_id,
            request.height,
            request.scroll,
        )
    }

    pub(crate) fn settings_field_hit_target(
        request: StorybookSettingsFieldHitRequest<'_>,
    ) -> Option<StorybookSettingsHitTarget> {
        let top_height = split_height(request.height);
        let settings_height = request.height.saturating_sub(top_height);
        let list = sidebar_settings::settings_list(
            request.scene,
            request.dark,
            request.interaction,
            request.typography,
            request.settings_state,
            request.width,
            settings_height,
        );
        let tree = settings_list_tree(
            &list,
            request.width,
            settings_height,
            request.scroll.settings_y,
        );
        host_action_hit_with_origin(
            tree.root(),
            request.width,
            settings_height,
            top_height,
            request.dark,
            |plan| {
                matches!(
                    list.action_from_host_plan(plan),
                    Some(SettingsListAction::UpdateField { ref field_id, .. })
                        if field_id == request.field_id
                )
            },
        )
        .and_then(|hit| {
            list.action_from_host_plan(&hit.action)
                .map(|action| StorybookSettingsHitTarget::from_host_action_hit(&hit, action))
        })
    }

    pub(crate) fn settings_field_canvas_point(
        request: StorybookSettingsFieldHitRequest<'_>,
    ) -> Option<StorybookSidebarCanvasPoint> {
        Self::settings_field_hit_target(request)
            .map(|target| StorybookSidebarCanvasPoint::from_settings_target(&target))
    }

    pub(crate) fn settings_section_hit_target(
        request: StorybookSettingsSectionHitRequest<'_>,
    ) -> Option<StorybookSettingsHitTarget> {
        let top_height = split_height(request.height);
        let settings_height = request.height.saturating_sub(top_height);
        let list = sidebar_settings::settings_list(
            request.scene,
            request.dark,
            request.interaction,
            request.typography,
            request.settings_state,
            request.width,
            settings_height,
        );
        let tree = settings_list_tree(
            &list,
            request.width,
            settings_height,
            request.scroll.settings_y,
        );
        host_action_hit_with_origin(
            tree.root(),
            request.width,
            settings_height,
            top_height,
            request.dark,
            |plan| {
                matches!(
                    list.action_from_host_plan(plan),
                    Some(SettingsListAction::ToggleSection { ref section_id })
                        if section_id == request.section_id
                )
            },
        )
        .and_then(|hit| {
            list.action_from_host_plan(&hit.action)
                .map(|action| StorybookSettingsHitTarget::from_host_action_hit(&hit, action))
        })
    }

    pub(crate) fn settings_section_canvas_point(
        request: StorybookSettingsSectionHitRequest<'_>,
    ) -> Option<StorybookSidebarCanvasPoint> {
        Self::settings_section_hit_target(request)
            .map(|target| StorybookSidebarCanvasPoint::from_settings_target(&target))
    }
}

impl StorybookSidebarCanvasPoint {
    fn from_host_action_hit(hit: &UiTreeHostActionHit) -> Self {
        let (x, y) = hit.center_point();
        Self {
            x: sidebar_canvas_x_coordinate(x),
            y: sidebar_canvas_y_coordinate(y),
        }
    }

    fn from_settings_target(target: &StorybookSettingsHitTarget) -> Self {
        Self {
            x: sidebar_canvas_x_coordinate(target.center_x),
            y: sidebar_canvas_y_coordinate(target.center_y),
        }
    }
}

impl StorybookSettingsHitTarget {
    pub(crate) fn canvas_point(&self) -> StorybookSidebarCanvasPoint {
        StorybookSidebarCanvasPoint::from_settings_target(self)
    }

    fn from_host_action_hit(hit: &UiTreeHostActionHit, action: SettingsListAction) -> Self {
        let (center_x, center_y) = hit.center_point();
        Self {
            left: hit.rect.x as f32,
            top: hit.rect.y as f32,
            right: hit.rect.x as f32 + hit.rect.width as f32,
            center_x,
            center_y,
            action: Some(action),
        }
    }
}

fn host_action_hit(
    root: &UiNode,
    width: usize,
    height: usize,
    dark: bool,
    accepts: impl Fn(&UiHostActionPlan) -> bool,
) -> Option<UiTreeHostActionHit> {
    host_action_hit_with_origin(root, width, height, 0, dark, accepts)
}

fn host_action_hit_with_origin(
    root: &UiNode,
    width: usize,
    height: usize,
    origin_y: usize,
    dark: bool,
    accepts: impl Fn(&UiHostActionPlan) -> bool,
) -> Option<UiTreeHostActionHit> {
    let host = UiTreeStorybookHost::new(theme_for_test(dark));
    host.host_action_hits(
        root,
        SurfaceArea {
            x: 0,
            y: origin_y,
            width,
            height,
            scroll_y: 0.0,
        },
    )
    .into_iter()
    .find(|hit| accepts(&hit.action))
}

fn theme_for_test(dark: bool) -> ThemeSnapshot {
    if dark {
        return ThemeSnapshot::dark();
    }
    ThemeSnapshot::light()
}

fn file_tree_plan_matches(plan: &UiHostActionPlan, item_id: &str) -> bool {
    match FileTree::action_from_host_plan(plan) {
        Some(FileTreeAction::SelectFile { file_id }) => file_id == item_id,
        Some(FileTreeAction::ToggleDirectory { directory_id }) => directory_id == item_id,
        Some(FileTreeAction::FocusItem {
            item_id: focused_id,
        }) => focused_id == item_id,
        Some(FileTreeAction::None) | None => false,
    }
}

fn sidebar_canvas_x_coordinate(local_value: f32) -> f32 {
    sidebar_content_x() as f32 + local_value
}

fn sidebar_canvas_y_coordinate(local_value: f32) -> f32 {
    SIDEBAR_CONTENT_INSET as f32 + local_value
}
