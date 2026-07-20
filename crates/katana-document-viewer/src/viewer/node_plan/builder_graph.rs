use super::{ParagraphLayout, ViewerNodePlanBuilder};
use crate::artifact::{Artifact, ArtifactFormat, ArtifactId};
use crate::forge::{BuildGraph, BuildProfile, BuildRequest, RenderedDiagram};
use crate::viewer::types::ViewerInput;
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};

#[cfg(test)]
#[path = "builder_graph_test_support.rs"]
mod builder_graph_test_support;

#[cfg(test)]
#[path = "builder_graph_tests.rs"]
mod tests;

impl ViewerNodePlanBuilder<'_> {
    pub(super) fn build_graph(
        input: &ViewerInput,
        paragraph_layout: ParagraphLayout,
    ) -> Option<BuildGraph> {
        if paragraph_layout != ParagraphLayout::PreserveSourceRows {
            return None;
        }
        let request = BuildRequest {
            snapshot: input.snapshot.clone(),
            profile: BuildProfile::markdown_export(),
            theme: input.theme.clone(),
        };
        Some(
            BuildGraph::from_request(&request)
                .with_rendered_diagrams(Self::rendered_diagrams(input)),
        )
    }

    fn rendered_diagrams(input: &ViewerInput) -> Vec<RenderedDiagram> {
        input
            .artifacts
            .iter()
            .filter_map(|artifact| Self::rendered_diagram(input, artifact))
            .collect()
    }

    fn rendered_diagram(input: &ViewerInput, artifact: &Artifact) -> Option<RenderedDiagram> {
        if artifact.manifest.format != ArtifactFormat::Svg
            || !artifact.manifest.diagnostics.entries.is_empty()
        {
            return None;
        }
        let node_id = Self::node_id_from_artifact_id(input, &artifact.manifest.id)?;
        let kind = Self::diagram_kind_for_node_id(&input.snapshot.document.nodes, node_id)?;
        let svg = std::str::from_utf8(&artifact.bytes.bytes).ok()?.trim();
        if !svg.starts_with("<svg") {
            return None;
        }
        Some(RenderedDiagram {
            node_id: node_id.to_string(),
            kind: kind.to_string(),
            svg: svg.to_string(),
        })
    }

    fn node_id_from_artifact_id<'id>(
        input: &ViewerInput,
        artifact_id: &'id ArtifactId,
    ) -> Option<&'id str> {
        artifact_id
            .0
            .strip_prefix(&format!("{}:", input.snapshot.id.0))?
            .strip_suffix(":Svg")
    }

    fn diagram_kind_for_node_id(nodes: &[KmmNode], node_id: &str) -> Option<&'static str> {
        for node in nodes {
            if node.id.0 == node_id {
                return Self::diagram_kind(node);
            }
            if let Some(kind) = Self::diagram_kind_for_node_id(&node.children, node_id) {
                return Some(kind);
            }
        }
        None
    }

    fn diagram_kind(node: &KmmNode) -> Option<&'static str> {
        let KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { kind }) = &node.kind else {
            return None;
        };
        Some(match kind {
            DiagramKind::Mermaid => "mermaid",
            DiagramKind::DrawIo => "drawio",
            DiagramKind::PlantUml => "plantuml",
        })
    }
}
