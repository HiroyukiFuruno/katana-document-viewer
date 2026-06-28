use super::types::{
    KrrMathMode, KrrRenderDiagnostic, KrrRenderKind, KrrRenderOutput, KrrRenderRequest,
    KrrRenderRuntime,
};
use katana_render_runtime::{
    MathJaxRenderer, RenderConfig, RenderContext, RenderError, RenderInput, RenderKind,
    RenderPolicy, RenderThemeSnapshot, Renderer, RuntimePathResolver,
};

#[derive(Default)]
pub(crate) struct KrrRenderRuntimeAdapter;

impl KrrRenderRuntimeAdapter {
    pub(crate) fn render_math_tex_with_theme(
        source: &str,
        math_mode: KrrMathMode,
        theme: Option<RenderThemeSnapshot>,
    ) -> KrrRenderOutput {
        Self.render(KrrRenderRequest::math_tex(source, math_mode).with_theme(theme))
    }
}

impl KrrRenderRuntime for KrrRenderRuntimeAdapter {
    fn render(&self, request: KrrRenderRequest) -> KrrRenderOutput {
        let source = request.source.trim();
        if source.is_empty() {
            return KrrRenderOutput::raw(
                String::new(),
                KrrRenderDiagnostic::new("empty-input", "KRR received empty source"),
            );
        }
        match request.kind {
            KrrRenderKind::MathTex => render_math_tex(source, request.math_mode, request.theme),
        }
    }
}

fn render_math_tex(
    source: &str,
    math_mode: KrrMathMode,
    theme: Option<RenderThemeSnapshot>,
) -> KrrRenderOutput {
    let runtime_path = match RuntimePathResolver::resolve(RenderKind::MathJax, None) {
        Ok(path) => path,
        Err(error) => return raw_error(source, error),
    };
    let renderer = MathJaxRenderer::with_runtime_path(runtime_path);
    let mathjax_source = MathJaxSourceNormalizer::normalize(source, math_mode);
    let input = MathJaxRenderInputFactory::create(&mathjax_source, math_mode, theme);
    render_math_tex_result(source, &mathjax_source, renderer.render(&input))
}

fn render_math_tex_result(
    source: &str,
    mathjax_source: &str,
    result: Result<katana_render_runtime::RenderOutput, RenderError>,
) -> KrrRenderOutput {
    match result {
        Ok(output) if output.diagnostics.errors.is_empty() && is_svg(&output.svg) => {
            KrrRenderOutput::svg(MathJaxSourceNormalizer::restore_metadata(
                &output.svg,
                source,
                mathjax_source,
            ))
        }
        Ok(output) => raw_render_failure(source, "render-failed", diagnostics_message(&output)),
        Err(error) => raw_error(source, error),
    }
}

struct MathJaxRenderInputFactory;

impl MathJaxRenderInputFactory {
    fn create(
        source: &str,
        math_mode: KrrMathMode,
        theme: Option<RenderThemeSnapshot>,
    ) -> RenderInput {
        RenderInput {
            kind: RenderKind::MathJax,
            source: source.to_string(),
            config: RenderConfig {
                vendor_config: serde_json::json!({
                    "display": matches!(math_mode, KrrMathMode::Display),
                }),
            },
            policy: RenderPolicy::default(),
            context: RenderContext {
                theme,
                ..RenderContext::default()
            },
        }
    }
}

struct MathJaxSourceNormalizer;

impl MathJaxSourceNormalizer {
    fn normalize(source: &str, math_mode: KrrMathMode) -> String {
        match math_mode {
            KrrMathMode::Inline if Self::needs_group(source) => format!("{{{source}}}"),
            _ => source.to_string(),
        }
    }

    fn needs_group(source: &str) -> bool {
        let trimmed = source.trim();
        !(trimmed.starts_with('{') && trimmed.ends_with('}'))
    }

    fn restore_metadata(svg: &str, source: &str, normalized_source: &str) -> String {
        if source == normalized_source {
            return svg.to_string();
        }
        svg.replace(
            &format!(r#"data-latex="{normalized_source}""#),
            &format!(r#"data-latex="{source}""#),
        )
        .replace(
            &format!(r#"data-latex="{{{source} }}""#),
            &format!(r#"data-latex="{source}""#),
        )
    }
}

fn is_svg(output: &str) -> bool {
    output.trim_start().starts_with("<svg")
}

fn diagnostics_message(output: &katana_render_runtime::RenderOutput) -> String {
    let message = output
        .diagnostics
        .errors
        .iter()
        .chain(output.diagnostics.warnings.iter())
        .cloned()
        .collect::<Vec<_>>()
        .join("; ");
    if message.is_empty() {
        return "renderer returned non-svg output".to_string();
    }
    message
}

fn raw_error(source: &str, error: RenderError) -> KrrRenderOutput {
    raw_render_failure(source, render_error_code(&error), error.to_string())
}

fn raw_render_failure(source: &str, code: &'static str, message: String) -> KrrRenderOutput {
    eprintln!("[kdv-render-runtime] {code}: {message}");
    KrrRenderOutput::raw(source.to_string(), KrrRenderDiagnostic::new(code, message))
}

fn render_error_code(error: &RenderError) -> &'static str {
    match error {
        RenderError::InvalidInput(_) => "invalid-input",
        RenderError::NotInstalled { .. } => "runtime-not-installed",
        RenderError::Runtime(_) => "runtime-failed",
        RenderError::RuntimeResolution(_) => "runtime-resolution-failed",
        RenderError::UnsupportedKind => "unsupported-kind",
    }
}

#[cfg(test)]
#[path = "adapter_diagnostics_tests.rs"]
mod diagnostics_tests;
#[cfg(test)]
#[path = "adapter_result_tests.rs"]
mod result_tests;
#[cfg(test)]
#[path = "adapter_tests.rs"]
mod tests;
