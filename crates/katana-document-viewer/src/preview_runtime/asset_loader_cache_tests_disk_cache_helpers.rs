use crate::preview_runtime::asset_loader_cache::PreviewDiagramAssetCache;
use crate::{
    DiagramRenderCacheOptions, KdvThemeSnapshot, MarkdownSource, PreviewConfig, PreviewError,
    PreviewOutput, PreviewOutputFactory,
};
use katana_markdown_model::DiagramKind;
use std::path::{Path, PathBuf};

pub(super) fn output_for(content: &str) -> Result<PreviewOutput, PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some("diagram-cache.md".to_string()),
        },
        &PreviewConfig {
            viewport: crate::ViewerViewport {
                width: 640.0,
                height: 480.0,
            },
            ..PreviewConfig::default()
        },
        320.0,
    )
}

pub(super) fn unique_cache_root(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let root = std::env::temp_dir().join(format!("kdv-diagram-cache-{label}-{nanos}"));
    let _ = std::fs::remove_dir_all(&root);
    root
}

pub(super) fn count_svg_files(root: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(root) else {
        return 0;
    };
    entries
        .flatten()
        .map(|entry| count_svg_files_in_entry(&entry.path()))
        .sum()
}

pub(super) fn locate_svg_file(root: &Path) -> Option<PathBuf> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return None;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = locate_svg_file(&path) {
                return Some(found);
            }
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) == Some("svg") {
            return Some(path);
        }
    }
    None
}

pub(super) fn count_svg_files_in_entry(path: &Path) -> usize {
    if path.is_dir() {
        return count_svg_files(path);
    }
    usize::from(path.extension().and_then(|value| value.to_str()) == Some("svg"))
}

pub(super) fn disk_cache_key_and_permissions(
    root: &Path,
) -> Result<(PathBuf, std::fs::Permissions), Box<dyn std::error::Error>> {
    let output = output_for("```mermaid\ngraph TD\n  ReadonlyParentA --> ReadonlyParentB\n```")?;
    let node = output.input.snapshot.document.nodes[0].clone();
    let key = PreviewDiagramAssetCache::key(
        "engine",
        &output,
        &node,
        &DiagramKind::Mermaid,
        "A --> B",
        &KdvThemeSnapshot::katana_light(),
        &DiagramRenderCacheOptions::default(),
    );
    let cache_path = key.cache_path_for_tests(root);
    let parent = cache_path
        .parent()
        .ok_or("cache key parent path missing")?
        .to_path_buf();
    std::fs::create_dir_all(&parent)?;
    let original_permissions = parent.metadata()?.permissions();
    Ok((cache_path, original_permissions))
}

pub(super) fn set_readonly(
    path: &Path,
    original_permissions: &std::fs::Permissions,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut readonly = original_permissions.clone();
    readonly.set_readonly(true);
    std::fs::set_permissions(path, readonly)?;
    Ok(())
}

pub(super) fn restore_permissions(
    path: &Path,
    permissions: std::fs::Permissions,
) -> std::io::Result<()> {
    std::fs::set_permissions(path, permissions)
}

pub(super) fn clean_cache_root(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all(root)?;
    Ok(())
}
