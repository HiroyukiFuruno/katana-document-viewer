use super::mouse_host_action::StorybookHostActionRouter;
use super::{DocumentPoint, StorybookMouseButton};
use crate::preview::PreviewScene;
use crate::task_marker_state::viewer_task_state;
use katana_document_viewer::{
    KmmNodeId, SourceSpan, ViewerCommand, ViewerCommandFactory, ViewerRect, ViewerTarget,
    ViewerTaskControlTarget, ViewerTaskState,
};
use katana_ui_core::render_model::UiTaskControlAction;

pub(super) struct StorybookTaskMouse;

impl StorybookTaskMouse {
    pub(super) fn command(
        scene: &PreviewScene,
        point: DocumentPoint,
        button: StorybookMouseButton,
        router: &StorybookHostActionRouter,
    ) -> Option<ViewerCommand> {
        if button != StorybookMouseButton::Left {
            return None;
        }
        let task = StorybookTaskHit::find(scene, point, router)?;
        Some(ViewerCommandFactory::toggle_task_control(
            task.target().clone(),
            task.task_target().clone(),
            task.marker,
        ))
    }
}

pub(super) struct StorybookTaskHit {
    target: ViewerTarget,
    task_target: ViewerTaskControlTarget,
    marker: ViewerTaskState,
    menu_items: Vec<StorybookTaskMenuItem>,
}

#[derive(Clone)]
pub(super) struct StorybookTaskMenuItem {
    pub(super) item_id: String,
    pub(super) label: String,
    pub(super) checked: bool,
    pub(super) node_id: String,
    pub(super) row_index: usize,
    pub(super) marker: String,
}

impl StorybookTaskHit {
    pub(super) fn find(
        scene: &PreviewScene,
        point: DocumentPoint,
        router: &StorybookHostActionRouter,
    ) -> Option<Self> {
        router
            .hits_at(point)
            .find_map(|hit| Self::from_hit(scene, router, hit))
    }

    pub(super) fn target(&self) -> &ViewerTarget {
        &self.target
    }

    pub(super) fn task_target(&self) -> &ViewerTaskControlTarget {
        &self.task_target
    }

    pub(super) fn menu_items(&self) -> &[StorybookTaskMenuItem] {
        &self.menu_items
    }

    fn from_hit(
        scene: &PreviewScene,
        router: &StorybookHostActionRouter,
        hit: katana_ui_core_storybook::UiTreeHostActionHit,
    ) -> Option<Self> {
        let action = hit
            .action
            .task_control_action_from_root(scene.tree.root())?;
        let hit = router.resolve_hit_target_for_node_id(action.node_id.as_str(), hit)?;
        let target = viewer_target(&action, hit.target(), hit.hit_rect());
        let task_target = task_control_target(&action);
        Some(Self {
            target,
            task_target,
            marker: viewer_task_state(action.current_marker),
            menu_items: action
                .menu_items
                .iter()
                .map(|item| StorybookTaskMenuItem {
                    item_id: item.item_id.clone(),
                    label: item.label.clone(),
                    checked: item.checked,
                    node_id: action.node_id.clone(),
                    row_index: action.row_index,
                    marker: item.marker.marker().to_string(),
                })
                .collect(),
        })
    }
}

fn viewer_target(
    action: &UiTaskControlAction,
    base_target: &ViewerTarget,
    rect: ViewerRect,
) -> ViewerTarget {
    ViewerTarget {
        node_id: KmmNodeId(action.node_id.clone()),
        source: row_source(action, base_target),
        artifact_id: base_target.artifact_id.clone(),
        rect,
    }
}

fn task_control_target(action: &UiTaskControlAction) -> ViewerTaskControlTarget {
    ViewerTaskControlTarget {
        node_id: KmmNodeId(action.node_id.clone()),
        row_index: action.row_index,
        state_id: action.state_id.clone(),
    }
}

fn row_source(action: &UiTaskControlAction, base_target: &ViewerTarget) -> SourceSpan {
    let mut source = base_target.source.clone();
    source.raw.text = base_target
        .source
        .raw
        .text
        .lines()
        .nth(action.row_index)
        .map_or_else(
            || action.current_marker.marker().to_string(),
            str::to_string,
        );
    source
}

#[cfg(test)]
#[path = "mouse_task/tests.rs"]
mod tests;
