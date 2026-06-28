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

const EMPTY_ID: &str = "html-node";

pub(super) fn graph() -> BuildGraph {
    let request = BuildRequest {
        snapshot: snapshot(),
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    };
    BuildGraph::from_request(&request)
}

pub(super) fn node(source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        source: source_span(source_text),
        children: Vec::new(),
    }
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
