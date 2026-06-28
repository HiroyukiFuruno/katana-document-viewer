use super::HtmlQualityScore;

#[test]
fn markdown_footnote_reference_must_not_leak_raw_marker() {
    let score = HtmlQualityScore::score(raw_footnote_html(), "body[^1]\n\n[^1]: note\n");

    assert!(
        score
            .fatal_failures()
            .iter()
            .any(|failure| failure == "Html: html hides raw markdown"),
        "{score:#?}"
    );
}

#[test]
fn markdown_incomplete_footnote_reference_is_not_treated_as_leak() {
    let score = HtmlQualityScore::score(
        br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>body[^1</p>
</main>
"#,
        "body[^1\n",
    );

    assert!(
        !score
            .fatal_failures()
            .contains(&"Html: html hides raw markdown".to_string()),
        "{score:#?}"
    );
}

fn raw_footnote_html() -> &'static [u8] {
    br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>body[^1]</p>
<section data-kdv-footnotes>
<ol><li data-kdv-footnote-definition="1">note</li></ol>
</section>
</main>
"#
}
