use crate::artifact::{ArtifactBytes, ArtifactFactory};
use crate::backend::diagram::KdrDiagramInputFactory;
use crate::export_html_ops::{diagram_kind_label, fenced_body};
use crate::export_payload::ExportPayloadFactory;
use crate::forge::{
    BuildGraph, BuildRequest, ExportOutput, ExportRequest, ForgeBackend, ForgeDiagnostics,
    ForgeError, RenderedDiagram,
};
use katana_diagram_renderer::{
    DiagramKind as KdrDiagramKind, DrawioRenderer, MermaidRenderer, PlantUmlRenderer,
    RenderContext, Renderer, RuntimePathResolver,
};
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};
use std::panic::{AssertUnwindSafe, catch_unwind};

pub trait DiagramRenderEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String>;
}

pub struct DiagramRenderRequest<'a> {
    pub node_id: &'a str,
    pub document_id: &'a str,
    pub kind: DiagramKind,
    pub source: String,
    pub theme: &'a crate::KdvThemeSnapshot,
}

pub struct DiagramRenderingBackend<E> {
    engine: E,
}

pub struct KdrDiagramRenderEngine;

impl<E> DiagramRenderingBackend<E> {
    pub fn new(engine: E) -> Self {
        Self { engine }
    }
}

impl<E: DiagramRenderEngine> ForgeBackend for DiagramRenderingBackend<E> {
    fn build(&self, request: &BuildRequest) -> Result<BuildGraph, ForgeError> {
        let mut rendered_diagrams = Vec::new();
        let mut messages = Vec::new();
        for node in &request.snapshot.document.nodes {
            self.collect_node(
                node,
                &request.snapshot.id.0,
                &request.theme,
                &mut rendered_diagrams,
                &mut messages,
            );
        }
        let mut graph = BuildGraph::from_request(request);
        graph.rendered_diagrams = rendered_diagrams;
        graph.diagnostics = ForgeDiagnostics { messages };
        Ok(graph)
    }

    fn export(&self, request: &ExportRequest) -> Result<ExportOutput, ForgeError> {
        let snapshot = &request.graph.snapshot;
        let bytes = ExportPayloadFactory::create(&request.graph, request.format, &request.theme)?;
        let artifact = ArtifactFactory::export(
            request.format.artifact_format(),
            snapshot.id.clone(),
            snapshot.revision.clone(),
            ArtifactBytes { bytes },
        );
        Ok(ExportOutput {
            artifact,
            diagnostics: request.graph.diagnostics.clone(),
        })
    }
}

impl<E: DiagramRenderEngine> DiagramRenderingBackend<E> {
    fn collect_node(
        &self,
        node: &KmmNode,
        document_id: &str,
        theme: &crate::KdvThemeSnapshot,
        rendered_diagrams: &mut Vec<RenderedDiagram>,
        messages: &mut Vec<String>,
    ) {
        if let KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { kind }) = &node.kind {
            self.collect_diagram(
                node,
                document_id,
                kind.clone(),
                theme,
                rendered_diagrams,
                messages,
            );
        }
        if let KmmNodeKind::List(list) = &node.kind {
            for item in &list.items {
                for child in &item.children {
                    self.collect_node(child, document_id, theme, rendered_diagrams, messages);
                }
            }
        }
        for child in &node.children {
            self.collect_node(child, document_id, theme, rendered_diagrams, messages);
        }
    }

    fn collect_diagram(
        &self,
        node: &KmmNode,
        document_id: &str,
        kind: DiagramKind,
        theme: &crate::KdvThemeSnapshot,
        rendered_diagrams: &mut Vec<RenderedDiagram>,
        messages: &mut Vec<String>,
    ) {
        let request = DiagramRenderRequest {
            node_id: &node.id.0,
            document_id,
            kind,
            source: fenced_body(&node.source.raw.text),
            theme,
        };
        match catch_diagram_render(|| self.engine.render(request)) {
            Ok(Ok(diagram)) => rendered_diagrams.push(diagram),
            Ok(Err(message)) => messages.push(message),
            Err(_) => messages.push(format!("diagram renderer panicked for node {}", node.id.0)),
        }
    }
}

