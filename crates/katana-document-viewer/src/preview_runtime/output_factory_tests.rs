use super::PreviewOutputFactory;
use crate::{
    MarkdownSource, PreviewConfig, PreviewError, PreviewOutput, PreviewSurfaceImage, PreviewTheme,
    ViewerViewport,
};

#[test]
fn dark_preview_config_selects_katana_dark_theme() {
    let config = PreviewConfig {
        theme: PreviewTheme {
            name: "KatanA Dark".to_string(),
            fingerprint: "mode=dark".to_string(),
        },
        ..PreviewConfig::default()
    };

    assert_eq!(
        crate::KdvThemeMode::Dark,
        PreviewOutputFactory::theme(&config).mode
    );
}

#[test]
fn content_height_for_surface_uses_identity_scale_without_width() {
    let config = PreviewConfig::default();
    let mut output = PreviewOutputFactory::from_content_height(&config, 10.0);
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "surface".to_string(),
        width: 0,
        height: 10,
        origin_y: 0,
        content_height: 120,
        rgba: Vec::new(),
    });

    assert_eq!(
        120.0,
        PreviewOutputFactory::content_height_for_surface(&output, &config)
    );
}

#[test]
fn content_height_for_surface_omits_tail_when_viewport_covers_surface() {
    let config = PreviewConfig {
        viewport: ViewerViewport {
            width: 1280.0,
            height: 480.0,
        },
        ..PreviewConfig::default()
    };
    let mut output = PreviewOutputFactory::from_content_height(&config, 10.0);
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "surface".to_string(),
        width: 1280,
        height: 480,
        origin_y: 0,
        content_height: 480,
        rgba: Vec::new(),
    });

    assert_eq!(
        480.0,
        PreviewOutputFactory::content_height_for_surface(&output, &config)
    );
}

#[test]
fn from_source_reports_empty_markdown_as_render_error() {
    let source = MarkdownSource {
        content: " \n".to_string(),
        document_id: Some("empty.md".to_string()),
    };

    let result = PreviewOutputFactory::from_source(&source, &PreviewConfig::default(), 0.0);
    assert!(result.is_err(), "empty Markdown must fail fast");
    let Err(error) = result else {
        return;
    };

    assert!(matches!(error, PreviewError::Render(_)));
    assert_eq!("render error: Markdown source is empty", error.to_string());
}

#[test]
fn reconfigure_function_entry_updates_output_config() {
    let config = PreviewConfig {
        scroll_offset: 12.0,
        ..PreviewConfig::default()
    };
    let output = PreviewOutputFactory::from_content_height(&PreviewConfig::default(), 10.0);
    let reconfigure: fn(&PreviewOutput, &PreviewConfig) -> PreviewOutput =
        PreviewOutputFactory::reconfigure;

    let updated = reconfigure(&output, &config);

    assert_eq!(12.0, updated.scroll_offset);
}

#[test]
fn from_content_height_tracks_html_conversion_for_html_source()
-> Result<(), Box<dyn std::error::Error>> {
    let output = PreviewOutputFactory::from_source(
        &crate::MarkdownSource {
            content: "<section><p>hello</p></section>".to_string(),
            document_id: Some("from_source_html.html".to_string()),
        },
        &crate::PreviewConfig {
            viewport: crate::ViewerViewport {
                width: 800.0,
                height: 480.0,
            },
            ..crate::PreviewConfig::default()
        },
        300.0,
    )?;

    assert_eq!(crate::DocumentKind::Html, output.input.snapshot.kind);
    assert_eq!(0.0, output.scroll_offset);
    Ok(())
}

#[test]
fn content_height_for_surface_scales_and_appends_viewport_when_needed() {
    let config = PreviewConfig {
        viewport: crate::ViewerViewport {
            width: 240.0,
            height: 50.0,
        },
        ..PreviewConfig::default()
    };
    let mut output = PreviewOutputFactory::from_content_height(&config, 120.0);
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "surface".to_string(),
        width: 120,
        height: 20,
        origin_y: 0,
        content_height: 320,
        rgba: Vec::new(),
    });

    assert_eq!(
        690.0,
        PreviewOutputFactory::content_height_for_surface(&output, &config)
    );
}

#[test]
fn reconfigure_clamps_scroll_and_updates_state_height() {
    let mut output = PreviewOutputFactory::from_content_height(&PreviewConfig::default(), 180.0);
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "surface".to_string(),
        width: 100,
        height: 10,
        origin_y: 0,
        content_height: 40,
        rgba: Vec::new(),
    });

    let updated = PreviewOutputFactory::reconfigure(
        &output,
        &PreviewConfig {
            scroll_offset: -20.0,
            viewport: crate::ViewerViewport {
                width: 200.0,
                height: 50.0,
            },
            ..PreviewConfig::default()
        },
    );

    assert_eq!(0.0, updated.scroll_offset);
    assert_eq!(130.0, updated.content_height);
}

#[test]
fn typography_clamps_base_font_size_bounds() {
    let small_font = PreviewOutputFactory::from_content_height(
        &PreviewConfig {
            base_font_size: Some(2.0),
            ..PreviewConfig::default()
        },
        20.0,
    );
    assert_eq!(12, small_font.input.typography.preview_font_size);

    let large_font = PreviewOutputFactory::from_content_height(
        &PreviewConfig {
            base_font_size: Some(40.0),
            ..PreviewConfig::default()
        },
        20.0,
    );
    assert_eq!(32, large_font.input.typography.preview_font_size);
}
