pub(crate) use super::fixture_score_matrix_assertions::FixtureScoreAssertions;
use crate::export_html_payload::HtmlExportPayloadFactory;
use crate::export_image_payload::ImageExportPayloadFactory;
use crate::export_pdf_payload::PdfExportPayloadFactory;
use crate::export_surface::DocumentSurfaceFactory;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, ExportQualityArtifacts, ExportQualityGate,
    ExportQualityReport, ForgeError, KdvThemeSnapshot, ViewerViewport,
};
use crate::{MarkdownSource, PreviewConfig, PreviewOutput, PreviewOutputFactory};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;

const CONTENT_HEIGHT: f32 = 20_000.0;
type ExportRasterPayloads = (Vec<u8>, Vec<u8>, Vec<u8>);

pub(crate) struct FixtureScoreCase<'a> {
    pub(crate) name: &'a str,
    pub(crate) document_id: &'a str,
    pub(crate) content: &'a str,
}

impl FixtureScoreCase<'_> {
    pub(crate) fn preview_output(&self) -> Result<PreviewOutput, Box<dyn std::error::Error>> {
        PreviewOutputFactory::from_source(
            &MarkdownSource {
                content: self.content.to_string(),
                document_id: Some(self.document_id.to_string()),
            },
            &config(),
            CONTENT_HEIGHT,
        )
        .map_err(|error| error.into())
    }
}

#[derive(Clone)]
pub(crate) struct ExportBytes {
    html: Vec<u8>,
    pdf: Vec<u8>,
    png: Vec<u8>,
    jpeg: Vec<u8>,
}

impl ExportBytes {
    pub(crate) fn from_output(output: &PreviewOutput) -> Result<Self, ForgeError> {
        let key = ExportCacheKey::from_output(output);
        if let Some(bytes) = export_cache_lock()?.get(&key).cloned() {
            return Ok(bytes);
        }

        let bytes = Self::build(output)?;
        export_cache_lock()?.insert(key, bytes.clone());
        Ok(bytes)
    }

    fn build(output: &PreviewOutput) -> Result<Self, ForgeError> {
        let theme = KdvThemeSnapshot::katana_light();
        let graph = BuildGraph::from_request(&BuildRequest {
            snapshot: output.input.snapshot.clone(),
            profile: BuildProfile::markdown_export(),
            theme: theme.clone(),
        });
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

pub(crate) struct FixtureExportPayloads {
    pub(crate) html: Vec<u8>,
    pub(crate) pdf: Vec<u8>,
    pub(crate) png: Vec<u8>,
    pub(crate) jpeg: Vec<u8>,
}

pub(crate) struct FixtureExportPayloadFactory;

impl FixtureExportPayloadFactory {
    pub(crate) fn from_graph(
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
    ) -> Result<FixtureExportPayloads, ForgeError> {
        let surface = DocumentSurfaceFactory::create(graph, theme);
        let html = HtmlExportPayloadFactory::create(graph, theme);
        let (pdf, png, jpeg) =
            thread::scope(|scope| -> Result<ExportRasterPayloads, ForgeError> {
                let pdf_handle = scope.spawn(|| PdfExportPayloadFactory::create(&surface));
                let png_handle = scope.spawn(|| ImageExportPayloadFactory::create_png(&surface));
                let jpeg_handle = scope.spawn(|| ImageExportPayloadFactory::create_jpeg(&surface));
                let pdf = join_payload("PDF", pdf_handle)?;
                let png = join_payload("PNG", png_handle)?;
                let jpeg = join_payload("JPEG", jpeg_handle)?;
                Ok((pdf, png, jpeg))
            })?;
        Ok(FixtureExportPayloads {
            html,
            pdf,
            png,
            jpeg,
        })
    }
}

fn join_payload(
    format: &str,
    handle: thread::ScopedJoinHandle<'_, Result<Vec<u8>, String>>,
) -> Result<Vec<u8>, ForgeError> {
    match handle.join() {
        Ok(result) => result.map_err(ForgeError::Export),
        Err(_) => Err(ForgeError::Export(format!(
            "{format} payload worker panicked"
        ))),
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct ExportCacheKey {
    document_id: String,
    revision: String,
    kind: String,
    source_path: String,
}

impl ExportCacheKey {
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

fn export_cache_lock()
-> Result<MutexGuard<'static, HashMap<ExportCacheKey, ExportBytes>>, ForgeError> {
    static CACHE: OnceLock<Mutex<HashMap<ExportCacheKey, ExportBytes>>> = OnceLock::new();
    CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|error| ForgeError::Backend(format!("fixture export cache lock failed: {error}")))
}

fn config() -> PreviewConfig {
    PreviewConfig {
        viewport: ViewerViewport {
            width: 1024.0,
            height: CONTENT_HEIGHT,
        },
        ..PreviewConfig::default()
    }
}