fn catch_diagram_render<T>(render: impl FnOnce() -> T) -> Result<T, Box<dyn std::any::Any + Send>> {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = catch_unwind(AssertUnwindSafe(render));
    std::panic::set_hook(previous_hook);
    result
}

impl DiagramRenderEngine for KdrDiagramRenderEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let context = RenderContext {
            document_id: Some(request.document_id.to_string()),
            theme: Some(request.theme.kdr_theme()),
            ..RenderContext::default()
        };
        let input = KdrDiagramInputFactory::create(request.kind.clone(), request.source, context);
        let runtime_path =
            RuntimePathResolver::resolve(input.kind, None).map_err(|error| error.to_string())?;
        let output = match input.kind {
            KdrDiagramKind::Mermaid => MermaidRenderer::with_runtime_path(runtime_path)
                .render(&input)
                .map_err(|error| error.to_string())?,
            KdrDiagramKind::Drawio => DrawioRenderer::with_runtime_path(runtime_path)
                .render(&input)
                .map_err(|error| error.to_string())?,
            KdrDiagramKind::PlantUml => PlantUmlRenderer::with_runtime_path(runtime_path)
                .render(&input)
                .map_err(|error| error.to_string())?,
        };
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: diagram_kind_label(&request.kind).to_string(),
            svg: output.svg,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BuildProfile, DocumentSnapshotFactory, DocumentSource, ForgePipeline, KdvThemeSnapshot,
        SourceKind, SourceRevision, SourceUri,
    };
    use katana_diagram_renderer::{RenderThemeMode, RenderThemeSnapshot};
    use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};
    use std::sync::{Arc, Mutex};

    #[test]
    fn diagram_renderer_panic_is_recorded_as_diagnostic() -> Result<(), Box<dyn std::error::Error>>
    {
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
    fn diagram_renderer_receives_app_supplied_complete_theme()
    -> Result<(), Box<dyn std::error::Error>> {
        let captured_themes = Arc::new(Mutex::new(Vec::new()));
        let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(RecordingDiagramEngine {
            themes: captured_themes.clone(),
        }));
        let mut theme = KdvThemeSnapshot::katana_dark();
        theme.name = "app-supplied-dark".to_string();
        theme.diagram_background = "transparent".to_string();
        theme.diagram_text = "#abcdef".to_string();
        theme.diagram_fill = "#123456".to_string();
        theme.diagram_stroke = "#654321".to_string();
        theme.diagram_arrow = "#fedcba".to_string();
        theme.mermaid_theme = "dark".to_string();
        let graph = pipeline.build(&BuildRequest {
            snapshot: snapshot_from_markdown("```plantuml\n@startuml\nAlice -> Bob\n@enduml\n```")?,
            profile: BuildProfile::markdown_export(),
            theme,
        })?;

        assert_eq!(graph.rendered_diagrams.len(), 1);
        let themes = captured_themes
            .lock()
            .map_err(|error| format!("theme capture lock failed: {error}"))?;
        let captured = themes.first().ok_or("captured KDR theme is missing")?;
        assert_eq!(captured.mode, RenderThemeMode::Dark);
        assert_eq!(captured.background, "transparent");
        assert_eq!(captured.text, "#abcdef");
        assert_eq!(captured.fill, "#123456");
        assert_eq!(captured.stroke, "#654321");
        assert_eq!(captured.arrow, "#fedcba");
        assert_eq!(captured.mermaid_theme, "dark");
        Ok(())
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
                .push(request.theme.kdr_theme());
            Ok(RenderedDiagram {
                node_id: request.node_id.to_string(),
                kind: diagram_kind_label(&request.kind).to_string(),
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
}
