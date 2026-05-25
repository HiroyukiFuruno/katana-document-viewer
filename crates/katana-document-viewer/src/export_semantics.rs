use katana_markdown_model::{KatanaMarkdownModel, KmmNode, KmmNodeKind, MarkdownInput};

pub(crate) struct EvaluatedMarkdownFragment {
    nodes: Vec<KmmNode>,
    source: String,
}

impl EvaluatedMarkdownFragment {
    pub(crate) fn evaluate(name: &str, source: &str) -> Self {
        let parsed =
            KatanaMarkdownModel::parse(MarkdownInput::from_content(name, source.to_string()));
        let nodes = match parsed {
            Ok(document) => document.nodes,
            Err(_) => Vec::new(),
        };
        Self {
            nodes,
            source: source.to_string(),
        }
    }

    pub(crate) fn nodes(&self) -> &[KmmNode] {
        &self.nodes
    }

    pub(crate) fn has_nodes(&self) -> bool {
        !self.nodes.is_empty()
    }

    pub(crate) fn contains_inline_markdown(&self) -> bool {
        self.source.contains('`')
            || self.source.contains("**")
            || self.source.contains("__")
            || self.source.contains("~~")
            || self.source.contains('*')
            || self.source.contains('_')
    }

    pub(crate) fn contains_structured_inline(&self) -> bool {
        self.nodes.iter().any(node_contains_structured_inline)
    }
}

fn node_contains_structured_inline(node: &KmmNode) -> bool {
    match &node.kind {
        KmmNodeKind::Strong(_)
        | KmmNodeKind::Emphasis(_)
        | KmmNodeKind::Strikethrough(_)
        | KmmNodeKind::InlineCode(_)
        | KmmNodeKind::InlineHtml(_)
        | KmmNodeKind::Link(_)
        | KmmNodeKind::Image(_)
        | KmmNodeKind::FootnoteReference(_)
        | KmmNodeKind::InlineMath(_)
        | KmmNodeKind::Emoji(_) => true,
        _ => node.children.iter().any(node_contains_structured_inline),
    }
}
