use super::*;
use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    ByteRange, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::PathBuf;

const EMPTY_ID: &str = "blockquote-node";

#[test]
fn legacy_children_block_quote_is_wrapped_as_note() {
    let graph = graph();
    let mut blocks = Vec::new();
    let children_quote = node(
        KmmNodeKind::BlockQuote,
        vec![
            node(
                KmmNodeKind::Text(TextSpan {
                    text: "Note".to_string(),
                }),
                Vec::new(),
                "Note",
            ),
            node(
                KmmNodeKind::Text(TextSpan {
                    text: "body".to_string(),
                }),
                Vec::new(),
                "body",
            ),
        ],
        "> body",
    );
    SurfaceBlockFactory::append_block_quote(&mut blocks, &graph, &children_quote, 0, 0, &theme());

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "Note body");
}

#[test]
fn nested_block_quote_is_split_by_depth() {
    let graph = graph();
    let mut blocks = Vec::new();
    let nested = node(KmmNodeKind::BlockQuote, Vec::new(), "> one\n> > nested");
    SurfaceBlockFactory::append_block_quote(&mut blocks, &graph, &nested, 0, 0, &theme());

    assert_eq!(blocks.len(), 2);
}

#[test]
fn legacy_note_raw_quote_is_wrapped_as_note() {
    let graph = graph();
    let mut blocks = Vec::new();
    let raw_quote = node(
        KmmNodeKind::BlockQuote,
        Vec::new(),
        "> **Note**\n> body line\n",
    );

    SurfaceBlockFactory::append_block_quote(&mut blocks, &graph, &raw_quote, 0, 0, &theme());

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "Note body line");
}

fn graph() -> BuildGraph {
    let document = KmmDocument {
        path: PathBuf::from("/tmp/block-quote.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "value".to_string(),
        },
        nodes: Vec::new(),
    };
    let snapshot = DocumentSnapshot {
        id: DocumentId("block-quote".to_string()),
        kind: crate::DocumentKind::Markdown,
        source_uri: SourceUri("file:///block-quote.md".to_string()),
        revision: SourceRevision("r1".to_string()),
        source_path: PathBuf::from("/tmp/block-quote.md"),
        document,
        outline: DocumentOutline { items: Vec::new() },
        metadata: DocumentMetadataView {
            unresolved_count: 0,
            diagnostic_keys: Vec::new(),
        },
    };
    let request = BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: theme(),
    };
    BuildGraph::from_request(&request)
}

fn node(kind: KmmNodeKind, children: Vec<KmmNode>, source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind,
        source: source_span(source_text),
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

fn theme() -> KdvThemeSnapshot {
    KdvThemeSnapshot::katana_light()
}
