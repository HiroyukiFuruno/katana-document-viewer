use super::rendered_diagram_figure;

#[test]
fn diagram_marker_without_figure_is_not_rendered() {
    assert!(!rendered_diagram_figure(
        r#"<div data-kdv-diagram="mermaid"><svg></svg></div>"#,
        "mermaid"
    ));
}
