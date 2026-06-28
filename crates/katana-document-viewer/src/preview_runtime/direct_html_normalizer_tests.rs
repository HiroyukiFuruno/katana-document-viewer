use super::DirectHtmlNormalizer;

#[test]
fn direct_html_is_split_into_top_level_blocks() {
    let normalized = DirectHtmlNormalizer::normalize(&source());

    assert!(!normalized.contains("<main>"));
    assert!(normalized.contains("<h1 align=\"center\">Title</h1>\n\n"));
    assert!(normalized.contains("<details open><summary>Details</summary><p>Body</p></details>"));
}

fn source() -> String {
    [
        "<main>",
        "<h1 align=\"center\">Title</h1>",
        "<details open>",
        "<summary>Details</summary>",
        "<p>Body</p>",
        "</details>",
        "</main>",
    ]
    .join("\n")
}
