use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_code_uses_syntax_colored_spans() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "code.md",
        code_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "fn:[\"monospace\", \"color\"]",
            "main:[\"monospace\", \"color\"]",
        ],
    );
    Ok(())
}

#[test]
fn pdf_surface_code_block_strips_trailing_line_endings() -> Result<(), Box<dyn std::error::Error>> {
    let text = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        "code-string.md",
        code_string_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(&text, &[r#"let code = "引用ブロックの直後";"#]);
    SurfaceTestSupport::assert_not_contains_any(&text, &["let code =\n\""]);
    Ok(())
}

#[test]
fn pdf_surface_code_block_after_blockquote_keeps_string_literal()
-> Result<(), Box<dyn std::error::Error>> {
    let text = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        "quote-code.md",
        quote_then_code_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(&text, &[r#"let code = "引用ブロックの直後";"#]);
    SurfaceTestSupport::assert_not_contains_any(&text, &["let code =\n\""]);
    Ok(())
}

fn code_markdown() -> String {
    [
        "# code",
        "",
        "```rust",
        "fn main() {",
        "  println!(\"hi\");",
        "}",
        "```",
    ]
    .join("\n")
}

fn code_string_markdown() -> String {
    [
        "# code",
        "",
        "```rust",
        r#"let code = "引用ブロックの直後";"#,
        "```",
    ]
    .join("\n")
}

fn quote_then_code_markdown() -> String {
    [
        "> 引用ブロック",
        "",
        "```rust",
        r#"let code = "引用ブロックの直後";"#,
        "```",
    ]
    .join("\n")
}
