use super::HtmlQualityScore;

#[test]
fn direct_png_rejects_inline_svg_placeholder_that_mentions_uri() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg><title>file:///tmp/icon.png</title></svg></main>"#,
        "file:///tmp/icon.png",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn direct_png_rejects_uri_outside_img_src_attributes() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="placeholder.png" alt="file:///tmp/icon.png" data-source="file:///tmp/icon.png"></main>"#,
        "file:///tmp/icon.png",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn direct_png_accepts_case_insensitive_img_src_attribute() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img SRC = 'file:///tmp/icon.png' alt="icon"></main>"#,
        "file:///tmp/icon.png",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn direct_png_accepts_img_srcset_candidate() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img srcset="file:///tmp/icon.png 1x, file:///tmp/icon@2x.png 2x" alt="icon"></main>"#,
        "file:///tmp/icon.png",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn direct_svg_rejects_empty_inline_svg_placeholder_that_mentions_uri() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg data-source="file:///tmp/icon.svg"></svg></main>"#,
        "file:///tmp/icon.svg",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn direct_svg_accepts_rendered_inline_svg_with_matching_uri() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg data-source="file:///tmp/icon.svg"><rect width="1" height="1"></rect></svg></main>"#,
        "file:///tmp/icon.svg",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}
