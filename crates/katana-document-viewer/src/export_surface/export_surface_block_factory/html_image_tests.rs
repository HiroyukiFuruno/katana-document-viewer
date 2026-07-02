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

#[test]
fn centered_data_svg_image_node_is_loaded_into_image_block() {
    let graph = graph();
    let mut blocks = Vec::new();
    let data_uri = "data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%2216%22%20height%3D%2210%22%3E%3Crect%20width%3D%2216%22%20height%3D%2210%22%20fill%3D%22%23000%22%2F%3E%3C%2Fsvg%3E";
    let source = format!("<p align=\"center\"><img src=\"{data_uri}\" alt=\"fixture\"></p>");
    let node = node(&source);

    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Centered,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].debug_for_tests(), "image:16x10@32x20:fixture");
}

#[test]
fn broken_katana_fixture_svg_data_uri_matches_export_surface_image_reference() {
    let graph = graph();
    let mut blocks = Vec::new();
    let data_uri = "data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22> width=%2216%22 height=%2210%22%3E%3Crect width=%2216%22 height=%2210%22 fill=%22%23000%22/%3E%3C/svg%3E";
    let source = format!("<p align=\"center\"><img src=\"{data_uri}\" alt=\"fixture\"></p>");
    let node = node(&source);

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
    assert_eq!(blocks[0].debug_for_tests(), "image:16x10@32x20:fixture");
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
