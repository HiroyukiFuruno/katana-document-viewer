use super::DocumentPoint;
use super::mouse_host_action::StorybookHostActionRouter;
use crate::preview::PreviewScene;
use katana_document_viewer::ViewerCommand;
use mouse_media_hit::MediaControlHit;

pub(super) struct StorybookMediaMouse;

impl StorybookMediaMouse {
    pub(super) fn command(
        scene: &PreviewScene,
        point: DocumentPoint,
        router: &StorybookHostActionRouter,
    ) -> Option<ViewerCommand> {
        if let Some(command) = Self::internal_diagram_command(scene, point, router) {
            return Some(command);
        }
        Self::hits_from_router_at(router, point)
            .into_iter()
            .find_map(|hit| hit.command(scene, router))
    }

    fn internal_diagram_command(
        scene: &PreviewScene,
        point: DocumentPoint,
        router: &StorybookHostActionRouter,
    ) -> Option<ViewerCommand> {
        let action = router.internal_diagram_action_at(point)?;
        let target = router.target_for_node_id(&action.node_id)?.clone();
        let fullscreen_open = scene
            .diagram_viewports
            .get(action.node_id.as_str())
            .is_some_and(|state| state.fullscreen_open);
        katana_document_viewer::ViewerCommandFactory::media_control_from_viewer_action(
            target,
            action,
            fullscreen_open,
        )
    }

    fn hits_from_router_at(
        router: &StorybookHostActionRouter,
        point: DocumentPoint,
    ) -> Vec<MediaControlHit> {
        router
            .hits_at(point)
            .map(MediaControlHit::from_hit)
            .collect()
    }

    #[cfg(test)]
    fn hits_from_router(router: &StorybookHostActionRouter) -> Vec<MediaControlHit> {
        router
            .hits()
            .iter()
            .cloned()
            .map(MediaControlHit::from_hit)
            .collect()
    }

    #[cfg(test)]
    pub(super) fn test_point_for_action(
        scene: &PreviewScene,
        action: &str,
        preview_width: usize,
    ) -> Option<(f32, f32)> {
        let router = StorybookHostActionRouter::for_preview_width(scene, preview_width);
        Self::hits_from_router(&router)
            .into_iter()
            .find(|hit| hit.action() == action)
            .map(MediaControlHit::center)
            .or_else(|| router.internal_diagram_point_for_action(action))
    }
}

#[path = "mouse_media_hit.rs"]
mod mouse_media_hit;
