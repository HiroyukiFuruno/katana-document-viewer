use crate::{
    DiagramRenderEngine, DiagramRenderRequest, KdvThemeSnapshot, RenderedDiagram, ViewerViewport,
};
use crate::{
    MarkdownSource, PreviewAssetLoader, PreviewConfig, PreviewError, PreviewOutput,
    PreviewOutputFactory,
};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

#[derive(Clone)]
struct ParallelProbeEngine {
    expected: usize,
    state: Arc<ParallelProbeState>,
}

struct ParallelProbeState {
    counts: Mutex<ParallelProbeCounts>,
    condvar: Condvar,
}

#[derive(Default)]
struct ParallelProbeCounts {
    started: usize,
    active: usize,
    max_active: usize,
}

impl ParallelProbeEngine {
    fn new(expected: usize) -> Self {
        Self {
            expected,
            state: Arc::new(ParallelProbeState {
                counts: Mutex::new(ParallelProbeCounts::default()),
                condvar: Condvar::new(),
            }),
        }
    }

    fn max_active(&self) -> Result<usize, String> {
        let counts = self
            .state
            .counts
            .lock()
            .map_err(|_| "parallel probe lock poisoned".to_string())?;
        Ok(counts.max_active)
    }
}

impl DiagramRenderEngine for ParallelProbeEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.mark_started()?;
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: format!("<svg><text>{}</text></svg>", request.source),
        })
    }
}

impl ParallelProbeEngine {
    fn mark_started(&self) -> Result<(), String> {
        let counts = self
            .state
            .counts
            .lock()
            .map_err(|_| "parallel probe lock poisoned".to_string())?;
        let counts = self.increment_active(counts);
        let mut counts = self.wait_for_peers(counts)?;
        counts.active = counts.active.saturating_sub(1);
        self.state.condvar.notify_all();
        Ok(())
    }

    fn increment_active<'a>(
        &self,
        mut counts: std::sync::MutexGuard<'a, ParallelProbeCounts>,
    ) -> std::sync::MutexGuard<'a, ParallelProbeCounts> {
        counts.started += 1;
        counts.active += 1;
        counts.max_active = counts.max_active.max(counts.active);
        self.state.condvar.notify_all();
        counts
    }

    fn wait_for_peers<'a>(
        &self,
        counts: std::sync::MutexGuard<'a, ParallelProbeCounts>,
    ) -> Result<std::sync::MutexGuard<'a, ParallelProbeCounts>, String> {
        let result = self
            .state
            .condvar
            .wait_timeout_while(counts, Duration::from_millis(200), |counts| {
                counts.started < self.expected
            })
            .map_err(|_| "parallel probe lock poisoned".to_string())?;
        Ok(result.0)
    }
}

#[test]
fn visible_asset_load_parallel_starts_visible_diagrams_together()
-> Result<(), Box<dyn std::error::Error>> {
    let output = output_for(two_visible_diagram_source())?;
    let engine = ParallelProbeEngine::new(2);

    let (_loaded, report) = PreviewAssetLoader::new(engine.clone())
        .load_visible_requested_parallel(&output, &KdvThemeSnapshot::katana_light())?;

    assert_eq!(2, report.loaded_artifact_count);
    assert_eq!(2, engine.max_active()?);
    Ok(())
}

#[test]
fn visible_asset_load_parallel_includes_failed_artifacts_in_report()
-> Result<(), Box<dyn std::error::Error>> {
    #[derive(Clone)]
    struct FailingEngine;

    impl DiagramRenderEngine for FailingEngine {
        fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
            Err(format!("forced fail: {}", request.node_id))
        }
    }

    let output = output_for("```mermaid\ngraph TD\n  ErrorA --> ErrorB\n```")?;
    let (_loaded, report) = PreviewAssetLoader::new(FailingEngine)
        .load_visible_requested_parallel(&output, &KdvThemeSnapshot::katana_light())?;

    assert_eq!(1, report.failed_artifact_count);
    assert_eq!(0, report.loaded_artifact_count);
    Ok(())
}

#[test]
fn load_requested_parallel_returns_all_visible_artifacts() -> Result<(), Box<dyn std::error::Error>>
{
    let output = output_for(two_visible_all_diagram_source())?;
    let (_loaded, report) = PreviewAssetLoader::new(ParallelProbeEngine::new(2))
        .load_requested_parallel(&output, &KdvThemeSnapshot::katana_light())?;

    assert_eq!(2, report.loaded_artifact_count);
    assert_eq!(0, report.failed_artifact_count);
    Ok(())
}

fn two_visible_all_diagram_source() -> &'static str {
    "\
```mermaid
graph TD
  AllParallelA --> AllParallelB
```

```mermaid
graph TD
  AllParallelC --> AllParallelD
```"
}

fn two_visible_diagram_source() -> &'static str {
    "\
```mermaid
graph TD
  VisibleParallelA --> VisibleParallelB
```

```mermaid
graph TD
  VisibleParallelC --> VisibleParallelD
```"
}

fn output_for(content: &str) -> Result<PreviewOutput, PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some("diagram-parallel.md".to_string()),
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
