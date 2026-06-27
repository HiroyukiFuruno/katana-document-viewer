use super::DirectVisualCase;
use katana_document_viewer::{
    BuildProfile, BuildRequest, DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend,
    ExportFormat, ExportQualityArtifacts, ExportQualityGate, ExportRequest, ForgePipeline,
    KdvPreviewSurfaceFactory, KdvThemeSnapshot, MarkdownSource, PreviewConfig,
    PreviewOutputFactory, RenderedDiagram, SurfaceEquivalenceArtifacts, SurfaceEquivalenceImage,
    ViewerViewport,
};
use katana_markdown_model::DiagramKind;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct ExportedDirectVisual {
    pub html: Vec<u8>,
    pdf: Vec<u8>,
    png: Vec<u8>,
    jpeg: Vec<u8>,
    reference_rgba: Vec<u8>,
    reference_width: u32,
    reference_height: u32,
}

impl ExportedDirectVisual {
    pub fn export(case: DirectVisualCase, output_dir: &Path) -> Result<Self, Box<dyn Error>> {
        let theme = KdvThemeSnapshot::katana_light();
        let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(StaticDiagramEngine));
        let snapshot = direct_snapshot(&case)?;
        let graph = pipeline.build(&BuildRequest {
            snapshot,
            profile: BuildProfile::markdown_export(),
            theme: theme.clone(),
        })?;
        let html = export_bytes(&pipeline, &graph, &theme, ExportFormat::Html)?;
        let pdf = export_bytes(&pipeline, &graph, &theme, ExportFormat::Pdf)?;
        let png = export_bytes(&pipeline, &graph, &theme, ExportFormat::Png)?;
        let jpeg = export_bytes(&pipeline, &graph, &theme, ExportFormat::Jpeg)?;
        let preview_surface = KdvPreviewSurfaceFactory::create(
            &graph,
            &theme,
            ViewerViewport {
                width: 1280.0,
                height: 720.0,
            },
            0.0,
        );
        fs::write(output_dir.join(format!("{}.html", case.label)), &html)?;
        Ok(Self {
            html,
            pdf,
            png,
            jpeg,
            reference_rgba: preview_surface.rgba,
            reference_width: preview_surface.width,
            reference_height: preview_surface.height,
        })
    }

    pub fn assert_quality(&self, case: &DirectVisualCase) -> Result<(), Box<dyn Error>> {
        let reference = SurfaceEquivalenceImage {
            width: self.reference_width,
            height: self.reference_height,
            rgba: &self.reference_rgba,
        };
        let quality = ExportQualityGate::evaluate(&ExportQualityArtifacts {
            html: &self.html,
            pdf: &self.pdf,
            png: &self.png,
            jpeg: &self.jpeg,
            source_markdown: &case.content,
            surface_equivalence: Some(SurfaceEquivalenceArtifacts {
                raster_reference: reference,
                pdf_reference: reference,
                pdf: &self.pdf,
                png: &self.png,
                jpeg: &self.jpeg,
            }),
        });
        assert!(
            quality.is_pass(),
            "{} quality failed: {quality:#?}",
            case.label
        );
        Ok(())
    }
}

fn direct_snapshot(
    case: &DirectVisualCase,
) -> Result<katana_document_viewer::DocumentSnapshot, Box<dyn Error>> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: case.content.clone(),
            document_id: Some(case.document_id.clone()),
        },
        &PreviewConfig::default(),
        720.0,
    )?;
    Ok(output.input.snapshot)
}

fn export_bytes(
    pipeline: &ForgePipeline<DiagramRenderingBackend<StaticDiagramEngine>>,
    graph: &katana_document_viewer::BuildGraph,
    theme: &KdvThemeSnapshot,
    format: ExportFormat,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let output = pipeline.export(&ExportRequest {
        graph: graph.clone(),
        format,
        theme: theme.clone(),
    })?;
    Ok(output.artifact.bytes.bytes)
}

struct StaticDiagramEngine;

impl DiagramRenderEngine for StaticDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let kind = match request.kind {
            DiagramKind::Mermaid => "mermaid",
            DiagramKind::DrawIo => "drawio",
            DiagramKind::PlantUml => return Err("PlantUML is external-backend-required".into()),
        };
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: kind.to_string(),
            svg: format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"520\" data-test=\"{}\"><rect width=\"800\" height=\"520\" fill=\"#f5f5f5\"/><rect x=\"120\" y=\"80\" width=\"560\" height=\"360\" fill=\"#225ea8\"/></svg>",
                request.node_id
            ),
        })
    }
}
