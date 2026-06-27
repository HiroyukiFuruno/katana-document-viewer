use crate::media_host_action::StorybookMediaHostAction;
use crate::preview::PreviewScene;
use katana_document_viewer::{ViewerMediaControlKind, ViewerTarget};
use katana_ui_core::render_model::UiHostActionPlan;

#[derive(Debug, Clone)]
pub(crate) struct SceneActionTarget {
    pub action: String,
    pub target: ViewerTarget,
}

pub(crate) fn collect_action_targets(
    scene: &PreviewScene,
    kind: ViewerMediaControlKind,
) -> Vec<SceneActionTarget> {
    UiHostActionPlan::collect_from_tree(&scene.tree)
        .into_iter()
        .filter_map(|action| action_target(scene, kind, action))
        .collect()
}

fn action_target(
    scene: &PreviewScene,
    kind: ViewerMediaControlKind,
    action: UiHostActionPlan,
) -> Option<SceneActionTarget> {
    let media_action =
        StorybookMediaHostAction::from_host_action_plan(&action)?.into_viewer_action();
    if media_action.kind != kind {
        return None;
    }
    let target = scene.target_for_node_id(media_action.node_id.as_str())?;
    Some(SceneActionTarget {
        action: media_action.command,
        target: target.clone(),
    })
}
