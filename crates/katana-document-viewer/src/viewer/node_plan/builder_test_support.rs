use super::super::ViewerNodePlan;
use crate::{
    DocumentSnapshot, DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, SourceKind,
    SourceRevision, SourceUri, ViewerInput, ViewerInteractionConfig, ViewerMode, ViewerSearchState,
    ViewerViewport,
};
use katana_markdown_model::{
    ByteRange, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    ListItemNode, ListNode, RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::PathBuf;

const VIEWPORT_WIDTH: f32 = 640.0;
const VIEWPORT_HEIGHT: f32 = 320.0;
const LONG_LIST_ITEM_COUNT: usize = 8;

pub(super) fn input_with_nodes(nodes: Vec<KmmNode>) -> ViewerInput {
    ViewerInput {
        snapshot: snapshot_with_nodes(nodes),
        artifacts: Vec::new(),
        theme: KdvThemeSnapshot::default(),
        mode: ViewerMode::Document,
        interaction: ViewerInteractionConfig::default(),
        typography: crate::ViewerTypographyConfig {
            preview_font_size: 24,
        },
        viewport: viewport(),
        search: ViewerSearchState::default(),
    }
}

pub(super) fn node(kind: KmmNodeKind, raw: &str, children: Vec<KmmNode>) -> KmmNode {
    node_at_line(kind, raw, children, 1)
}

pub(super) fn node_at_line(
    kind: KmmNodeKind,
    raw: &str,
    children: Vec<KmmNode>,
    line: usize,
) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("node-{raw}")),
        kind,
        source: source_at_line(raw, line),
        children,
    }
}

pub(super) fn text_node_at_line(raw: &str, line: usize) -> KmmNode {
    node_at_line(text_kind(raw), raw, Vec::new(), line)
}

pub(super) fn text_node(text: &str) -> KmmNode {
    node(text_kind(text), text, Vec::new())
}

pub(super) fn source(raw: &str) -> SourceSpan {
    source_at_line(raw, 1)
}

pub(super) fn long_list() -> ListNode {
    let mut items = Vec::new();
    for index in 0..LONG_LIST_ITEM_COUNT {
        items.push(ListItemNode {
            marker: "-".to_string(),
            ordered_number: None,
            task_marker: None,
            body: vec![text_node(&format!("item {index}"))],
            children: Vec::new(),
            source: source("item"),
        });
    }
    ListNode {
        ordered: false,
        task_markers: Vec::new(),
        items,
    }
}

pub(super) fn assert_node_texts(plan: &ViewerNodePlan, expected: &[&str]) {
    let actual = plan
        .nodes
        .iter()
        .map(|node| node.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(expected, actual.as_slice());
}

pub(super) fn assert_node_span_texts(plan: &ViewerNodePlan, expected: &[&str]) {
    let actual = plan
        .nodes
        .iter()
        .map(|node| node.spans.iter().map(|span| span.text.as_str()).collect())
        .collect::<Vec<String>>();
    assert_eq!(expected, actual.as_slice());
}

fn snapshot_with_nodes(nodes: Vec<KmmNode>) -> DocumentSnapshot {
    DocumentSnapshotFactory::from_kmm(source_document(), document_with_nodes(nodes))
}

fn document_with_nodes(nodes: Vec<KmmNode>) -> KmmDocument {
    KmmDocument {
        path: PathBuf::from("builder.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "builder-revision".to_string(),
        },
        nodes,
    }
}

fn source_document() -> DocumentSource {
    DocumentSource {
        uri: SourceUri("preview://builder.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("builder-revision".to_string()),
        content: String::new(),
    }
}

fn viewport() -> ViewerViewport {
    ViewerViewport {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
    }
}

fn text_kind(text: &str) -> KmmNodeKind {
    KmmNodeKind::Text(TextSpan {
        text: text.to_string(),
    })
}

fn source_at_line(raw: &str, line: usize) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line, column: 1 },
            end: LineColumn {
                line,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
