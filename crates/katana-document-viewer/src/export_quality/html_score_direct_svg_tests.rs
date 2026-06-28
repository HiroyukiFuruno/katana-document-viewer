use super::HtmlQualityScore;

#[test]
fn direct_raw_svg_rejects_unrelated_placeholder_svg() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg><title>placeholder</title></svg></main>"#,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><rect id="actual-shape"/></svg>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn direct_raw_svg_accepts_matching_inline_svg_signature() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg><rect id="actual-shape"/></svg></main>"#,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><rect id="actual-shape"/></svg>"#,
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}

#[test]
fn direct_raw_svg_without_unique_token_rejects_bare_placeholder_svg() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg></svg></main>"#,
        r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn direct_raw_svg_without_unique_token_accepts_matching_opening_tag() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg xmlns="http://www.w3.org/2000/svg"></svg></main>"#,
        r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#,
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}
