use crate::{
    DiagramRenderEngine, DiagramRenderRequest, MarkdownSource, PreviewConfig, PreviewError,
    PreviewOutput, PreviewOutputFactory, RenderedDiagram,
};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) struct FakeDiagramEngine;

impl DiagramRenderEngine for FakeDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="10"><rect width="20" height="10"/></svg>"#.to_string(),
        })
    }
}

pub(super) struct ErrorDiagramEngine;

impl DiagramRenderEngine for ErrorDiagramEngine {
    fn render(&self, _: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        Err("render failed".to_string())
    }
}

pub(super) fn output_for(content: &str) -> Result<PreviewOutput, PreviewError> {
    output_for_document(content, "diagram.md")
}

pub(super) fn output_for_document(
    content: &str,
    document_id: &str,
) -> Result<PreviewOutput, PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some(document_id.to_string()),
        },
        &PreviewConfig {
            viewport: crate::ViewerViewport {
                width: 640.0,
                height: 480.0,
            },
            ..PreviewConfig::default()
        },
        320.0,
    )
}

pub(super) fn temp_image_path(file_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("kdv-asset-loader-{nanos}-{file_name}")))
}
