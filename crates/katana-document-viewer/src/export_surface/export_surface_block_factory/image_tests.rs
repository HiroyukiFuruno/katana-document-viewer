use super::*;
use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentKind, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use image::Rgba;
use katana_markdown_model::{
    ByteRange, ImageNode, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::{Path, PathBuf};

#[test]
fn standalone_image_paragraph_becomes_surface_image_block() -> Result<(), Box<dyn std::error::Error>>
{
    let temp_dir = std::env::temp_dir().join("kdv-surface-image-block");
    std::fs::create_dir_all(&temp_dir)?;
    let image_path = temp_dir.join("screen.png");
    image::RgbaImage::from_pixel(24, 10, Rgba([4, 5, 6, 255])).save(&image_path)?;
    let graph = graph(&temp_dir.join("doc.md"));
    let mut blocks = Vec::new();

    SurfaceBlockFactory::append_node_with_parts(
        &mut blocks,
        &graph,
        &paragraph_with_image("screen", "screen.png"),
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(1, blocks.len());
    assert!(blocks[0].debug_for_tests().starts_with("image:24x10"));
    Ok(())
}

#[test]
fn mixed_text_image_paragraph_stays_rich_text_line() {
    let graph = graph(Path::new("/tmp/doc.md"));
    let mut blocks = Vec::new();
    let node = KmmNode {
        id: KmmNodeId("mixed".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("before ![screen](screen.png)"),
        children: vec![
            text_node("before "),
            image_node("screen", "screen.png", "![screen](screen.png)"),
        ],
    };

    SurfaceBlockFactory::append_node_with_parts(
        &mut blocks,
        &graph,
        &node,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(1, blocks.len());
    assert!(!blocks[0].debug_for_tests().starts_with("image:"));
}

fn graph(source_path: &Path) -> BuildGraph {
    let document = KmmDocument {
        path: PathBuf::from("doc.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "fingerprint".to_string(),
        },
        nodes: Vec::new(),
    };
    let snapshot = DocumentSnapshot {
        id: DocumentId("image-id".to_string()),
        kind: DocumentKind::Markdown,
        source_uri: SourceUri(format!("file://{}", source_path.display())),
        revision: SourceRevision("r1".to_string()),
        source_path: source_path.to_path_buf(),
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

fn paragraph_with_image(alt: &str, src: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId("paragraph".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span(&format!("![{alt}]({src})")),
        children: vec![image_node(alt, src, &format!("![{alt}]({src})"))],
    }
}

fn image_node(alt: &str, src: &str, raw: &str) -> KmmNode {
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

fn text_node(text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("text-{text}")),
        kind: KmmNodeKind::Text(TextSpan {
            text: text.to_string(),
        }),
        source: source_span(text),
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
