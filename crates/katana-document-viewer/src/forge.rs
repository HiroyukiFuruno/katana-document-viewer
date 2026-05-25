use crate::artifact::{Artifact, ArtifactBytes, ArtifactFactory, ArtifactFormat};
use crate::document::DocumentSnapshot;
use crate::export_payload::ExportPayloadFactory;
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

pub struct ForgePipeline<B> {
    backend: B,
}

impl BuildProfile {
    pub fn markdown_export() -> Self {
        Self {
            evaluation_targets: vec![
                MarkdownEvaluationTarget::CommonMark,
                MarkdownEvaluationTarget::Gfm,
                MarkdownEvaluationTarget::Math,
                MarkdownEvaluationTarget::GitHubAlert,
                MarkdownEvaluationTarget::KatanaCompatibility,
                MarkdownEvaluationTarget::ExternalRendering,
            ],
            transform_steps: vec![
                TransformStep::EvaluateMarkdown,
                TransformStep::RenderDiagrams,
                TransformStep::BuildArtifactManifest,
            ],
        }
    }
}

impl BuildGraph {
    pub fn from_request(request: &BuildRequest) -> Self {
        Self {
            snapshot: request.snapshot.clone(),
            profile: request.profile.clone(),
            theme: request.theme.clone(),
            diagnostics: ForgeDiagnostics {
                messages: Vec::new(),
            },
            rendered_diagrams: Vec::new(),
        }
    }

    pub fn with_rendered_diagrams(mut self, rendered_diagrams: Vec<RenderedDiagram>) -> Self {
        self.rendered_diagrams = rendered_diagrams;
        self
    }
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

impl<B: ForgeBackend> ForgePipeline<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn build(&self, request: &BuildRequest) -> Result<BuildGraph, ForgeError> {
        self.backend.build(request)
    }

    pub fn export(&self, request: &ExportRequest) -> Result<ExportOutput, ForgeError> {
        let output = self.backend.export(request)?;
        if output.artifact.bytes.bytes.is_empty() {
            return Err(ForgeError::EmptyExportArtifact(request.format));
        }
        Ok(output)
    }
}

pub struct ManifestOnlyBackend;

impl ForgeBackend for ManifestOnlyBackend {
    fn build(&self, request: &BuildRequest) -> Result<BuildGraph, ForgeError> {
        Ok(BuildGraph::from_request(request))
    }

    fn export(&self, request: &ExportRequest) -> Result<ExportOutput, ForgeError> {
        let snapshot = &request.graph.snapshot;
        let bytes = ExportPayloadFactory::create(&request.graph, request.format, &request.theme)?;
        let artifact = ArtifactFactory::export(
            request.format.artifact_format(),
            snapshot.id.clone(),
            snapshot.revision.clone(),
            ArtifactBytes { bytes },
        );
        Ok(ExportOutput {
            artifact,
            diagnostics: request.graph.diagnostics.clone(),
        })
    }
}

#[cfg(test)]
#[path = "forge_tests.rs"]
mod tests;
