use crate::HtmlExportContractMatrix;

#[test]
fn html_export_renders_links_from_kmm_v0_2_dto() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("- [通常のリンク](https://github.com)\n")?;
    let matrix = HtmlExportContractMatrix::v0_1();

    assert!(matrix.contains_feature("commonmark-link", crate::HtmlExportReadiness::Implemented));
    assert!(html.contains(r#"<a href="https://github.com">通常のリンク</a>"#));
    assert!(!html.contains("[通常のリンク](https://github.com)"));
    Ok(())
}

#[test]
fn html_export_renders_inline_decoration_from_kmm_v0_2_dto()
-> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("- **太字**\n- *斜体*\n- `code`\n")?;
    let matrix = HtmlExportContractMatrix::v0_1();

    for feature in [
        "commonmark-strong",
        "commonmark-emphasis",
        "commonmark-inline-code",
    ] {
        assert!(matrix.contains_feature(feature, crate::HtmlExportReadiness::Implemented));
    }
    assert!(html.contains("<strong>太字</strong>"));
    assert!(html.contains("<em>斜体</em>"));
    assert!(html.contains("<code>code</code>"));
    Ok(())
}

#[test]
fn html_export_renders_inline_markup_inside_heading() -> Result<(), Box<dyn std::error::Error>> {
    let html =
        super::support::export_html("### 1.1 `<h1 align=\"center\">` — *中央*で**見出し**\n")?;

    assert!(html.contains(
        r#"<h3>1.1 <code>&lt;h1 align=&quot;center&quot;&gt;</code> — <em>中央</em>で<strong>見出し</strong></h3>"#
    ));
    assert!(html.contains("em{font-style:italic;}"));
    assert!(html.contains(":not(pre)>code{"));
    assert!(!html.contains("###"));
    Ok(())
}

#[test]
fn html_export_renders_inline_markup_inside_list_item() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("- `インラインコード` と *イタリック*\n")?;

    assert!(html.contains("<li><code>インラインコード</code> と <em>イタリック</em></li>"));
    assert!(!html.contains("`インラインコード`"));
    assert!(!html.contains("*イタリック*"));
    Ok(())
}

#[test]
fn html_export_keeps_unmatched_inline_markers_as_text() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("- 未完了の `inline code\n")?;

    assert!(html.contains("未完了の `inline code"));
    assert!(!html.contains("<code>inline code</code>"));
    Ok(())
}

#[test]
fn html_export_strips_markdown_heading_markers() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("# H1 見出し\n\n### `code` 見出し\n")?;

    assert!(html.contains("<h1>H1 見出し</h1>"));
    assert!(html.contains("<h3><code>code</code> 見出し</h3>"));
    assert!(!html.contains("<h1># "));
    assert!(!html.contains("<h3>###"));
    Ok(())
}

#[test]
fn html_export_strips_fenced_code_markers() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("```rust\nfn main() {}\n```\n")?;

    assert!(html.contains(
        r#"<pre data-kdv-code-role="plain" data-kdv-code-language="rust" data-kdv-code-highlighter="syntect" data-kdv-syntax-theme="InspiredGitHub">"#
    ));
    assert!(html.contains(r#"<code class="language-rust">"#));
    assert!(html.contains("<span style="));
    assert!(html.contains("main"));
    assert!(!html.contains("```rust"));
    Ok(())
}

#[test]
fn html_export_renders_alert_body_without_marker() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("> [!NOTE]\n> 本文\n")?;

    assert!(html.contains(r#"<aside data-github-alert="NOTE" data-kdv-blockquote="alert">"#));
    assert!(html.contains(
        r#"<p data-kdv-alert-title="NOTE"><span data-kdv-alert-icon="NOTE" aria-hidden="true"><svg data-kdv-alert-icon-svg="NOTE""#
    ));
    assert!(html.contains("<p>本文</p>"));
    assert!(!html.contains("[!NOTE]"));
    Ok(())
}

#[test]
fn html_export_decodes_html_entities_in_text() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("- HTML エンティティ: &amp; &lt; &gt; &quot;\n")?;

    assert!(html.contains("HTML エンティティ: &amp; &lt; &gt; &quot;"));
    assert!(!html.contains("&amp;amp;"));
    assert!(!html.contains("&amp;lt;"));
    assert!(!html.contains("&amp;gt;"));
    assert!(!html.contains("&amp;quot;"));
    Ok(())
}

#[test]
fn html_export_preserves_katana_html_block() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("<p align=\"center\">\n  中央\n</p>\n")?;

    assert!(html.contains(r#"<p align="center">"#));
    assert!(html.contains("中央"));
    Ok(())
}

#[test]
fn html_export_uses_table_header_and_body() -> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("| 左 | 右 |\n| :--- | ---: |\n| A | B |\n")?;

    assert!(html.contains("<thead><tr>"));
    assert!(html.contains(r#"<th data-align="left" data-kdv-column-size="short">左</th>"#));
    assert!(html.contains(r#"<td data-align="right" data-kdv-column-size="short">B</td>"#));
    assert!(!html.contains(":---"));
    Ok(())
}

#[test]
fn html_export_renders_fenced_math_with_shared_svg_renderer()
-> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("```math\nE = mc^2\n```\n")?;

    assert!(html.contains(r#"data-kdv-math="block""#));
    assert!(html.contains(r#"data-kdv-render-runtime="katana-render-runtime""#));
    assert!(!html.contains(r#"data-kdv-render-runtime="katana-render-runtime-stub""#));
    assert!(!html.contains(r#"data-kdv-math-renderer="mathjax-svg""#));
    assert!(html.contains("<svg"));
    assert!(!html.contains(r#"data-kdv-export-readiness="external-backend-required""#));
    assert!(!html.contains("```math"));
    Ok(())
}
