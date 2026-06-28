use super::HtmlQualityScore;

#[test]
fn markdown_image_source_requires_html_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "![diagram](assets/diagram.png)\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders markdown image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_image_reference_source_requires_html_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "![diagram][diagram]\n\n[diagram]: assets/diagram.png\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders markdown image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_image_collapsed_reference_source_requires_html_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "![diagram][]\n\n[diagram]: assets/diagram.png\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders markdown image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_image_shortcut_reference_source_requires_html_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "![diagram]\n\n[diagram]: assets/diagram.png\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders markdown image".to_string()),
        "{score:#?}"
    );
}

#[test]
fn markdown_image_source_accepts_html_img() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="assets/diagram.png" alt="diagram"></main>"#,
        "![diagram](assets/diagram.png)\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_image_reference_source_accepts_html_img() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="assets/diagram.png" alt="diagram"></main>"#,
        "![diagram][diagram]\n\n[diagram]: assets/diagram.png\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_image_collapsed_reference_source_accepts_html_img() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="assets/diagram.png" alt="diagram"></main>"#,
        "![diagram][]\n\n[diagram]: assets/diagram.png\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn markdown_image_shortcut_reference_source_accepts_html_img() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="assets/diagram.png" alt="diagram"></main>"#,
        "![diagram]\n\n[diagram]: assets/diagram.png\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn fenced_image_reference_definition_does_not_require_html_media() {
    assert_fenced_image_reference_definition_does_not_require_media(
        "![diagram][diagram]\n\n```text\n[diagram]: assets/diagram.png\n```\n",
    );
}

#[test]
fn fenced_collapsed_image_reference_definition_does_not_require_html_media() {
    assert_fenced_image_reference_definition_does_not_require_media(
        "![diagram][]\n\n```text\n[diagram]: assets/diagram.png\n```\n",
    );
}

#[test]
fn fenced_shortcut_image_reference_definition_does_not_require_html_media() {
    assert_fenced_image_reference_definition_does_not_require_media(
        "![diagram]\n\n```text\n[diagram]: assets/diagram.png\n```\n",
    );
}

fn assert_fenced_image_reference_definition_does_not_require_media(source: &str) {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        source,
    );

    assert!(
        !score
            .fatal_failures()
            .contains(&"Html: html renders markdown image".to_string()),
        "{score:#?}"
    );
}
