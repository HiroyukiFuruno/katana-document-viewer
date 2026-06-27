use super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::preview::PreviewBuilder;
use katana_document_viewer::{DiagramRenderEngine, DiagramRenderRequest, RenderedDiagram};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[test]
fn partial_asset_scene_does_not_restart_job_when_scope_shrinks()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        catalog_with_fixture(temp_markdown_fixture(
            "tmp/partial-scope-diagrams.md",
            diagrams_fixture_source(),
        )?),
        PreviewBuilder::with_diagram_engine(Arc::new(FastDiagramEngine)),
    );
    storybook.update_frame_size(1280, 720);
    storybook.update_scene(1280, 720)?;

    let initial_scope = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job missing")?
        .scope_key()
        .to_string();
    let partial_scene = wait_for_partial_scene(&mut storybook)?;
    assert_ne!(initial_scope, partial_scene.asset_request_key);

    wait_for_next_asset_event_without_scope_restart(&mut storybook, &initial_scope)?;
    Ok(())
}

#[test]
fn scroll_refresh_does_not_restart_job_when_pending_scope_shrinks()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        catalog_with_fixture(temp_markdown_fixture(
            "tmp/partial-scope-scroll-diagrams.md",
            diagrams_fixture_source(),
        )?),
        PreviewBuilder::with_diagram_engine(Arc::new(FastDiagramEngine)),
    );
    storybook.update_frame_size(1280, 720);
    storybook.update_scene(1280, 720)?;

    let initial_scope = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job missing")?
        .scope_key()
        .to_string();
    let partial_scene = wait_for_partial_scene(&mut storybook)?;
    assert_ne!(initial_scope, partial_scene.asset_request_key);

    storybook.scroll_y = 320.0;
    storybook.start_asset_job_for_current_viewport(1280, 720);

    let job = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job missing after scroll refresh")?;
    assert_eq!(initial_scope, job.scope_key());
    Ok(())
}

fn wait_for_partial_scene(
    storybook: &mut StorybookWindow,
) -> Result<crate::preview::PreviewScene, Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if storybook.apply_asset_job()? && storybook.asset_job.is_some() {
            return storybook
                .scene
                .as_ref()
                .cloned()
                .ok_or_else(|| "partial scene missing".into());
        }
        std::thread::yield_now();
    }
    Err("partial asset scene did not arrive before deadline".into())
}

fn wait_for_next_asset_event_without_scope_restart(
    storybook: &mut StorybookWindow,
    initial_scope: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        let changed = storybook.apply_asset_job()?;
        if let Some(job) = storybook.asset_job.as_ref() {
            assert_eq!(
                initial_scope,
                job.scope_key(),
                "asset job restarted after partial scene scope changed"
            );
        }
        if changed {
            return Ok(());
        }
        std::thread::yield_now();
    }
    Err("next asset event did not arrive before deadline".into())
}

struct FastDiagramEngine;

impl DiagramRenderEngine for FastDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "diagram".to_string(),
            svg: "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 80 40\"></svg>"
                .to_string(),
        })
    }
}

fn catalog_with_fixture(fixture: StorybookFixture) -> FixtureCatalog {
    FixtureCatalog {
        fixtures: vec![fixture],
    }
}

fn temp_markdown_fixture(
    relative: &str,
    content: &str,
) -> Result<StorybookFixture, Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!(
        "kdv-storybook-{}",
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

fn diagrams_fixture_source() -> &'static str {
    "\
```mermaid
graph TD
  A --> B
```

```mermaid
graph TD
  C --> D
```

```mermaid
graph TD
  E --> F
```"
}
