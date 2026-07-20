use crate::preview_runtime::asset_loader_cache::{
    PreviewDiagramAssetCache, PreviewDiagramAssetCacheKey,
};
use crate::{
    DiagramRenderCacheOptions, DiagramRenderEngine, DiagramRenderRequest, KdvThemeSnapshot,
    MarkdownSource, PreviewAssetLoader, PreviewConfig, PreviewError, PreviewOutput,
    PreviewOutputFactory, RenderedDiagram, ViewerNodePlanner, ViewerViewport,
};
use katana_markdown_model::DiagramKind;
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
fn diagram_asset_cache_skips_repeated_engine_render_for_same_source_theme()
-> Result<(), Box<dyn std::error::Error>> {
    let count = Arc::new(AtomicUsize::new(0));
    let output = output_for("```mermaid\ngraph TD\n  CacheUniqueA --> CacheUniqueB\n```")?;
    let theme = KdvThemeSnapshot::katana_light();

    PreviewAssetLoader::new(CountingDiagramEngine {
        count: count.clone(),
    })
    .load_requested(&output, &theme)?;
    PreviewAssetLoader::new(CountingDiagramEngine {
        count: count.clone(),
    })
    .load_requested(&output, &theme)?;

    assert_eq!(1, count.load(Ordering::SeqCst));
    Ok(())
}

#[test]
fn diagram_asset_cache_renders_again_when_theme_changes() -> Result<(), Box<dyn std::error::Error>>
{
    let count = Arc::new(AtomicUsize::new(0));
    let output = output_for("```mermaid\ngraph TD\n  ThemeUniqueA --> ThemeUniqueB\n```")?;

    PreviewAssetLoader::new(CountingDiagramEngine {
        count: count.clone(),
    })
    .load_requested(&output, &KdvThemeSnapshot::katana_light())?;
    PreviewAssetLoader::new(CountingDiagramEngine {
        count: count.clone(),
    })
    .load_requested(&output, &KdvThemeSnapshot::katana_dark())?;

    assert_eq!(2, count.load(Ordering::SeqCst));
    Ok(())
}

#[test]
fn visible_asset_load_scope_skips_near_viewport_diagram() -> Result<(), Box<dyn std::error::Error>>
{
    let output = output_for(VISIBLE_THEN_NEAR_DIAGRAM_SOURCE)?;
    let theme = KdvThemeSnapshot::katana_light();
    let count = Arc::new(AtomicUsize::new(0));
    let plan = ViewerNodePlanner::create(&output.input, output.scroll_offset);

    assert_eq!(1, plan.visible_artifact_ids.len());
    assert_eq!(1, plan.near_viewport_artifact_ids.len());

    let (_loaded, report) = PreviewAssetLoader::new(CountingDiagramEngine {
        count: count.clone(),
    })
    .load_visible_requested(&output, &theme)?;

    assert_eq!(1, report.loaded_artifact_count);
    assert_eq!(1, count.load(Ordering::SeqCst));
    Ok(())
}

#[test]
fn diagram_cache_key_distinguishes_diagram_kinds() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_for("```mermaid\ngraph TD\n  KindA --> KindB\n```")?;
    let theme = KdvThemeSnapshot::katana_light();
    let options = DiagramRenderCacheOptions::default();
    let node = output.input.snapshot.document.nodes[0].clone();
    let mermaid = diagram_cache_key(&output, &node, &theme, &options, DiagramKind::Mermaid);
    let drawio = diagram_cache_key(&output, &node, &theme, &options, DiagramKind::DrawIo);
    let plantuml = diagram_cache_key(&output, &node, &theme, &options, DiagramKind::PlantUml);

    assert_ne!(format!("{:?}", mermaid), format!("{:?}", drawio));
    assert_ne!(format!("{:?}", drawio), format!("{:?}", plantuml));
    Ok(())
}

fn diagram_cache_key(
    output: &PreviewOutput,
    node: &katana_markdown_model::KmmNode,
    theme: &KdvThemeSnapshot,
    options: &DiagramRenderCacheOptions,
    kind: DiagramKind,
) -> PreviewDiagramAssetCacheKey {
    PreviewDiagramAssetCache::key("engine", output, node, &kind, "A --> B", theme, options)
}

fn output_for(content: &str) -> Result<PreviewOutput, PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some("diagram-cache.md".to_string()),
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

const VISIBLE_THEN_NEAR_DIAGRAM_SOURCE: &str = "\
```mermaid
graph TD
  VisibleUniqueA --> VisibleUniqueB
```

Spacer one

Spacer two

Spacer three

Spacer four

Spacer five

Spacer six

Spacer seven

Spacer eight

Spacer nine

Spacer ten

Spacer eleven

Spacer twelve

Spacer thirteen

Spacer fourteen

Spacer fifteen

Spacer sixteen

Spacer seventeen

Spacer eighteen

Spacer nineteen

Spacer twenty

```mermaid
graph TD
  NearUniqueA --> NearUniqueB
```";
