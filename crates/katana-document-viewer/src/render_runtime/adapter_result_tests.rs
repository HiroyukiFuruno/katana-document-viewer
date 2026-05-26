use super::*;

#[test]
fn render_math_tex_result_converts_renderer_error_to_raw_diagnostic() {
    let output = render_math_tex_result(
        "source",
        "{source}",
        Err(RenderError::Runtime("runtime failed".to_string())),
    );

    assert_eq!(output.raw_payload(), "source");
    assert_eq!(
        output.diagnostic_message(),
        "runtime-failed: runtime error: runtime failed"
    );
}
