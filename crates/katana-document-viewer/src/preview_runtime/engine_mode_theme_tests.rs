use super::*;
use crate::KdvThemeMode;
use crate::{MarkdownPreview, PreviewConfig, PreviewTheme, ViewerMode, ViewerViewport};
use std::time::{Duration, Instant};

const INITIAL_RENDER_BUDGET: Duration = Duration::from_millis(600);
const SLIDESHOW_TEST_VIEWPORT_WIDTH: f32 = 960.0;
const SLIDESHOW_TEST_VIEWPORT_HEIGHT: f32 = 540.0;

#[test]
fn render_engine_uses_document_surface_for_slideshow_mode() -> Result<(), Box<dyn std::error::Error>>
{
    let source = MarkdownSource {
        content: include_str!("../../../../assets/fixtures/katana/sample_basic.md").to_string(),
        document_id: Some("sample_basic.md".to_string()),
    };
    let document = PreviewRenderEngine.render(&source, &viewport_config(ViewerMode::Document))?;
    let slideshow = PreviewRenderEngine.render(&source, &viewport_config(ViewerMode::Slideshow))?;
    let Some(document_surface) = document.surface else {
        return Err(Box::new(std::io::Error::other("document surface missing")));
    };
    let Some(slideshow_surface) = slideshow.surface else {
        return Err(Box::new(std::io::Error::other("slideshow surface missing")));
    };

    assert_eq!(slideshow.input.mode, ViewerMode::Slideshow);
    assert_eq!(document_surface.fingerprint, slideshow_surface.fingerprint);
    assert_eq!(document_surface.width, slideshow_surface.width);
    assert_eq!(document_surface.height, slideshow_surface.height);
    assert_eq!(document_surface.rgba, slideshow_surface.rgba);
    Ok(())
}

#[test]
fn render_engine_dark_theme_selects_dark_surface_theme() {
    let config = PreviewConfig {
        theme: PreviewTheme {
            name: "katana-dark".to_string(),
            fingerprint: "dark".to_string(),
        },
        ..PreviewConfig::default()
    };

    assert_eq!(
        KdvThemeMode::Dark,
        PreviewSurfaceExporter::theme(&config).mode
    );
}

#[test]
fn render_engine_dark_theme_accepts_caller_fingerprint() {
    let config = PreviewConfig {
        theme: PreviewTheme {
            name: "caller-theme".to_string(),
            fingerprint: "mode=Dark;caller=theme".to_string(),
        },
        ..PreviewConfig::default()
    };

    assert_eq!(
        KdvThemeMode::Dark,
        PreviewSurfaceExporter::theme(&config).mode
    );
}

#[test]
fn render_engine_export_reference_theme_uses_katana_reference_tokens() {
    let config = PreviewConfig {
        theme: PreviewTheme {
            name: "katana-export-reference".to_string(),
            fingerprint: "katana-export-reference".to_string(),
        },
        ..PreviewConfig::default()
    };

    let theme = PreviewSurfaceExporter::theme(&config);

    assert_eq!("#ffffff", theme.background);
    assert_eq!("#f6f8fa", theme.code_background);
    assert_eq!("#f3f3f3", theme.table_header_background);
    assert_eq!("#fff2cc", theme.diagram_fill);
    assert_eq!("default", theme.mermaid_theme);
}

#[test]
#[ignore = "wall-clock budget is validated by `just storybook-check` with one test thread"]
fn render_engine_keeps_diagram_fixture_inside_interactive_budget()
-> Result<(), Box<dyn std::error::Error>> {
    PreviewRenderEngine.render(&warmup_source(), &PreviewConfig::default())?;
    let started_at = Instant::now();
    let output = PreviewRenderEngine.render(&diagram_source(), &PreviewConfig::default())?;
    let elapsed = started_at.elapsed();

    assert!(
        elapsed <= INITIAL_RENDER_BUDGET,
        "preview render took {elapsed:?}, budget is {INITIAL_RENDER_BUDGET:?}"
    );
    assert!(output.surface.is_some());
    Ok(())
}

fn viewport_config(mode: ViewerMode) -> PreviewConfig {
    PreviewConfig {
        mode,
        viewport: ViewerViewport {
            width: SLIDESHOW_TEST_VIEWPORT_WIDTH,
            height: SLIDESHOW_TEST_VIEWPORT_HEIGHT,
        },
        ..PreviewConfig::default()
    }
}

fn warmup_source() -> MarkdownSource {
    MarkdownSource {
        content: "# Warmup".to_string(),
        document_id: Some("warmup.md".to_string()),
    }
}

fn diagram_source() -> MarkdownSource {
    MarkdownSource {
        content: include_str!("../../../../assets/fixtures/katana/sample_diagrams.md").to_string(),
        document_id: Some("sample_diagrams.md".to_string()),
    }
}
