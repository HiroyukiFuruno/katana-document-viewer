use katana_document_viewer::ViewerMediaControlAction;
use katana_ui_core::render_model::{UiHostActionPayload, UiHostActionPlan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StorybookMediaHostAction {
    action: ViewerMediaControlAction,
}

impl StorybookMediaHostAction {
    pub(crate) fn from_host_action_plan(action: &UiHostActionPlan) -> Option<Self> {
        let UiHostActionPayload::SurfaceControl(payload) = &action.typed_payload else {
            return None;
        };
        let action = ViewerMediaControlAction::from_host_action(
            action.action_id.as_str(),
            payload.node_id.as_str(),
        )?;
        Some(Self { action })
    }

    pub(crate) fn node_id(&self) -> &str {
        self.action.node_id.as_str()
    }

    pub(crate) fn into_viewer_action(self) -> ViewerMediaControlAction {
        self.action
    }
}
