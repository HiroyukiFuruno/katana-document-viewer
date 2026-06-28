use super::HtmlQualityScore;

#[test]
fn plantuml_fence_rejects_mermaid_figure() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg><text>wrong</text></svg></figure></main>"#,
        "```plantuml\n@startuml\nA -> B\n@enduml\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html embeds render runtime".to_string()),
        "{score:#?}"
    );
}

#[test]
fn drawio_fence_rejects_mermaid_figure() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg><text>wrong</text></svg></figure></main>"#,
        "```drawio\n<mxfile />\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html embeds render runtime".to_string()),
        "{score:#?}"
    );
}

#[test]
fn multiple_diagram_fences_require_each_matching_kind() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg><text>one</text></svg></figure></main>"#,
        "```mermaid\ngraph TD\n```\n\n```plantuml\n@startuml\nA -> B\n@enduml\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html embeds render runtime".to_string()),
        "{score:#?}"
    );
}
