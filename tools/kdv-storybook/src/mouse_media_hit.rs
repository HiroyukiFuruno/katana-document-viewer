use super::super::mouse_host_action::StorybookHostActionRouter;
use crate::media_host_action::StorybookMediaHostAction;
use crate::preview::PreviewScene;
use katana_document_viewer::{ViewerCommand, ViewerCommandFactory};
use katana_ui_core_storybook::UiTreeHostActionHit;

pub(super) struct MediaControlHit {
    hit: UiTreeHostActionHit,
}

impl MediaControlHit {
    pub(super) fn from_hit(hit: UiTreeHostActionHit) -> Self {
        Self { hit }
    }

    pub(super) fn command(
        self,
        scene: &PreviewScene,
        router: &StorybookHostActionRouter,
    ) -> Option<ViewerCommand> {
        let action = StorybookMediaHostAction::from_host_action_plan(&self.hit.action)?;
        let resolved = router.resolve_hit_target_for_node_id(action.node_id(), self.hit)?;
        let target = resolved.target().clone();
        let fullscreen_open = scene
            .diagram_viewports
            .get(action.node_id())
            .is_some_and(|state| state.fullscreen_open);
        ViewerCommandFactory::media_control_from_viewer_action(
            target,
            action.into_viewer_action(),
            fullscreen_open,
        )
    }

    #[cfg(test)]
    pub(super) fn action(&self) -> &str {
        self.hit
            .action
            .action_id
            .rsplit_once('.')
            .map(|(_, command)| command)
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub(super) fn center(self) -> (f32, f32) {
        self.hit.center_point()
    }
}
