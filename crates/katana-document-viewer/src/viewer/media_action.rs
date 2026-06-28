use super::{ViewerCommand, ViewerCommandFactory, ViewerTarget};
use serde::{Deserialize, Serialize};

const IMAGE_ACTION_PREFIX: &str = "viewer.image.";
const DIAGRAM_ACTION_PREFIX: &str = "viewer.diagram.";
const CODE_ACTION_PREFIX: &str = "viewer.code.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerMediaControlKind {
    Image,
    Diagram,
    Code,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerMediaControlAction {
    pub kind: ViewerMediaControlKind,
    pub node_id: String,
    pub command: String,
}

impl ViewerMediaControlAction {
    #[must_use]
    pub fn new(
        kind: ViewerMediaControlKind,
        node_id: impl Into<String>,
        command: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            node_id: node_id.into(),
            command: command.into(),
        }
    }

    #[must_use]
    pub fn host_action_id(&self) -> String {
        Self::host_action_id_for(self.kind, self.command.as_str())
    }

    #[must_use]
    pub fn host_action_id_for(kind: ViewerMediaControlKind, command: &str) -> String {
        format!("{}{command}", kind.action_prefix())
    }

    #[must_use]
    pub fn from_host_action(action_id: &str, payload: &str) -> Option<Self> {
        let (prefix, kind) = Self::prefix_kind(action_id)?;
        Some(Self {
            kind,
            node_id: payload.to_string(),
            command: action_id.strip_prefix(prefix)?.to_string(),
        })
    }

    fn prefix_kind(action_id: &str) -> Option<(&'static str, ViewerMediaControlKind)> {
        if action_id.starts_with(IMAGE_ACTION_PREFIX) {
            return Some((IMAGE_ACTION_PREFIX, ViewerMediaControlKind::Image));
        }
        if action_id.starts_with(DIAGRAM_ACTION_PREFIX) {
            return Some((DIAGRAM_ACTION_PREFIX, ViewerMediaControlKind::Diagram));
        }
        if action_id.starts_with(CODE_ACTION_PREFIX) {
            return Some((CODE_ACTION_PREFIX, ViewerMediaControlKind::Code));
        }
        None
    }
}

impl ViewerMediaControlKind {
    #[must_use]
    pub fn action_prefix(self) -> &'static str {
        match self {
            Self::Image => IMAGE_ACTION_PREFIX,
            Self::Diagram => DIAGRAM_ACTION_PREFIX,
            Self::Code => CODE_ACTION_PREFIX,
        }
    }
}

impl ViewerCommandFactory {
    pub fn media_control_from_host_action(
        target: ViewerTarget,
        action_id: &str,
        payload: &str,
        fullscreen_open: bool,
    ) -> Option<ViewerCommand> {
        let action = ViewerMediaControlAction::from_host_action(action_id, payload)?;
        Self::media_control_from_viewer_action(target, action, fullscreen_open)
    }

    pub fn media_control_from_viewer_action(
        target: ViewerTarget,
        action: ViewerMediaControlAction,
        fullscreen_open: bool,
    ) -> Option<ViewerCommand> {
        if action.node_id != target.node_id.0 {
            return None;
        }
        match action.kind {
            ViewerMediaControlKind::Image => {
                Self::image_control_from_action(target, action.command.as_str())
            }
            ViewerMediaControlKind::Diagram => {
                Self::diagram_control_from_action(target, action.command.as_str(), fullscreen_open)
            }
            ViewerMediaControlKind::Code => {
                Self::code_control_from_action(target, action.command.as_str())
            }
        }
    }
}
