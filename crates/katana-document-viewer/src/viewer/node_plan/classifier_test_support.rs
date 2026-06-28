use katana_markdown_model::{
    ByteRange, ImageNode, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    ListItemNode, RawSnippet, SourceSpan, TableCell, TextSpan,
};

pub(super) fn list_item(
    ordered_number: Option<usize>,
    task_marker: Option<&str>,
    body: Vec<KmmNode>,
) -> ListItemNode {
    ListItemNode {
        marker: "-".to_string(),
        ordered_number,
        task_marker: task_marker.map(str::to_string),
        body,
        children: Vec::new(),
        source: source("- item"),
    }
}

pub(super) fn image(alt: &str) -> ImageNode {
    ImageNode {
        alt: alt.to_string(),
        src: "image.png".to_string(),
        title: None,
    }
}

pub(super) fn text_node(text: &str) -> KmmNode {
    node(
        KmmNodeKind::Text(TextSpan {
            text: text.to_string(),
        }),
        text,
        Vec::new(),
    )
}

pub(super) fn cell(text: &str) -> TableCell {
    TableCell {
        text: text.to_string(),
        source: source(text),
    }
}

pub(super) fn node(kind: KmmNodeKind, raw: &str, children: Vec<KmmNode>) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("node-{raw}")),
        kind,
        source: source(raw),
        children,
    }
}

pub(super) fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
