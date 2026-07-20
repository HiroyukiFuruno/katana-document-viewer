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
use std::thread;

#[path = "fixture_score_matrix_export_cache.rs"]
mod export_cache;
use export_cache::{ExportCacheKey, export_cache, export_cache_lock};

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
        .map_err(Into::into)
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
        if let Some(bytes) = export_cache_lock().get(&key).cloned() {
            return Ok(bytes);
        }

        cache_built_export(key, Self::build(output))
    }

    fn build(output: &PreviewOutput) -> Result<Self, ForgeError> {
        let theme = KdvThemeSnapshot::katana_light();
        let graph = BuildGraph::from_request(&BuildRequest {
            snapshot: output.input.snapshot.clone(),
            profile: BuildProfile::markdown_export(),
            theme: theme.clone(),
        });
        export_bytes_from_payloads(FixtureExportPayloadFactory::from_graph(&graph, &theme))
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

fn cache_built_export(
    key: ExportCacheKey,
    result: Result<ExportBytes, ForgeError>,
) -> Result<ExportBytes, ForgeError> {
    match result {
        Ok(bytes) => {
            export_cache_lock().insert(key, bytes.clone());
            Ok(bytes)
        }
        Err(error) => Err(error),
    }
}

fn export_bytes_from_payloads(
    result: Result<FixtureExportPayloads, ForgeError>,
) -> Result<ExportBytes, ForgeError> {
    match result {
        Ok(payloads) => Ok(ExportBytes {
            html: payloads.html,
            pdf: payloads.pdf,
            png: payloads.png,
            jpeg: payloads.jpeg,
        }),
        Err(error) => Err(error),
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
        let raster_payloads = thread::scope(|scope| {
            let pdf_handle = scope.spawn(|| PdfExportPayloadFactory::create(&surface));
            let png_handle = scope.spawn(|| ImageExportPayloadFactory::create_png(&surface));
            let jpeg_handle = scope.spawn(|| ImageExportPayloadFactory::create_jpeg(&surface));
            join_raster_payloads(
                join_payload("PDF", pdf_handle),
                join_payload("PNG", png_handle),
                join_payload("JPEG", jpeg_handle),
            )
        });
        fixture_export_payloads(html, raster_payloads)
    }
}

fn join_raster_payloads(
    pdf: Result<Vec<u8>, ForgeError>,
    png: Result<Vec<u8>, ForgeError>,
    jpeg: Result<Vec<u8>, ForgeError>,
) -> Result<ExportRasterPayloads, ForgeError> {
    match (pdf, png, jpeg) {
        (Err(error), _, _) => Err(error),
        (Ok(_), Err(error), _) => Err(error),
        (Ok(_), Ok(_), Err(error)) => Err(error),
        (Ok(pdf), Ok(png), Ok(jpeg)) => Ok((pdf, png, jpeg)),
    }
}

fn fixture_export_payloads(
    html: Vec<u8>,
    raster_payloads: Result<ExportRasterPayloads, ForgeError>,
) -> Result<FixtureExportPayloads, ForgeError> {
    match raster_payloads {
        Ok((pdf, png, jpeg)) => Ok(FixtureExportPayloads {
            html,
            pdf,
            png,
            jpeg,
        }),
        Err(error) => Err(error),
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

fn config() -> PreviewConfig {
    PreviewConfig {
        viewport: ViewerViewport {
            width: 1024.0,
            height: CONTENT_HEIGHT,
        },
        ..PreviewConfig::default()
    }
}

#[cfg(test)]
#[path = "fixture_score_matrix_support_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "fixture_score_matrix_support_private_tests.rs"]
mod private_tests;
