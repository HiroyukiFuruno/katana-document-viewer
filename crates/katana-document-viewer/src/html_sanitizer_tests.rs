#[cfg(test)]
use super::HtmlFragmentNormalizer;

#[test]
fn preserves_valid_html_without_rewriting_attributes() {
    let fragment = r#"<img src="data:image/svg+xml,%3Csvg%3E%3C/svg%3E" alt="icon">"#;

    assert_eq!(fragment, HtmlFragmentNormalizer::normalize(fragment));
    assert!(!HtmlFragmentNormalizer::has_malformed_image_source_attribute(fragment));
}

#[test]
fn identifies_the_visible_tail_of_a_malformed_quoted_attribute() {
    let fragment = r#"<img src="custom:payload<broken> visible tail" alt="icon">"#;

    let tail = HtmlFragmentNormalizer::malformed_image_source_attribute_tail(fragment);

    assert_eq!(Some(" visible tail\" alt=\"icon\">"), tail);
}

#[test]
fn ignores_valid_non_source_quoted_angles_and_non_image_content() {
    for fragment in [
        r#"<img title="a < b > c" src="icon.svg">"#,
        r#"<div title='<img src="custom:payload<broken>z">'>ok</div>"#,
        r#"<!-- <img src="custom:payload<broken>"> -->"#,
        r#"<script>const sample = "custom:payload<broken>";</script>"#,
    ] {
        assert!(!HtmlFragmentNormalizer::has_malformed_image_source_attribute(fragment));
        assert_eq!(fragment, HtmlFragmentNormalizer::normalize(fragment));
    }
}

#[test]
fn normalizes_a_malformed_svg_namespace_without_matching_a_fixture_uri() {
    let fragment = r#"<img src="data:image/svg+xml,%3Csvg xmlns=%22<https://example.test/svg%22> width=%22128%22%3E" alt="icon">"#;

    let normalized = HtmlFragmentNormalizer::normalize(fragment);

    assert!(normalized.contains("xmlns=%22https%3A%2F%2Fexample.test%2Fsvg%22%20"));
    assert!(!normalized.contains("xmlns=%22<https://example.test/svg%22>"));
}

#[test]
fn accepts_boolean_and_unquoted_image_attributes_without_rewriting() {
    let fragment = r#"<img loading src=icon.svg alt="icon">"#;

    assert!(!HtmlFragmentNormalizer::has_malformed_image_source_attribute(fragment));
    assert_eq!(fragment, HtmlFragmentNormalizer::normalize(fragment));
}

#[test]
fn preserves_a_malformed_svg_namespace_that_cannot_be_repaired_safely() {
    let fragment =
        r#"<img src="data:image/svg+xml,%3Csvg xmlns=%22<bad namespace%22>%3C/svg%3E" alt="icon">"#;

    assert!(HtmlFragmentNormalizer::has_malformed_image_source_attribute(fragment));
    assert_eq!(fragment, HtmlFragmentNormalizer::normalize(fragment));
}
