use super::*;
use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    ByteRange, HtmlBlockRole, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint,
};
use std::path::PathBuf;

#[test]
fn generic_html_link_preserves_surface_link_span() -> Result<(), String> {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node(r#"<div><a href="https://example.com">Go</a></div>"#);
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    let SurfaceBlock::Line(line) = &blocks[0] else {
        return Err("generic html link must be emitted as surface line".to_string());
    };
    assert_eq!("Go", line.text);
    assert_eq!(
        Some("https://example.com"),
        line.spans[0].link_target.as_deref()
    );
    Ok(())
}

fn graph() -> BuildGraph {
    let request = BuildRequest {
        snapshot: snapshot(),
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    };
    BuildGraph::from_request(&request)
}

fn snapshot() -> DocumentSnapshot {
    DocumentSnapshot {
        id: DocumentId("html".to_string()),
        kind: crate::DocumentKind::Markdown,
        source_uri: SourceUri("file:///html.md".to_string()),
        revision: SourceRevision("r1".to_string()),
        source_path: PathBuf::from("/tmp/html.md"),
        document: document(),
        outline: DocumentOutline { items: Vec::new() },
        metadata: DocumentMetadataView {
            unresolved_count: 0,
            diagnostic_keys: Vec::new(),
        },
    }
}

fn document() -> KmmDocument {
    KmmDocument {
        path: PathBuf::from("/tmp/html.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "value".to_string(),
        },
        nodes: Vec::new(),
    }
}

fn node(source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId("html-link-node".to_string()),
        kind: KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        source: source_span(source_text),
        children: Vec::new(),
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
