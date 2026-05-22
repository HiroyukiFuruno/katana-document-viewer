use super::mathjax_svg::MathJaxSvgRenderer;
use super::types::{
    KrrMathMode, KrrRenderDiagnostic, KrrRenderKind, KrrRenderOutput, KrrRenderRequest,
    KrrRenderRuntime,
};

#[derive(Default)]
pub(crate) struct StubKrrRenderRuntime;

impl StubKrrRenderRuntime {
    pub(crate) fn render_math_tex(source: &str, math_mode: KrrMathMode) -> KrrRenderOutput {
        Self.render(KrrRenderRequest::math_tex(source, math_mode))
    }
}

impl KrrRenderRuntime for StubKrrRenderRuntime {
    fn render(&self, request: KrrRenderRequest) -> KrrRenderOutput {
        let source = request.source.trim();
        if source.is_empty() {
            return KrrRenderOutput::raw(
                String::new(),
                KrrRenderDiagnostic::new("empty-input", "KRR stub received empty source"),
            );
        }
        match request.kind {
            KrrRenderKind::MathTex => render_math_tex(source, request.math_mode),
        }
    }
}

fn render_math_tex(source: &str, math_mode: KrrMathMode) -> KrrRenderOutput {
    let inline = matches!(math_mode, KrrMathMode::Inline);
    match MathJaxSvgRenderer::render(source, inline) {
        Ok(svg) => KrrRenderOutput::svg(svg),
        Err(message) => KrrRenderOutput::raw(
            source.to_string(),
            KrrRenderDiagnostic::new("render-failed", message),
        ),
    }
}
