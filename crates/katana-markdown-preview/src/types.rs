use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownSource {
    pub content: String,
    pub document_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewTheme {
    pub name: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewConfig {
    pub theme: PreviewTheme,
    pub base_font_size: Option<f32>,
    pub line_height: Option<f32>,
}

/// Neutral render target abstraction.
///
/// MVP: egui implementation renders into egui. Future: custom UI implementation
/// renders into its own surface. KatanA only sees this trait.
pub trait RenderTarget: Send + Sync {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewOutput {
    pub scroll_offset: f32,
    pub content_height: f32,
    pub diagnostics: PreviewDiagnostics,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewDiagnostics {
    pub warnings: Vec<String>,
}

#[derive(Debug, Error)]
pub enum PreviewError {
    #[error("render error: {0}")]
    Render(String),
    #[error("not implemented")]
    NotImplemented,
}
