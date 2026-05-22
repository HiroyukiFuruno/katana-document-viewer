use super::{
    KrrMathMode, KrrRenderPayload, KrrRenderRequest, KrrRenderRuntime, StubKrrRenderRuntime,
};

#[test]
fn stub_runtime_renders_tex_to_svg_without_parsing_markdown_ast() {
    let output =
        StubKrrRenderRuntime.render(KrrRenderRequest::math_tex(r"E = mc^2", KrrMathMode::Inline));

    match output.payload {
        KrrRenderPayload::Svg(svg) => assert!(svg.contains("<svg")),
        KrrRenderPayload::Raw(raw) => panic!("expected svg, got raw: {raw}"),
    }
    assert!(output.diagnostics.is_empty());
}

#[test]
fn stub_runtime_returns_raw_source_for_invalid_input() {
    let output = StubKrrRenderRuntime.render(KrrRenderRequest::math_tex("", KrrMathMode::Display));

    assert_eq!(output.payload, KrrRenderPayload::Raw(String::new()));
    assert_eq!(output.diagnostics[0].code, "empty-input");
}
