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
