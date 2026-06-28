use super::HtmlQualityScore;

#[test]
fn html_alignment_source_requires_body_alignment_markup() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style>p{text-align:center}</style><p>Centered</p></main>"#,
        r#"<p align="center">Centered</p>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html preserves html alignment".to_string()),
        "{score:#?}"
    );
}

#[test]
fn html_alignment_source_accepts_preserved_align_attribute() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p align="center">Centered</p></main>"#,
        r#"<p align="center">Centered</p>"#,
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn html_alignment_source_accepts_single_quoted_uppercase_right_alignment() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><P ALIGN='RIGHT'>Right</P></main>"#,
        r#"<P ALIGN='RIGHT'>Right</P>"#,
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn html_alignment_source_accepts_inline_style_left_alignment() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p style="text-align:left">Left</p></main>"#,
        r#"<p style="text-align:left">Left</p>"#,
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}
