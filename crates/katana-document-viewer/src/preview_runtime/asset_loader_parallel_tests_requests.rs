use crate::{
    ArtifactFormat, DiagramRenderEngine, DiagramRenderRequest, KdvThemeSnapshot, RenderedDiagram,
    ViewerNodePlanner, ViewerViewport,
};
use crate::{
    MarkdownSource, PreviewAssetLoader, PreviewConfig, PreviewError, PreviewOutput,
    PreviewOutputFactory,
};

#[derive(Clone)]
struct ParallelEngine;

impl DiagramRenderEngine for ParallelEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: format!("<svg><text>{}</text></svg>", request.source),
        })
    }
}

#[test]
fn load_requested_parallel_skips_html_requests_but_loads_diagrams()
-> Result<(), Box<dyn std::error::Error>> {
    let output = output_for(
        "<p>Mixed request sample</p>\n\n```mermaid\ngraph TD\n  VisibleParallelA --> VisibleParallelB\n```",
    )?;
    let plan = ViewerNodePlanner::create(&output.input, output.scroll_offset);
    assert!(
        plan_has_html_request(&plan),
        "html requests should be included in planning"
    );
    assert!(
        plan_has_non_html_request(&plan),
        "diagram requests should be included in planning"
    );

    let (_loaded, report) = PreviewAssetLoader::new(ParallelEngine)
        .load_requested_parallel(&output, &KdvThemeSnapshot::katana_light())?;

    assert_eq!(1, report.loaded_artifact_count);
    Ok(())
}

#[test]
fn load_requested_parallel_reports_render_error_on_worker_panic()
-> Result<(), Box<dyn std::error::Error>> {
    #[derive(Clone)]
    struct PanicDiagramEngine;

    impl DiagramRenderEngine for PanicDiagramEngine {
        fn render(&self, _: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
            let render_should_succeed = false;
            assert!(render_should_succeed, "diagram worker panicked");
            Err("unreachable panic test result".to_string())
        }
    }

    let output = output_for("```mermaid\ngraph TD\n  PanicA --> PanicB\n```")?;
    let result = PreviewAssetLoader::new(PanicDiagramEngine)
        .load_requested_parallel(&output, &KdvThemeSnapshot::katana_light());

    assert!(
        matches!(result, Err(PreviewError::Render(message)) if message == "asset loader worker panicked")
    );
    Ok(())
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

fn plan_has_html_request(plan: &crate::ViewerNodePlan) -> bool {
    plan.asset_requests
        .iter()
        .any(|request| request.format == ArtifactFormat::Html)
}

fn plan_has_non_html_request(plan: &crate::ViewerNodePlan) -> bool {
    plan.asset_requests
        .iter()
        .any(|request| request.format != ArtifactFormat::Html)
}
