use super::super::{DiagramRenderEngine, DiagramRenderRequest, KrrDiagramRenderEngine};
use super::support::{must_render_error, with_runtime_env};
use crate::KdvThemeSnapshot;
use katana_markdown_model::DiagramKind;

#[test]
fn diagram_render_engine_mermaid_requests_pass_through_renderer_path() -> Result<(), String> {
    with_runtime_env("MERMAID_JS", Some("/tmp/does-not-exist-mermaid.js"), || {
        let engine = KrrDiagramRenderEngine;
        let theme = KdvThemeSnapshot::katana_light();
        let request = DiagramRenderRequest {
            node_id: "node-2",
            document_id: "doc-2",
            kind: DiagramKind::Mermaid,
            source: "graph TD\nA-->B".to_string(),
            theme: &theme,
        };

        let output = must_render_error(engine.render(request))?;
        assert!(!output.is_empty());
        Ok(())
    })
}

#[test]
fn diagram_render_engine_mermaid_reports_runtime_resolution_failure() -> Result<(), String> {
    with_runtime_env("MERMAID_JS", Some(""), || {
        let engine = KrrDiagramRenderEngine;
        let theme = KdvThemeSnapshot::katana_light();
        let request = DiagramRenderRequest {
            node_id: "node-5",
            document_id: "doc-5",
            kind: DiagramKind::Mermaid,
            source: "graph TD\nA-->B".to_string(),
            theme: &theme,
        };

        let output = must_render_error(engine.render(request))?;
        assert!(output.contains("MERMAID_JS"));
        Ok(())
    })
}

#[test]
fn diagram_render_engine_drawio_requests_try_runtime_path_and_report_failure() -> Result<(), String>
{
    with_runtime_env("DRAWIO_JS", Some("/tmp/does-not-exist-drawio.js"), || {
        let engine = KrrDiagramRenderEngine;
        let theme = KdvThemeSnapshot::katana_light();
        let request = DiagramRenderRequest {
            node_id: "node-3",
            document_id: "doc-3",
            kind: DiagramKind::DrawIo,
            source: "<mxfile></mxfile>".to_string(),
            theme: &theme,
        };

        let output = must_render_error(engine.render(request))?;
        assert!(!output.is_empty());
        Ok(())
    })
}

#[test]
fn diagram_render_engine_plantuml_request_propagates_renderer_result() {
    let engine = KrrDiagramRenderEngine;
    let theme = KdvThemeSnapshot::katana_light();
    let request = DiagramRenderRequest {
        node_id: "node-4",
        document_id: "doc-4",
        kind: DiagramKind::PlantUml,
        source: "@startuml\nAlice -> Bob\n@enduml\n".to_string(),
        theme: &theme,
    };

    match engine.render(request) {
        Ok(rendered) => assert_eq!("node-4", rendered.node_id),
        Err(error) => assert!(!error.is_empty()),
    }
}
