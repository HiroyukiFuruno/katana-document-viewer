use super::super::contract_test_support::HtmlContractTestSupport;

#[test]
fn red_detects_details_markdown_body_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(
        "<details><summary>詳細を見る</summary><div>\n\n- 刀\n  - 孫六兼元\n  - 菊一文字則宗\n  - 備前長船長義\n\n</div></details>\n",
    )?;

    HtmlContractTestSupport::assert_contains_all(
        &html,
        &[
            (
                "details semantic wrapper",
                r#"<details data-kdv-accordion="true" open><summary>詳細を見る</summary><div data-kdv-accordion-body>"#,
            ),
            ("details list root", "<ul><li>刀<ul>"),
            ("details nested item", "<li>孫六兼元</li>"),
            (
                "details body spacing",
                r#"details[data-kdv-accordion]>[data-kdv-accordion-body]{margin-top:.75rem;}"#,
            ),
        ],
    );
    assert!(
        !html.contains("\n- 刀") && !html.contains("- 孫六兼元"),
        "details body must render Markdown instead of raw list markers: {html}"
    );
    Ok(())
}

#[test]
fn red_detects_nested_list_marker_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(nested_list_markdown())?;
    assert_nested_list_body(&html);
    assert_nested_list_accordion_body(&html);
    assert_nested_list_marker_style(&html)?;
    Ok(())
}

fn nested_list_markdown() -> &'static str {
    "- 外側1\n  - 内側2\n    - 最深3\n- 外側4\n\n<details><summary>詳細リスト</summary><div>\n\n- A\n  - B\n    - C\n\n</div></details>\n"
}

fn assert_nested_list_body(html: &str) {
    assert!(
        html.contains("<li>外側1<ul>")
            && html.contains("<li>内側2<ul>")
            && html.contains("<li>最深3</li>")
            && html.contains("<li>外側4</li>"),
        "nested list should preserve structure and depth"
    );
}

fn assert_nested_list_accordion_body(html: &str) {
    assert!(
        html.contains(
            r#"<details data-kdv-accordion="true" open><summary>詳細リスト</summary><div data-kdv-accordion-body>"#,
        ) &&
            html.contains("<li>B") &&
            html.contains("<li>C</li>") &&
            html.contains("<li>A"),
        "nested list should be preserved inside accordion body"
    );
}

fn assert_nested_list_marker_style(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let style = HtmlContractTestSupport::extract_export_style(html)
        .ok_or("export style block must exist")?;
    let has_explicit_depth_marker_contract =
        style.contains("ul ul") && style.contains("list-style-type");
    let has_default_marker_contract =
        !style.contains("ul{list-style:none;") && !style.contains("ol{list-style:none;");
    assert!(
        has_explicit_depth_marker_contract || has_default_marker_contract,
        "nested list marker contract should either define depth styles or keep default list markers"
    );
    Ok(())
}

#[test]
fn red_detects_html_exports_with_only_current_krr_naming() -> Result<(), Box<dyn std::error::Error>>
{
    let html = HtmlContractTestSupport::export_html(
        "`E = mc^2` を使う。[^1]\n\n[^1]: KDV/ KRR のテスト\n",
    )?;
    let lowered = html.to_lowercase();

    assert!(
        !lowered.contains("kdr") && !lowered.contains("katana-diagram-renderer"),
        "HTML output should not contain legacy renderer strings: {html}"
    );
    Ok(())
}

#[test]
fn red_detects_ordered_list_start_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(ordered_list_start_markdown())?;
    assert_ordered_list_start_contract(&html);
    Ok(())
}

fn ordered_list_start_markdown() -> &'static str {
    r##"1. 最初のステップ:

   ```sh
   cargo build --release
   ```

2. 次のステップ:

   ```sh
   ./target/release/KatanA
   ```

3. 確認:
   - サブ項目 A
   - サブ項目 B
"##
}

fn assert_ordered_list_start_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            ("first ordered list", "<ol><li>最初のステップ:"),
            (
                "second ordered list start",
                r#"<ol start="2"><li>次のステップ:"#,
            ),
            ("third ordered list start", r#"<ol start="3"><li>確認:"#),
        ],
    );
}
