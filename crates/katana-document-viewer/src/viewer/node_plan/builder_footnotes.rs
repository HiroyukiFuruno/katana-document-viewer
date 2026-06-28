use super::ViewerNodePlanner;
use katana_markdown_model::{KmmNode, KmmNodeKind};

impl ViewerNodePlanner {
    pub(super) fn document_nodes_with_footnotes_last(nodes: &[KmmNode]) -> Vec<(usize, &KmmNode)> {
        let mut ordered = Vec::with_capacity(nodes.len());
        ordered.extend(Self::non_footnote_nodes(nodes));
        ordered.extend(Self::footnote_nodes(nodes));
        ordered
    }

    fn non_footnote_nodes(nodes: &[KmmNode]) -> impl Iterator<Item = (usize, &KmmNode)> {
        nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| !Self::is_footnote_definition(node))
    }

    fn footnote_nodes(nodes: &[KmmNode]) -> impl Iterator<Item = (usize, &KmmNode)> {
        nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| Self::is_footnote_definition(node))
    }

    fn is_footnote_definition(node: &KmmNode) -> bool {
        matches!(node.kind, KmmNodeKind::FootnoteDefinition(_))
    }
}
