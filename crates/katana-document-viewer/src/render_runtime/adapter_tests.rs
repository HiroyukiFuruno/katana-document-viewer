use super::*;
use crate::render_runtime::test_env::RenderRuntimeTestEnv;
use std::path::PathBuf;

#[test]
fn normalize_wraps_inline_expression_without_braces() {
    let normalized = MathJaxSourceNormalizer::normalize("a+b", KrrMathMode::Inline);
    assert_eq!(normalized, "{a+b}");
}

#[test]
fn normalize_keeps_inline_expression_with_existing_braces() {
    let normalized = MathJaxSourceNormalizer::normalize("{a+b}", KrrMathMode::Inline);
    assert_eq!(normalized, "{a+b}");
}

#[test]
fn normalize_keeps_display_expression_as_is() {
    let normalized = MathJaxSourceNormalizer::normalize("a+b", KrrMathMode::Display);
    assert_eq!(normalized, "a+b");
}

#[test]
fn restore_metadata_replaces_normalized_source() {
    let rendered = r#"<svg data-latex="{a+b}"></svg>"#;
    let restored = MathJaxSourceNormalizer::restore_metadata(rendered, "a+b", "{a+b}");

    assert_eq!(restored, r#"<svg data-latex="a+b"></svg>"#);
}

#[test]
fn restore_metadata_leaves_non_matched_payload() {
    let rendered = r#"<svg data-latex="keep"></svg>"#;

    assert_eq!(
        MathJaxSourceNormalizer::restore_metadata(rendered, "a", "a"),
        rendered
    );
}

#[test]
fn helper_detects_svg_payload() {
    assert!(is_svg("<svg><path /></svg>"));
    assert!(!is_svg("text"));
}

#[test]
fn diagnostics_message_joins_errors_and_warnings() {
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
            errors: vec!["missing runtime".to_string()],
            warnings: vec!["line wrap".to_string()],
        },
        cache_fingerprint: String::new(),
    };

    assert_eq!(diagnostics_message(&output), "missing runtime; line wrap");
}

#[test]
fn raw_error_wraps_input_and_code() {
    let output = raw_error(
        "raw-source",
        RenderError::InvalidInput("bad-input".to_string()),
    );

    assert_eq!(output.raw_payload(), "raw-source");
    assert_eq!(output.diagnostics.len(), 1);
    assert_eq!(
        output.diagnostic_message(),
        "invalid-input: invalid input: bad-input"
    );
}

#[test]
fn renderer_failure_keeps_source_raw_even_when_output_has_text() {
    let result = render_math_tex_result("x+1", "{x+1}", Ok(render_output_with_error("not-svg")));

    assert_eq!(result.raw_payload(), "x+1");
    assert_eq!(result.diagnostic_message(), "render-failed: failed");
}

fn render_output_with_error(svg: &str) -> katana_render_runtime::RenderOutput {
    katana_render_runtime::RenderOutput {
        svg: svg.to_string(),
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
            errors: vec!["failed".to_string()],
            warnings: Vec::new(),
        },
        cache_fingerprint: String::new(),
    }
}

#[test]
fn render_math_tex_empty_source_short_circuits_as_raw_output() {
    let output =
        KrrRenderRuntimeAdapter::render_math_tex_with_theme("", KrrMathMode::Display, None);

    assert_eq!(output.raw_payload(), "");
    assert_eq!(output.diagnostics.len(), 1);
    assert_eq!(output.diagnostics[0].code, "empty-input");
}

#[test]
fn render_math_tex_runtime_resolution_error_is_returned_as_raw() {
    RenderRuntimeTestEnv::with_mathjax_env(Some(""), || {
        let output =
            KrrRenderRuntimeAdapter::render_math_tex_with_theme("x+1", KrrMathMode::Inline, None);

        assert_eq!(output.raw_payload(), "x+1");
        assert_eq!(
            output.diagnostic_message(),
            "runtime-resolution-failed: runtime path resolution failed: MATHJAX_JS is empty"
        );
    });
}

#[test]
fn render_error_code_maps_variants() {
    assert_eq!(
        render_error_code(&RenderError::RuntimeResolution("missing".to_string())),
        "runtime-resolution-failed"
    );
    assert_eq!(
        render_error_code(&RenderError::InvalidInput("bad".to_string())),
        "invalid-input"
    );
    assert_eq!(
        render_error_code(&RenderError::NotInstalled {
            kind: "JS".to_string(),
            download_url: "https://example.com/download".to_string(),
            install_path: PathBuf::from("/tmp/mathjax"),
        }),
        "runtime-not-installed"
    );
    assert_eq!(
        render_error_code(&RenderError::Runtime("failure".to_string())),
        "runtime-failed"
    );
    assert_eq!(
        render_error_code(&RenderError::UnsupportedKind),
        "unsupported-kind"
    );
}

#[test]
fn mathjax_input_factory_configures_theme() {
    let theme = Some(RenderThemeSnapshot {
        mode: katana_render_runtime::RenderThemeMode::Dark,
        background: "bg".to_string(),
        text: "txt".to_string(),
        fill: "fill".to_string(),
        stroke: "stroke".to_string(),
        arrow: "arrow".to_string(),
        drawio_label_color: "drawio".to_string(),
        mermaid_theme: "dark".to_string(),
        plantuml_class_bg: String::new(),
        plantuml_note_bg: String::new(),
        plantuml_note_text: String::new(),
        syntax_theme_dark: String::new(),
        syntax_theme_light: String::new(),
        preview_text: String::new(),
    });
    let expected_theme = theme.clone();
    let input = MathJaxRenderInputFactory::create("x+1", KrrMathMode::Display, theme);

    assert_eq!(input.kind, katana_render_runtime::RenderKind::MathJax);
    assert_eq!(input.source, "x+1");
    assert_eq!(
        input.config.vendor_config,
        serde_json::json!({ "display": true })
    );
    assert_eq!(input.context.theme, expected_theme);
}
