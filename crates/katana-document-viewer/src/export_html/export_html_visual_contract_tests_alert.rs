use super::super::contract_test_support::HtmlContractTestSupport;

#[test]
fn red_detects_katana_task_checkbox_visual_contract_gaps() -> Result<(), Box<dyn std::error::Error>>
{
    let markdown = "- [x] 完了\n- [ ] 未完了\n- [-] 横棒\n- [/] 進行中\n";
    let html = HtmlContractTestSupport::export_html(markdown)?;

    HtmlContractTestSupport::assert_contains_all(
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
                r#"<li data-kdv-task-item="true"><input type="checkbox" disabled data-kdv-task-marker="[-]" data-kdv-task-state="blocked" aria-checked="mixed"><span data-kdv-task-visual="blocked-dash" aria-hidden="true"></span>"#,
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
fn red_detects_quote_and_alert_visual_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(quote_and_alert_markdown())?;
    assert_quote_and_alert_contract(&html);
    assert_legacy_note_is_plain_quote(&html);
    Ok(())
}

fn quote_and_alert_markdown() -> &'static str {
    r##"> 引用です

> **Note**
> 旧形式の補足です

> [!WARNING]
> 注意です
"##
}

fn assert_quote_and_alert_contract(html: &str) {
    assert_quote_and_alert_contains_contract(html);
    assert_alert_panel_style_is_not_filled(html);
}

fn assert_quote_and_alert_contains_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
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
}

fn assert_alert_panel_style_is_not_filled(html: &str) {
    assert!(
        !html.contains("background:var(--kdv-alert-bg)"),
        "GFM alert must use KatanA left-rule style, not a filled panel: {html}"
    );
}

fn assert_legacy_note_is_plain_quote(html: &str) {
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
}

#[test]
fn red_detects_distinct_gfm_alert_icon_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(gfm_alert_icon_markdown())?;
    assert_gfm_alert_icon_contract(&html);
    assert_gfm_alert_title_style_contract(&html);
    Ok(())
}

fn gfm_alert_icon_markdown() -> &'static str {
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
"##
}

fn assert_gfm_alert_icon_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
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
        ],
    );
}

fn assert_gfm_alert_title_style_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[(
            "tip title color",
            r#"[data-kdv-alert-title="TIP"]{color:var(--kdv-alert-tip);"#,
        )],
    );
}
