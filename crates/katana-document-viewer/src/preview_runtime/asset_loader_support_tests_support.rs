use katana_markdown_model::{
    ByteRange, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, ListItemNode,
    ListNode, RawSnippet, SourceSpan, TextSpan,
};

pub(crate) struct AssetLoaderSupportTestFixtures;

impl AssetLoaderSupportTestFixtures {
    pub(crate) fn list_parent_node() -> KmmNode {
        build_list_parent_node()
    }

    pub(crate) fn build_root_node() -> KmmNode {
        KmmNode {
            id: KmmNodeId("root-node".to_string()),
            kind: KmmNodeKind::Text(TextSpan {
                text: "root text".to_string(),
            }),
            source: source_span("root text"),
            children: Vec::new(),
        }
    }

    pub(crate) fn build_regular_child_node() -> KmmNode {
        KmmNode {
            id: KmmNodeId("regular-child".to_string()),
            kind: KmmNodeKind::Paragraph,
            source: source_span("child"),
            children: Vec::new(),
        }
    }

    pub(crate) fn build_regular_parent_node(child: KmmNode) -> KmmNode {
        KmmNode {
            id: KmmNodeId("regular-parent".to_string()),
            kind: KmmNodeKind::BlockQuote,
            source: source_span("> child"),
            children: vec![child],
        }
    }

    pub(crate) fn source_span(text: &str) -> SourceSpan {
        SourceSpan {
            byte_range: ByteRange {
                start: 0,
                end: text.len(),
            },
            line_column_range: LineColumnRange {
                start: LineColumn { line: 1, column: 1 },
                end: LineColumn {
                    line: 1,
                    column: text.len() + 1,
                },
            },
            raw: RawSnippet {
                text: text.to_string(),
            },
        }
    }
}

fn source_span(text: &str) -> SourceSpan {
    AssetLoaderSupportTestFixtures::source_span(text)
}

fn build_list_parent_node() -> KmmNode {
    KmmNode {
        id: KmmNodeId("list-parent".to_string()),
        kind: KmmNodeKind::List(ListNode {
            ordered: false,
            task_markers: Vec::new(),
            items: vec![build_list_item()],
        }),
        source: source_span("parent"),
        children: Vec::new(),
    }
}

fn build_list_item() -> ListItemNode {
    ListItemNode {
        marker: "-".to_string(),
        ordered_number: None,
        task_marker: None,
        body: vec![KmmNode {
            id: KmmNodeId("list-body".to_string()),
            kind: KmmNodeKind::Text(TextSpan {
                text: "item".to_string(),
            }),
            source: source_span("item"),
            children: Vec::new(),
        }],
        children: vec![build_list_child_node()],
        source: source_span("item"),
    }
}

fn build_list_child_node() -> KmmNode {
    KmmNode {
        id: KmmNodeId("list-child".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("child node"),
        children: Vec::new(),
    }
}
