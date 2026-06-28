use katana_markdown_model::{KmmNode, KmmNodeKind};

#[derive(Clone, Copy)]
pub(super) struct ViewerNodeContext<'a> {
    previous: Option<&'a KmmNode>,
    next: Option<&'a KmmNode>,
}

impl<'a> ViewerNodeContext<'a> {
    pub(super) fn empty() -> Self {
        Self {
            previous: None,
            next: None,
        }
    }

    pub(super) fn top_level(nodes: &'a [KmmNode], index: usize) -> Self {
        Self {
            previous: index.checked_sub(1).and_then(|value| nodes.get(value)),
            next: nodes.get(index + 1),
        }
    }

    pub(super) fn is_blank_line_isolated(self, node: &KmmNode) -> bool {
        self.has_blank_line_before(node) && self.has_blank_line_after(node)
    }

    pub(super) fn is_adjacent_to_list(self, node: &KmmNode) -> bool {
        self.previous
            .is_some_and(|previous| Self::touches_list_before(previous, node))
            || self
                .next
                .is_some_and(|next| Self::touches_list_after(node, next))
    }

    fn has_blank_line_before(self, node: &KmmNode) -> bool {
        self.previous.is_none_or(|previous| {
            previous.source.line_column_range.end.line + 1
                < node.source.line_column_range.start.line
        })
    }

    fn has_blank_line_after(self, node: &KmmNode) -> bool {
        self.next.is_none_or(|next| {
            node.source.line_column_range.end.line + 1 < next.source.line_column_range.start.line
        })
    }

    fn touches_list_before(previous: &KmmNode, node: &KmmNode) -> bool {
        matches!(previous.kind, KmmNodeKind::List(_))
            && previous.source.line_column_range.end.line + 2
                >= node.source.line_column_range.start.line
    }

    fn touches_list_after(node: &KmmNode, next: &KmmNode) -> bool {
        matches!(next.kind, KmmNodeKind::List(_))
            && node.source.line_column_range.end.line + 2
                >= next.source.line_column_range.start.line
    }
}
