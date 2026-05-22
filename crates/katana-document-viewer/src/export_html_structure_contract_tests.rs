use super::contract_test_support::{assert_contains_all, export_html};

#[test]
fn red_detects_unbounded_nested_blockquote_contract_gaps() -> Result<(), Box<dyn std::error::Error>>
{
    let html = export_html("> 外側の引用\n> > 内側の引用\n> > > さらに内側\n> > > > 四段目\n")?;

    assert_contains_all(
        &html,
        &[
            (
                "nested blockquote",
                r#"<blockquote data-kdv-blockquote="quote" data-kdv-quote-depth="1"><p>外側の引用</p>"#,
            ),
            (
                "second blockquote",
                r#"<blockquote data-kdv-blockquote="quote" data-kdv-quote-depth="2"><p>内側の引用</p>"#,
            ),
            (
                "third blockquote",
                r#"<blockquote data-kdv-blockquote="quote" data-kdv-quote-depth="3"><p>さらに内側</p>"#,
            ),
            (
                "fourth blockquote",
                r#"<blockquote data-kdv-blockquote="quote" data-kdv-quote-depth="4"><p>四段目</p>"#,
            ),
            (
                "nested quote bottom aligned style",
                r#"blockquote[data-kdv-quote-depth] blockquote[data-kdv-quote-depth]{margin-top:.75rem;margin-bottom:0;}"#,
            ),
        ],
    );
    assert!(
        !html.contains("&gt; 内側の引用") && !html.contains("&gt; &gt; さらに内側"),
        "nested blockquote markers must not leak as text: {html}"
    );
    Ok(())
}

#[test]
fn red_detects_footnote_reference_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("本文です[^1]。追加です[^2]\n\n[^1]: 脚注本文。\n[^2]: 二番目。\n")?;

    assert_contains_all(
        &html,
        &[
            (
                "first footnote reference",
                r##"<sup id="fnref-1" data-kdv-footnote-ref="1"><a href="#fn-1">[1]</a></sup>"##,
            ),
            (
                "second footnote reference",
                r##"<sup id="fnref-2" data-kdv-footnote-ref="2"><a href="#fn-2">[2]</a></sup>"##,
            ),
        ],
    );
    Ok(())
}

#[test]
fn red_detects_footnote_section_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        "本文です[^1]。追加です[^2]\n\n# 後続見出し\n\n[^1]: 脚注本文。\n[^2]: 二番目。\n",
    )?;

    assert_contains_all(
        &html,
        &[
            ("footnote list section", r#"<section data-kdv-footnotes>"#),
            (
                "footnote definition",
                r#"<li id="fn-1" data-kdv-footnote-definition="1">"#,
            ),
            (
                "footnote back reference",
                r##"<a href="#fnref-1" data-kdv-footnote-backref="1">↩</a>"##,
            ),
        ],
    );
    assert!(
        html.find("<h1>後続見出し</h1>") < html.find(r#"<section data-kdv-footnotes>"#),
        "footnotes must be emitted after document body"
    );
    Ok(())
}

#[test]
fn red_detects_details_markdown_body_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        "<details><summary>詳細を見る</summary><div>\n\n- 刀\n  - 孫六兼元\n  - 菊一文字則宗\n  - 備前長船長義\n\n</div></details>\n",
    )?;

    assert_contains_all(
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
fn red_detects_ordered_list_start_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
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
"##,
    )?;

    assert_contains_all(
        &html,
        &[
            ("first ordered list", "<ol><li>最初のステップ:"),
            (
                "second ordered list start",
                r#"<ol start="2"><li>次のステップ:"#,
            ),
            ("third ordered list start", r#"<ol start="3"><li>確認:"#),
        ],
    );
    Ok(())
}
