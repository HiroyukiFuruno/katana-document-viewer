use crate::export_surface_text::SurfaceTextParser;
use crate::{
    Artifact, ArtifactFormat, ViewerArtifactTextExtraction, ViewerNode, ViewerRenderedAnchor,
    ViewerSearchLayoutResolver, ViewerSearchTarget, ViewerTarget,
};

pub struct ViewerArtifactSearchResolver;

impl ViewerArtifactSearchResolver {
    pub fn resolve_targets(
        query: &str,
        nodes: &[ViewerNode],
        artifacts: &[Artifact],
    ) -> Vec<ViewerSearchTarget> {
        if query.is_empty() {
            return Vec::new();
        }
        let extractions = Self::extractions(nodes, artifacts);
        let matches = ViewerSearchLayoutResolver::matches_from_artifact_text(query, &extractions);
        ViewerSearchLayoutResolver::resolve_matches(&matches, &Self::anchors(nodes))
    }

    fn extractions(
        nodes: &[ViewerNode],
        artifacts: &[Artifact],
    ) -> Vec<ViewerArtifactTextExtraction> {
        nodes
            .iter()
            .filter_map(|node| Self::extraction_for_node(node, artifacts))
            .collect()
    }

    fn extraction_for_node(
        node: &ViewerNode,
        artifacts: &[Artifact],
    ) -> Option<ViewerArtifactTextExtraction> {
        let artifact_id = node.artifact_id.clone()?;
        let artifact = artifacts
            .iter()
            .find(|candidate| candidate.manifest.id == artifact_id)?;
        let text = Self::artifact_text(artifact)?;
        Some(ViewerArtifactTextExtraction {
            artifact_id,
            node_id: node.node_id.clone(),
            source: node.source.clone(),
            text,
        })
    }

    pub fn artifact_text(artifact: &Artifact) -> Option<String> {
        if !artifact.manifest.diagnostics.entries.is_empty() {
            return None;
        }
        if let Some(extraction) = &artifact.text_extraction {
            return Self::non_empty_text(extraction.text.clone());
        }
        if !matches!(
            artifact.manifest.format,
            ArtifactFormat::Html | ArtifactFormat::Svg
        ) {
            return None;
        }
        let raw = std::str::from_utf8(&artifact.bytes.bytes).ok()?;
        let text = SurfaceTextParser::html_fragment_text(raw);
        Self::non_empty_text(text)
    }

    fn non_empty_text(text: String) -> Option<String> {
        if text.is_empty() {
            return None;
        }
        Some(text)
    }

    fn anchors(nodes: &[ViewerNode]) -> Vec<ViewerRenderedAnchor> {
        nodes
            .iter()
            .enumerate()
            .filter_map(|(index, node)| Self::anchor_for_node(index, node))
            .collect()
    }

    fn anchor_for_node(index: usize, node: &ViewerNode) -> Option<ViewerRenderedAnchor> {
        Some(ViewerRenderedAnchor {
            target: ViewerTarget {
                node_id: node.node_id.clone(),
                source: node.source.clone(),
                artifact_id: node.artifact_id.clone()?,
                rect: node.rect,
            },
            anchor_index: index,
        })
    }
}

#[cfg(test)]
#[path = "artifact_search_tests.rs"]
mod tests;
