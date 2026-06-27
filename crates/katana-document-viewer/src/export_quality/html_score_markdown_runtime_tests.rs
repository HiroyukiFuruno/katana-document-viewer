use super::{HtmlQualityScore, requires_runtime};

#[test]
fn latex_fence_requires_rendered_runtime() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><pre>```latex
x = 1
```</pre></main>"#,
        "```latex\nx = 1\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html embeds render runtime".to_string()),
        "{score:#?}"
    );
    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html hides raw markdown".to_string()),
        "{score:#?}"
    );
}

#[test]
fn latex_fence_accepts_rendered_runtime_svg() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><svg data-kdv-render-runtime="katana-render-runtime"><text>x = 1</text></svg></main>"#,
        "```latex\nx = 1\n```\n",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn mermaid_dollar_text_does_not_require_math_runtime() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg><text>Revenue</text></svg></figure></main>"#,
        "```mermaid\nxychart-beta\ny-axis \"Revenue (in $)\" 4000 --> 11000\n```\n",
    );

    assert_eq!(100, score.score);
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

#[test]
fn inline_math_requires_runtime() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><p>Inline x + 1 math.</p></main>"#,
        "Inline $x + 1$ math.",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html embeds render runtime".to_string()),
        "{score:#?}"
    );
}

#[test]
fn escaped_or_unclosed_dollar_does_not_require_runtime() {
    assert!(!requires_runtime(r"Price is \$100."));
    assert!(!requires_runtime("Revenue (in $)."));
}

#[test]
fn raw_fence_with_space_after_marker_is_rejected() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><pre>``` mermaid
graph TD
```</pre></main>"#,
        "``` mermaid\ngraph TD\n```\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html hides raw markdown".to_string()),
        "{score:#?}"
    );
}

#[test]
fn raw_tilde_fence_with_space_after_marker_is_rejected() {
    let score = HtmlQualityScore::score(
        br#"<main data-kdv-export><style data-kdv-export-style></style><pre>~~~ plantuml
@startuml
@enduml
~~~</pre></main>"#,
        "~~~ plantuml\n@startuml\n@enduml\n~~~\n",
    );

    assert!(
        score
            .fatal_failures()
            .contains(&"Html: html hides raw markdown".to_string()),
        "{score:#?}"
    );
}
