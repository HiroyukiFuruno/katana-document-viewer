use super::PreviewOutputFactory;
use super::types::{
    MarkdownSource, PreviewConfig, PreviewError, PreviewOutput, PreviewSurfaceImage, PreviewTheme,
};
use crate::{ViewerMode, ViewerViewport};
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};

const CONTENT_HEIGHT: f32 = 120.0;
const SURFACE_WIDTH: u32 = 100;
const SURFACE_HEIGHT: u32 = 200;
const VIEWPORT_WIDTH: f32 = 50.0;
const VIEWPORT_HEIGHT: f32 = 25.0;
const EXPECTED_RECONFIGURED_HEIGHT: f32 = 125.0;

#[test]
fn preview_source_revision_tracks_document_fingerprint() -> Result<(), PreviewError> {
    let first = output_for("# First")?;
    let second = output_for("# Second")?;

    assert_ne!(
        first.input.snapshot.revision,
        second.input.snapshot.revision
    );
    assert_ne!(first.input.snapshot.revision.0, "live");
    Ok(())
}

#[test]
fn preview_source_without_document_id_uses_default_source_name() -> Result<(), PreviewError> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: "# Untitled".to_string(),
            document_id: None,
        },
        &PreviewConfig::default(),
        CONTENT_HEIGHT,
    )?;

    assert_eq!(
        output.input.snapshot.source_path.to_string_lossy(),
        "preview.md"
    );
    Ok(())
}

#[test]
fn preview_source_keeps_raw_markdown_spans() -> Result<(), PreviewError> {
    let output = output_for("## Raw\n\n```rust\nfn main() {}\n```\n\n<div>raw html</div>")?;

    assert!(raw_spans(&output).iter().any(|raw| raw.contains("```rust")));
    assert!(
        raw_spans(&output)
            .iter()
            .any(|raw| raw.contains("<div>raw html</div>"))
    );
    Ok(())
}

#[test]
fn preview_source_accepts_tilde_fenced_diagrams() -> Result<(), PreviewError> {
    let output = output_for("~~~mermaid\ngraph TD\nA-->B\n~~~")?;
    let node = output
        .input
        .snapshot
        .document
        .nodes
        .first()
        .ok_or_else(|| PreviewError::Render("diagram node missing".to_string()))?;

    assert!(matches!(
        &node.kind,
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::Mermaid
        })
    ));
    assert_eq!("```mermaid\ngraph TD\nA-->B\n```", node.source.raw.text);
    Ok(())
}

#[test]
fn preview_theme_light_fingerprint_does_not_become_dark_from_syntax_theme_name() {
    let theme = PreviewTheme {
        name: "katana-light".to_string(),
        fingerprint: concat!(
            "name=katana-light;mode=Light;background=#ffffff;",
            "syntax_dark=base16-ocean.dark;syntax_light=base16-ocean.light"
        )
        .to_string(),
    };

    assert!(
        !theme.is_dark(),
        "light theme must not become dark only because fingerprint carries syntax_dark metadata"
    );
}

#[test]
fn preview_theme_dark_fingerprint_uses_explicit_mode() {
    let theme = PreviewTheme {
        name: "katana".to_string(),
        fingerprint: "name=katana;mode=Dark;background=#1e1e1e".to_string(),
    };

    assert!(theme.is_dark());
}

#[test]
fn reconfigure_reuses_surface_and_updates_viewport_state() -> Result<(), PreviewError> {
    let mut output = output_for("# Title")?;
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "surface".to_string(),
        width: SURFACE_WIDTH,
        height: SURFACE_HEIGHT,
        origin_y: 0,
        content_height: SURFACE_HEIGHT,
        rgba: Vec::new(),
    });
    let config = PreviewConfig {
        viewport: ViewerViewport {
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        },
        ..PreviewConfig::default()
    };

    let updated = PreviewOutputFactory::reconfigure(&output, &config);

    assert_eq!(updated.input.viewport, config.viewport);
    assert_eq!(updated.content_height, EXPECTED_RECONFIGURED_HEIGHT);
    assert_eq!(
        updated.surface.as_ref().map(|surface| &surface.fingerprint),
        Some(&"surface".to_string())
    );
    Ok(())
}

#[test]
fn reconfigure_scales_surface_height_with_wider_viewport() -> Result<(), PreviewError> {
    let mut output = output_for("# Title")?;
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "wide-surface".to_string(),
        width: SURFACE_WIDTH,
        height: SURFACE_HEIGHT,
        origin_y: 0,
        content_height: SURFACE_HEIGHT,
        rgba: Vec::new(),
    });
    let config = PreviewConfig {
        viewport: ViewerViewport {
            width: SURFACE_WIDTH as f32 * 2.0,
            height: VIEWPORT_HEIGHT,
        },
        ..PreviewConfig::default()
    };

    let updated = PreviewOutputFactory::reconfigure(&output, &config);

    assert_eq!(
        updated.content_height,
        SURFACE_HEIGHT as f32 * 2.0 + VIEWPORT_HEIGHT
    );
    Ok(())
}

#[test]
fn reconfigure_keeps_surface_when_switching_to_slideshow() -> Result<(), PreviewError> {
    let mut output = output_for("# Slide\n\nBody")?;
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "shared-surface".to_string(),
        width: SURFACE_WIDTH,
        height: SURFACE_HEIGHT,
        origin_y: 0,
        content_height: SURFACE_HEIGHT,
        rgba: vec![255; SURFACE_WIDTH as usize * SURFACE_HEIGHT as usize * 4],
    });
    let config = PreviewConfig {
        mode: ViewerMode::Slideshow,
        viewport: ViewerViewport {
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        },
        scroll_offset: VIEWPORT_HEIGHT,
        ..PreviewConfig::default()
    };

    let updated = PreviewOutputFactory::reconfigure(&output, &config);

    assert_eq!(updated.input.mode, ViewerMode::Slideshow);
    assert_eq!(
        updated.surface.as_ref().map(|surface| &surface.fingerprint),
        Some(&"shared-surface".to_string())
    );
    assert_eq!(1, updated.state.slideshow.current_page_index);
    Ok(())
}

fn output_for(content: &str) -> Result<PreviewOutput, PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some("document.md".to_string()),
        },
        &PreviewConfig::default(),
        CONTENT_HEIGHT,
    )
}

fn raw_spans(output: &PreviewOutput) -> Vec<&str> {
    let mut raws = Vec::new();
    for node in &output.input.snapshot.document.nodes {
        collect_raw_spans(node, &mut raws);
    }
    raws
}

fn collect_raw_spans<'a>(node: &'a KmmNode, raws: &mut Vec<&'a str>) {
    raws.push(&node.source.raw.text);
    for child in &node.children {
        collect_raw_spans(child, raws);
    }
}
