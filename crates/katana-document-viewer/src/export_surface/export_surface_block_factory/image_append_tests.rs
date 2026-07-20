use crate::export_surface::SurfaceBlockFactory;
use crate::theme::KdvThemeSnapshot;
use image::Rgba;
use katana_markdown_model::{KmmNode, KmmNodeId, KmmNodeKind};
use std::path::Path;

use super::support::{
    graph, image_node, paragraph_node, paragraph_with_image, source_span, text_node,
};

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

#[test]
fn standalone_image_rejects_non_single_child_nodes() {
    let graph = graph(Path::new("/tmp/doc.md"));
    let mut blocks = Vec::new();
    let mut node = paragraph_node("[![missing]](missing)");
    node.children.clear();

    assert!(!SurfaceBlockFactory::append_standalone_image(
        &mut blocks,
        &graph,
        &node
    ));
    assert!(blocks.is_empty());
}

#[test]
fn standalone_image_rejects_non_image_child() {
    let graph = graph(Path::new("/tmp/doc.md"));
    let mut blocks = Vec::new();
    let node = KmmNode {
        id: KmmNodeId("non-image".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("text"),
        children: vec![text_node("text")],
    };

    assert!(!SurfaceBlockFactory::append_standalone_image(
        &mut blocks,
        &graph,
        &node
    ));
    assert!(blocks.is_empty());
}

#[test]
fn standalone_image_rejects_non_file_src_reference() {
    let graph = graph(Path::new("/tmp/doc.md"));
    let mut blocks = Vec::new();
    let node = KmmNode {
        id: KmmNodeId("remote-image".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("https://example.com/image.svg"),
        children: vec![image_node(
            "screen",
            "https://example.com/image.svg",
            "![screen](https://example.com/image.svg)",
        )],
    };

    assert!(!SurfaceBlockFactory::append_standalone_image(
        &mut blocks,
        &graph,
        &node
    ));
    assert!(blocks.is_empty());
}

#[test]
fn standalone_image_rejects_unreadable_file_reference() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir().join("kdv-surface-image-block-unreadable");
    std::fs::create_dir_all(&temp_dir)?;
    let file_path = temp_dir.join("broken.txt");
    std::fs::write(&file_path, "not-an-image")?;

    let graph = graph(&temp_dir.join("doc.md"));
    let mut blocks = Vec::new();
    let node = KmmNode {
        id: KmmNodeId("broken-image".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("![broken](broken.txt)"),
        children: vec![image_node("broken", "broken.txt", "![broken](broken.txt)")],
    };

    assert!(!SurfaceBlockFactory::append_standalone_image(
        &mut blocks,
        &graph,
        &node
    ));
    assert!(blocks.is_empty());
    Ok(())
}
