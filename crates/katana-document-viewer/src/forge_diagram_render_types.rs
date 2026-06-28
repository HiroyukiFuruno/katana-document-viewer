use crate::forge::RenderedDiagram;
use katana_markdown_model::DiagramKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramRenderCacheOptions {
    pub dpi: u32,
    pub renderer_options: String,
}

impl Default for DiagramRenderCacheOptions {
    fn default() -> Self {
        Self {
            dpi: 96,
            renderer_options: "default".to_string(),
        }
    }
}

pub trait DiagramRenderEngine {
    fn cache_namespace(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn cache_options(&self) -> DiagramRenderCacheOptions {
        DiagramRenderCacheOptions::default()
    }

    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String>;
}

impl<T: DiagramRenderEngine + ?Sized> DiagramRenderEngine for std::sync::Arc<T> {
    fn cache_namespace(&self) -> &'static str {
        self.as_ref().cache_namespace()
    }

    fn cache_options(&self) -> DiagramRenderCacheOptions {
        self.as_ref().cache_options()
    }

    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.as_ref().render(request)
    }
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
