use super::DirectVisualSource;

#[test]
fn source_uri_rejects_spaced_non_windows_path_with_colon() {
    assert_eq!(DirectVisualSource::source_uri("C:bad path"), None);
}

#[test]
fn source_uri_rejects_windows_like_path_without_separator_after_drive() {
    assert_eq!(
        DirectVisualSource::source_uri("AB:folder/name with space"),
        None
    );
}
