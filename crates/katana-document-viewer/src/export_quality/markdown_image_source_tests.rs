use super::markdown_image_source::MarkdownImageSource;

#[test]
fn inline_markdown_image_requires_closing_label_bracket() {
    let definitions = vec!["logo".to_string()];
    assert!(!MarkdownImageSource::line_contains_markdown_image(
        "![broken",
        &definitions
    ));
}

#[test]
fn reference_markdown_image_requires_closing_reference_bracket() {
    let definitions = vec!["logo".to_string()];
    assert!(!MarkdownImageSource::line_contains_markdown_image(
        "![label][logo",
        &definitions
    ));
}

#[test]
fn inline_markdown_image_without_target_allows_next_rules_without_match() {
    let definitions = vec!["logo".to_string()];
    assert!(!MarkdownImageSource::line_contains_markdown_image(
        "![label]",
        &definitions
    ));
}

#[test]
fn reference_markdown_image_with_hidden_label_is_not_detected() {
    let definitions = vec!["hidden".to_string()];
    assert!(!MarkdownImageSource::line_contains_markdown_image(
        "![^hidden][hidden]",
        &definitions
    ));
}

#[test]
fn reference_definition_extracts_normalized_label() {
    let source = "[foo]: /path/image.png\n! [foo]: /unused\n[logo]: /path/logo.png";
    let definitions = MarkdownImageSource::reference_definitions(source);
    assert!(definitions.contains(&"foo".to_string()));
    assert!(definitions.contains(&"logo".to_string()));
    assert!(!definitions.contains(&"".to_string()));
}
