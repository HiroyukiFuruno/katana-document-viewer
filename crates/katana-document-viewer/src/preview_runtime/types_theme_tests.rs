use super::types::PreviewTheme;

#[test]
fn preview_theme_detects_dark_from_name_or_fingerprint() {
    assert!(
        PreviewTheme {
            name: "katana-dark".to_string(),
            fingerprint: "caller".to_string(),
        }
        .is_dark()
    );
    assert!(
        PreviewTheme {
            name: "caller-theme".to_string(),
            fingerprint: "mode=Dark;caller=theme".to_string(),
        }
        .is_dark()
    );
    assert!(
        !PreviewTheme {
            name: "light".to_string(),
            fingerprint: "caller-light".to_string(),
        }
        .is_dark()
    );
}

#[test]
fn preview_theme_detects_katana_export_reference_from_name_or_fingerprint() {
    assert!(
        PreviewTheme {
            name: "katana-export-reference".to_string(),
            fingerprint: "caller".to_string(),
        }
        .is_katana_export_reference()
    );
    assert!(
        PreviewTheme {
            name: "caller-theme".to_string(),
            fingerprint: "katana-export-reference".to_string(),
        }
        .is_katana_export_reference()
    );
    assert!(
        !PreviewTheme {
            name: "katana-light".to_string(),
            fingerprint: "caller-light".to_string(),
        }
        .is_katana_export_reference()
    );
}
