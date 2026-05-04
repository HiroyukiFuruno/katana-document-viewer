//! katana-markdown-preview-egui: egui implementation of MarkdownPreview.
//!
//! MVP backend. KatanA depends on `katana-markdown-preview` (neutral interface)
//! and wires in this implementation at startup. When the custom UI replaces
//! egui, only this crate changes; KatanA's interface dependency is untouched.
//!
//! Status: scaffolding. Full implementation migrated from KatanA v0.26.0.

use katana_markdown_preview::{
    MarkdownPreview, MarkdownSource, PreviewConfig, PreviewError, PreviewOutput,
};

pub struct EguiMarkdownPreview;

impl MarkdownPreview for EguiMarkdownPreview {
    fn render(
        &self,
        _source: &MarkdownSource,
        _config: &PreviewConfig,
    ) -> Result<PreviewOutput, PreviewError> {
        Err(PreviewError::NotImplemented)
    }
}

/// Stateful egui widget that wraps `EguiMarkdownPreview` and draws into a
/// `egui::Ui`. KatanA calls this directly during the MVP phase.
pub struct MarkdownPreviewWidget {
    inner: EguiMarkdownPreview,
}

impl MarkdownPreviewWidget {
    pub fn new() -> Self {
        Self {
            inner: EguiMarkdownPreview,
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, source: &MarkdownSource, config: &PreviewConfig) {
        match self.inner.render(source, config) {
            Ok(_) => {
                ui.label("[scaffold] katana-markdown-preview-egui");
            }
            Err(e) => {
                ui.label(format!("[error] {e}"));
            }
        }
    }
}

impl Default for MarkdownPreviewWidget {
    fn default() -> Self {
        Self::new()
    }
}
