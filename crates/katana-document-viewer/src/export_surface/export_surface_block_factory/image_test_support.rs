use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentKind, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    ByteRange, ImageNode, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::{Path, PathBuf};

pub(super) fn graph(source_path: &Path) -> BuildGraph {
    BuildGraph::from_request(&build_request(source_path))
}

pub(super) fn paragraph_node(text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId("paragraph-with-image".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span(text),
        children: vec![image_node("screen", "missing.png", text)],
    }
}

pub(super) fn paragraph_with_image(alt: &str, src: &str) -> KmmNode {
    let source = format!("![{alt}]({src})");
    KmmNode {
        id: KmmNodeId("paragraph".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span(&source),
        children: vec![image_node(alt, src, &source)],
    }
}

pub(super) fn image_node(alt: &str, src: &str, raw: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("image-{alt}")),
        kind: KmmNodeKind::Image(ImageNode {
            alt: alt.to_string(),
            src: src.to_string(),
            title: None,
        }),
        source: source_span(raw),
        children: Vec::new(),
    }
}

pub(super) fn text_node(text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("text-{text}")),
        kind: KmmNodeKind::Text(TextSpan {
            text: text.to_string(),
        }),
        source: source_span(text),
        children: Vec::new(),
    }
}

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

fn build_request(source_path: &Path) -> BuildRequest {
    BuildRequest {
        snapshot: snapshot(source_path),
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    }
}

fn snapshot(source_path: &Path) -> DocumentSnapshot {
    DocumentSnapshot {
        id: DocumentId("image-id".to_string()),
        kind: DocumentKind::Markdown,
        source_uri: SourceUri(format!("file://{}", source_path.display())),
        revision: SourceRevision("r1".to_string()),
        source_path: source_path.to_path_buf(),
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
        path: PathBuf::from("doc.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "fingerprint".to_string(),
        },
        nodes: Vec::new(),
    }
}
