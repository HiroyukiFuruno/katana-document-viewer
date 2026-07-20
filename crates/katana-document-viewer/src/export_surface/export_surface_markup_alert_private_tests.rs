use super::alert_body_line_from_quote;

#[test]
fn different_alert_marker_is_preserved_as_body_text() {
    assert_eq!(
        Some("[!WARNING] keep marker"),
        alert_body_line_from_quote("[!WARNING] keep marker", "TIP")
    );
}
