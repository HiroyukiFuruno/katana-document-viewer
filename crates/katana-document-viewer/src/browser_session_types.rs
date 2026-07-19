use katana_render_runtime::{
    HtmlBrowserError, HtmlBrowserFrame, HtmlBrowserInput, HtmlBrowserNavigation,
    HtmlBrowserNavigationEvent, HtmlBrowserSource, HtmlBrowserViewport,
};
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
    #[error("browser command queue is full")]
    CommandQueueFull,
    #[error("browser worker has stopped")]
    WorkerStopped,
    #[error("browser worker panicked")]
    WorkerPanicked,
}

#[derive(Debug)]
pub(crate) enum BrowserSessionCommand {
    Input(HtmlBrowserInput),
    Resize(HtmlBrowserViewport),
    Navigate(HtmlBrowserNavigation),
    Refresh,
    Close,
}
