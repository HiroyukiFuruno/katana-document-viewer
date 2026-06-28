use super::*;

#[test]
fn anchor_with_no_boundary_is_skipped() {
    assert!(!html_contains_anchor_with_href(
        "<afoo href=\"https://example.com\"></afoo>"
    ));
}

#[test]
fn anchor_with_boundary_is_detected() {
    assert!(html_contains_anchor_with_href(
        "<a href=\"https://example.com\"></a>"
    ));
}

#[test]
fn tag_starts_with_boundary_requires_space_or_close() {
    assert!(tag_starts_with_boundary(" target"));
    assert!(tag_starts_with_boundary(">"));
    assert!(!tag_starts_with_boundary("href"));
}

#[test]
fn contains_delimited_inline_requires_a_complete_pair() {
    assert!(contains_delimited_inline("**strong**", "**"));
    assert!(!contains_delimited_inline("**strong", "**"));
    assert!(!contains_delimited_inline("**  **", "**"));
}

#[test]
fn single_delimiter_requires_non_empty_inside_content() {
    assert!(contains_single_delimited_inline("*emphasis*", '*'));
    assert!(!contains_single_delimited_inline("* *", '*'));
    assert!(!contains_single_delimited_inline("**", '*'));
    assert!(!contains_single_delimited_inline("*star", '*'));
}

#[test]
fn single_delimiter_has_closing_detects_boundary_and_content() {
    assert!(single_delimiter_has_closing("*em*", 1, '*'));
    assert!(!single_delimiter_has_closing("*  *", 1, '*'));
}

#[test]
fn touches_same_delimiter_detects_left_or_right_neighbor() {
    let bytes = b"*em*";
    assert!(touches_same_delimiter(bytes, 1, '*'));
    assert!(touches_same_delimiter(bytes, 2, '*'));
}

#[test]
fn source_outside_fences_ignores_markdown_text_inside_fences() {
    let lines = source_lines_outside_fences("```text\n**strong**\n```\nnormal");
    assert_eq!(vec!["normal"], lines);
}
