use super::{HtmlAttributeScanner, HtmlImgSourceQuality};

#[test]
fn malformed_img_without_closing_bracket_is_not_accepted() {
    assert!(!HtmlImgSourceQuality::has_uri(
        "<img src='asset.png'",
        "asset.png"
    ));
}

#[test]
fn scanner_skips_boolean_attributes_before_matching_src() {
    let scanner = HtmlAttributeScanner::new("<img disabled loading=lazy src=asset.png>");

    assert_eq!(scanner.value("src"), Some("asset.png"));
    assert_eq!(scanner.value("missing"), None);
}

#[test]
fn scanner_handles_empty_quoted_and_unquoted_attribute_boundaries() {
    assert_eq!(
        HtmlAttributeScanner::new("<img src=").value("src"),
        Some("")
    );
    assert_eq!(
        HtmlAttributeScanner::new("<img src='unterminated").value("src"),
        Some("unterminated")
    );
    assert_eq!(
        HtmlAttributeScanner::new("<img src=asset.png>").value("src"),
        Some("asset.png")
    );
    assert_eq!(
        HtmlAttributeScanner::new("<img src=asset.png").value("src"),
        Some("asset.png")
    );
    assert_eq!(
        HtmlAttributeScanner::new("<img src=asset.png").value("missing"),
        None
    );
}

#[test]
fn scanner_matches_quoted_attributes_case_insensitively() {
    let scanner = HtmlAttributeScanner::new("<IMG SRC=\"asset.png\" ALT='preview'>");

    assert_eq!(scanner.value("src"), Some("asset.png"));
    assert_eq!(scanner.value("alt"), Some("preview"));
}
