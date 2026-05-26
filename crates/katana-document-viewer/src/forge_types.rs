use crate::artifact::{Artifact, ArtifactFormat};
use crate::document::DocumentSnapshot;
use crate::theme::KdvThemeSnapshot;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkdownEvaluationTarget {
    CommonMark,
    Gfm,
    Math,
    GitHubAlert,
    KatanaCompatibility,
    ExternalRendering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransformStep {
    EvaluateMarkdown,
    RenderDiagrams,
    BuildArtifactManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildProfile {
    pub evaluation_targets: Vec<MarkdownEvaluationTarget>,
    pub transform_steps: Vec<TransformStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRequest {
    pub snapshot: DocumentSnapshot,
    pub profile: BuildProfile,
    pub theme: KdvThemeSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildGraph {
    pub snapshot: DocumentSnapshot,
    pub profile: BuildProfile,
    pub theme: KdvThemeSnapshot,
    pub diagnostics: ForgeDiagnostics,
    pub rendered_diagrams: Vec<RenderedDiagram>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedDiagram {
    pub node_id: String,
    pub kind: String,
    pub svg: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    Html,
    Pdf,
    Png,
    Jpeg,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportRequest {
    pub graph: BuildGraph,
    pub format: ExportFormat,
    pub theme: KdvThemeSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportOutput {
    pub artifact: Artifact,
    pub diagnostics: ForgeDiagnostics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForgeDiagnostics {
    pub messages: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ForgeError {
    #[error("backend failed: {0}")]
    Backend(String),
    #[error("export failed: {0}")]
    Export(String),
    #[error("export produced empty artifact bytes for {0:?}")]
    EmptyExportArtifact(ExportFormat),
}

pub trait ForgeBackend {
    fn build(&self, request: &BuildRequest) -> Result<BuildGraph, ForgeError>;
    fn export(&self, request: &ExportRequest) -> Result<ExportOutput, ForgeError>;
}

impl ExportFormat {
    pub fn artifact_format(self) -> ArtifactFormat {
        match self {
            Self::Html => ArtifactFormat::Html,
            Self::Pdf => ArtifactFormat::Pdf,
            Self::Png => ArtifactFormat::Png,
            Self::Jpeg => ArtifactFormat::Jpeg,
        }
    }
}
