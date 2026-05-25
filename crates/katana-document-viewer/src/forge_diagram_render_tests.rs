use super::*;
use crate::{
    BuildProfile, DocumentSnapshotFactory, DocumentSource, ForgePipeline, KdvThemeSnapshot,
    SourceKind, SourceRevision, SourceUri,
};
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};
use katana_render_runtime::{RenderThemeMode, RenderThemeSnapshot};
use std::sync::{Arc, Mutex};

#[test]
fn diagram_renderer_panic_is_recorded_as_diagnostic() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(PanicDiagramEngine));
    let source = DocumentSource {
        uri: SourceUri("file:///panic.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("rev-1".to_string()),
        content: "```mermaid\ngraph TD\n  A --> B\n```".to_string(),
    };
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "panic.md",
        source.content.clone(),
    ))?;
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
    let graph = pipeline.build(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    })?;

    assert!(graph.rendered_diagrams.is_empty());
    assert!(
        graph
            .diagnostics
            .messages
            .iter()
            .any(|message| { message.contains("diagram renderer panicked") })
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
        snapshot: snapshot_from_markdown("```plantuml\n@startuml\nAlice -> Bob\n@enduml\n```")?,
        profile: BuildProfile::markdown_export(),
        theme: app_supplied_dark_theme(),
    })?;

    assert_eq!(graph.rendered_diagrams.len(), 1);
    let themes = captured_themes
        .lock()
        .map_err(|error| format!("theme capture lock failed: {error}"))?;
    let captured = themes.first().ok_or("captured KRR theme is missing")?;
    assert_app_supplied_theme_forwarded(captured);
    Ok(())
}

fn app_supplied_dark_theme() -> KdvThemeSnapshot {
    let mut theme = KdvThemeSnapshot::katana_dark();
    theme.name = "app-supplied-dark".to_string();
    theme.diagram_background = "transparent".to_string();
    theme.diagram_text = "#abcdef".to_string();
    theme.diagram_fill = "#123456".to_string();
    theme.diagram_stroke = "#654321".to_string();
    theme.diagram_arrow = "#fedcba".to_string();
    theme.mermaid_theme = "dark".to_string();
    theme
}

fn assert_app_supplied_theme_forwarded(captured: &RenderThemeSnapshot) {
    assert_eq!(captured.mode, RenderThemeMode::Dark);
    assert_eq!(captured.background, "transparent");
    assert_eq!(captured.text, "#abcdef");
    assert_eq!(captured.fill, "#123456");
    assert_eq!(captured.stroke, "#654321");
    assert_eq!(captured.arrow, "#fedcba");
    assert_eq!(captured.mermaid_theme, "dark");
}

struct PanicDiagramEngine;

impl DiagramRenderEngine for PanicDiagramEngine {
    fn render(&self, _request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        std::panic::resume_unwind(Box::new("diagram backend panic"));
    }
}

struct RecordingDiagramEngine {
    themes: Arc<Mutex<Vec<RenderThemeSnapshot>>>,
}

impl DiagramRenderEngine for RecordingDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.themes
            .lock()
            .map_err(|error| error.to_string())?
            .push(request.theme.krr_theme());
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: ExportHtmlOps::diagram_kind_label(&request.kind).to_string(),
            svg: "<svg data-test=\"theme-forwarded\"></svg>".to_string(),
        })
    }
}

fn snapshot_from_markdown(
    markdown: &str,
) -> Result<crate::DocumentSnapshot, Box<dyn std::error::Error>> {
    let source = DocumentSource {
        uri: SourceUri("file:///diagram-theme.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("rev-1".to_string()),
        content: markdown.to_string(),
    };
    let document =
        KatanaMarkdownModel::parse(MarkdownInput::from_content("diagram-theme.md", markdown))?;
    Ok(DocumentSnapshotFactory::from_kmm(source, document))
}
