use super::SurfaceDetailsParts;

#[test]
fn parses_details_summary_and_body_variants() {
    assert_details_body(
        "<details><summary>Summary</summary><div>Body text</div></details>",
        "Body text",
    );
    assert_details_body(
        "<details><summary>Summary</summary><div data-kdv-accordion-body>Body text</div></details>",
        "Body text",
    );
    assert_details_body(
        "<details>\n<summary>Summary</summary> Body only </details>",
        "Body only",
    );
}

fn assert_details_body(fragment: &str, expected_body: &str) {
    let parts = SurfaceDetailsParts::parse(fragment);
    assert!(parts.is_some(), "details block should be parseable");
    let Some(parts) = parts else {
        return;
    };
    assert_eq!(parts.summary, "Summary");
    assert_eq!(parts.body, expected_body);
}

#[test]
fn detail_body_keeps_raw_markup_when_not_wrapped_in_div() {
    assert_details_body(
        "<details><summary>Summary</summary><span>Body</span></details>",
        "<span>Body</span>",
    );
}

#[test]
fn detail_body_strips_div_without_closing_markup() {
    assert_details_body(
        "<details><summary>Summary</summary><div>Body</details>",
        "Body",
    );
}

#[test]
fn detail_body_without_div_prefix_is_trimmed_as_is() {
    assert_details_body(
        "<details><summary>Summary</summary><div Body</details>",
        "<div Body",
    );
}
