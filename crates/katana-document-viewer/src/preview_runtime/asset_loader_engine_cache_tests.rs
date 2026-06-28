use super::asset_loader_cache::PreviewDiagramAssetCache;
use super::asset_loader_engine_cache_test_engines::{
    configured_engine, first_engine, second_engine,
};
use super::{MarkdownSource, PreviewAssetLoader, PreviewConfig, PreviewOutputFactory};
use crate::{DiagramRenderEngine, KdvThemeSnapshot, ViewerViewport};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn diagram_disk_cache_key_includes_dynamic_engine_namespace()
-> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("engine-namespace");
    let first_count = Arc::new(AtomicUsize::new(0));
    let second_count = Arc::new(AtomicUsize::new(0));
    let output = output_for("diagram-disk-engine.md", diagram_source("EngineCacheA"))?;

    loader(first_engine(first_count.clone()))
        .with_diagram_cache_root(&root)
        .load_requested(&output, &KdvThemeSnapshot::katana_light())?;
    PreviewDiagramAssetCache::clear_memory_for_tests();
    loader(second_engine(second_count.clone()))
        .with_diagram_cache_root(&root)
        .load_requested(&output, &KdvThemeSnapshot::katana_light())?;

    assert_both_engines_rendered_once(&first_count, &second_count);
    assert_eq!(2, count_svg_files(&root));
    cleanup_cache_root(root);
    Ok(())
}

#[test]
fn diagram_disk_cache_key_includes_renderer_options_and_dpi()
-> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("renderer-options");
    let first_count = Arc::new(AtomicUsize::new(0));
    let second_count = Arc::new(AtomicUsize::new(0));
    let output = output_for("diagram-disk-renderer.md", diagram_source("RendererCacheA"))?;

    load_configured_engine(
        &root,
        &output,
        first_count.clone(),
        "first",
        96,
        "retina=false;backend=first",
    )?;
    PreviewDiagramAssetCache::clear_memory_for_tests();
    load_configured_engine(
        &root,
        &output,
        second_count.clone(),
        "second",
        192,
        "retina=true;backend=second",
    )?;

    assert_both_engines_rendered_once(&first_count, &second_count);
    assert_eq!(2, count_svg_files(&root));
    cleanup_cache_root(root);
    Ok(())
}

fn loader(
    engine: Arc<dyn DiagramRenderEngine + Send + Sync>,
) -> PreviewAssetLoader<Arc<dyn DiagramRenderEngine + Send + Sync>> {
    PreviewAssetLoader::new(engine)
}

fn assert_both_engines_rendered_once(first_count: &AtomicUsize, second_count: &AtomicUsize) {
    assert_eq!(1, first_count.load(Ordering::SeqCst));
    assert_eq!(1, second_count.load(Ordering::SeqCst));
}

fn cleanup_cache_root(root: PathBuf) {
    let _ = std::fs::remove_dir_all(root);
}

fn load_configured_engine(
    root: &Path,
    output: &super::PreviewOutput,
    count: Arc<AtomicUsize>,
    label: &'static str,
    dpi: u32,
    renderer_options: &str,
) -> Result<(), super::PreviewError> {
    loader(configured_engine(count, label, dpi, renderer_options))
        .with_diagram_cache_root(root)
        .load_requested(output, &KdvThemeSnapshot::katana_light())?;
    Ok(())
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
