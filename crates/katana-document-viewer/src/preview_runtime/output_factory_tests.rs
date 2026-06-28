use super::PreviewOutputFactory;
use crate::{
    MarkdownSource, PreviewConfig, PreviewError, PreviewOutput, PreviewSurfaceImage, ViewerViewport,
};

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
