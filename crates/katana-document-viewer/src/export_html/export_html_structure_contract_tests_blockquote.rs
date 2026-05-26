use super::super::contract_test_support::HtmlContractTestSupport;

#[test]
fn red_detects_unbounded_nested_blockquote_contract_gaps() -> Result<(), Box<dyn std::error::Error>>
{
    let markdown = "> 外側の引用\n> > 内側の引用\n> > > さらに内側の引用\n> > > > 四段目\n";
    let html = HtmlContractTestSupport::export_html(markdown)?;
    assert_nested_blockquote_contract(&html);
    assert_nested_blockquote_markers_hidden(&html);
    Ok(())
}

fn assert_nested_blockquote_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
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
                r#"<blockquote data-kdv-blockquote="quote" data-kdv-quote-depth="3"><p>さらに内側の引用</p>"#,
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
}

fn assert_nested_blockquote_markers_hidden(html: &str) {
    assert!(
        !html.contains("&gt; 内側の引用") && !html.contains("&gt; &gt; さらに内側"),
        "nested blockquote markers must not leak as text: {html}"
    );
}

#[test]
fn red_detects_footnote_reference_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "本文です[^1]。追加です[^2]\n\n[^1]: 脚注本文。\n[^2]: 二番目。\n";
    let html = HtmlContractTestSupport::export_html(markdown)?;

    HtmlContractTestSupport::assert_contains_all(
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
    let markdown =
        "本文です[^1]。追加です[^2]\n\n# 後続見出し\n\n[^1]: 脚注本文。\n[^2]: 二番目。\n";
    let html = HtmlContractTestSupport::export_html(markdown)?;
    assert_footnote_section_contract(&html);
    assert_footnote_section_after_body(&html);
    Ok(())
}

fn assert_footnote_section_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            ("footnote list section", r#"<section data-kdv-footnotes>"#),
            (
                "footnote definition",
                r#"<li id="fn-1" data-kdv-footnote-definition="1">"#,
            ),
            (
                "second footnote definition",
                r#"<li id="fn-2" data-kdv-footnote-definition="2">"#,
            ),
            (
                "footnote back reference",
                r##"<a href="#fnref-1" data-kdv-footnote-backref="1">↩</a>"##,
            ),
            (
                "second footnote back reference",
                r##"<a href="#fnref-2" data-kdv-footnote-backref="2">↩</a>"##,
            ),
        ],
    );
}

fn assert_footnote_section_after_body(html: &str) {
    assert!(
        html.find("<h1>後続見出し</h1>") < html.find(r#"<section data-kdv-footnotes>"#),
        "footnotes must be emitted after document body"
    );
}

#[test]
fn red_detects_footnote_target_highlight_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "本文です[^1]。追加です[^2]\n\n[^1]: 脚注本文。\n[^2]: 二番目。\n";
    let html = HtmlContractTestSupport::export_html(markdown)?;

    HtmlContractTestSupport::assert_contains_all(
        &html,
        &[
            (
                "footnote target highlight",
                "li[data-kdv-footnote-definition]:target{background:",
            ),
            (
                "footnote target scroll margin",
                "li[data-kdv-footnote-definition]:target{",
            ),
        ],
    );
    assert_footnote_target_css(&html)?;
    Ok(())
}

fn assert_footnote_target_css(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target_css = footnote_target_css(html)?;
    assert!(target_css.contains("background:var(--kdv-table-even);"));
    assert!(target_css.contains("scroll-margin-top:1rem;"));
    Ok(())
}

fn footnote_target_css(html: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let start = html
        .find("li[data-kdv-footnote-definition]:target{")
        .ok_or("footnote target css rule should be present")?;
    let tail = &html[start..];
    let end = tail.find('}').ok_or("footnote target css should close")?;
    Ok(&html[start..start + end + 1])
}
