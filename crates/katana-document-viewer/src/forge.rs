use crate::artifact::{ArtifactBytes, ArtifactFactory};
use crate::export_payload::ExportPayloadFactory;
pub use crate::forge_types::{
    BuildGraph, BuildProfile, BuildRequest, ExportFormat, ExportOutput, ExportRequest,
    ForgeBackend, ForgeDiagnostics, ForgeError, MarkdownEvaluationTarget, RenderedDiagram,
    TransformStep,
};

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

impl<B: ForgeBackend> ForgePipeline<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn build(&self, request: &BuildRequest) -> Result<BuildGraph, ForgeError> {
        self.backend.build(request)
    }

    pub fn export(&self, request: &ExportRequest) -> Result<ExportOutput, ForgeError> {
        validate_export_output(self.backend.export(request), request.format)
    }
}

fn validate_export_output(
    result: Result<ExportOutput, ForgeError>,
    format: ExportFormat,
) -> Result<ExportOutput, ForgeError> {
    match result {
        Ok(output) => reject_empty_export_output(output, format),
        Err(error) => Err(error),
    }
}

fn reject_empty_export_output(
    output: ExportOutput,
    format: ExportFormat,
) -> Result<ExportOutput, ForgeError> {
    if output.artifact.bytes.bytes.is_empty() {
        Err(ForgeError::EmptyExportArtifact(format))
    } else {
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
        ExportPayloadFactory::create(&request.graph, request.format, &request.theme).map(|bytes| {
            let artifact = ArtifactFactory::export(
                request.format.artifact_format(),
                snapshot.id.clone(),
                snapshot.revision.clone(),
                ArtifactBytes { bytes },
            );
            ExportOutput {
                artifact,
                diagnostics: request.graph.diagnostics.clone(),
            }
        })
    }
}

#[cfg(test)]
#[path = "forge_tests.rs"]
mod tests;
