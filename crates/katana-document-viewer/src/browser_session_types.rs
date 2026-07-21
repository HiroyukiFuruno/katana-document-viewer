use katana_render_runtime::{
    HtmlBrowserError, HtmlBrowserFrame, HtmlBrowserInput, HtmlBrowserNavigation,
    HtmlBrowserNavigationEvent, HtmlBrowserSource, HtmlBrowserViewport,
};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct BrowserSessionRequest {
    pub source: HtmlBrowserSource,
    pub viewport: HtmlBrowserViewport,
}

impl BrowserSessionRequest {
    pub fn new(source: HtmlBrowserSource, viewport: HtmlBrowserViewport) -> Self {
        Self { source, viewport }
    }
}

#[derive(Debug, PartialEq)]
pub enum BrowserSessionUpdate {
    Frame(HtmlBrowserFrame),
    Navigation(HtmlBrowserNavigationEvent),
    Error(BrowserSessionAdapterError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BrowserSessionAdapterError {
    #[error(transparent)]
    Browser(#[from] HtmlBrowserError),
    #[error(
        "HTML browser operation failed\nLayer: KRR runtime\nOperation: {operation}\nDocument: {document_origin}\nCause: {source}"
    )]
    BrowserOperation {
        operation: BrowserSessionOperation,
        document_origin: String,
        #[source]
        source: HtmlBrowserError,
    },
    #[error("browser command queue is full")]
    CommandQueueFull,
    #[error("browser worker has stopped")]
    WorkerStopped,
    #[error("browser worker panicked")]
    WorkerPanicked,
}

impl BrowserSessionAdapterError {
    pub(crate) fn browser_operation(
        operation: BrowserSessionOperation,
        document_origin: impl Into<String>,
        source: HtmlBrowserError,
    ) -> Self {
        Self::BrowserOperation {
            operation,
            document_origin: document_origin.into(),
            source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserSessionOperation {
    Start,
    Input,
    Resize,
    Navigate,
    Refresh,
}

impl fmt::Display for BrowserSessionOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Start => "start",
            Self::Input => "input",
            Self::Resize => "resize",
            Self::Navigate => "navigate",
            Self::Refresh => "refresh",
        })
    }
}

#[derive(Debug)]
pub(crate) enum BrowserSessionCommand {
    Input(HtmlBrowserInput),
    Resize(HtmlBrowserViewport),
    Navigate(HtmlBrowserNavigation),
    Refresh,
    Close,
}

#[cfg(test)]
#[path = "browser_session_types_tests.rs"]
mod tests;
