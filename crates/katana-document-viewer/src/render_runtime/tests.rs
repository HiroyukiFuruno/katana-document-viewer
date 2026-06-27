use super::{
    KrrMathMode, KrrRenderPayload, KrrRenderRequest, KrrRenderRuntime, KrrRenderRuntimeAdapter,
};
use crate::render_runtime::test_env::RenderRuntimeTestEnv;

#[test]
fn krr_runtime_renders_tex_to_svg_without_parsing_markdown_ast()
-> Result<(), Box<dyn std::error::Error>> {
    RenderRuntimeTestEnv::with_mathjax_env(None, || {
        let output = KrrRenderRuntimeAdapter
            .render(KrrRenderRequest::math_tex(r"E = mc^2", KrrMathMode::Inline));

        match output.payload {
            KrrRenderPayload::Svg(svg) => {
                assert!(svg.contains("<svg"));
                assert!(
                    svg.contains(r#"data-latex="E = mc^2""#),
                    "root svg metadata must keep the full input: {svg}"
                );
                assert!(
                    svg.contains(r#"data-latex="=""#) && svg.contains(r#"data-latex="m""#),
                    "KRR must render the full inline expression instead of the first token only: {svg}"
                );
            }
            KrrRenderPayload::Raw(raw) => {
                return Err(std::io::Error::other(format!("expected svg, got raw: {raw}")).into());
            }
        }
        assert!(output.diagnostics.is_empty());
        Ok(())
    })
}

#[test]
fn krr_runtime_returns_raw_source_for_invalid_input() {
    let output =
        KrrRenderRuntimeAdapter.render(KrrRenderRequest::math_tex("", KrrMathMode::Display));

    assert_eq!(output.payload, KrrRenderPayload::Raw(String::new()));
    assert_eq!(output.diagnostics[0].code, "empty-input");
}
