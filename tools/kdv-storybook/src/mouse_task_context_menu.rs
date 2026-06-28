use super::mouse_host_action::StorybookHostActionRouter;
use super::mouse_task::{StorybookTaskHit, StorybookTaskMenuItem};
use super::{DocumentPoint, StorybookPointer};
use crate::preview::PreviewScene;
use crate::task_marker_state::viewer_task_state;
use katana_document_viewer::{
    KmmNodeId, ViewerCommand, ViewerCommandFactory, ViewerTarget, ViewerTaskControlTarget,
};
use katana_ui_core::render_model::{
    UiContextMenuAnchor, UiContextMenuItem, UiContextMenuItemKind, UiContextMenuProps, UiNode,
    UiNodeKind,
};
#[cfg(test)]
use katana_ui_core::render_model::{UiHostActionPlan, UiNodeId};
use katana_ui_core_storybook::UiTreeSurfaceHost;

pub(crate) struct StorybookTaskContextMenu {
    target: ViewerTarget,
    node: UiNode,
}

impl StorybookTaskContextMenu {
    pub(crate) fn open(
        scene: &PreviewScene,
        scroll_y: f32,
        pointer: StorybookPointer,
        window_width: usize,
        window_height: usize,
    ) -> Option<Self> {
        let point = DocumentPoint::from_scene_pointer(
            scene,
            pointer,
            scroll_y,
            window_width,
            window_height,
        )?;
        let router = StorybookHostActionRouter::for_window_with_scroll(
            scene,
            window_width,
            window_height,
            scroll_y,
        );
        let hit = StorybookTaskHit::find(scene, point, &router)?;
        let target = hit.target();
        Some(Self {
            target: target.clone(),
            node: context_menu_node(pointer, hit.menu_items()),
        })
    }

    pub(crate) fn command_for_pointer(&self, pointer: StorybookPointer) -> Option<ViewerCommand> {
        let state_action =
            UiTreeSurfaceHost::context_menu_host_action_at(&self.node, pointer.x, pointer.y)?
                .task_control_state_action()?;
        let mut target = self.target.clone();
        target.node_id = KmmNodeId(state_action.node_id.clone());
        Some(ViewerCommandFactory::set_task_control_state(
            target,
            ViewerTaskControlTarget {
                node_id: KmmNodeId(state_action.node_id),
                row_index: state_action.row_index,
                state_id: state_action.state_id,
            },
            viewer_task_state(state_action.marker),
        ))
    }

    pub(crate) fn node(&self) -> &UiNode {
        &self.node
    }

    #[cfg(test)]
    pub(crate) fn test_pointer_for_marker(&self, marker: &str) -> Option<StorybookPointer> {
        let item_id = self
            .node
            .props()
            .context_menu
            .items
            .iter()
            .find(|item| context_menu_item_marker(item).as_deref() == Some(marker))?
            .id
            .as_str();
        let (x, y) = UiTreeSurfaceHost::context_menu_item_center_for_id(&self.node, item_id)?;
        Some(StorybookPointer::new(
            x,
            y,
            super::StorybookMouseButton::Left,
        ))
    }
}

fn context_menu_node(pointer: StorybookPointer, items: &[StorybookTaskMenuItem]) -> UiNode {
    UiNode::new(UiNodeKind::ContextMenu, "task-context-menu").context_menu(UiContextMenuProps {
        anchor: UiContextMenuAnchor::Pointer {
            x: pointer.x.round() as i32,
            y: pointer.y.round() as i32,
        },
        items: items.iter().map(context_menu_item).collect(),
        ..UiContextMenuProps::default()
    })
}

fn context_menu_item(item: &StorybookTaskMenuItem) -> UiContextMenuItem {
    UiContextMenuItem::new(
        item.item_id.clone(),
        item.label.clone(),
        UiContextMenuItemKind::Radio,
    )
    .checked(item.checked)
    .radio_group("ui-task-state")
    .task_control_state_action(
        item.label.clone(),
        item.node_id.clone(),
        item.row_index,
        item.marker.clone(),
    )
}

#[cfg(test)]
fn context_menu_item_marker(item: &UiContextMenuItem) -> Option<String> {
    let plan = UiHostActionPlan::from_context_menu_item(
        UiNodeId::new("test-context-menu-item"),
        item,
        true,
        &[],
    )?;
    plan.task_control_state_action()
        .map(|action| action.marker.marker().to_owned())
}
