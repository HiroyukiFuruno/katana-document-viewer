use super::asset_loader_cache::PreviewDiagramAssetCache;
use super::{MarkdownSource, PreviewAssetLoader, PreviewConfig, PreviewOutputFactory};
use crate::{
    DiagramRenderEngine, DiagramRenderRequest, KdvThemeSnapshot, RenderedDiagram, ViewerViewport,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
struct CountingDiagramEngine {
    count: Arc<AtomicUsize>,
}

impl DiagramRenderEngine for CountingDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.count.fetch_add(1, Ordering::SeqCst);
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: format!("<svg><text>{}</text></svg>", request.source),
        })
    }
}

#[test]
fn diagram_disk_cache_survives_memory_cache_clear() -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("reuse");
    let count = Arc::new(AtomicUsize::new(0));
    let output = output_for("diagram-disk-reuse.md", diagram_source("DiskCacheA"))?;

    loader(count.clone(), &root).load_requested(&output, &KdvThemeSnapshot::katana_light())?;
    assert_eq!(1, count.load(Ordering::SeqCst));
    assert_eq!(1, count_svg_files(&root));

    PreviewDiagramAssetCache::clear_memory_for_tests();
    loader(count.clone(), &root).load_requested(&output, &KdvThemeSnapshot::katana_light())?;

    assert_eq!(1, count.load(Ordering::SeqCst));
    let _ = std::fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn diagram_disk_cache_key_includes_source_position() -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("position");
    let count = Arc::new(AtomicUsize::new(0));
    let first = output_for("diagram-disk-position.md", diagram_source("PositionCacheA"))?;
    let shifted = output_for(
        "diagram-disk-position.md",
        format!("intro\n\n{}", diagram_source("PositionCacheA")),
    )?;

    loader(count.clone(), &root).load_requested(&first, &KdvThemeSnapshot::katana_light())?;
    PreviewDiagramAssetCache::clear_memory_for_tests();
    loader(count.clone(), &root).load_requested(&shifted, &KdvThemeSnapshot::katana_light())?;

    assert_eq!(2, count.load(Ordering::SeqCst));
    assert_eq!(2, count_svg_files(&root));
    let _ = std::fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn diagram_disk_cache_key_includes_document_path() -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("document-path");
    let count = Arc::new(AtomicUsize::new(0));
    let first = output_for(
        "diagram-disk-document-a.md",
        diagram_source("DocumentCacheA"),
    )?;
    let second = output_for(
        "diagram-disk-document-b.md",
        diagram_source("DocumentCacheA"),
    )?;

    loader(count.clone(), &root).load_requested(&first, &KdvThemeSnapshot::katana_light())?;
    PreviewDiagramAssetCache::clear_memory_for_tests();
    loader(count.clone(), &root).load_requested(&second, &KdvThemeSnapshot::katana_light())?;

    assert_eq!(2, count.load(Ordering::SeqCst));
    assert_eq!(2, count_svg_files(&root));
    let _ = std::fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn diagram_disk_cache_key_includes_theme() -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("theme");
    let count = Arc::new(AtomicUsize::new(0));
    let output = output_for("diagram-disk-theme.md", diagram_source("ThemeCacheA"))?;

    loader(count.clone(), &root).load_requested(&output, &KdvThemeSnapshot::katana_light())?;
    PreviewDiagramAssetCache::clear_memory_for_tests();
    loader(count.clone(), &root).load_requested(&output, &KdvThemeSnapshot::katana_dark())?;

    assert_eq!(2, count.load(Ordering::SeqCst));
    assert_eq!(2, count_svg_files(&root));
    let _ = std::fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn diagram_disk_cache_key_includes_mermaid_effective_text_color()
-> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("theme-text");
    let count = Arc::new(AtomicUsize::new(0));
    let output = output_for(
        "diagram-disk-theme-text.md",
        diagram_source("ThemeTextCacheA"),
    )?;
    let first_theme = KdvThemeSnapshot::katana_dark();
    let mut second_theme = first_theme.clone();
    second_theme.text = "#abcdef".to_string();

    loader(count.clone(), &root).load_requested(&output, &first_theme)?;
    PreviewDiagramAssetCache::clear_memory_for_tests();
    loader(count.clone(), &root).load_requested(&output, &second_theme)?;

    assert_eq!(2, count.load(Ordering::SeqCst));
    assert_eq!(2, count_svg_files(&root));
    let _ = std::fs::remove_dir_all(root);
    Ok(())
}

fn loader(count: Arc<AtomicUsize>, root: &Path) -> PreviewAssetLoader<CountingDiagramEngine> {
    PreviewAssetLoader::new(CountingDiagramEngine { count }).with_diagram_cache_root(root)
}

fn output_for(
    document_id: &str,
    content: String,
) -> Result<super::PreviewOutput, super::PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content,
            document_id: Some(document_id.to_string()),
        },
        &PreviewConfig {
            viewport: ViewerViewport {
                width: 640.0,
                height: 480.0,
            },
            ..PreviewConfig::default()
        },
        320.0,
    )
}

fn diagram_source(label: &str) -> String {
    format!(
        "\
```mermaid
graph TD
  {label} --> Target
```"
    )
}

fn unique_cache_root(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let root = std::env::temp_dir().join(format!("kdv-diagram-cache-{label}-{nanos}"));
    let _ = std::fs::remove_dir_all(&root);
    root
}

fn count_svg_files(root: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(root) else {
        return 0;
    };
    entries
        .flatten()
        .map(|entry| count_svg_files_in_entry(&entry.path()))
        .sum()
}

fn count_svg_files_in_entry(path: &Path) -> usize {
    if path.is_dir() {
        return count_svg_files(path);
    }
    usize::from(path.extension().and_then(|value| value.to_str()) == Some("svg"))
}
