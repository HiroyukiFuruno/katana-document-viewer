use super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::preview::PreviewBuilder;
use katana_document_viewer::{DiagramRenderEngine, DiagramRenderRequest, RenderedDiagram};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const SAMPLE_DIAGRAMS: &str = "katana/sample_diagrams.md";

#[test]
fn katana_sample_diagrams_assets_finish_incrementally() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        catalog_with(SAMPLE_DIAGRAMS),
        PreviewBuilder::with_diagram_engine(Arc::new(FastDiagramEngine)),
    );

    storybook.update_scene(1280, 720)?;
    let pending_scene = storybook.scene.as_ref().ok_or("pending scene missing")?;
    let pending_image_surface_count = pending_scene.image_surface_count;
    assert!(
        pending_scene.asset_request_count >= 2,
        "KatanA sample diagrams must expose multiple pending visible assets"
    );
    assert_eq!(0, pending_scene.loaded_asset_count);

    let loaded_scene = wait_for_loaded_scene(&mut storybook)?;
    assert_eq!(0, loaded_scene.asset_request_count);
    assert!(
        loaded_scene.loaded_asset_count >= 2,
        "KatanA sample diagrams must load multiple diagram assets: loaded={}, failed={}, pending={}, image_surfaces={}",
        loaded_scene.loaded_asset_count,
        loaded_scene.failed_asset_count,
        loaded_scene.asset_request_count,
        loaded_scene.image_surface_count,
    );
    assert!(
        loaded_scene.image_surface_count >= pending_image_surface_count,
        "loaded scene must keep or increase image surfaces"
    );
    Ok(())
}

#[test]
fn resized_loaded_diagram_scene_reuses_cached_artifacts_without_pending_reload()
-> Result<(), Box<dyn std::error::Error>> {
    let render_count = Arc::new(AtomicUsize::new(0));
    let fixture = unique_markdown_fixture(
        "tmp/resize-cache-diagrams.md",
        "\
```mermaid
graph TD
  ResizeCacheA --> ResizeCacheB
```

```mermaid
graph TD
  ResizeCacheC --> ResizeCacheD
```",
    )?;
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::with_diagram_engine(Arc::new(CountingDiagramEngine {
            count: render_count.clone(),
        })),
    );
    storybook.update_scene(1280, 720)?;
    let loaded_scene = wait_for_loaded_scene(&mut storybook)?;
    assert_eq!(0, loaded_scene.asset_request_count);
    let first_render_count = render_count.load(Ordering::SeqCst);
    assert!(
        first_render_count > 0,
        "test must render at least one diagram before validating resize reuse"
    );

    storybook.update_scene_for_refresh(1000, 720)?;
    let resized_scene = storybook.scene.as_ref().ok_or("resized scene missing")?;

    assert_eq!(
        0, resized_scene.asset_request_count,
        "resizing a fully loaded diagram scene must not show pending placeholders again"
    );
    assert!(resized_scene.loaded_asset_count > 0);
    assert_eq!(
        first_render_count,
        render_count.load(Ordering::SeqCst),
        "resize must reuse cached diagram artifacts instead of rendering them again"
    );
    Ok(())
}

fn wait_for_loaded_scene(
    storybook: &mut StorybookWindow,
) -> Result<crate::preview::PreviewScene, Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut changed_count = 0usize;
    let mut last_asset_request_count = usize::MAX;
    let mut last_loaded_asset_count = 0usize;
    let mut last_failed_asset_count = 0usize;
    let mut last_image_surface_count = 0usize;
    while Instant::now() < deadline {
        if storybook.apply_asset_job()? {
            changed_count += 1;
            let scene = storybook.scene.as_ref().ok_or("loaded scene missing")?;
            last_asset_request_count = scene.asset_request_count;
            last_loaded_asset_count = scene.loaded_asset_count;
            last_failed_asset_count = scene.failed_asset_count;
            last_image_surface_count = scene.image_surface_count;
            if scene.asset_request_count == 0 {
                return Ok(scene.clone());
            }
        }
        std::thread::yield_now();
    }
    Err(format!(
        "KatanA sample diagrams asset job did not complete before deadline: changed={changed_count}, pending={last_asset_request_count}, loaded={last_loaded_asset_count}, failed={last_failed_asset_count}, image_surfaces={last_image_surface_count}, job_alive={}",
        storybook.asset_job.is_some()
    )
    .into())
}

struct FastDiagramEngine;

impl DiagramRenderEngine for FastDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let kind = diagram_kind_label(&request);
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: kind.to_string(),
            svg: format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 160 72\">\
                 <text x=\"16\" y=\"42\">{kind}</text></svg>"
            ),
        })
    }
}

struct CountingDiagramEngine {
    count: Arc<AtomicUsize>,
}

impl DiagramRenderEngine for CountingDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.count.fetch_add(1, Ordering::SeqCst);
        FastDiagramEngine.render(request)
    }
}

fn diagram_kind_label(request: &DiagramRenderRequest<'_>) -> &'static str {
    match format!("{:?}", request.kind).as_str() {
        "Mermaid" => "mermaid",
        "DrawIo" => "drawio",
        "PlantUml" => "plantuml",
        _ => "diagram",
    }
}

fn catalog_with(path: &str) -> FixtureCatalog {
    FixtureCatalog {
        fixtures: vec![StorybookFixture {
            label: path.to_string(),
            path: fixture_path(path),
        }],
    }
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
}

fn unique_markdown_fixture(
    relative: &str,
    content: &str,
) -> Result<StorybookFixture, Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!(
        "kdv-storybook-diagram-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().ok_or("fixture parent missing")?)?;
    std::fs::write(&path, content)?;
    Ok(StorybookFixture {
        label: relative.to_string(),
        path,
    })
}
