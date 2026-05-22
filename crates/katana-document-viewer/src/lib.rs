//! UI-independent document artifact and export foundation for KatanA.
//!
//! KDV receives KMM public DTOs, delegates supported diagram rendering to KDR,
//! and keeps viewer UI concerns outside this crate.

pub mod artifact;
pub mod backend;
pub mod cli_api;
pub mod document;
pub mod evaluation;
mod export_block_payload;
mod export_code_payload;
pub mod export_contract;
mod export_contract_entries;
mod export_details_payload;
mod export_footnote_payload;
mod export_heading_payload;
mod export_html_ops;
mod export_html_payload;
mod export_html_style;
mod export_image_payload;
mod export_inline_payload;
mod export_legacy_note_payload;
mod export_list_payload;
mod export_math_payload;
mod export_payload;
mod export_pdf_payload;
mod export_surface;
mod export_surface_code;
mod export_surface_font;
mod export_surface_helpers;
mod export_surface_line;
mod export_surface_math;
mod export_surface_span;
mod export_surface_svg;
mod export_surface_text;
mod export_table_payload;
pub mod forge;
mod forge_diagram_render;
mod html_sanitizer;
mod render_runtime;
mod theme;

pub use artifact::{
    Artifact, ArtifactBytes, ArtifactDiagnostic, ArtifactDiagnostics, ArtifactFormat, ArtifactId,
    ArtifactKind, ArtifactManifest, ArtifactUri, DiagnosticSeverity,
};
pub use backend::diagram::{KdrDiagramInputFactory, KdrRenderOutputFactory};
pub use cli_api::{
    CliApi, CliBuildRequest, CliDiagnostics, CliExportDebugRequest, CliExportRequest, CliOutput,
    CliRequest, CliThemeMode,
};
pub use document::{
    DocumentId, DocumentKind, DocumentMetadataView, DocumentModelError, DocumentOutline,
    DocumentOutlineItem, DocumentSnapshot, DocumentSnapshotFactory, DocumentSource, SourceKind,
    SourceRevision, SourceUri,
};
pub use evaluation::{
    BackendCapability, BackendCapabilityMatrix, CoverageStatus, EvaluationCoverageMatrix,
    EvaluationFeatureCoverage, EvaluationFixture, EvaluationFixtureMatrix, FixtureCategory,
};
pub use export_contract::{HtmlExportContractEntry, HtmlExportContractMatrix, HtmlExportReadiness};
pub use forge::{
    BuildGraph, BuildProfile, BuildRequest, ExportFormat, ExportOutput, ExportRequest,
    ForgeBackend, ForgeDiagnostics, ForgeError, ForgePipeline, ManifestOnlyBackend,
    MarkdownEvaluationTarget, RenderedDiagram, TransformStep,
};
pub use forge_diagram_render::{
    DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend, KdrDiagramRenderEngine,
};
pub use theme::{KdvThemeMode, KdvThemeSnapshot};

#[cfg(test)]
mod test_support;
