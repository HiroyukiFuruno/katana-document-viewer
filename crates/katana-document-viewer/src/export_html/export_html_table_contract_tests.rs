use super::contract_test_support::HtmlContractTestSupport;
use crate::KdvThemeSnapshot;

#[test]
fn red_detects_table_alignment_css_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(
        "| 左寄せ | 中央寄せ | 右寄せ |\n| :--- | :---: | ---: |\n| テキスト | テキスト | テキスト |\n| 長いテキスト | 短い | 12345 |\n",
    )?;
    assert_table_alignment_style_contract(&html);
    assert_table_alignment_cell_contract(&html);
    Ok(())
}

fn assert_table_alignment_style_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "left align css",
                r#"th[data-align="left"],td[data-align="left"]{text-align:left;}"#,
            ),
            (
                "center align css",
                r#"th[data-align="center"],td[data-align="center"]{text-align:center;}"#,
            ),
            (
                "right align css",
                r#"th[data-align="right"],td[data-align="right"]{text-align:right;}"#,
            ),
        ],
    );
}

fn assert_table_alignment_cell_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "left body cell",
                r#"<td data-align="left" data-kdv-column-size="short">長いテキスト</td>"#,
            ),
            (
                "center body cell",
                r#"<td data-align="center" data-kdv-column-size="short">短い</td>"#,
            ),
            (
                "right body cell",
                r#"<td data-align="right" data-kdv-column-size="short">12345</td>"#,
            ),
        ],
    );
}

#[test]
fn table_theme_css_variables_follow_active_export_theme() -> Result<(), Box<dyn std::error::Error>>
{
    let style = themed_table_export_style()?;
    assert_active_table_theme_style(&style);
    Ok(())
}

fn themed_table_export_style() -> Result<String, Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html_with_theme(
        "| A | B |\n| --- | --- |\n| C | D |\n| E | F |\n",
        active_table_theme(),
    )?;
    let style = HtmlContractTestSupport::extract_export_style(&html)
        .ok_or("export style block must exist")?;
    Ok(style.to_string())
}

fn active_table_theme() -> KdvThemeSnapshot {
    let mut theme = KdvThemeSnapshot::katana_dark();
    theme.table_border = "#112233".to_string();
    theme.table_header_background = "#223344".to_string();
    theme.table_even_row_background = "#334455".to_string();
    theme
}

fn assert_active_table_theme_style(style: &str) {
    HtmlContractTestSupport::assert_contains_all(
        style,
        &[
            ("table border var", "--kdv-table-border:#112233;"),
            ("table header var", "--kdv-table-header:#223344;"),
            ("table header text var", "--kdv-table-header-text:#d4d4d4;"),
            ("table even var", "--kdv-table-even:#334455;"),
            (
                "table border consumes var",
                r#"[data-kdv-table="katana"] th,[data-kdv-table="katana"] td{border:1px solid var(--kdv-table-border);"#,
            ),
            (
                "table header consumes var",
                r#"[data-kdv-table="katana"] th{background:var(--kdv-table-header);color:var(--kdv-table-header-text);"#,
            ),
            (
                "table even row consumes var",
                r#"[data-kdv-table="katana"] tbody tr:nth-child(even) td{background:var(--kdv-table-even);"#,
            ),
        ],
    );
}

#[test]
fn table_theme_css_variables_derive_from_generic_document_surface_when_table_tokens_are_default()
-> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html_with_theme(
        "| A | B |\n| --- | --- |\n| C | D |\n| E | F |\n",
        generic_document_theme_without_table_tokens(),
    )?;
    let style = HtmlContractTestSupport::extract_export_style(&html)
        .ok_or("export style block must exist")?;

    HtmlContractTestSupport::assert_contains_all(
        style,
        &[
            ("derived table border", "--kdv-table-border:#31475f;"),
            ("derived table header", "--kdv-table-header:#0078d4;"),
            (
                "derived table header text",
                "--kdv-table-header-text:#101820;",
            ),
            ("derived table even", "--kdv-table-even:#101820;"),
        ],
    );
    Ok(())
}

fn generic_document_theme_without_table_tokens() -> KdvThemeSnapshot {
    let mut theme = KdvThemeSnapshot::katana_light();
    theme.name = "generic-document".to_string();
    theme.background = "#101820".to_string();
    theme.text = "#f2f4f8".to_string();
    theme.code_background = "#162534".to_string();
    theme.code_border = "#31475f".to_string();
    theme
}

#[test]
fn red_detects_table_cell_markdown_and_short_column_width_gaps()
-> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(
        "| コンポーネント | 役割 |\n| --- | --- |\n| `PreviewPane` | セクション管理 |\n| `show_content` | UI描画 |\n",
    )?;

    HtmlContractTestSupport::assert_contains_all(
        &html,
        &[
            ("inline code in first row", "<code>PreviewPane</code>"),
            ("inline code in second row", "<code>show_content</code>"),
            (
                "short column width fits cjk",
                r#"col[data-kdv-column-size="short"]{width:12em;}"#,
            ),
            (
                "short cell width fits cjk",
                r#"th[data-kdv-column-size="short"],td[data-kdv-column-size="short"]{width:12em;max-width:12em;"#,
            ),
        ],
    );
    assert!(
        !html.contains("`PreviewPane`") && !html.contains("`show_content`"),
        "table cells must render inline Markdown instead of raw backticks: {html}"
    );
    Ok(())
}
