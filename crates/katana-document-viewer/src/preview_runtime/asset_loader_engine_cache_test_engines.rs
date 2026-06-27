use crate::{
    DiagramRenderCacheOptions, DiagramRenderEngine, DiagramRenderRequest, RenderedDiagram,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) fn first_engine(count: Arc<AtomicUsize>) -> Arc<dyn DiagramRenderEngine + Send + Sync> {
    Arc::new(FirstDiagramEngine { count })
}

pub(super) fn second_engine(count: Arc<AtomicUsize>) -> Arc<dyn DiagramRenderEngine + Send + Sync> {
    Arc::new(SecondDiagramEngine { count })
}

pub(super) fn configured_engine(
    count: Arc<AtomicUsize>,
    label: &'static str,
    dpi: u32,
    renderer_options: &str,
) -> Arc<dyn DiagramRenderEngine + Send + Sync> {
    Arc::new(ConfiguredDiagramEngine {
        count,
        label,
        cache_options: DiagramRenderCacheOptions {
            dpi,
            renderer_options: renderer_options.to_string(),
        },
    })
}

struct FirstDiagramEngine {
    count: Arc<AtomicUsize>,
}

impl DiagramRenderEngine for FirstDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.count.fetch_add(1, Ordering::SeqCst);
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: "<svg><text>first</text></svg>".to_string(),
        })
    }
}

struct SecondDiagramEngine {
    count: Arc<AtomicUsize>,
}

impl DiagramRenderEngine for SecondDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.count.fetch_add(1, Ordering::SeqCst);
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: "<svg><text>second</text></svg>".to_string(),
        })
    }
}

struct ConfiguredDiagramEngine {
    count: Arc<AtomicUsize>,
    label: &'static str,
    cache_options: DiagramRenderCacheOptions,
}

impl DiagramRenderEngine for ConfiguredDiagramEngine {
    fn cache_namespace(&self) -> &'static str {
        "configured"
    }

    fn cache_options(&self) -> DiagramRenderCacheOptions {
        self.cache_options.clone()
    }

    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        self.count.fetch_add(1, Ordering::SeqCst);
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: format!("<svg><text>{}</text></svg>", self.label),
        })
    }
}
