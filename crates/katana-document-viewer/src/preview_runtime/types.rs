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

impl PreviewTheme {
    pub fn is_dark(&self) -> bool {
        self.name.to_ascii_lowercase().contains("dark")
            || self.fingerprint.to_ascii_lowercase().contains("mode=dark")
    }

    pub fn is_katana_export_reference(&self) -> bool {
        self.name
            .to_ascii_lowercase()
            .contains("katana-export-reference")
            || self
                .fingerprint
                .to_ascii_lowercase()
                .contains("katana-export-reference")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewConfig {
    pub theme: PreviewTheme,
    pub base_font_size: Option<f32>,
    pub line_height: Option<f32>,
    pub mode: crate::ViewerMode,
    pub interaction: crate::ViewerInteractionConfig,
    pub viewport: crate::ViewerViewport,
    pub scroll_offset: f32,
    pub search: crate::ViewerSearchState,
}

/// Neutral render target abstraction.
///
/// Implementations render into their own UI surface. KatanA only sees this
/// neutral trait.
pub trait RenderTarget: Send + Sync {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewOutput {
    pub scroll_offset: f32,
    pub content_height: f32,
    pub diagnostics: PreviewDiagnostics,
    pub surface: Option<PreviewSurfaceImage>,
    pub input: crate::ViewerInput,
    pub state: crate::ViewerStateSnapshot,
    pub commands: Vec<crate::ViewerCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewSurfaceImage {
    pub fingerprint: String,
    pub width: u32,
    pub height: u32,
    pub origin_y: u32,
    pub content_height: u32,
    pub rgba: Vec<u8>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewDiagnostics {
    pub warnings: Vec<String>,
}

#[derive(Debug, Error)]
pub enum PreviewError {
    #[error("render error: {0}")]
    Render(String),
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            theme: PreviewTheme::default(),
            base_font_size: None,
            line_height: None,
            mode: crate::ViewerMode::Document,
            interaction: crate::ViewerInteractionConfig::default(),
            viewport: crate::ViewerViewport {
                width: 0.0,
                height: 0.0,
            },
            scroll_offset: 0.0,
            search: crate::ViewerSearchState::default(),
        }
    }
}
