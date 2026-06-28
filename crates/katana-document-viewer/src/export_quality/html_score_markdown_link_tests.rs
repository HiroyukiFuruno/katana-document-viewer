use super::HtmlQualityScore;

#[test]
fn fenced_markdown_link_source_does_not_require_html_link() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<pre data-kdv-code-role="plain"><code>[link](https://example.com)</code></pre>
</main>
"#;

    let score = HtmlQualityScore::score(html, "```text\n[link](https://example.com)\n```\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn fenced_reference_link_source_does_not_require_html_link() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<pre data-kdv-code-role="plain"><code>[link][id]</code></pre>
</main>
"#;

    let source = "```text\n[link][id]\n\n[id]: https://example.com\n```\n";

    let score = HtmlQualityScore::score(html, source);

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn unresolved_reference_link_source_does_not_require_html_link() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>[link][missing]</p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "[link][missing]\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn unresolved_collapsed_reference_link_source_does_not_require_html_link() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>[link][]</p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "[link][]\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn html_anchor_source_requires_html_anchor_output_even_with_uppercase_tags() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>no anchor</p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "<A HREF='https://example.com'>link</A>");

    assert!(
        score
            .fatal_failures()
            .iter()
            .any(|failure| failure == "Html: html evaluates inline markdown")
    );
}

#[test]
fn html_anchor_source_requires_html_anchor_output_even_with_newline_attribute() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>no anchor</p>
</main>
"#;

    let source = "<a\nhref=\"https://example.com\">link</a>";
    let score = HtmlQualityScore::score(html, source);

    assert!(
        score
            .fatal_failures()
            .iter()
            .any(|failure| failure == "Html: html evaluates inline markdown")
    );
}

#[test]
fn html_anchor_source_requires_html_anchor_output_with_attribute_order() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>no anchor</p>
</main>
"#;

    let source = "<a class=\"button\" href=\"https://example.com\">link</a>";
    let score = HtmlQualityScore::score(html, source);

    assert!(
        score
            .fatal_failures()
            .iter()
            .any(|failure| failure == "Html: html evaluates inline markdown")
    );
}

#[test]
fn data_href_only_html_anchor_source_does_not_require_html_anchor_output() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p>not a link</p>
</main>
"#;

    let source = r#"<a data-href="https://example.com">Not a link</a>"#;
    let score = HtmlQualityScore::score(html, source);

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_source_link_accepts_html_with_uppercase_anchor_tag() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p><A HREF='https://example.com'>Reference</A></p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "[link](https://example.com)\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_source_link_accepts_html_with_newline_attribute() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p><a
href='https://example.com'>Reference</a></p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "[link](https://example.com)\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_source_link_accepts_html_with_attribute_order() {
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<p><a class="button" href="https://example.com">Reference</a></p>
</main>
"#;

    let score = HtmlQualityScore::score(html, "[link](https://example.com)\n");

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}
