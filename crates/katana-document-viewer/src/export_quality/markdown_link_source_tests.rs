use super::markdown_link_source::MarkdownLinkSource;

#[test]
fn inline_markdown_link_requires_closing_parenthesis() {
    let source = "See [top";
    assert!(!MarkdownLinkSource::contains_markdown_link(source));
}

#[test]
fn reference_markdown_link_without_closing_reference_is_not_detected() {
    let source = "See [top][section";
    assert!(!MarkdownLinkSource::contains_markdown_link(source));
}

#[test]
fn shortcut_markdown_link_requires_definition() {
    let source = "[section]\n\n[section]: https://example.com";
    assert!(MarkdownLinkSource::contains_markdown_link(source));
}

#[test]
fn shortcut_markdown_link_without_definition_is_not_detected() {
    let source = "[section]\n\ntext";
    assert!(!MarkdownLinkSource::contains_markdown_link(source));
}

#[test]
fn autolink_is_detected_when_uri_is_valid() {
    assert!(MarkdownLinkSource::contains_markdown_link(
        "<https://example.com>"
    ));
}

#[test]
fn malformed_reference_without_opening_label_is_not_detected() {
    let source = "broken][section]\n\n[section]: https://example.com";
    assert!(!MarkdownLinkSource::contains_markdown_link(source));
}

#[test]
fn malformed_inline_link_without_opening_label_is_not_detected() {
    assert!(!MarkdownLinkSource::contains_markdown_link(
        "broken](https://example.com)"
    ));
}
