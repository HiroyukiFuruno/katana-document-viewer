use super::super::contract_test_support::HtmlContractTestSupport;

#[test]
fn red_detects_export_stylesheet_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "> 引用\n\n| 短い | 長い |\n| --- | --- |\n| ID | text |\n";
    let html = HtmlContractTestSupport::export_html(markdown)?;
    assert_export_table_base_styles(&html);
    assert_export_table_column_styles(&html);
    assert_export_block_styles(&html);
    assert_export_task_list_styles(&html);
    assert_export_task_visual_styles(&html);
    Ok(())
}

fn assert_export_table_base_styles(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            ("style block", "<style data-kdv-export-style>"),
            (
                "table width",
                r#"[data-kdv-table="katana"]{width:100%;border-collapse:collapse;table-layout:fixed;"#,
            ),
            (
                "table border",
                r#"[data-kdv-table="katana"] th,[data-kdv-table="katana"] td{border:1px solid"#,
            ),
            (
                "table header background",
                r#"[data-kdv-table="katana"] th{background:var(--kdv-table-header);"#,
            ),
            (
                "table even row background",
                r#"[data-kdv-table="katana"] tbody tr:nth-child(even) td{background:var(--kdv-table-even);"#,
            ),
        ],
    );
}

fn assert_export_table_column_styles(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "short column col width",
                r#"col[data-kdv-column-size="short"]{width:12em;}"#,
            ),
            (
                "short column cell max width",
                r#"th[data-kdv-column-size="short"],td[data-kdv-column-size="short"]{width:12em;max-width:12em;"#,
            ),
        ],
    );
}

fn assert_export_block_styles(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "blockquote border",
                "blockquote{border-left:4px solid var(--kdv-quote-border);",
            ),
            (
                "alert note color",
                r#"[data-github-alert="NOTE"]{border-left-color:var(--kdv-alert-note);"#,
            ),
        ],
    );
}

fn assert_export_task_list_styles(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "task list marker hidden",
                r#"li[data-kdv-task-item="true"]{list-style:none;"#,
            ),
            (
                "custom task native input hidden",
                r#"li[data-kdv-task-item="true"]>input{position:absolute;opacity:0;width:1px;height:1px;margin:0;}"#,
            ),
            (
                "katana task active background",
                "--kdv-task-active-bg:#add6ff;",
            ),
            (
                "katana task empty background",
                "--kdv-task-empty-bg:#f3f3f3;",
            ),
            ("katana task active accent", "--kdv-task-done:#0078d4;"),
        ],
    );
}

fn assert_export_task_visual_styles(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "custom task visual box",
                r#"span[data-kdv-task-visual]{display:inline-flex;align-items:center;justify-content:center;width:.85em;height:.85em;"#,
            ),
            (
                "custom task slash",
                r#"span[data-kdv-task-visual="in-progress-slash"]::before{content:"";width:.72em;border-top:.14em solid currentColor;transform:rotate(-45deg);}"#,
            ),
        ],
    );
}

#[test]
fn red_detects_katana_table_layout_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = r##"| 短い | 長いカラムのテスト | 短い |
| --- | --- | --- |
| ID | このテキストは非常に長い行です。ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 | 備考 |
"##;
    let html = HtmlContractTestSupport::export_html(markdown)?;

    HtmlContractTestSupport::assert_contains_all(
        &html,
        &[
            ("katana table", r#"<table data-kdv-table="katana">"#),
            ("colgroup", "<colgroup>"),
            ("short left column", r#"<col data-kdv-column-size="short">"#),
            ("wide middle column", r#"<col data-kdv-column-size="wide">"#),
            (
                "short right column",
                r#"<td data-align="unspecified" data-kdv-column-size="short">備考</td>"#,
            ),
            (
                "wide cell",
                r#"<td data-align="unspecified" data-kdv-column-size="wide">このテキストは非常に長い行です。"#,
            ),
        ],
    );
    Ok(())
}

#[test]
fn red_detects_malformed_pipe_sentence_table_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "↑ 「English | 日本語」が中央揃えの同一行に表示されること。\n";
    let html = HtmlContractTestSupport::export_html(markdown)?;

    HtmlContractTestSupport::assert_contains_all(
        &html,
        &[(
            "pipe sentence paragraph",
            "<p>↑ 「English | 日本語」が中央揃えの同一行に表示されること。</p>",
        )],
    );
    assert!(
        !html.contains("<table data-kdv-table=\"katana\">"),
        "single pipe sentence must not be rendered as table: {html}"
    );
    Ok(())
}

#[test]
fn red_detects_diagram_light_theme_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html("```mermaid\ngraph TD\n  A --> B\n```\n")?;

    HtmlContractTestSupport::assert_contains_all(
        &html,
        &[(
            "diagram light theme marker",
            r#"<figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg"#,
        )],
    );
    Ok(())
}
