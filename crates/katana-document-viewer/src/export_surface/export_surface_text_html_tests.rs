use super::SurfaceTextParser;

#[test]
fn html_fragment_text_uses_alt_attribute_and_tag_strip() {
    let alt_text = SurfaceTextParser::html_fragment_text("<img alt=\"A&B\" src=\"x\" />");
    assert_eq!(alt_text, "A&B");

    let stripped = SurfaceTextParser::html_fragment_text("<b> left</b><i> right</i>");
    let normalized = stripped.split_whitespace().collect::<Vec<_>>().join(" ");
    assert_eq!(normalized, "left right");
}

#[test]
fn markdown_decoder_handles_entities_and_link_targets() {
    assert_eq!(
        SurfaceTextParser::decode_basic_entities("A &lt; B &amp; C &gt; D\"'"),
        "A < B & C > D\"'",
    );

    let cleaned = SurfaceTextParser::remove_link_targets("text [l1](u1) and [l2](u2)");
    assert_eq!(cleaned, "text l1 and l2");

    let malformed = SurfaceTextParser::remove_link_targets("text [broken(u2)");
    assert_eq!(malformed, "text [broken(u2)");

    let missing_target = SurfaceTextParser::remove_link_targets("text [label] without target");
    assert_eq!(missing_target, "text [label] without target");

    let attrs =
        SurfaceTextParser::extract_attribute_values("<img alt=\"one\" alt=\"two\"/>", "alt");
    assert_eq!(attrs, vec!["one".to_string(), "two".to_string()]);
}

#[test]
fn html_fragment_text_falls_back_to_stripped_text_without_alt_attribute() {
    assert_eq!(
        SurfaceTextParser::html_fragment_text("<span> A <b>text</b></span>")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
        "A text",
    );
}

#[test]
fn html_fragment_text_ignores_gt_inside_quoted_attributes() {
    let text = SurfaceTextParser::html_fragment_text(
        "<p><img src=\"data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22> width=%2216%22%3E\"></p>",
    );

    assert!(text.is_empty());
}

#[test]
fn extract_attribute_values_stops_on_missing_end_quote() {
    let attrs =
        SurfaceTextParser::extract_attribute_values("<img src=\"https://example.com/>", "src");

    assert!(attrs.is_empty());
}

#[test]
fn decode_basic_entities_handles_all_supported_tokens() {
    assert_eq!(
        SurfaceTextParser::decode_basic_entities("&amp;&lt;&gt;&quot;&#39;"),
        "&<>\"'",
    );
}

#[test]
fn inline_markdown_text_keeps_text_without_markup_targets() {
    let text = "plain [a](b) `code`";

    assert_eq!(
        SurfaceTextParser::inline_markdown_text(text),
        "plain a code"
    );
}
