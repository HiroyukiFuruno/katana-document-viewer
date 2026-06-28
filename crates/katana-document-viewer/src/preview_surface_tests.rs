use super::*;
use crate::{
    BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, SourceKind,
    SourceRevision, SourceUri,
};
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};
use std::error::Error;
use std::path::PathBuf;

const VIEWPORT_WIDTH: f32 = 944.0;
const VIEWPORT_HEIGHT: f32 = 812.0;
const RGBA_CHANNEL_COUNT: usize = 4;
const LONG_DOCUMENT_LINES: usize = 180;

#[test]
fn preview_surface_keeps_full_logical_surface_for_stable_scroll() -> Result<(), Box<dyn Error>> {
    let graph = graph_from_markdown(&long_document())?;
    let surface = KdvPreviewSurfaceFactory::create(
        &graph,
        &KdvThemeSnapshot::katana_light(),
        ViewerViewport {
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        },
        0.0,
    );

    assert_eq!(SURFACE_WIDTH, surface.width);
    assert_eq!(surface.content_height, surface.height);
    assert!(surface.height > VIEWPORT_HEIGHT as u32);
    assert_eq!(
        surface.width as usize * surface.height as usize * RGBA_CHANNEL_COUNT,
        surface.rgba.len()
    );
    Ok(())
}

#[test]
fn pdf_surface_concatenates_rendered_pages() -> Result<(), Box<dyn Error>> {
    let graph = graph_from_markdown(&long_document())?;
    let surface = KdvPdfSurfaceFactory::create(&graph, &KdvThemeSnapshot::katana_light());

    assert_eq!(SURFACE_WIDTH, surface.width);
    assert_eq!(surface.height, surface.content_height);
    assert_eq!(0, surface.origin_y);
    assert_eq!(
        surface.width as usize * surface.height as usize * RGBA_CHANNEL_COUNT,
        surface.rgba.len()
    );
    Ok(())
}

fn graph_from_markdown(content: &str) -> Result<BuildGraph, Box<dyn Error>> {
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        PathBuf::from("preview.md"),
        content.to_string(),
    ))?;
    let source = DocumentSource {
        uri: SourceUri("preview://surface".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision(document.fingerprint.value.clone()),
        content: content.to_string(),
    };
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
    let request = BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    };
    let graph = crate::ForgePipeline::new(crate::ManifestOnlyBackend).build(&request)?;
    Ok(graph)
}

fn long_document() -> String {
    (0..LONG_DOCUMENT_LINES)
        .map(|index| format!("## Heading {index}\n\nParagraph {index}."))
        .collect::<Vec<_>>()
        .join("\n\n")
}
