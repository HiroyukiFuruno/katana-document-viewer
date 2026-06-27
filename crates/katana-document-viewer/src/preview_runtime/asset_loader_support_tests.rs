use super::asset_loader_support::PreviewAssetLoaderSupport;
use std::path::PathBuf;

#[test]
fn file_path_from_uri_supports_encoded_file_uri() {
    let path =
        PreviewAssetLoaderSupport::file_path_from_uri("file:///tmp/kdv%20icon.png?cache=1#preview");

    assert_eq!(Some(PathBuf::from("/tmp/kdv icon.png")), path);
}

#[test]
fn file_path_from_uri_supports_localhost_file_uri() {
    let path = PreviewAssetLoaderSupport::file_path_from_uri("file://localhost/tmp/kdv-icon.png");

    assert_eq!(Some(PathBuf::from("/tmp/kdv-icon.png")), path);
}

#[test]
fn file_path_from_uri_rejects_remote_file_host() {
    let path = PreviewAssetLoaderSupport::file_path_from_uri("file://example.com/tmp/kdv-icon.png");

    assert_eq!(None, path);
}
