use super::HtmlExportPayloadFactory;
use crate::{BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, ForgePipeline};
use crate::{DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend};
use crate::{RenderedDiagram, SourceKind, SourceRevision, SourceUri};
use katana_markdown_model::DiagramKind;
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

pub(crate) struct HtmlContractTestSupport;

impl HtmlContractTestSupport {
    pub(crate) fn export_html(markdown: &str) -> Result<String, Box<dyn std::error::Error>> {
        Self::export_html_with_diagrams(markdown, Vec::new())
    }

    pub(crate) fn export_html_with_theme(
        markdown: &str,
        theme: crate::KdvThemeSnapshot,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let graph = Self::build_graph_with_theme(markdown, theme.clone())?;
        Self::export_html_with_graph(graph, theme)
    }

    pub(crate) fn export_html_with_graph(
        graph: crate::BuildGraph,
        theme: crate::KdvThemeSnapshot,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = HtmlExportPayloadFactory::create(&graph, &theme);
        Ok(String::from_utf8(bytes)?)
    }

    pub(crate) fn export_html_with_diagrams(
        markdown: &str,
        rendered_diagrams: Vec<RenderedDiagram>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut graph = Self::build_graph(markdown)?;
        if !rendered_diagrams.is_empty() {
            graph = graph.with_rendered_diagrams(rendered_diagrams);
        }
        let bytes =
            HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
        Ok(String::from_utf8(bytes)?)
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

    pub(crate) fn extract_export_style(html: &str) -> Option<&str> {
        let style_open = "<style data-kdv-export-style>";
        let start = html.find(style_open)?;
        let style_start = start + style_open.len();
        let style_end = html[style_start..].find("</style>")?;
        Some(&html[style_start..style_start + style_end])
    }

    fn build_graph(markdown: &str) -> Result<crate::BuildGraph, Box<dyn std::error::Error>> {
        Self::build_graph_with_theme(markdown, crate::KdvThemeSnapshot::katana_light())
    }

    fn build_graph_with_theme(
        markdown: &str,
        theme: crate::KdvThemeSnapshot,
    ) -> Result<crate::BuildGraph, Box<dyn std::error::Error>> {
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
            theme,
        };
        let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(StaticDiagramEngine));
        Ok(pipeline.build(&request)?)
    }
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
