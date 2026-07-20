use katana_markdown_model::{
    ByteRange, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, ListItemNode,
    RawSnippet, SourceSpan, TextSpan,
};

pub const EMPTY_ID: &str = "id-0";

pub(super) fn source_span(text: &str) -> SourceSpan {
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

pub(super) fn text_node(text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: text.to_string(),
        }),
        source: source_span(text),
        children: Vec::new(),
    }
}

pub(super) fn list_item(task_marker: &str, ordered: bool) -> ListItemNode {
    let mut item = ListItemNode {
        marker: if ordered {
            "1.".to_string()
        } else {
            "-".to_string()
        },
        ordered_number: Some(1),
        task_marker: Some(task_marker.to_string()),
        body: Vec::new(),
        children: Vec::new(),
        source: source_span(""),
    };
    if task_marker.is_empty() {
        item.task_marker = None;
    }
    item
}
