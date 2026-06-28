use katana_ui_core::render_model::{UiHostActionPlan, UiNode, UiTaskMarker};

pub(crate) fn collect_task_markers(root: &UiNode) -> Vec<UiTaskMarker> {
    UiHostActionPlan::collect_from_root(root)
        .into_iter()
        .filter_map(|plan| marker_for_plan(root, &plan))
        .collect()
}

fn marker_for_plan(root: &UiNode, plan: &UiHostActionPlan) -> Option<UiTaskMarker> {
    let action = plan.task_control_action_from_root(root)?;
    Some(action.current_marker)
}
