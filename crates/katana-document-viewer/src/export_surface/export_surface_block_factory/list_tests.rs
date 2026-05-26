use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentKind, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    ByteRange, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    ListItemNode, ListNode, RawSnippet, SourceSpan, TextSpan,
};

use super::*;

#[test]
fn skips_empty_list_items() {
    let graph = graph();
    let mut blocks = Vec::new();
    let list = ListNode {
        ordered: false,
        task_markers: Vec::new(),
        items: vec![ListItemNode {
            marker: "-".to_string(),
            ordered_number: None,
            task_marker: None,
            body: vec![node(KmmNodeKind::Text(TextSpan {
                text: "   ".to_string(),
            }))],
            children: Vec::new(),
            source: source_span("   "),
        }],
    };

    SurfaceBlockFactory::append_list(&mut blocks, &graph, &list, 0, 0, &theme());
    assert!(blocks.is_empty());
}

#[test]
fn appends_nested_child_blocks() {
    let graph = graph();
    let mut blocks = Vec::new();
    let list = ListNode {
        ordered: false,
        task_markers: Vec::new(),
        items: vec![ListItemNode {
            marker: "1.".to_string(),
            ordered_number: Some(1),
            task_marker: None,
            body: vec![node(KmmNodeKind::Text(TextSpan {
                text: "parent item".to_string(),
            }))],
            children: vec![node_with_children(
                KmmNodeKind::Paragraph,
                vec![node(KmmNodeKind::Text(TextSpan {
                    text: "child".to_string(),
                }))],
            )],
            source: source_span("parent item"),
        }],
    };

    SurfaceBlockFactory::append_list(&mut blocks, &graph, &list, 0, 0, &theme());
    assert_eq!(blocks.len(), 2);
    assert!(blocks[0].text_for_tests().contains("parent item"));
    assert!(blocks[1].text_for_tests().contains("child"));
}

#[test]
fn wraps_long_list_item_body_as_multiple_lines() {
    let graph = graph();
    let mut blocks = Vec::new();
    let list = ListNode {
        ordered: true,
        task_markers: Vec::new(),
        items: vec![ListItemNode {
            marker: "1.".to_string(),
            ordered_number: Some(1),
            task_marker: None,
            body: vec![node(KmmNodeKind::Text(TextSpan {
                text: "body ".repeat(220),
            }))],
            children: Vec::new(),
            source: source_span(&"body ".repeat(220)),
        }],
    };

    SurfaceBlockFactory::append_list(&mut blocks, &graph, &list, 0, 0, &theme());
    assert!(blocks.len() > 1);
    assert!(blocks[0].text_for_tests().contains("1."));
}

fn graph() -> BuildGraph {
    let document = KmmDocument {
        path: "list.md".into(),
        fingerprint: katana_markdown_model::TextFingerprint {
            algorithm: "alg".to_string(),
            value: "list".to_string(),
        },
        nodes: Vec::new(),
    };
    let snapshot = DocumentSnapshot {
        id: DocumentId("list".to_string()),
        kind: DocumentKind::Markdown,
        source_uri: SourceUri("file:///list.md".to_string()),
        revision: SourceRevision("r1".to_string()),
        source_path: "/tmp/list.md".into(),
        document,
        outline: DocumentOutline { items: Vec::new() },
        metadata: DocumentMetadataView {
            unresolved_count: 0,
            diagnostic_keys: Vec::new(),
        },
    };
    BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: theme(),
    })
}

fn theme() -> KdvThemeSnapshot {
    KdvThemeSnapshot::katana_light()
}

fn node(kind: KmmNodeKind) -> KmmNode {
    KmmNode {
        id: KmmNodeId("list-item".to_string()),
        kind,
        source: source_span("node"),
        children: Vec::new(),
    }
}

fn node_with_children(kind: KmmNodeKind, children: Vec<KmmNode>) -> KmmNode {
    KmmNode {
        id: KmmNodeId("list-child".to_string()),
        kind,
        source: source_span("node"),
        children,
    }
}

fn source_span(text: &str) -> SourceSpan {
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
