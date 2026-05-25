use crate::SourceUri;
use std::path::{Path, PathBuf};

pub(crate) struct ExportAssetResolver;

impl ExportAssetResolver {
    pub(crate) fn resolve_src(source_uri: &SourceUri, src: &str) -> String {
        let Some(path) = Self::resolve_file_path(source_uri, src) else {
            return src.to_string();
        };
        Self::file_url(&path)
    }

    pub(crate) fn resolve_file_path(source_uri: &SourceUri, src: &str) -> Option<PathBuf> {
        if Self::is_non_file_reference(src) {
            return None;
        }
        let path = Path::new(src);
        if path.is_absolute() {
            return Some(path.to_path_buf());
        }
        let base_dir = Self::source_base_dir(source_uri)?;
        Some(base_dir.join(path))
    }

    pub(crate) fn rewrite_html_image_sources(fragment: &str, source_uri: &SourceUri) -> String {
        let mut output = String::with_capacity(fragment.len());
        let mut rest = fragment;
        while let Some(src_start) = rest.find("src=\"") {
            let value_start = src_start + "src=\"".len();
            let Some(value_end) = rest[value_start..].find('"') else {
                break;
            };
            let src = &rest[value_start..value_start + value_end];
            output.push_str(&rest[..value_start]);
            output.push_str(&Self::resolve_src(source_uri, src));
            rest = &rest[value_start + value_end..];
        }
        output.push_str(rest);
        output
    }

    fn source_base_dir(source_uri: &SourceUri) -> Option<PathBuf> {
        let path = source_uri
            .0
            .strip_prefix("file://")
            .unwrap_or(&source_uri.0);
        let path = Path::new(path);
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().ok()?.join(path)
        };
        absolute.parent().map(Path::to_path_buf)
    }

    fn is_non_file_reference(src: &str) -> bool {
        src.is_empty()
            || src.starts_with('#')
            || src.starts_with("data:")
            || src.starts_with("http://")
            || src.starts_with("https://")
            || src.starts_with("file://")
    }

    fn file_url(path: &Path) -> String {
        format!("file://{}", path.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_relative_image_from_source_markdown_directory() {
        let source_uri = SourceUri("file:///workspace/docs/README.md".to_string());

        let resolved = ExportAssetResolver::resolve_src(&source_uri, "assets/icon.png");

        assert_eq!(resolved, "file:///workspace/docs/assets/icon.png");
    }

    #[test]
    fn keeps_remote_and_data_images_unchanged() {
        let source_uri = SourceUri("file:///workspace/docs/README.md".to_string());

        assert_eq!(
            ExportAssetResolver::resolve_src(&source_uri, "https://example.com/icon.png"),
            "https://example.com/icon.png"
        );
        assert_eq!(
            ExportAssetResolver::resolve_src(&source_uri, "data:image/png;base64,AA=="),
            "data:image/png;base64,AA=="
        );
    }
}
