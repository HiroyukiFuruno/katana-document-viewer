use super::HtmlQualityScore;

#[test]
fn markdown_block_source_requires_matching_html_structures() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>flat</p></main>"#,
        markdown_blocks(),
    );

    let failures = score.fatal_failures();
    assert_contains(&failures, "Html: html renders heading block");
    assert_contains(&failures, "Html: html renders list block");
    assert_contains(&failures, "Html: html renders blockquote block");
    assert_contains(&failures, "Html: html renders table block");
    assert_contains(&failures, "Html: html renders code block");
    assert_contains(&failures, "Html: html renders syntax highlighted code");
    assert_contains(&failures, "Html: html renders thematic break");
    assert_contains(&failures, "Html: html renders footnote definition");
}

#[test]
fn parenthesized_ordered_list_source_requires_html_list() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>item</p></main>"#,
        "1) item\n",
    );

    assert_contains(&score.fatal_failures(), "Html: html renders list block");
}

#[test]
fn markdown_block_source_accepts_matching_html_structures() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<h1>Title</h1>
<ul><li>item</li></ul>
<blockquote data-kdv-blockquote="quote"><p>quote</p></blockquote>
<table data-kdv-table="katana"><tr><td>cell</td></tr></table>
<pre data-kdv-code-role="plain" data-kdv-code-highlighter="syntect"><code><span style="color:#111">let</span> x = 1;</code></pre>
<hr>
<section data-kdv-footnotes><ol><li data-kdv-footnote-definition="1">note</li></ol></section>
</main>
"#;

    let score = HtmlQualityScore::score(html, markdown_blocks());

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_inline_source_requires_emphasis_and_strikethrough_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>italic gone</p></main>"#,
        "*italic* and ~~gone~~\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html evaluates inline markdown",
    );
}

#[test]
fn markdown_inline_source_accepts_emphasis_and_strikethrough_html() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p><em>italic</em> and <s>gone</s></p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "*italic* and ~~gone~~\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_inline_source_requires_reference_link_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>Reference</p></main>"#,
        "[Reference][id]\n\n[id]: https://example.com\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html evaluates inline markdown",
    );
}

#[test]
fn markdown_inline_source_accepts_reference_link_html() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p><a href="https://example.com">Reference</a></p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "[Reference][id]\n\n[id]: https://example.com\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_inline_source_requires_shortcut_reference_link_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>Shortcut</p></main>"#,
        "[Shortcut]\n\n[Shortcut]: https://example.com\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html evaluates inline markdown",
    );
}

#[test]
fn markdown_inline_source_requires_autolink_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>https://example.com</p></main>"#,
        "<https://example.com>\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html evaluates inline markdown",
    );
}

#[test]
fn markdown_inline_source_requires_email_autolink_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>user@example.com</p></main>"#,
        "<user@example.com>\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html evaluates inline markdown",
    );
}

#[test]
fn markdown_inline_source_ignores_fenced_code_markers() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<pre data-kdv-code-role="plain" data-kdv-code-highlighter="syntect"><code><span style="color:#111">**not strong** and *not italic* and ~~not gone~~</span></code></pre>
</main>
"#;
    let source = "```rust\n**not strong** and *not italic* and ~~not gone~~\n```\n";

    let score = HtmlQualityScore::score(html, source);

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_task_source_requires_blocked_task_state_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><ul><li>blocked</li></ul></main>"#,
        "- [-] blocked\n",
    );

    assert_contains(&score.fatal_failures(), "Html: html evaluates task state");
}

#[test]
fn markdown_task_source_accepts_blocked_task_state_html() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<ul><li data-kdv-task-item="true"><input data-kdv-task-marker="[-]" data-kdv-task-state="blocked"><span data-kdv-task-visual="blocked-dash"></span>blocked</li></ul>
</main>
"#;

    let score = HtmlQualityScore::score(html, "- [-] blocked\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

fn markdown_blocks() -> &'static str {
    "# Title\n\n- item\n\n> quote\n\n| A |\n| - |\n| B |\n\n```rust\nlet x = 1;\n```\n\n---\n\nbody[^1]\n\n[^1]: note\n"
}

fn assert_contains(failures: &[String], expected: &str) {
    assert!(
        failures.iter().any(|failure| failure == expected),
        "{failures:#?}"
    );
}
