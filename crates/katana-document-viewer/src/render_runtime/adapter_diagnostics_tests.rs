use super::*;
use crate::render_runtime::test_env::RenderRuntimeTestEnv;

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

#[test]
fn render_runtime_test_env_restores_existing_mathjax_env() {
    unsafe { std::env::set_var("MATHJAX_JS", "/tmp/existing-mathjax.js") };

    RenderRuntimeTestEnv::with_mathjax_env(None, || {
        assert!(std::env::var_os("MATHJAX_JS").is_none());
    });

    assert_eq!(
        std::env::var_os("MATHJAX_JS"),
        Some(std::ffi::OsString::from("/tmp/existing-mathjax.js"))
    );
    unsafe { std::env::remove_var("MATHJAX_JS") };
}
