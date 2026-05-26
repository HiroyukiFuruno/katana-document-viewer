use super::*;
use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use image::Rgba;
use katana_markdown_model::{
    ByteRange, HtmlBlockRole, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint,
};
use std::path::PathBuf;

const EMPTY_ID: &str = "html-image-node";

#[test]
fn local_html_image_node_is_loaded_into_image_block() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = graph();
    let temp_dir = std::env::temp_dir();
    let image_file_name = "kdv-html-local-image.png";
    image::RgbaImage::from_pixel(8, 10, Rgba([1, 2, 3, 255]))
        .save(temp_dir.join(image_file_name))?;

    graph.snapshot.source_uri = SourceUri(format!(
        "file://{}",
        temp_dir.join("document.html").display()
    ));
    graph.snapshot.source_path = temp_dir.join("document.html");

    let mut blocks = Vec::new();
    let node = node(&image_html(image_file_name));
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "fixture");
    Ok(())
}

fn image_html(image_file_name: &str) -> String {
    format!("<p><img src=\"{image_file_name}\" alt=\"fixture\" /></p>")
}

fn graph() -> BuildGraph {
    let document = KmmDocument {
        path: PathBuf::from("/tmp/html.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "value".to_string(),
        },
        nodes: Vec::new(),
    };
    let snapshot = DocumentSnapshot {
        id: DocumentId("html".to_string()),
        kind: crate::DocumentKind::Markdown,
        source_uri: SourceUri("file:///html.md".to_string()),
        revision: SourceRevision("r1".to_string()),
        source_path: PathBuf::from("/tmp/html.md"),
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

fn node(source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
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
