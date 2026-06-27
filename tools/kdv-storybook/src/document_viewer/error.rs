use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucMissingCapability {
    #[expect(
        dead_code,
        reason = "Capability errors are part of the adapter boundary even though the current viewer projection succeeds."
    )]
    MarkdownBlockModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KucViewerError {
    #[expect(
        dead_code,
        reason = "Capability errors are reserved for adapter-boundary failures; the current Storybook path has no such failure."
    )]
    MissingCapability(KucMissingCapability),
}

impl Display for KucViewerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCapability(capability) => {
                write!(
                    formatter,
                    "KUC missing document viewer capability: {capability:?}"
                )
            }
        }
    }
}

impl Error for KucViewerError {}
