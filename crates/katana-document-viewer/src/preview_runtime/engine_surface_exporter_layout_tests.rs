use super::*;

fn image(width: u32) -> PreviewSurfaceImage {
    PreviewSurfaceImage {
        fingerprint: "image".to_string(),
        width,
        height: 50,
        origin_y: 5,
        content_height: 40,
        rgba: Vec::new(),
    }
}

fn config(width: f32, height: f32) -> PreviewConfig {
    PreviewConfig {
        viewport: crate::ViewerViewport { width, height },
        ..PreviewConfig::default()
    }
}

#[test]
fn theme_uses_export_reference() {
    let output_theme = PreviewSurfaceExporter::theme(&PreviewConfig {
        theme: crate::PreviewTheme {
            name: "katana-export-reference".to_string(),
            fingerprint: String::new(),
        },
        ..PreviewConfig::default()
    });

    assert_eq!(KdvThemeSnapshot::katana_export_reference(), output_theme);
}

#[test]
fn content_height_scales_and_omits_tail_when_viewport_larger() {
    assert_eq!(
        160.0,
        PreviewSurfaceExporter::content_height(40, &image(100), &config(300.0, 40.0))
    );
    assert_eq!(
        12.0,
        PreviewSurfaceExporter::content_height(40, &image(100), &config(30.0, 200.0))
    );
}

#[test]
fn scale_defaults_for_zero_width_or_viewport() {
    assert_eq!(
        50.0,
        PreviewSurfaceExporter::content_height(40, &image(0), &config(0.0, 10.0))
    );
}
