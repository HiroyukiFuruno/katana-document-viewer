use crate::artifact::{
    Artifact, ArtifactBytes, ArtifactDiagnostic, ArtifactDiagnostics, ArtifactFactory,
    ArtifactFormat, DiagnosticSeverity,
};
use crate::document::{DocumentId, SourceRevision};
use katana_diagram_renderer::{
    DiagramKind as KdrDiagramKind, RenderConfig, RenderContext, RenderInput,
    RenderOutput as KdrRenderOutput, RenderPolicy,
};
use katana_markdown_model::DiagramKind as KmmDiagramKind;

const KDR_BACKEND: &str = "katana-diagram-renderer";

pub struct KdrDiagramInputFactory;
pub struct KdrRenderOutputFactory;

impl KdrDiagramInputFactory {
    pub fn create(kind: KmmDiagramKind, source: String, context: RenderContext) -> RenderInput {
        RenderInput {
            kind: Self::diagram_kind(kind),
            source,
            config: RenderConfig::default(),
            policy: RenderPolicy::default(),
            context,
        }
    }

    fn diagram_kind(kind: KmmDiagramKind) -> KdrDiagramKind {
        match kind {
            KmmDiagramKind::Mermaid => KdrDiagramKind::Mermaid,
            KmmDiagramKind::DrawIo => KdrDiagramKind::Drawio,
            KmmDiagramKind::PlantUml => KdrDiagramKind::PlantUml,
        }
    }
}

impl KdrRenderOutputFactory {
    pub fn artifact_from_render(
        output: &KdrRenderOutput,
        document_id: DocumentId,
        source_revision: SourceRevision,
    ) -> Artifact {
        ArtifactFactory::image_with_backend(
            ArtifactFormat::Svg,
            document_id,
            source_revision,
            ArtifactBytes {
                bytes: output.svg.as_bytes().to_vec(),
            },
            KDR_BACKEND,
            Self::diagnostics(output),
        )
    }

    fn diagnostics(output: &KdrRenderOutput) -> ArtifactDiagnostics {
        let warning_entries = output
            .diagnostics
            .warnings
            .iter()
            .map(|message| Self::diagnostic(DiagnosticSeverity::Warning, message));
        let error_entries = output
            .diagnostics
            .errors
            .iter()
            .map(|message| Self::diagnostic(DiagnosticSeverity::Error, message));
        ArtifactDiagnostics {
            entries: warning_entries.chain(error_entries).collect(),
        }
    }

    fn diagnostic(severity: DiagnosticSeverity, message: &str) -> ArtifactDiagnostic {
        ArtifactDiagnostic {
            severity,
            code: "kdr-render-diagnostic".to_string(),
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact::ArtifactKind;
    use katana_diagram_renderer::{RenderDiagnostics, RendererProfile, RuntimeVersion};

    #[test]
    fn maps_kmm_drawio_to_kdr_drawio() {
        let input = KdrDiagramInputFactory::create(
            KmmDiagramKind::DrawIo,
            "<mxfile />".to_string(),
            RenderContext::default(),
        );

        assert_eq!(input.kind, KdrDiagramKind::Drawio);
    }

    #[test]
    fn maps_kmm_plantuml_to_kdr_plantuml() {
        let input = KdrDiagramInputFactory::create(
            KmmDiagramKind::PlantUml,
            "@startuml".to_string(),
            RenderContext::default(),
        );

        assert_eq!(input.kind, KdrDiagramKind::PlantUml);
    }

    #[test]
    fn converts_kdr_render_output_to_svg_image_artifact() {
        let artifact = KdrRenderOutputFactory::artifact_from_render(
            &render_output(),
            DocumentId("doc".to_string()),
            SourceRevision("rev".to_string()),
        );

        assert_eq!(artifact.manifest.backend, KDR_BACKEND);
        assert_eq!(artifact.manifest.kind, ArtifactKind::Image);
        assert_eq!(artifact.manifest.format, ArtifactFormat::Svg);
        assert_eq!(artifact.manifest.diagnostics.entries.len(), 2);
    }

    fn render_output() -> KdrRenderOutput {
        KdrRenderOutput {
            svg: "<svg />".to_string(),
            width: 1.0,
            height: 1.0,
            view_box: "0 0 1 1".to_string(),
            runtime: RuntimeVersion {
                name: "test".to_string(),
                version: "0".to_string(),
                checksum: None,
            },
            profile: RendererProfile {
                id: "test".to_string(),
                description: None,
            },
            diagnostics: RenderDiagnostics {
                warnings: vec!["warn".to_string()],
                errors: vec!["err".to_string()],
            },
            cache_fingerprint: "fp".to_string(),
        }
    }
}
