use super::{
    DiagramRenderTestSupport, ErrorDiagramEngine, PanicDiagramEngine, RecordingDiagramEngine,
};
use crate::artifact::ArtifactFormat;
use crate::forge_diagram_render_types::DiagramRenderingBackend;
use crate::{
    BuildProfile, BuildRequest, ExportFormat, ExportRequest, ForgePipeline, KdvThemeSnapshot,
};
use std::sync::{Arc, Mutex};

#[test]
fn diagram_renderer_panic_is_recorded_as_diagnostic() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(PanicDiagramEngine));
    let graph = pipeline.build(&BuildRequest {
        snapshot: DiagramRenderTestSupport::snapshot_from_markdown(
            "```mermaid\ngraph TD\n  A --> B\n```",
        )?,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    })?;

    assert!(graph.rendered_diagrams.is_empty());
    assert!(
        graph
            .diagnostics
            .messages
            .iter()
            .any(|message| message.contains("diagram renderer panicked for node"))
    );
    Ok(())
}

#[test]
fn diagram_renderer_receives_app_supplied_complete_theme() -> Result<(), Box<dyn std::error::Error>>
{
    let captured_themes = Arc::new(Mutex::new(Vec::new()));
    let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(RecordingDiagramEngine {
        themes: captured_themes.clone(),
    }));
    let graph = pipeline.build(&BuildRequest {
        snapshot: DiagramRenderTestSupport::snapshot_from_markdown(
            "```plantuml\n@startuml\nAlice -> Bob\n@enduml\n```",
        )?,
        profile: BuildProfile::markdown_export(),
        theme: DiagramRenderTestSupport::app_supplied_dark_theme(),
    })?;

    assert_eq!(graph.rendered_diagrams.len(), 1);
    let themes = captured_themes
        .lock()
        .map_err(|error| format!("theme capture lock failed: {error}"))?;
    let captured = themes.first().ok_or("captured KRR theme is missing")?;
    DiagramRenderTestSupport::assert_app_supplied_theme_forwarded(captured);
    Ok(())
}

#[test]
fn diagram_renderer_records_render_error_without_panic() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(ErrorDiagramEngine));
    let graph = pipeline.build(&BuildRequest {
        snapshot: DiagramRenderTestSupport::snapshot_from_markdown(
            "```mermaid\ngraph TD\n  A --> B\n```",
        )?,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    })?;

    assert_eq!(graph.rendered_diagrams.len(), 0);
    assert_eq!(graph.diagnostics.messages.len(), 1);
    assert!(graph.diagnostics.messages[0].contains("render failed"));
    assert!(
        graph.diagnostics.messages[0].contains("diagram renderer failed for node"),
        "{:?}",
        graph.diagnostics.messages
    );
    Ok(())
}

#[test]
fn diagram_renderer_walks_nested_list_items() {
    let pipeline = DiagramRenderingBackend::new(RecordingDiagramEngine {
        themes: Arc::new(Mutex::new(Vec::new())),
    });
    let root = DiagramRenderTestSupport::nested_list_root_with_diagram();

    let mut rendered_diagrams = Vec::new();
    let mut messages = Vec::new();
    pipeline.collect_node(
        &root,
        "document-id",
        &KdvThemeSnapshot::katana_light(),
        &mut rendered_diagrams,
        &mut messages,
    );

    assert_eq!(rendered_diagrams.len(), 1);
    assert!(messages.is_empty());
}

#[test]
fn diagram_renderer_exports_payload() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(RecordingDiagramEngine {
        themes: Arc::new(Mutex::new(Vec::new())),
    }));
    let graph = pipeline.build(&BuildRequest {
        snapshot: DiagramRenderTestSupport::snapshot_from_markdown(
            "```mermaid\ngraph TD\n  A --> B\n```",
        )?,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    })?;

    let output = pipeline.export(&ExportRequest {
        graph,
        format: ExportFormat::Html,
        theme: KdvThemeSnapshot::katana_light(),
    })?;

    assert_eq!(output.artifact.manifest.format, ArtifactFormat::Html);
    assert_eq!(output.artifact.manifest.backend, "katana-document-viewer");
    assert!(!output.artifact.bytes.bytes.is_empty());

    Ok(())
}
