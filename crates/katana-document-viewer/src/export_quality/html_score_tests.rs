use super::{HtmlQualityScore, requires_runtime};

#[test]
fn source_without_optional_semantics_scores_html_as_complete() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><h1>Plain</h1></main>"#,
        "# Plain\n",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}

#[test]
fn source_with_diagram_requires_rendered_external_block() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid"><pre>graph TD</pre></figure></main>"#,
        "```mermaid\ngraph TD\n```\n",
    );

    assert!(score.score < 100);
    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html embeds render runtime".to_string())
    );
}

#[test]
fn source_with_diagram_accepts_rendered_svg_figure() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg><g><path d="M0 0"></path></g></svg></figure></main>"#,
        "```mermaid\ngraph TD\n```\n",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty());
}

#[test]
fn source_with_diagram_rejects_empty_placeholder_svg_figure() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg></svg></figure></main>"#,
        "```mermaid\ngraph TD\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html embeds render runtime".to_string())
    );
}

#[test]
fn source_with_diagram_rejects_runtime_error_raw_payload() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-render-runtime="katana-render-runtime" data-kdv-render-error="runtime-failed"><pre>graph TD</pre></figure></main>"#,
        "```mermaid\ngraph TD\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html has no render errors".to_string())
    );
}

#[test]
fn source_with_diagram_rejects_runtime_error_attribute_case_variation() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-render-runtime="katana-render-runtime" data-KDV-render-error="runtime-failed"><pre>graph TD</pre></figure></main>"#,
        "```mermaid\ngraph TD\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html has no render errors".to_string())
    );
}

#[test]
fn empty_fence_info_does_not_require_runtime_marker() {
    assert!(!requires_runtime("```\nplain\n```\n"));
}
