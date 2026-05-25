use crate::forge::RenderedDiagram;
use katana_markdown_model::DiagramKind;

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
    pub(crate) engine: E,
}

pub struct KrrDiagramRenderEngine;
