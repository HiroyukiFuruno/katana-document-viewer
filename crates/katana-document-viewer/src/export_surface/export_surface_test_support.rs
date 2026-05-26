use super::super::{SurfaceBlock, SurfaceBlockFactory};
use crate::export_surface::page_plan::SurfacePagePlan;
use crate::forge::BuildGraph;
use crate::{
    BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot,
    RenderedDiagram, SourceKind, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    CodeBlockRole, KatanaMarkdownModel, KmmNode, KmmNodeKind, MarkdownInput,
};

pub(crate) struct SurfaceTestSupport;

impl SurfaceTestSupport {
    pub(crate) fn graph_with_rendered_diagram(
        markdown: String,
    ) -> Result<BuildGraph, Box<dyn std::error::Error>> {
        Self::graph_with_rendered_diagram_svg(
            markdown,
            "<svg><text>Rendered diagram</text></svg>".to_string(),
        )
    }

    pub(crate) fn graph_with_rendered_diagram_svg(
        markdown: String,
        svg: String,
    ) -> Result<BuildGraph, Box<dyn std::error::Error>> {
        let graph = Self::graph_from_markdown("surface.md", markdown)?;
        let node_id = Self::diagram_node_id(&graph.snapshot.document.nodes)?;
        Ok(graph.with_rendered_diagrams(vec![RenderedDiagram {
            node_id,
            kind: "mermaid".to_string(),
            svg,
        }]))
    }

    pub(crate) fn graph_from_markdown(
        path: &str,
        markdown: String,
    ) -> Result<BuildGraph, Box<dyn std::error::Error>> {
        let markdown = markdown.replace("\r\n", "\n").replace('\r', "\n");
        let source = DocumentSource {
            uri: SourceUri(format!("file://{path}")),
            kind: SourceKind::Markdown,
            revision: SourceRevision("rev-1".to_string()),
            content: markdown,
        };
        let document =
            KatanaMarkdownModel::parse(MarkdownInput::from_content(path, source.content.clone()))?;
        let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
        Ok(BuildGraph::from_request(&BuildRequest {
            snapshot,
            profile: BuildProfile::markdown_export(),
            theme: KdvThemeSnapshot::katana_light(),
        }))
    }

    pub(crate) fn surface_text(graph: &BuildGraph) -> String {
        SurfaceBlockFactory::create(graph, &graph.theme)
            .iter()
            .map(SurfaceBlock::text_for_tests)
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(crate) fn surface_debug(graph: &BuildGraph) -> String {
        SurfaceBlockFactory::create(graph, &graph.theme)
            .iter()
            .map(SurfaceBlock::debug_for_tests)
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(crate) fn surface_page_texts(graph: &BuildGraph) -> Vec<String> {
        let blocks = SurfaceBlockFactory::create(graph, &graph.theme);
        SurfacePagePlan::from_blocks(&blocks)
            .pages
            .iter()
            .map(|page| {
                page.iter()
                    .map(|index| blocks[*index].text_for_tests())
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect()
    }

    pub(crate) fn assert_contains_all(text: &str, needles: &[&str]) {
        for needle in needles {
            assert!(text.contains(needle), "missing {needle:?} in {text}");
        }
    }

    pub(crate) fn assert_not_contains_any(text: &str, needles: &[&str]) {
        for needle in needles {
            assert!(!text.contains(needle), "unexpected {needle:?} in {text}");
        }
    }

    fn diagram_node_id(nodes: &[KmmNode]) -> Result<String, Box<dyn std::error::Error>> {
        for node in nodes {
            if matches!(
                node.kind,
                KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { .. })
            ) {
                return Ok(node.id.0.clone());
            }
        }
        Err("diagram node is required".into())
    }
}

#[cfg(test)]
mod tests {
    use super::SurfaceTestSupport;

    #[test]
    fn diagram_node_id_reports_missing_diagram_node() {
        assert!(SurfaceTestSupport::diagram_node_id(&[]).is_err());
    }
}
