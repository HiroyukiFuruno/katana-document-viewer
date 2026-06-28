use super::html_score_direct_visual_source::DirectVisualSource;

#[test]
fn source_uri_keeps_file_uri_prefix() {
    assert_eq!(
        DirectVisualSource::source_uri("file:///tmp/icon.png").as_deref(),
        Some("file:///tmp/icon.png")
    );
}

#[test]
fn source_uri_formats_path_without_file_scheme() {
    assert_eq!(
        DirectVisualSource::source_uri("/tmp/icon.png").as_deref(),
        Some("file:///tmp/icon.png")
    );
}

#[test]
fn source_uri_keeps_direct_path_with_whitespace_as_direct_source() {
    assert_eq!(
        DirectVisualSource::source_uri("/tmp/kdv fixtures/icon sample.png").as_deref(),
        Some("file:///tmp/kdv fixtures/icon sample.png")
    );
}

#[test]
fn source_uri_rejects_multiline_text_ending_with_image_extension() {
    assert_eq!(
        DirectVisualSource::source_uri("notes about icon.png\nnot a direct path"),
        None
    );
}

#[test]
fn mermaid_flow_detection_stops_without_direction_when_keyword_differs() {
    assert!(!DirectVisualSource::line_starts_mermaid_flow_diagram(
        "sequencediagram"
    ));
}

#[test]
fn mermaid_flow_detection_stops_on_empty_line() {
    assert!(!DirectVisualSource::line_starts_mermaid_flow_diagram(""));
}
