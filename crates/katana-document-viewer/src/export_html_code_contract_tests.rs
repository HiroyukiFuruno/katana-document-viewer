use super::contract_test_support::{assert_contains_all, export_html};

#[test]
fn red_detects_code_block_syntax_highlighting_contract_gaps()
-> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r#"```rust
fn main() {
    let name = "KatanA";
}
```

```toml
[package]
name = "katana-document-viewer"
```
"#,
    )?;

    assert_contains_all(
        &html,
        &[
            (
                "rust syntax highlighter",
                r#"<pre data-kdv-code-role="plain" data-kdv-code-language="rust" data-kdv-code-highlighter="syntect" data-kdv-syntax-theme="InspiredGitHub">"#,
            ),
            ("rust language class", r#"<code class="language-rust">"#),
            ("highlight spans", "<span style="),
            (
                "toml syntax highlighter",
                r#"<pre data-kdv-code-role="plain" data-kdv-code-language="toml" data-kdv-code-highlighter="syntect" data-kdv-syntax-theme="InspiredGitHub">"#,
            ),
            ("toml language class", r#"<code class="language-toml">"#),
        ],
    );
    Ok(())
}

#[test]
fn red_detects_quoted_code_block_marker_leaks() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r#"> ```rust
> let quoted_code = true;
> ```
"#,
    )?;

    assert_contains_all(
        &html,
        &[
            (
                "quoted code language",
                r#"<pre data-kdv-code-role="plain" data-kdv-code-language="rust" data-kdv-code-highlighter="syntect" data-kdv-syntax-theme="InspiredGitHub">"#,
            ),
            ("quoted code body", "quoted_code"),
        ],
    );
    assert!(
        !html.contains("&gt; ```") && !html.contains("&gt; let"),
        "quoted fenced code must not leak blockquote markers: {html}"
    );
    Ok(())
}
