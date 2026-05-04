//! katana-document-preview: vendor-neutral Markdown preview interface.
//!
//! This crate defines the neutral trait surface and data-only types that hosts
//! (KatanA, future non-egui hosts) depend on. It has no dependency on egui or
//! any specific UI framework.
//!
//! The egui implementation lives in `katana-document-preview-egui`. When KatanA
//! eventually migrates away from egui, only the `-egui` crate is replaced;
//! KatanA's dependency on this crate stays unchanged.

pub mod types;
pub use types::{
    MarkdownSource, PreviewConfig, PreviewDiagnostics, PreviewError, PreviewOutput, PreviewTheme,
    RenderTarget,
};

/// Vendor-neutral Markdown preview renderer trait.
///
/// Implementations are free to use any UI framework internally. The trait
/// surface contains only neutral types so KatanA never sees egui types.
pub trait MarkdownPreview {
    fn render(&self, source: &MarkdownSource, config: &PreviewConfig)
        -> Result<PreviewOutput, PreviewError>;
}
