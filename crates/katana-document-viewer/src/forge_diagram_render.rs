use crate::artifact::{ArtifactBytes, ArtifactFactory};
use crate::backend::diagram::KrrDiagramInputFactory;
use crate::export_html_ops::ExportHtmlOps;
use crate::export_payload::ExportPayloadFactory;
use crate::forge::{
    BuildGraph, BuildRequest, ExportOutput, ExportRequest, ForgeBackend, ForgeDiagnostics,
    ForgeError, RenderedDiagram,
};
use crate::forge_diagram_render_types::{
    DiagramRenderCacheOptions, DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend,
    KrrDiagramRenderEngine,
};
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};
use katana_render_runtime::markdown::{
    drawio_renderer::{DRAWIO_JS_CHECKSUM, DRAWIO_JS_VERSION},
    mermaid_renderer::{MERMAID_JS_CHECKSUM, MERMAID_JS_VERSION},
    plantuml_renderer::{PLANTUML_JAR_CHECKSUM, PLANTUML_JAR_VERSION},
};
use katana_render_runtime::{
    DrawioRenderer, MermaidRenderer, PlantUmlRenderer, RenderContext, Renderer, RuntimePathResolver,
};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Mutex, MutexGuard};

#[cfg(test)]
#[path = "forge_diagram_render_runtime_tests.rs"]
mod forge_diagram_render_runtime_tests;
#[cfg(test)]
#[path = "forge_diagram_render_tests.rs"]
mod forge_diagram_render_tests;

static KRR_PLANTUML_RENDER_LOCK: Mutex<()> = Mutex::new(());

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
        ExportPayloadFactory::create(&request.graph, request.format, &request.theme).map(|bytes| {
            let artifact = ArtifactFactory::export(
                request.format.artifact_format(),
                snapshot.id.clone(),
                snapshot.revision.clone(),
                ArtifactBytes { bytes },
            );
            ExportOutput {
                artifact,
                diagnostics: request.graph.diagnostics.clone(),
            }
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
        record_diagram_result(
            &node.id.0,
            catch_diagram_render(|| self.engine.render(request)),
            rendered_diagrams,
            messages,
        );
    }
}

fn record_diagram_result(
    node_id: &str,
    result: Result<Result<RenderedDiagram, String>, Box<dyn std::any::Any + Send>>,
    rendered_diagrams: &mut Vec<RenderedDiagram>,
    messages: &mut Vec<String>,
) {
    match result {
        Ok(Ok(diagram)) => rendered_diagrams.push(diagram),
        Ok(Err(message)) => {
            let diagnostic = format!("diagram renderer failed for node {node_id}: {message}");
            eprintln!("[kdv-render-runtime] {diagnostic}");
            messages.push(diagnostic);
        }
        Err(_) => {
            let diagnostic = format!("diagram renderer panicked for node {node_id}");
            eprintln!("[kdv-render-runtime] {diagnostic}");
            messages.push(diagnostic);
        }
    }
}

fn catch_diagram_render<T>(render: impl FnOnce() -> T) -> Result<T, Box<dyn std::any::Any + Send>> {
    catch_unwind(AssertUnwindSafe(render))
}

impl DiagramRenderEngine for KrrDiagramRenderEngine {
    fn cache_options(&self) -> DiagramRenderCacheOptions {
        DiagramRenderCacheOptions {
            dpi: 96,
            renderer_options: krr_renderer_cache_options(),
        }
    }

    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let context = RenderContext {
            document_id: Some(request.document_id.to_string()),
            theme: Some(request.theme.krr_theme_for_diagram(&request.kind)),
            ..RenderContext::default()
        };
        let input =
            KrrDiagramInputFactory::create(request.kind.clone(), request.source.clone(), context);
        match RuntimePathResolver::resolve(input.kind, None) {
            Ok(runtime_path) => {
                let rendered = match request.kind {
                    DiagramKind::Mermaid => {
                        MermaidRenderer::with_runtime_path(runtime_path).render(&input)
                    }
                    DiagramKind::DrawIo => {
                        DrawioRenderer::with_runtime_path(runtime_path).render(&input)
                    }
                    DiagramKind::PlantUml => with_krr_plantuml_render_lock(|| {
                        PlantUmlRenderer::with_runtime_path(runtime_path).render(&input)
                    }),
                };
                Self::rendered_diagram_from_output(request, rendered)
            }
            Err(error) => Err(krr_error_message(error)),
        }
    }
}

fn krr_renderer_cache_options() -> String {
    format!(
        "kdv={};mermaid={}:{};drawio={}:{};plantuml={}:{}",
        env!("CARGO_PKG_VERSION"),
        MERMAID_JS_VERSION,
        MERMAID_JS_CHECKSUM,
        DRAWIO_JS_VERSION,
        DRAWIO_JS_CHECKSUM,
        PLANTUML_JAR_VERSION,
        PLANTUML_JAR_CHECKSUM
    )
}

impl KrrDiagramRenderEngine {
    fn rendered_diagram_from_output(
        request: DiagramRenderRequest<'_>,
        rendered: Result<katana_render_runtime::RenderOutput, katana_render_runtime::RenderError>,
    ) -> Result<RenderedDiagram, String> {
        match rendered {
            Ok(output) => Ok(RenderedDiagram {
                node_id: request.node_id.to_string(),
                kind: ExportHtmlOps::diagram_kind_label(&request.kind).to_string(),
                svg: output.svg,
            }),
            Err(error) => Err(krr_error_message(error)),
        }
    }
}

fn with_krr_plantuml_render_lock<T>(render: impl FnOnce() -> T) -> T {
    let _guard = krr_plantuml_render_guard();
    render()
}

fn krr_plantuml_render_guard() -> MutexGuard<'static, ()> {
    match KRR_PLANTUML_RENDER_LOCK.lock() {
        Ok(guard) => guard,
        Err(error) => error.into_inner(),
    }
}

fn krr_error_message(error: impl std::fmt::Display) -> String {
    error.to_string()
}
