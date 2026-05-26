//! UI-independent document artifact and export foundation for KatanA.
//!
//! KDV receives KMM public DTOs, delegates supported rendering to KRR,
//! and keeps viewer UI concerns outside this crate.

pub mod artifact;
pub mod backend;
pub mod cli_api;
pub mod document;
pub mod evaluation;
mod export_assets;
#[path = "export_html/export_block_payload.rs"]
mod export_block_payload;
#[path = "export_html/export_code_payload.rs"]
mod export_code_payload;
pub mod export_contract;
mod export_contract_entries;
#[path = "export_html/export_details_payload.rs"]
mod export_details_payload;
#[path = "export_html/export_footnote_payload.rs"]
mod export_footnote_payload;
#[path = "export_html/export_heading_payload.rs"]
mod export_heading_payload;
#[path = "export_html/export_html_ops.rs"]
mod export_html_ops;
#[path = "export_html/export_html_payload.rs"]
mod export_html_payload;
#[path = "export_html/export_html_style.rs"]
mod export_html_style;
#[path = "export_payload/export_image_payload.rs"]
mod export_image_payload;
#[path = "export_html/export_inline_payload.rs"]
mod export_inline_payload;
#[path = "export_html/export_legacy_note_payload.rs"]
mod export_legacy_note_payload;
#[path = "export_html/export_list_payload.rs"]
mod export_list_payload;
#[path = "export_html/export_math_payload.rs"]
mod export_math_payload;
#[path = "export_payload/export_payload.rs"]
mod export_payload;
#[path = "export_payload/export_pdf_payload.rs"]
mod export_pdf_payload;
mod export_quality;
mod export_semantics;
mod export_surface;
#[path = "export_surface/export_surface_code.rs"]
mod export_surface_code;
#[path = "export_surface/export_surface_font.rs"]
mod export_surface_font;
#[path = "export_surface/export_surface_helpers.rs"]
mod export_surface_helpers;
#[path = "export_surface/export_surface_line.rs"]
mod export_surface_line;
#[path = "export_surface/export_surface_math.rs"]
mod export_surface_math;
#[path = "export_surface/export_surface_span.rs"]
mod export_surface_span;
#[path = "export_surface/export_surface_svg.rs"]
mod export_surface_svg;
#[path = "export_surface/export_surface_text.rs"]
mod export_surface_text;
#[path = "export_html/export_table_payload.rs"]
mod export_table_payload;
pub mod forge;
mod forge_diagram_render;
mod forge_diagram_render_types;
mod forge_types;
mod html_sanitizer;
mod render_runtime;
mod theme;

pub use artifact::{
    Artifact, ArtifactBytes, ArtifactDiagnostic, ArtifactDiagnostics, ArtifactFormat, ArtifactId,
    ArtifactKind, ArtifactManifest, ArtifactUri, DiagnosticSeverity,
};
pub use backend::diagram::{KrrDiagramInputFactory, KrrRenderOutputFactory};
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
pub use export_quality::{
    ExportFormatQualityScore, ExportQualityArtifacts, ExportQualityCheck, ExportQualityGate,
    ExportQualityReport,
};
pub use forge::{
    BuildGraph, BuildProfile, BuildRequest, ExportFormat, ExportOutput, ExportRequest,
    ForgeBackend, ForgeDiagnostics, ForgeError, ForgePipeline, ManifestOnlyBackend,
    MarkdownEvaluationTarget, RenderedDiagram, TransformStep,
};
pub use forge_diagram_render_types::{
    DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend, KrrDiagramRenderEngine,
};
pub use theme::{KdvThemeMode, KdvThemeSnapshot};

#[cfg(test)]
mod test_support;
