use super::DirectHtmlTableNormalizer;

#[test]
fn html_table_becomes_gfm_table() {
    let raw = [
        "<table>",
        "<thead><tr><th>Feature</th><th>Status</th></tr></thead>",
        "<tbody><tr><td>HTML alignment</td><td>covered</td></tr></tbody>",
        "</table>",
    ]
    .join("\n");

    assert_eq!(
        [
            "| Feature | Status |",
            "| --- | --- |",
            "| HTML alignment | covered |"
        ]
        .join("\n"),
        DirectHtmlTableNormalizer::normalize(&raw)
    );
}

#[test]
fn unparsable_table_returns_raw() {
    let raw = "<table><tbody></tbody></table>";

    assert_eq!(raw, DirectHtmlTableNormalizer::normalize(raw));
}

#[test]
fn table_with_missing_row_closure_returns_raw() {
    let raw = "<table><tr><th>A</th><td>1";

    assert_eq!(raw, DirectHtmlTableNormalizer::normalize(raw));
}

#[test]
fn table_with_missing_row_opening_marker_returns_raw() {
    let raw = "<table><tr";

    assert_eq!(raw, DirectHtmlTableNormalizer::normalize(raw));
}

#[test]
fn parses_td_before_th_in_row() {
    let raw = [
        "<table><tbody>",
        "<tr><td>Feature</td><th>Value</th></tr>",
        "<tr><td>coverage</td><td>full</td></tr>",
        "</tbody></table>",
    ]
    .join("\n");

    assert_eq!(
        [
            "| Feature | Value |",
            "| --- | --- |",
            "| coverage | full |",
        ]
        .join("\n"),
        DirectHtmlTableNormalizer::normalize(&raw)
    );
}

#[test]
fn parses_th_before_td_in_row() {
    let raw = [
        "<table><tbody>",
        "<tr><th>Feature</th><td>status</td></tr>",
        "<tr><td>coverage</td><td>full</td></tr>",
        "</tbody></table>",
    ]
    .join("\n");

    assert_eq!(
        [
            "| Feature | status |",
            "| --- | --- |",
            "| coverage | full |",
        ]
        .join("\n"),
        DirectHtmlTableNormalizer::normalize(&raw)
    );
}

#[test]
fn normalize_pad_missing_cells() {
    let raw = [
        "<table><tbody>",
        "<tr><th>Feature</th><td>Status</td><td>Owner</td></tr>",
        "<tr><td>coverage</td></tr>",
        "</tbody></table>",
    ]
    .join("\n");

    assert_eq!(
        [
            "| Feature | Status | Owner |",
            "| --- | --- | --- |",
            "| coverage |  |  |",
        ]
        .join("\n"),
        DirectHtmlTableNormalizer::normalize(&raw)
    );
}

#[test]
fn html_table_text_strips_tags_and_unescapes_entities() {
    let raw = [
        "<table><tbody>",
        "<tr><td>hello <strong>world</strong> &amp; <em>universe</em></td></tr>",
        "</tbody></table>",
    ]
    .join("\n");

    assert_eq!(
        "| hello world & universe |\n| --- |",
        DirectHtmlTableNormalizer::normalize(&raw)
    );
}
