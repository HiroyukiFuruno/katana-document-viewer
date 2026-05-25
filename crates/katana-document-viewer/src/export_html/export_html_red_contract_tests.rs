use super::contract_test_support::HtmlContractTestSupport;

#[test]
fn red_detects_markdown_standard_html_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(markdown_standard_contract())?;
    assert_markdown_standard_inline_contract(&html);
    assert_markdown_standard_block_contract(&html);
    Ok(())
}

fn markdown_standard_contract() -> &'static str {
    r##"# H1 見出し
- **太字と*イタリック*の混在**
- *イタリックテキスト*
- [通常のリンク](https://github.com)
- 自動リンク: <https://github.com>
- ![アイコン](https://example.com/icon.png)
> **太字の引用**
>
> - リスト項目 1
1. 最初のステップ:

   ```sh
   cargo build --release
   ```
これは脚注付きです[^1].

[^1]: 脚注本文。
"##
}

fn assert_markdown_standard_inline_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            ("heading", "<h1>H1 見出し</h1>"),
            (
                "nested emphasis",
                "<strong>太字と<em>イタリック</em>の混在</strong>",
            ),
            ("simple emphasis", "<em>イタリックテキスト</em>"),
            ("link", r#"<a href="https://github.com">通常のリンク</a>"#),
            (
                "autolink",
                r#"<a href="https://github.com" data-kdv-autolink="true">https://github.com</a>"#,
            ),
            (
                "image",
                r#"<img src="https://example.com/icon.png" alt="アイコン">"#,
            ),
        ],
    );
}

fn assert_markdown_standard_block_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "blockquote strong",
                "<blockquote data-kdv-blockquote=\"quote\"><p><strong>太字の引用</strong></p>",
            ),
            ("blockquote list", "<ul><li>リスト項目 1</li></ul>"),
            (
                "list code block",
                r#"<li>最初のステップ:<pre data-kdv-code-role="plain" data-kdv-code-language="sh" data-kdv-code-highlighter="syntect" data-kdv-syntax-theme="InspiredGitHub"><code class="language-sh">"#,
            ),
            (
                "footnote",
                r##"<sup id="fnref-1" data-kdv-footnote-ref="1"><a href="#fn-1">[1]</a></sup>"##,
            ),
        ],
    );
}

#[test]
fn red_detects_krr_diagram_svg_export_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(diagram_contract_markdown())?;
    assert_diagram_svg_contract(&html);
    assert_diagram_raw_sources_are_hidden(&html);
    Ok(())
}

fn diagram_contract_markdown() -> &'static str {
    r##"```mermaid
graph TD
    A[開始] --> B[終了]
```

```drawio
<mxGraphModel><root><mxCell id="0"/></root></mxGraphModel>
```

```plantuml
@startuml
Alice -> Bob : OK
@enduml
```
"##
}

fn assert_diagram_svg_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "mermaid svg",
                r#"<figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg"#,
            ),
            (
                "drawio svg",
                r#"<figure data-kdv-diagram="drawio" data-kdv-diagram-theme="light"><svg"#,
            ),
            (
                "plantuml svg",
                r#"<figure data-kdv-diagram="plantuml" data-kdv-diagram-theme="light"><svg"#,
            ),
        ],
    );
}

fn assert_diagram_raw_sources_are_hidden(html: &str) {
    HtmlContractTestSupport::assert_not_contains_any(
        html,
        &[
            (
                "mermaid unresolved",
                r#"data-kdv-export-readiness="requires-krr-render""#,
            ),
            (
                "plantuml unresolved",
                r#"data-kdv-export-readiness="external-backend-required""#,
            ),
            ("drawio raw mxGraph", "&lt;mxGraphModel&gt;"),
            ("plantuml raw source", "@startuml"),
        ],
    );
}

#[test]
fn red_detects_github_alert_body_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(
        r##"> [!NOTE]
> **重要** な補足
>
> - item

> [!WARNING]
> `rm` は実行しない
"##,
    )?;

    HtmlContractTestSupport::assert_contains_all(
        &html,
        &[
            (
                "note title",
                r#"<aside data-github-alert="NOTE" data-kdv-blockquote="alert"><p data-kdv-alert-title="NOTE"><span data-kdv-alert-icon="NOTE" aria-hidden="true"><svg data-kdv-alert-icon-svg="NOTE""#,
            ),
            ("note inline body", "<p><strong>重要</strong> な補足</p>"),
            ("note list body", "<ul><li>item</li></ul>"),
            (
                "warning label",
                r#"<aside data-github-alert="WARNING" data-kdv-blockquote="alert">"#,
            ),
            ("warning code", "<code>rm</code> は実行しない"),
        ],
    );
    Ok(())
}
