use super::HtmlQualityScore;

#[test]
fn markdown_setext_heading_source_requires_html_heading() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>Title</p></main>"#,
        "Title\n=====\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders heading block".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_setext_heading_source_accepts_html_heading() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><h1>Title</h1></main>"#,
        "Title\n=====\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_setext_h2_source_does_not_require_thematic_break() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><h2>Title</h2></main>"#,
        "Title\n---\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_setext_like_marker_after_list_item_requires_thematic_break() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><ul><li>item</li></ul></main>"#,
        "- item\n---\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders thematic break".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_setext_like_marker_after_ordered_list_requires_thematic_break() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><ol><li>item</li></ol></main>"#,
        "1. item\n---\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders thematic break".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_setext_like_marker_after_atx_heading_requires_thematic_break() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><h1>Title</h1></main>"#,
        "# Title\n---\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders thematic break".to_string()),
        "{score:#?}"
    );
}
