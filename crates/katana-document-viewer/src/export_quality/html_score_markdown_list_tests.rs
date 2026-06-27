use super::HtmlQualityScore;

#[test]
fn parenthesized_ordered_list_with_tab_requires_html_list() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>item</p></main>"#,
        "1)\titem\n",
    );

    assert_contains(&score.fatal_failures(), "Html: html renders list block");
}

#[test]
fn ten_digit_ordered_marker_does_not_require_html_list() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>item</p></main>"#,
        "1234567890. item\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

fn assert_contains(failures: &[String], expected: &str) {
    assert!(
        failures.iter().any(|failure| failure == expected),
        "{failures:#?}"
    );
}
