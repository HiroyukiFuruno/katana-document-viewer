use super::contract_test_support::{assert_contains_all, export_html};

#[test]
fn red_detects_table_alignment_css_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        "| 左寄せ | 中央寄せ | 右寄せ |\n| :--- | :---: | ---: |\n| テキスト | テキスト | テキスト |\n| 長いテキスト | 短い | 12345 |\n",
    )?;

    assert_contains_all(
        &html,
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
    Ok(())
}

#[test]
fn red_detects_table_cell_markdown_and_short_column_width_gaps()
-> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        "| コンポーネント | 役割 |\n| --- | --- |\n| `PreviewPane` | セクション管理 |\n| `show_content` | UI描画 |\n",
    )?;

    assert_contains_all(
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
