use super::*;

#[test]
fn diagnostics_message_uses_default_for_empty_diagnostics() {
    let output = katana_render_runtime::RenderOutput {
        svg: String::new(),
        width: 0.0,
        height: 0.0,
        view_box: String::new(),
        runtime: katana_render_runtime::RuntimeVersion {
            name: String::new(),
            version: String::new(),
            checksum: None,
        },
        profile: katana_render_runtime::RendererProfile {
            id: String::new(),
            description: None,
        },
        diagnostics: katana_render_runtime::RenderDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
        },
        cache_fingerprint: String::new(),
    };

    assert_eq!(
        diagnostics_message(&output),
        "renderer returned non-svg output"
    );
}
