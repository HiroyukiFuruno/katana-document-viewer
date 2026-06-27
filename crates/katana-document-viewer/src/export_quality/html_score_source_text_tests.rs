use super::HtmlQualityScore;

#[test]
fn direct_html_source_rejects_structure_only_output() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><h1></h1><p></p></main>"#,
        r#"<main><h1>Katana Preview</h1><p>Visible body text</p></main>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html preserves source visible text".to_string()),
        "{score:#?}"
    );
}

#[test]
fn direct_html_source_accepts_preserved_visible_text() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><h1>Katana Preview</h1><p>Visible body text</p></main>"#,
        r#"<main><h1>Katana Preview</h1><p>Visible body text</p></main>"#,
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn direct_html_source_rejects_reordered_visible_text() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>Third Second First</p></main>"#,
        r#"<main><p>First Second Third</p></main>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html preserves source visible text".to_string()),
        "{score:#?}"
    );
}

#[test]
fn direct_html_source_rejects_missing_text_after_early_tokens() {
    let source = format!("<main><p>{}</p></main>", numbered_tokens(30));
    let output = format!(
        "<main data-kdv-export><style data-kdv-export-style></style><p>{}</p></main>",
        numbered_tokens(24)
    );

    let score = HtmlQualityScore::score(output.as_bytes(), &source);

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html preserves source visible text".to_string()),
        "{score:#?}"
    );
}

#[test]
fn html_link_source_requires_visible_label_not_only_href() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><a href="https://example.com"></a></main>"#,
        r#"<a href="https://example.com">Example Link</a>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html preserves source visible text".to_string()),
        "{score:#?}"
    );
}

fn numbered_tokens(count: usize) -> String {
    (1..=count)
        .map(|number| format!("token{number:02}"))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn style_text_does_not_count_as_visible_text() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style>.hidden{content:"Katana Preview"}</style><h1></h1></main>"#,
        r#"<main><h1>Katana Preview</h1></main>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html preserves source visible text".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_document_with_embedded_html_does_not_require_global_source_tokens() {
    let score = HtmlQualityScore::score(
        br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<h1>Contract</h1>
<div align="left">Left HTML</div>
</main>
"#,
        r#"# Contract

**strong**

<div align="left">Left HTML</div>
"#,
    );

    assert!(
        !score
            .fatal_failures()
            .contains(&"Html: html preserves source visible text".to_string()),
        "{score:#?}"
    );
}
