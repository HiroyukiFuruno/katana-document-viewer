use super::HtmlExportPayloadFactory;
use crate::{BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, ForgePipeline};
use crate::{DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend};
use crate::{RenderedDiagram, SourceKind, SourceRevision, SourceUri};
use katana_markdown_model::DiagramKind;
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

pub(crate) fn export_html(markdown: &str) -> Result<String, Box<dyn std::error::Error>> {
    export_html_with_diagrams(markdown, Vec::new())
}

pub(crate) fn export_html_with_graph(
    graph: crate::BuildGraph,
    theme: crate::KdvThemeSnapshot,
) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = HtmlExportPayloadFactory::create(&graph, &theme);
    Ok(String::from_utf8(bytes)?)
}

#[allow(dead_code)]
pub(crate) fn export_html_with_diagrams(
    markdown: &str,
    rendered_diagrams: Vec<RenderedDiagram>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut graph = build_graph(markdown)?;
    if !rendered_diagrams.is_empty() {
        graph = graph.with_rendered_diagrams(rendered_diagrams);
    }
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    Ok(String::from_utf8(bytes)?)
}

fn build_graph(markdown: &str) -> Result<crate::BuildGraph, Box<dyn std::error::Error>> {
    let source = DocumentSource {
        uri: SourceUri("file:///red-contract.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("red-contract".to_string()),
        content: markdown.to_string(),
    };
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "red-contract.md",
        markdown.to_string(),
    ))?;
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
    let request = BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    };
    let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(StaticDiagramEngine));
    Ok(pipeline.build(&request)?)
}

pub(crate) fn assert_contains_all(html: &str, expectations: &[(&str, &str)]) {
    let missing = expectations
        .iter()
        .filter(|(_, needle)| !html.contains(needle))
        .map(|(label, needle)| format!("{label}: {needle}"))
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "HTML export missing expected fragments:\n{}",
        missing.join("\n")
    );
}

struct StaticDiagramEngine;

impl DiagramRenderEngine for StaticDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let kind = match request.kind {
            DiagramKind::Mermaid => "mermaid",
            DiagramKind::DrawIo => "drawio",
            DiagramKind::PlantUml => "plantuml",
        };
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: kind.to_string(),
            svg: format!("<svg data-test=\"{}\"></svg>", request.node_id),
        })
    }
}

pub(crate) fn assert_not_contains_any(html: &str, expectations: &[(&str, &str)]) {
    let unexpected = expectations
        .iter()
        .filter(|(_, needle)| html.contains(needle))
        .map(|(label, needle)| format!("{label}: {needle}"))
        .collect::<Vec<_>>();
    assert!(
        unexpected.is_empty(),
        "HTML export still contains forbidden fragments:\n{}",
        unexpected.join("\n")
    );
}
