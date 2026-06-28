use super::HtmlQualityScore;

#[test]
fn direct_file_image_requires_html_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style></main>"#,
        "file:///tmp/icon.png",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string())
    );
}

#[test]
fn direct_file_image_accepts_html_img() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="file:///tmp/icon.png" alt="icon"></main>"#,
        "file:///tmp/icon.png",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}

#[test]
fn direct_source_without_file_scheme_triggers_direct_image_check() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>/tmp/icon.png</p></main>"#,
        "/tmp/icon.png",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string())
    );
}

#[test]
fn direct_image_path_with_whitespace_rejects_raw_fallback() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>/tmp/kdv fixtures/icon sample.png</p></main>"#,
        "/tmp/kdv fixtures/icon sample.png",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string())
    );
}

#[test]
fn direct_file_image_rejects_unrelated_placeholder_img() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="placeholder.png" alt="icon"></main>"#,
        "file:///tmp/icon.png",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string())
    );
}

#[test]
fn direct_raw_svg_requires_html_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style>&lt;svg&gt;&lt;/svg&gt;</main>"#,
        r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#,
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string())
    );
}

#[test]
fn direct_drawio_requires_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style>&lt;mxfile&gt;&lt;/mxfile&gt;</main>"#,
        "<mxfile><diagram /></mxfile>",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct Draw.io".to_string())
    );
}

#[test]
fn direct_drawio_accepts_runtime_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="drawio" data-kdv-diagram-theme="light"><svg><g data-kdv-rendered="drawio"><rect width="10" height="10"></rect></g></svg></figure></main>"#,
        "<mxfile><diagram /></mxfile>",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}
