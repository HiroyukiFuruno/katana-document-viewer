use super::HtmlQualityScore;

#[test]
fn direct_diagrams_reject_empty_placeholder_svg_even_with_theme() {
    for (source, kind, expected) in [
        (
            "<mxfile><diagram /></mxfile>",
            "drawio",
            "Html: html renders direct Draw.io",
        ),
        (
            "file:///tmp/diagram.mermaid",
            "mermaid",
            "Html: html renders direct Mermaid",
        ),
        (
            "file:///tmp/diagram.puml",
            "plantuml",
            "Html: html renders direct PlantUML",
        ),
    ] {
        let html = format!(
            r#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="{kind}" data-kdv-diagram-theme="light"><svg></svg></figure></main>"#
        );
        let score = HtmlQualityScore::score(html.as_bytes(), source);

        assert!(
            score.fatal_failures().contains(&expected.to_string()),
            "{score:#?}"
        );
    }
}

#[test]
fn direct_drawio_rejects_placeholder_diagram_without_kdv_theme() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="drawio"><svg></svg></figure></main>"#,
        "<mxfile><diagram /></mxfile>",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct Draw.io".to_string())
    );
}

#[test]
fn direct_drawio_rejects_runtime_error_raw_payload() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="drawio" data-kdv-diagram-theme="light" data-kdv-render-runtime="katana-render-runtime" data-kdv-render-error="runtime-failed"><pre>&lt;mxfile&gt;</pre></figure></main>"#,
        "<mxfile><diagram /></mxfile>",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html has no render errors".to_string())
    );
}

#[test]
fn direct_mermaid_requires_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>flowchart TD</p></main>"#,
        "flowchart TD\nA --> B",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct Mermaid".to_string())
    );
}

#[test]
fn plain_text_starting_with_graph_word_is_not_direct_mermaid() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>graph theory notes</p></main>"#,
        "graph theory notes\n\nThis is a normal markdown paragraph.",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}

#[test]
fn graph_with_mermaid_direction_requires_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>graph TD</p></main>"#,
        "graph TD\nA --> B",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct Mermaid".to_string())
    );
}

#[test]
fn direct_mermaid_file_accepts_runtime_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg><path d="M0 0"></path></svg></figure></main>"#,
        "file:///tmp/diagram.mermaid",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}

#[test]
fn direct_plantuml_requires_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>@startuml</p></main>"#,
        "@startuml\nA -> B\n@enduml",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html renders direct PlantUML".to_string())
    );
}

#[test]
fn direct_plantuml_file_accepts_runtime_diagram_media() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="plantuml" data-kdv-diagram-theme="light"><svg><text>PlantUML</text></svg></figure></main>"#,
        "file:///tmp/diagram.puml",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}
