use super::contract_test_support::{assert_contains_all, export_html};

#[test]
fn red_detects_export_stylesheet_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("> 引用\n\n| 短い | 長い |\n| --- | --- |\n| ID | text |\n")?;

    assert_contains_all(
        &html,
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
            (
                "short column col width",
                r#"col[data-kdv-column-size="short"]{width:12em;}"#,
            ),
            (
                "short column cell max width",
                r#"th[data-kdv-column-size="short"],td[data-kdv-column-size="short"]{width:12em;max-width:12em;"#,
            ),
            (
                "blockquote border",
                "blockquote{border-left:4px solid var(--kdv-quote-border);",
            ),
            (
                "alert note color",
                r#"[data-github-alert="NOTE"]{border-left-color:var(--kdv-alert-note);"#,
            ),
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
    Ok(())
}

#[test]
fn red_detects_katana_task_checkbox_visual_contract_gaps() -> Result<(), Box<dyn std::error::Error>>
{
    let html = export_html("- [x] 完了\n- [ ] 未完了\n- [-] 横棒\n- [/] 進行中\n")?;

    assert_contains_all(
        &html,
        &[
            (
                "check item wrapper",
                r#"<li data-kdv-task-item="true"><input type="checkbox" disabled data-kdv-task-marker="[x]" data-kdv-task-state="done" checked><span data-kdv-task-visual="done-check" aria-hidden="true"></span>"#,
            ),
            (
                "todo item wrapper",
                r#"<li data-kdv-task-item="true"><input type="checkbox" disabled data-kdv-task-marker="[ ]" data-kdv-task-state="todo"><span data-kdv-task-visual="todo" aria-hidden="true"></span>"#,
            ),
            (
                "dash item wrapper",
                r#"<li data-kdv-task-item="true"><input type="checkbox" disabled data-kdv-task-marker="[-]" data-kdv-task-state="in-progress" aria-checked="mixed"><span data-kdv-task-visual="in-progress-dash" aria-hidden="true"></span>"#,
            ),
            (
                "progress item wrapper",
                r#"<li data-kdv-task-item="true"><input type="checkbox" disabled data-kdv-task-marker="[/]" data-kdv-task-state="in-progress" aria-checked="mixed"><span data-kdv-task-visual="in-progress-slash" aria-hidden="true"></span>"#,
            ),
        ],
    );
    Ok(())
}

#[test]
fn red_detects_katana_table_layout_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r##"| 短い | 長いカラムのテスト | 短い |
| --- | --- | --- |
| ID | このテキストは非常に長い行です。ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 | 備考 |
"##,
    )?;

    assert_contains_all(
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
    let html = export_html("↑ 「English | 日本語」が中央揃えの同一行に表示されること。\n")?;

    assert_contains_all(
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
fn red_detects_quote_and_alert_visual_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r##"> 引用です

> **Note**
> 旧形式の補足です

> [!WARNING]
> 注意です
"##,
    )?;

    assert_contains_all(
        &html,
        &[
            (
                "blockquote semantic marker",
                r#"<blockquote data-kdv-blockquote="quote">"#,
            ),
            (
                "legacy note normal blockquote",
                r#"<blockquote data-kdv-blockquote="quote"><p><strong>Note</strong> 旧形式の補足です</p></blockquote>"#,
            ),
            (
                "alert semantic marker",
                r#"<aside data-github-alert="WARNING" data-kdv-blockquote="alert">"#,
            ),
            (
                "warning alert color",
                r#"[data-github-alert="WARNING"]{border-left-color:var(--kdv-alert-warning);"#,
            ),
            (
                "warning alert title icon",
                r#"<p data-kdv-alert-title="WARNING"><span data-kdv-alert-icon="WARNING" aria-hidden="true"><svg data-kdv-alert-icon-svg="WARNING""#,
            ),
        ],
    );
    assert!(
        !html.contains(r#"<aside data-github-alert="NOTE" data-kdv-blockquote="alert">"#),
        "legacy note block must not be rendered as GFM alert: {html}"
    );
    assert!(
        !html.contains("data-kdv-legacy-note"),
        "legacy note block must not carry alert-like export attributes: {html}"
    );
    assert!(
        !html.contains("<p><strong>Note</strong></p><p>旧形式の補足です</p>"),
        "legacy note title and body must stay on the same quote line: {html}"
    );
    Ok(())
}

#[test]
fn red_detects_distinct_gfm_alert_icon_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r##"> [!NOTE]
> note

> [!TIP]
> tip

> [!IMPORTANT]
> important

> [!WARNING]
> warning

> [!CAUTION]
> caution
"##,
    )?;

    assert_contains_all(
        &html,
        &[
            (
                "note icon",
                r#"<span data-kdv-alert-icon="NOTE" aria-hidden="true"><svg data-kdv-alert-icon-svg="NOTE""#,
            ),
            (
                "tip icon",
                r#"<span data-kdv-alert-icon="TIP" aria-hidden="true"><svg data-kdv-alert-icon-svg="TIP""#,
            ),
            (
                "important icon",
                r#"<span data-kdv-alert-icon="IMPORTANT" aria-hidden="true"><svg data-kdv-alert-icon-svg="IMPORTANT""#,
            ),
            (
                "warning icon",
                r#"<span data-kdv-alert-icon="WARNING" aria-hidden="true"><svg data-kdv-alert-icon-svg="WARNING""#,
            ),
            (
                "caution icon",
                r#"<span data-kdv-alert-icon="CAUTION" aria-hidden="true"><svg data-kdv-alert-icon-svg="CAUTION""#,
            ),
            (
                "tip title color",
                r#"[data-kdv-alert-title="TIP"]{color:var(--kdv-alert-tip);"#,
            ),
        ],
    );
    Ok(())
}

#[test]
fn red_detects_diagram_light_theme_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("```mermaid\ngraph TD\n  A --> B\n```\n")?;

    assert_contains_all(
        &html,
        &[(
            "diagram light theme marker",
            r#"<figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg"#,
        )],
    );
    Ok(())
}
