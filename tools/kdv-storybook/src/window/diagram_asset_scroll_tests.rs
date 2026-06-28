use super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::preview::PreviewBuilder;
use katana_document_viewer::{DiagramRenderEngine, DiagramRenderRequest, RenderedDiagram};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[test]
fn loaded_diagram_scene_scroll_does_not_restart_or_rerender_assets()
-> Result<(), Box<dyn std::error::Error>> {
    let render_count = Arc::new(AtomicUsize::new(0));
    let fixture = unique_markdown_fixture(
        "tmp/scroll-cache-diagrams.md",
        "\
# Scroll cache

Before diagram.

```mermaid
graph TD
  ScrollCacheA --> ScrollCacheB
```

Middle paragraph.

```mermaid
graph TD
  ScrollCacheC --> ScrollCacheD
```

After diagram.

Paragraph 01
Paragraph 02
Paragraph 03
Paragraph 04
Paragraph 05
Paragraph 06
Paragraph 07
Paragraph 08
Paragraph 09
Paragraph 10",
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
    storybook.update_scene(1280, 360)?;
    let loaded_scene = wait_for_loaded_scene(&mut storybook)?;
    assert_eq!(0, loaded_scene.asset_request_count);
    assert!(loaded_scene.image_surface_count > 0);
    let first_render_count = render_count.load(Ordering::SeqCst);
    assert!(first_render_count > 0);

    assert!(storybook.apply_preview_scroll(-300.0, 360));
    assert!(!storybook.scene_refresh_needed(false));
    storybook.start_asset_job_for_current_viewport(1280, 360);

    assert!(storybook.asset_job.is_none());
    assert_eq!(first_render_count, render_count.load(Ordering::SeqCst));
    let scene = storybook.scene.as_ref().ok_or("scrolled scene missing")?;
    assert_eq!(0, scene.asset_request_count);
    assert!(scene.image_surface_count > 0);
    Ok(())
}

fn wait_for_loaded_scene(
    storybook: &mut StorybookWindow,
) -> Result<crate::preview::PreviewScene, Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if storybook.apply_asset_job()? {
            let scene = storybook.scene.as_ref().ok_or("loaded scene missing")?;
            if scene.asset_request_count == 0 {
                return Ok(scene.clone());
            }
        }
        std::thread::yield_now();
    }
    Err("asset job did not complete before scroll cache assertion".into())
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
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"240\" height=\"120\" \
                 viewBox=\"0 0 240 120\"><text x=\"16\" y=\"64\">{}</text></svg>",
                request.node_id
            ),
        })
    }
}

fn unique_markdown_fixture(
    relative: &str,
    content: &str,
) -> Result<StorybookFixture, Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!(
        "kdv-storybook-scroll-cache-{}",
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
