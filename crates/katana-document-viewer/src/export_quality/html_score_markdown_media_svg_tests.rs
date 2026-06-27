use super::HtmlQualityScore;

#[test]
fn markdown_image_source_rejects_empty_svg_placeholder() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg></svg></main>"#,
        "![diagram](assets/diagram.svg)\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders markdown image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_image_source_accepts_rendered_inline_svg() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg><rect width="1" height="1"></rect></svg></main>"#,
        "![diagram](assets/diagram.svg)\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}
