use super::support::{DiagramActionHit, FRAME_WIDTH, MediaActionHit, MediaControlFrameSupport};
use crate::layout::preview_content_width;
use crate::media_host_action::StorybookMediaHostAction;
use crate::mouse::StorybookHostActionHits;
use crate::preview::PreviewScene;
use katana_document_viewer::ViewerMediaControlKind;
use katana_ui_core_storybook::UiTreeHostActionHit;

impl MediaControlFrameSupport {
    pub(super) fn diagram_action_hits(
        scene: &PreviewScene,
    ) -> Result<Vec<DiagramActionHit>, std::io::Error> {
        Self::media_action_hits(scene, ViewerMediaControlKind::Diagram)
    }

    pub(super) fn image_action_hits(
        scene: &PreviewScene,
    ) -> Result<Vec<MediaActionHit>, std::io::Error> {
        Self::media_action_hits(scene, ViewerMediaControlKind::Image)
    }

    pub(super) fn code_action_hits(
        scene: &PreviewScene,
    ) -> Result<Vec<MediaActionHit>, std::io::Error> {
        Self::media_action_hits(scene, ViewerMediaControlKind::Code)
    }

    fn media_action_hits(
        scene: &PreviewScene,
        kind: ViewerMediaControlKind,
    ) -> Result<Vec<MediaActionHit>, std::io::Error> {
        let hits = StorybookHostActionHits::hits_for_preview_width(
            scene,
            preview_content_width(FRAME_WIDTH),
        );
        let action_hits = hits
            .into_iter()
            .filter_map(|hit| media_action_hit(hit, kind))
            .collect::<Vec<_>>();
        if action_hits.is_empty() {
            return Err(std::io::Error::other(format!(
                "missing media host action hit for {kind:?}"
            )));
        }
        Ok(action_hits)
    }
}

fn media_action_hit(
    hit: UiTreeHostActionHit,
    kind: ViewerMediaControlKind,
) -> Option<MediaActionHit> {
    let action = StorybookMediaHostAction::from_host_action_plan(&hit.action)?.into_viewer_action();
    if action.kind != kind {
        return None;
    }
    Some(MediaActionHit {
        command: action.command,
        hit,
    })
}
