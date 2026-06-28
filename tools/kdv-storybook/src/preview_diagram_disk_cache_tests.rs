use super::PreviewBuilder;
use crate::catalog::StorybookFixture;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use katana_document_viewer::{
    DiagramRenderEngine, DiagramRenderRequest, KdvThemeSnapshot, RenderedDiagram,
    ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerViewport,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn storybook_loaded_diagram_scene_writes_svg_to_physical_cache_root()
-> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("storybook-loaded")?;
    let render_count = Arc::new(AtomicUsize::new(0));
    let fixture = unique_diagram_fixture()?;
    let builder = PreviewBuilder::with_diagram_engine_and_cache_root(
        Arc::new(CountingDiagramEngine {
            count: render_count.clone(),
        }),
        root.clone(),
    );

    let scene = builder.build_scene(loaded_request(&fixture))?;

    assert_eq!(1, render_count.load(Ordering::SeqCst));
    assert_eq!(0, scene.asset_request_count);
    assert!(scene.loaded_asset_count > 0);
    assert!(scene.image_surface_count > 0);
    assert_eq!(1, count_svg_files(&root)?);
    let _ = std::fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn storybook_artifact_cache_does_not_reuse_diagram_when_theme_tokens_change()
-> Result<(), Box<dyn std::error::Error>> {
    let root = unique_cache_root("storybook-theme-cache")?;
    let render_count = Arc::new(AtomicUsize::new(0));
    let fixture = unique_diagram_fixture()?;
    let builder = PreviewBuilder::with_diagram_engine_and_cache_root(
        Arc::new(CountingDiagramEngine {
            count: render_count.clone(),
        }),
        root.clone(),
    );
    let mut first_theme = KdvThemeSnapshot::katana_dark();
    first_theme.diagram_background = first_theme.background.clone();
    first_theme.diagram_text = first_theme.text.clone();
    let mut second_theme = first_theme.clone();
    second_theme.diagram_text = "#ffffff".to_string();

    let mut first = loaded_request(&fixture);
    first.theme = Some(first_theme);
    builder.build_scene(first)?;
    let mut second = loaded_request(&fixture);
    second.theme = Some(second_theme);
    builder.build_scene(second)?;

    assert_eq!(
        2,
        render_count.load(Ordering::SeqCst),
        "diagram render must rerun when caller theme tokens change, even for the same source and dark mode"
    );
    let _ = std::fs::remove_dir_all(root);
    Ok(())
}

struct CountingDiagramEngine {
    count: Arc<AtomicUsize>,
}

impl DiagramRenderEngine for CountingDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.count.fetch_add(1, Ordering::SeqCst);
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"180\" height=\"80\" \
                 viewBox=\"0 0 180 80\"><text x=\"12\" y=\"42\">{}</text></svg>",
                request.node_id
            ),
        })
    }
}

fn loaded_request(fixture: &StorybookFixture) -> PreviewBuildRequest<'_> {
    PreviewBuildRequest {
        fixture,
        viewport: ViewerViewport {
            width: 900.0,
            height: 600.0,
        },
        dark: true,
        theme: None,
        interaction: ViewerInteractionConfig::default(),
        mode: ViewerMode::Document,
        typography: Default::default(),
        search: ViewerSearchState::default(),
        diagram_viewports: BTreeMap::new(),
        image_viewports: BTreeMap::new(),
        task_state_overrides: BTreeMap::new(),
        accordion_open_overrides: BTreeMap::new(),
        copied_code_node_ids: Default::default(),
        asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
        attach_surface: false,
        export_surface: false,
    }
}

fn unique_diagram_fixture() -> Result<StorybookFixture, Box<dyn std::error::Error>> {
    let root = unique_cache_root("storybook-fixture")?;
    let path = root.join("sample-diagram.md");
    std::fs::write(
        &path,
        "\
# Storybook Disk Cache

```mermaid
graph TD
  StorybookDiskCache --> PhysicalSvg
```",
    )?;
    Ok(StorybookFixture {
        label: "tmp/sample-diagram.md".to_string(),
        path,
    })
}

fn unique_cache_root(label: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("kdv-{label}-{nanos}"));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

fn count_svg_files(root: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    if !root.exists() {
        return Ok(0);
    }
    let mut count = 0;
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            count += count_svg_files(&path)?;
        } else if path.extension().and_then(std::ffi::OsStr::to_str) == Some("svg") {
            count += 1;
        }
    }
    Ok(count)
}
