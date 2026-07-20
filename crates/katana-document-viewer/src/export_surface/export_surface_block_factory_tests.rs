use super::*;
use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentKind, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    ByteRange, HeadingNode, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint,
};
use std::path::PathBuf;

#[test]
fn create_with_typography_skips_nodes_that_generate_no_blocks() {
    let graph = graph();
    let blocks = SurfaceBlockFactory::create_with_typography(
        &graph,
        &KdvThemeSnapshot::katana_light(),
        crate::export_surface_line::SurfaceTypographyConfig::default(),
    );

    assert_eq!(1, blocks.len());
    assert!(blocks[0].text_for_tests().contains("Visible heading"));
}

fn graph() -> BuildGraph {
    let document = KmmDocument {
        path: PathBuf::from("block-factory.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "fingerprint".to_string(),
        },
        nodes: vec![empty_paragraph_node(), heading_node(1, "Visible heading")],
    };
    let snapshot = DocumentSnapshot {
        id: DocumentId("block-factory-id".to_string()),
        kind: DocumentKind::Markdown,
        source_uri: SourceUri("file:///block-factory.md".to_string()),
        revision: SourceRevision("rev".to_string()),
        source_path: PathBuf::from("/tmp/block-factory.md"),
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
        theme: KdvThemeSnapshot::katana_light(),
    })
}

fn empty_paragraph_node() -> KmmNode {
    node(KmmNodeKind::Paragraph, Vec::new(), "")
}

fn heading_node(level: u8, title: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("heading-{level}")),
        kind: KmmNodeKind::Heading(HeadingNode {
            level,
            text: title.to_string(),
        }),
        source: source_span(title),
        children: Vec::new(),
    }
}

fn node(kind: KmmNodeKind, children: Vec<KmmNode>, source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId("factory-node".to_string()),
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
