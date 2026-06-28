use super::HtmlQualityScore;

#[test]
fn direct_mermaid_with_query_fragment_checks_direct_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "file:///tmp/diagram.mmd?cache=1#viewer",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct Mermaid".to_string())
    );
}

#[test]
fn direct_drawio_with_query_fragment_checks_direct_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "file:///tmp/diagram.drawio?cache=1#viewer",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct Draw.io".to_string())
    );
}

#[test]
fn direct_drowio_with_query_fragment_checks_direct_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "file:///tmp/diagram.drowio?cache=1#viewer",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct Draw.io".to_string())
    );
}

#[test]
fn direct_drowio_with_query_fragment_accepts_runtime_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="drawio" data-kdv-diagram-theme="light"><svg><rect width="1" height="1"></rect></svg></figure></main>"#,
        "file:///tmp/diagram.drowio?cache=1#viewer",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}

#[test]
fn direct_plantuml_with_query_fragment_checks_direct_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>diagram</p></main>"#,
        "file:///tmp/diagram.plantuml?cache=1#viewer",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct PlantUML".to_string())
    );
}

#[test]
fn direct_image_uppercase_query_fragment_checks_direct_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>image</p></main>"#,
        "file:///tmp/ICON.BMP?cache=1#viewer",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct image".to_string())
    );
}

#[test]
fn direct_image_uppercase_query_fragment_accepts_matching_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><img src="file:///tmp/ICON.BMP?cache=1#viewer" alt="icon"></main>"#,
        "file:///tmp/ICON.BMP?cache=1#viewer",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}
