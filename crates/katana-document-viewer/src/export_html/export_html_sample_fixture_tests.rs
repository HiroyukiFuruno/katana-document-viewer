use super::contract_test_support::HtmlContractTestSupport;

const SAMPLE_JA_MD: &str = include_str!("../../fixtures/rendering/sample.ja.md");

#[test]
fn sample_ja_fixture_is_exported_from_repo_local_copy() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(SAMPLE_JA_MD)?;
    assert_sample_fixture_contains_inline_contracts(&html);
    assert_sample_fixture_contains_export_contracts(&html);
    assert_sample_fixture_hides_raw_markers(&html);
    Ok(())
}

#[test]
fn sample_ja_fixture_with_crlf_is_exported_from_repo_local_copy()
-> Result<(), Box<dyn std::error::Error>> {
    let markdown = SAMPLE_JA_MD.replace('\n', "\r\n");
    let html = HtmlContractTestSupport::export_html(&markdown)?;
    assert_sample_fixture_contains_inline_contracts(&html);
    assert_sample_fixture_contains_export_contracts(&html);
    assert_sample_fixture_hides_raw_markers(&html);
    Ok(())
}

fn assert_sample_fixture_contains_inline_contracts(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "nested inline decoration",
                "<strong>太字と<em>イタリック</em>の混在</strong>",
            ),
            ("katana table layout", "table-layout:fixed"),
            (
                "short table column",
                r#"col[data-kdv-column-size="short"]{width:12em;}"#,
            ),
        ],
    );
}

fn assert_sample_fixture_contains_export_contracts(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "syntax highlighter",
                r#"data-kdv-code-highlighter="syntect""#,
            ),
            ("github alert", r#"data-kdv-blockquote="alert""#),
            ("diagram theme", r#"data-kdv-diagram-theme="light""#),
            ("footnote section", r#"data-kdv-footnotes"#),
            (
                "pipe sentence remains paragraph",
                "<p>↑ 「English | 日本語」が中央揃えの同一行に表示されること。</p>",
            ),
            (
                "legacy note heading",
                "<h2>7. Note ブロック（旧 <code>&gt; **Type**</code> 形式）</h2>",
            ),
            (
                "legacy note inline quote",
                "<blockquote data-kdv-blockquote=\"quote\"><p><strong>Note</strong> GitHub では note 系ブロックを blockquote として表現する。</p></blockquote>",
            ),
        ],
    );
}

fn assert_sample_fixture_hides_raw_markers(html: &str) {
    HtmlContractTestSupport::assert_not_contains_any(
        html,
        &[
            ("heading marker", "<h1>#"),
            ("quoted fence marker", "&gt; ```"),
            ("quoted code marker", "&gt; let"),
            (
                "malformed pipe sentence table",
                "<th data-align=\"unspecified\" data-kdv-column-size=\"wide\">↑ 「English</th>",
            ),
        ],
    );
}
