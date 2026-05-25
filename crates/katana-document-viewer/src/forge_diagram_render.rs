use crate::artifact::{ArtifactBytes, ArtifactFactory};
use crate::backend::diagram::KrrDiagramInputFactory;
use crate::export_html_ops::ExportHtmlOps;
use crate::export_payload::ExportPayloadFactory;
use crate::forge::{
    BuildGraph, BuildRequest, ExportOutput, ExportRequest, ForgeBackend, ForgeDiagnostics,
    ForgeError, RenderedDiagram,
};
use crate::forge_diagram_render_types::{
    DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend, KrrDiagramRenderEngine,
};
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};
use katana_render_runtime::{
    DiagramKind as KrrDiagramKind, DrawioRenderer, MermaidRenderer, PlantUmlRenderer,
    RenderContext, Renderer, RuntimePathResolver,
};
use std::panic::{AssertUnwindSafe, catch_unwind};

#[cfg(test)]
#[path = "forge_diagram_render_tests.rs"]
mod forge_diagram_render_tests;

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
            source: ExportHtmlOps::fenced_body(&node.source.raw.text),
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

impl DiagramRenderEngine for KrrDiagramRenderEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let context = RenderContext {
            document_id: Some(request.document_id.to_string()),
            theme: Some(request.theme.krr_theme()),
            ..RenderContext::default()
        };
        let input = KrrDiagramInputFactory::create(request.kind.clone(), request.source, context);
        let runtime_path =
            RuntimePathResolver::resolve(input.kind, None).map_err(|error| error.to_string())?;
        let output = match input.kind {
            KrrDiagramKind::Mermaid => MermaidRenderer::with_runtime_path(runtime_path)
                .render(&input)
                .map_err(|error| error.to_string())?,
            KrrDiagramKind::Drawio => DrawioRenderer::with_runtime_path(runtime_path)
                .render(&input)
                .map_err(|error| error.to_string())?,
            KrrDiagramKind::PlantUml => PlantUmlRenderer::with_runtime_path(runtime_path)
                .render(&input)
                .map_err(|error| error.to_string())?,
            KrrDiagramKind::MathJax => {
                return Err("MathJax is handled by KRR math runtime".to_string());
            }
        };
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: ExportHtmlOps::diagram_kind_label(&request.kind).to_string(),
            svg: output.svg,
        })
    }
}
