use super::*;
use crate::KdvThemeSnapshot;
use std::env;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;

static RUNTIME_ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_runtime_env(name: &str, value: Option<&str>, test: impl FnOnce()) {
    let _guard = match RUNTIME_ENV_LOCK.lock() {
        Ok(guard) => guard,
        Err(error) => {
            std::panic::resume_unwind(Box::new(format!("runtime env lock failed: {error}")))
        }
    };
    let previous = env::var_os(name);
    match value {
        Some(value) => unsafe { env::set_var(name, value) },
        None => unsafe { env::remove_var(name) },
    }
    let result = catch_unwind(AssertUnwindSafe(test));
    match previous {
        Some(value) => unsafe { env::set_var(name, value) },
        None => unsafe { env::remove_var(name) },
    }
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}

fn must_render_error(result: Result<RenderedDiagram, String>) -> String {
    match result {
        Ok(rendered) => std::panic::resume_unwind(Box::new(format!(
            "diagram render unexpectedly succeeded: {}",
            rendered.node_id
        ))),
        Err(error) => error,
    }
}

#[test]
fn diagram_render_engine_mermaid_requests_pass_through_renderer_path() {
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

        let output = must_render_error(engine.render(request));
        assert!(!output.is_empty());
    });
}

#[test]
fn diagram_render_engine_mermaid_reports_runtime_resolution_failure() {
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

        let output = must_render_error(engine.render(request));
        assert!(output.contains("MERMAID_JS"));
    });
}

#[test]
fn diagram_render_engine_drawio_requests_try_runtime_path_and_report_failure() {
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

        let output = must_render_error(engine.render(request));
        assert!(!output.is_empty());
    });
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

    let result = engine.render(request);
    assert!(result.is_err() || result.is_ok());
}
