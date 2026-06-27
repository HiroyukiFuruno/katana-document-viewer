use crate::preview_runtime::fixture_score_matrix_support::FixtureExportPayloadFactory;
use crate::{
    BuildProfile, BuildRequest, DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend,
    ExportQualityArtifacts, ExportQualityGate, ExportQualityReport, ForgeError, ForgePipeline,
    KdvThemeSnapshot, PreviewOutput, RenderedDiagram,
};
use katana_markdown_model::DiagramKind;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

const RENDER_RUNTIME_ID: &str = "katana-render-runtime";

#[derive(Clone)]
pub(crate) struct RenderedExportBytes {
    html: Vec<u8>,
    pdf: Vec<u8>,
    png: Vec<u8>,
    jpeg: Vec<u8>,
}

impl RenderedExportBytes {
    pub(crate) fn from_output(output: &PreviewOutput) -> Result<Self, ForgeError> {
        let key = RenderedExportCacheKey::from_output(output);
        if let Some(bytes) = rendered_export_cache_lock()?.get(&key).cloned() {
            return Ok(bytes);
        }

        let bytes = Self::build(output)?;
        rendered_export_cache_lock()?.insert(key, bytes.clone());
        Ok(bytes)
    }

    fn build(output: &PreviewOutput) -> Result<Self, ForgeError> {
        let theme = KdvThemeSnapshot::katana_light();
        let request = BuildRequest {
            snapshot: output.input.snapshot.clone(),
            profile: BuildProfile::markdown_export(),
            theme: theme.clone(),
        };
        let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(FixtureDiagramRenderEngine));
        let graph = pipeline.build(&request)?;
        let payloads = FixtureExportPayloadFactory::from_graph(&graph, &theme)?;

        Ok(Self {
            html: payloads.html,
            pdf: payloads.pdf,
            png: payloads.png,
            jpeg: payloads.jpeg,
        })
    }

    pub(crate) fn score_report(&self, source: &str) -> ExportQualityReport {
        ExportQualityGate::evaluate(&ExportQualityArtifacts {
            html: &self.html,
            pdf: &self.pdf,
            png: &self.png,
            jpeg: &self.jpeg,
            source_markdown: source,
            surface_equivalence: None,
        })
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct RenderedExportCacheKey {
    document_id: String,
    revision: String,
    kind: String,
    source_path: String,
}

impl RenderedExportCacheKey {
    fn from_output(output: &PreviewOutput) -> Self {
        let snapshot = &output.input.snapshot;
        Self {
            document_id: snapshot.id.0.clone(),
            revision: snapshot.revision.0.clone(),
            kind: format!("{:?}", snapshot.kind),
            source_path: snapshot.source_path.to_string_lossy().to_string(),
        }
    }
}

fn rendered_export_cache_lock()
-> Result<MutexGuard<'static, HashMap<RenderedExportCacheKey, RenderedExportBytes>>, ForgeError> {
    static CACHE: OnceLock<Mutex<HashMap<RenderedExportCacheKey, RenderedExportBytes>>> =
        OnceLock::new();
    CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|error| {
            ForgeError::Backend(format!(
                "fixture rendered export cache lock failed: {error}"
            ))
        })
}

struct FixtureDiagramRenderEngine;

impl DiagramRenderEngine for FixtureDiagramRenderEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let kind = diagram_kind_label(&request.kind);
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: kind.to_string(),
            svg: fixture_svg(kind, request.theme.diagram_theme_label()),
        })
    }
}

fn diagram_kind_label(kind: &DiagramKind) -> &'static str {
    match kind {
        DiagramKind::Mermaid => "mermaid",
        DiagramKind::DrawIo => "drawio",
        DiagramKind::PlantUml => "plantuml",
    }
}

fn fixture_svg(kind: &str, theme: &str) -> String {
    format!(
        "<svg data-kdv-render-runtime=\"{RENDER_RUNTIME_ID}\" viewBox=\"0 0 160 72\" \
         xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" \
         aria-label=\"rendered {kind} diagram\"><g data-kdv-rendered=\"{kind}\" \
         data-kdv-theme=\"{theme}\"><rect x=\"2\" y=\"2\" width=\"156\" height=\"68\" \
         rx=\"4\"></rect><text x=\"16\" y=\"42\">{kind}</text></g></svg>"
    )
}
