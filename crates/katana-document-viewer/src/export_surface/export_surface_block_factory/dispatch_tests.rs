use super::*;
use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentKind, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    ByteRange, FootnoteDefinitionNode, HeadingNode, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind,
    LineColumn, LineColumnRange, RawSnippet, SourceSpan, TextFingerprint,
};
use std::path::PathBuf;

const EMPTY_ID: &str = "dispatch-node";

#[test]
fn dispatches_primary_heading_to_line_block() {
    let graph = graph();
    let mut blocks = Vec::new();
    let heading = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 1,
            text: "Title".to_string(),
        }),
        Vec::new(),
        "Title",
    );
    SurfaceBlockFactory::append_node_with_parts(
        &mut blocks,
        &graph,
        &heading,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].text_for_tests().contains("Title"));
}

#[test]
fn dispatches_footnote_definition_to_fallback_node() {
    let graph = graph();
    let mut blocks = Vec::new();
    let footnote = node(
        KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
            label: "1".to_string(),
            text: "note".to_string(),
        }),
        Vec::new(),
        "1: note",
    );
    SurfaceBlockFactory::append_node_with_parts(
        &mut blocks,
        &graph,
        &footnote,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].text_for_tests().contains("1."));
}

#[test]
fn dispatches_raw_block_to_wrapped_fallback() {
    let graph = graph();
    let mut blocks = Vec::new();
    let raw = node(
        KmmNodeKind::RawBlock {
            reason: "raw".to_string(),
        },
        Vec::new(),
        "raw line",
    );
    SurfaceBlockFactory::append_node_with_parts(
        &mut blocks,
        &graph,
        &raw,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "raw line");
}

#[test]
fn dispatches_description_list_to_wrapped_unknown_path() {
    let graph = graph();
    let mut blocks = Vec::new();
    let description_list = node(
        KmmNodeKind::DescriptionList { items: Vec::new() },
        Vec::new(),
        "description item",
    );
    SurfaceBlockFactory::append_node_with_parts(
        &mut blocks,
        &graph,
        &description_list,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].text_for_tests().contains("description item"));
}

fn graph() -> BuildGraph {
    let document = KmmDocument {
        path: PathBuf::from("dispatch.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "fingerprint".to_string(),
        },
        nodes: Vec::new(),
    };
    let snapshot = DocumentSnapshot {
        id: DocumentId("dispatch-id".to_string()),
        kind: DocumentKind::Markdown,
        source_uri: SourceUri("file:///dispatch.md".to_string()),
        revision: SourceRevision("r1".to_string()),
        source_path: PathBuf::from("/tmp/dispatch.md"),
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
        theme: KdvThemeSnapshot::katana_light(),
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
