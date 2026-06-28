use super::HtmlQualityScore;

#[test]
fn latex_fence_is_external_block_and_does_not_require_code_block_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>math</p></main>"#,
        "```latex\nx = 1\n```\n",
    );

    assert!(
        !score
            .fatal_failures()
            .contains(&"Html: html renders code block".to_string()),
        "{score:#?}"
    );
}

#[test]
fn nested_list_source_requires_nested_html_list() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><ul><li>parent child</li></ul></main>"#,
        "- parent\n  - child\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html preserves nested list depth",
    );
}

#[test]
fn multiple_markdown_headings_require_matching_html_heading_count() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><h1>First</h1><p>Second</p></main>"#,
        "# First\n\n## Second\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html preserves heading count",
    );
}

#[test]
fn multiple_markdown_list_items_require_matching_html_list_item_count() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><ul><li>First</li></ul></main>"#,
        "- First\n- Second\n- Third\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html preserves list item count",
    );
}

#[test]
fn multiple_markdown_code_fences_require_matching_html_code_count() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><pre><code>first</code></pre></main>"#,
        "```text\nfirst\n```\n\n```text\nsecond\n```\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html preserves code block count",
    );
}

#[test]
fn multiple_markdown_tables_require_matching_html_table_count() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><table><tr><td>One</td></tr></table></main>"#,
        "| One |\n| --- |\n| 1 |\n\n| Two |\n| --- |\n| 2 |\n",
    );

    assert_contains(&score.fatal_failures(), "Html: html preserves table count");
}

#[test]
fn language_code_fence_requires_syntax_highlighted_html() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><pre data-kdv-code-role="plain"><code>let x = 1;</code></pre></main>"#,
        "```rust\nlet x = 1;\n```\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html renders syntax highlighted code",
    );
}

#[test]
fn details_source_requires_accordion_html_contract() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>Title Body</p></main>"#,
        "<details><summary>Title</summary><div>Body</div></details>\n",
    );

    assert_contains(
        &score.fatal_failures(),
        "Html: html renders details accordion",
    );
}

fn assert_contains(failures: &[String], expected: &str) {
    assert!(
        failures.iter().any(|failure| failure == expected),
        "{failures:#?}"
    );
}
