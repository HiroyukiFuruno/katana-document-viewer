use super::*;
use crate::artifact::{ArtifactBytes, ArtifactFactory, ArtifactFormat};
use crate::test_support::SampleSnapshotFactory;

#[test]
fn manifest_backend_exports_non_empty_formats() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(ManifestOnlyBackend);
    let graph = BuildGraph::from_request(&BuildRequest {
        snapshot: SampleSnapshotFactory::create()?,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    });

    for (format, artifact_format) in [
        (ExportFormat::Html, ArtifactFormat::Html),
        (ExportFormat::Pdf, ArtifactFormat::Pdf),
        (ExportFormat::Png, ArtifactFormat::Png),
        (ExportFormat::Jpeg, ArtifactFormat::Jpeg),
    ] {
        let output = pipeline.export(&ExportRequest {
            graph: graph.clone(),
            format,
            theme: crate::KdvThemeSnapshot::katana_light(),
        })?;
        assert_eq!(output.artifact.manifest.format, artifact_format);
        assert_eq!(output.artifact.manifest.source_revision.0, "rev-1");
        assert!(!output.artifact.bytes.bytes.is_empty());
    }
    Ok(())
}

#[test]
fn forge_pipeline_rejects_empty_export_artifact() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(EmptyBackend);
    let graph = BuildGraph::from_request(&BuildRequest {
        snapshot: SampleSnapshotFactory::create()?,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    });
    let result = pipeline.export(&ExportRequest {
        graph,
        format: ExportFormat::Html,
        theme: crate::KdvThemeSnapshot::katana_light(),
    });

    assert!(matches!(
        result,
        Err(ForgeError::EmptyExportArtifact(ExportFormat::Html))
    ));
    Ok(())
}

#[test]
fn non_html_exports_do_not_claim_html_backed_rendering() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(ManifestOnlyBackend);
    let graph = BuildGraph::from_request(&BuildRequest {
        snapshot: SampleSnapshotFactory::create()?,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    });
    let theme = crate::KdvThemeSnapshot::katana_light();
    let outputs = [
        export_bytes(&pipeline, &graph, ExportFormat::Pdf, &theme)?,
        export_bytes(&pipeline, &graph, ExportFormat::Png, &theme)?,
        export_bytes(&pipeline, &graph, ExportFormat::Jpeg, &theme)?,
    ];

    for bytes in outputs {
        assert!(
            !contains_bytes(&bytes, b"KDV_HTML_FINGERPRINT"),
            "non-HTML export must not claim rendered HTML fidelity before the Rust backend exists"
        );
    }
    Ok(())
}

#[test]
fn non_html_exports_use_rust_rendered_document_surface() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(ManifestOnlyBackend);
    let graph = BuildGraph::from_request(&BuildRequest {
        snapshot: SampleSnapshotFactory::create()?,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    });
    let theme = crate::KdvThemeSnapshot::katana_light();
    let pdf = export_bytes(&pipeline, &graph, ExportFormat::Pdf, &theme)?;
    let png = export_bytes(&pipeline, &graph, ExportFormat::Png, &theme)?;
    let jpeg = export_bytes(&pipeline, &graph, ExportFormat::Jpeg, &theme)?;

    assert!(
        contains_bytes(&pdf, b"/Subtype /Image"),
        "PDF export must embed a Rust-rendered document surface image"
    );
    assert!(
        !contains_bytes(&pdf, b"not implemented"),
        "PDF export must not be a placeholder message"
    );
    assert_eq!(png_dimensions(&png), Some((1280, 720)));
    assert!(
        jpeg.len() > TINY_JPEG_BYTE_LEN * 10,
        "JPEG export must not be the 1x1 placeholder"
    );
    Ok(())
}

struct EmptyBackend;

impl ForgeBackend for EmptyBackend {
    fn build(&self, request: &BuildRequest) -> Result<BuildGraph, ForgeError> {
        Ok(BuildGraph::from_request(request))
    }

    fn export(&self, request: &ExportRequest) -> Result<ExportOutput, ForgeError> {
        let snapshot = &request.graph.snapshot;
        let artifact = ArtifactFactory::export(
            request.format.artifact_format(),
            snapshot.id.clone(),
            snapshot.revision.clone(),
            ArtifactBytes { bytes: Vec::new() },
        );
        Ok(ExportOutput {
            artifact,
            diagnostics: request.graph.diagnostics.clone(),
        })
    }
}

fn export_bytes(
    pipeline: &ForgePipeline<ManifestOnlyBackend>,
    graph: &BuildGraph,
    format: ExportFormat,
    theme: &crate::KdvThemeSnapshot,
) -> Result<Vec<u8>, ForgeError> {
    let output = pipeline.export(&ExportRequest {
        graph: graph.clone(),
        format,
        theme: theme.clone(),
    })?;
    Ok(output.artifact.bytes.bytes)
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    let signature = b"\x89PNG\r\n\x1a\n";
    if bytes.len() < 24 || &bytes[..8] != signature {
        return None;
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    Some((width, height))
}

const TINY_JPEG_BYTE_LEN: usize = 717;
