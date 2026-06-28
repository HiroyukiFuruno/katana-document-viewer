use super::DocumentPoint;
use super::mouse_host_action::StorybookHostActionRouter;
use crate::KucDiagramControlResolver;
use crate::preview::PreviewScene;
use katana_ui_core::render_model::{UiCursor, UiNodeId};
use katana_ui_core_storybook::{UiTreeHostActionHit, UiTreeNodeHit, UiTreeSurfaceHost};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StorybookHoverState {
    pub(crate) cursor: UiCursor,
    pub(crate) hovered_node_id: Option<String>,
    pub(crate) hovered_action_node_id: Option<UiNodeId>,
}

pub(crate) struct StorybookMouseCursor;

impl Default for StorybookHoverState {
    fn default() -> Self {
        Self {
            cursor: UiCursor::Default,
            hovered_node_id: None,
            hovered_action_node_id: None,
        }
    }
}

impl StorybookMouseCursor {
    pub(crate) fn hover_state_for_hover(
        scene: &PreviewScene,
        scroll_y: f32,
        x: f32,
        y: f32,
        window_width: usize,
        window_height: usize,
    ) -> StorybookHoverState {
        let Some(point) =
            DocumentPoint::from_scene_position(scene, x, y, scroll_y, window_width, window_height)
        else {
            return StorybookHoverState::default();
        };
        let router = StorybookHostActionRouter::for_window_with_scroll(
            scene,
            window_width,
            window_height,
            scroll_y,
        );
        Self::hover_state_for_point(point, &router)
    }

    #[cfg(test)]
    pub(crate) fn cursor_for_hover(
        scene: &PreviewScene,
        scroll_y: f32,
        x: f32,
        y: f32,
        window_width: usize,
        window_height: usize,
    ) -> UiCursor {
        Self::hover_state_for_hover(scene, scroll_y, x, y, window_width, window_height).cursor
    }

    #[cfg(test)]
    pub(crate) fn hovered_node_id_for_hover(
        scene: &PreviewScene,
        scroll_y: f32,
        x: f32,
        y: f32,
        window_width: usize,
        window_height: usize,
    ) -> Option<String> {
        Self::hover_state_for_hover(scene, scroll_y, x, y, window_width, window_height)
            .hovered_node_id
    }

    #[cfg(test)]
    pub(crate) fn hovered_action_node_id_for_hover(
        scene: &PreviewScene,
        scroll_y: f32,
        x: f32,
        y: f32,
        window_width: usize,
        window_height: usize,
    ) -> Option<UiNodeId> {
        Self::hover_state_for_hover(scene, scroll_y, x, y, window_width, window_height)
            .hovered_action_node_id
    }

    fn hover_state_for_point(
        point: DocumentPoint,
        router: &StorybookHostActionRouter,
    ) -> StorybookHoverState {
        let internal_control_node_id = router.internal_diagram_node_id_at(point);
        let cursor = if internal_control_node_id.is_some() {
            UiCursor::Pointer
        } else {
            router.cursor_at(point)
        };
        let hovering_internal_control = internal_control_node_id.is_some();
        let hovered_action_node_id = router
            .hovered_action_node_id_at(point)
            .or(internal_control_node_id);
        let hovered_node_id = if hovering_internal_control {
            None
        } else {
            router
                .hovered_node_id_at(point)
                .map(|node_id| node_id.as_str().to_string())
        };
        StorybookHoverState {
            cursor,
            hovered_node_id,
            hovered_action_node_id,
        }
    }

    pub(crate) fn hover_state_for_cached_hits(
        scene: &PreviewScene,
        point: DocumentPoint,
        hits: &[UiTreeHostActionHit],
        node_hits: &[UiTreeNodeHit],
    ) -> StorybookHoverState {
        let internal_control_node_id = KucDiagramControlResolver::internal_control_node_id_at(
            scene.tree.root(),
            node_hits,
            point.x,
            point.y,
        );
        let cursor = if internal_control_node_id.is_some() {
            UiCursor::Pointer
        } else {
            UiTreeSurfaceHost::cursor_at(hits, point.x, point.y)
        };
        let hovering_internal_control = internal_control_node_id.is_some();
        let hovered_action_node_id =
            UiTreeSurfaceHost::hovered_action_node_id_at(hits, point.x, point.y)
                .or(internal_control_node_id);
        let hovered_node_id = if hovering_internal_control {
            None
        } else {
            UiTreeSurfaceHost::hovered_node_id_at(node_hits, point.x, point.y)
                .map(|node_id| node_id.as_str().to_string())
                .or_else(|| hovered_scene_target_node_id(scene, point))
        };
        StorybookHoverState {
            cursor,
            hovered_node_id,
            hovered_action_node_id,
        }
    }
}

fn hovered_scene_target_node_id(scene: &PreviewScene, point: DocumentPoint) -> Option<String> {
    scene
        .targets
        .iter()
        .rev()
        .find(|target| {
            point.x >= target.rect.x
                && point.y >= target.rect.y
                && point.x < target.rect.x + target.rect.width
                && point.y < target.rect.y + target.rect.height
        })
        .map(|target| target.node_id.0.clone())
}
