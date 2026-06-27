use super::{SlideshowCommand, ViewerCommand, ViewerCommandFactory};
use serde::{Deserialize, Serialize};

const SLIDESHOW_ACTION_PREFIX: &str = "viewer.slideshow.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerSlideshowControlAction {
    PreviousPage,
    NextPage,
    Close,
}

impl ViewerSlideshowControlAction {
    #[must_use]
    pub fn command(self) -> &'static str {
        match self {
            Self::PreviousPage => "previous-page",
            Self::NextPage => "next-page",
            Self::Close => "close",
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::PreviousPage => "Previous page",
            Self::NextPage => "Next page",
            Self::Close => "Close slideshow",
        }
    }

    #[must_use]
    pub fn host_action_id(self) -> String {
        Self::host_action_id_for(self.command())
    }

    #[must_use]
    pub fn host_action_id_for(command: &str) -> String {
        format!("{SLIDESHOW_ACTION_PREFIX}{command}")
    }

    #[must_use]
    pub fn from_host_action(action_id: &str) -> Option<Self> {
        let command = action_id.strip_prefix(SLIDESHOW_ACTION_PREFIX)?;
        match command {
            "previous-page" => Some(Self::PreviousPage),
            "next-page" => Some(Self::NextPage),
            "close" => Some(Self::Close),
            _ => None,
        }
    }
}

impl ViewerCommandFactory {
    pub fn slideshow_control_from_host_action(action_id: &str) -> Option<ViewerCommand> {
        let command = match ViewerSlideshowControlAction::from_host_action(action_id)? {
            ViewerSlideshowControlAction::PreviousPage => SlideshowCommand::PreviousPage,
            ViewerSlideshowControlAction::NextPage => SlideshowCommand::NextPage,
            ViewerSlideshowControlAction::Close => SlideshowCommand::Close,
        };
        Some(ViewerCommand::Slideshow(command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slideshow_host_actions_roundtrip_to_commands() {
        assert_eq!(
            Some(ViewerSlideshowControlAction::NextPage),
            ViewerSlideshowControlAction::from_host_action(
                ViewerSlideshowControlAction::NextPage
                    .host_action_id()
                    .as_str()
            )
        );
        assert_eq!(
            Some(ViewerCommandFactory::next_slideshow_page()),
            ViewerCommandFactory::slideshow_control_from_host_action(
                ViewerSlideshowControlAction::NextPage
                    .host_action_id()
                    .as_str()
            )
        );
    }
}
