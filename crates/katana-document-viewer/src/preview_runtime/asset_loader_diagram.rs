use crate::preview_runtime::asset_loader::PreviewAssetLoader;
use crate::preview_runtime::asset_loader_cache::{
    PreviewDiagramAssetCache, PreviewDiagramAssetCacheKey,
};
use crate::preview_runtime::asset_loader_media_types::{
    DiagramArtifactContext, PreviewAssetDiagnostics,
};
use crate::preview_runtime::asset_loader_support::PreviewAssetLoaderSupport;
use crate::{Artifact, ArtifactFormat, DiagramRenderEngine, DiagramRenderRequest};

impl<E: DiagramRenderEngine> PreviewAssetLoader<E> {
    pub(crate) fn render_diagram_artifact(&self, context: DiagramArtifactContext<'_>) -> Artifact {
        let (bytes, diagnostics) =
            render_diagram_payload(&self.engine, self.diagram_cache_root(), &context);
        Self::image_artifact(
            context.output,
            context.request.artifact_id.clone(),
            ArtifactFormat::Svg,
            bytes,
            diagnostics,
        )
    }
}

fn render_diagram_payload(
    engine: &dyn DiagramRenderEngine,
    disk_root: Option<&std::path::Path>,
    context: &DiagramArtifactContext<'_>,
) -> (Vec<u8>, crate::ArtifactDiagnostics) {
    let cache_key = PreviewDiagramAssetCache::key(
        engine.cache_namespace(),
        context.output,
        context.node,
        &context.kind,
        &context.source,
        context.theme,
        &engine.cache_options(),
    );
    if let Some(cached) = cached_diagram_svg(disk_root, &cache_key) {
        return (cached.svg, PreviewAssetDiagnostics::empty());
    }
    match engine.render(diagram_request(context)) {
        Ok(diagram) => (
            cache_diagram_svg(disk_root, cache_key, diagram.svg),
            PreviewAssetDiagnostics::empty(),
        ),
        Err(message) => (
            context.node.source.raw.text.as_bytes().to_vec(),
            PreviewAssetLoaderSupport::error_diagnostics(message),
        ),
    }
}

fn cached_diagram_svg(
    disk_root: Option<&std::path::Path>,
    cache_key: &PreviewDiagramAssetCacheKey,
) -> Option<crate::preview_runtime::asset_loader_cache::PreviewDiagramAssetCacheValue> {
    if let Some(cached) = PreviewDiagramAssetCache::get(cache_key) {
        return Some(cached);
    }
    let root = disk_root?;
    let cached = PreviewDiagramAssetCache::get_disk(root, cache_key)?;
    PreviewDiagramAssetCache::put(cache_key.clone(), cached.svg.clone());
    Some(cached)
}

fn diagram_request<'a>(context: &'a DiagramArtifactContext<'_>) -> DiagramRenderRequest<'a> {
    DiagramRenderRequest {
        node_id: &context.node.id.0,
        document_id: &context.output.input.snapshot.id.0,
        kind: context.kind.clone(),
        source: context.source.clone(),
        theme: context.theme,
    }
}

fn cache_diagram_svg(
    disk_root: Option<&std::path::Path>,
    cache_key: PreviewDiagramAssetCacheKey,
    svg: String,
) -> Vec<u8> {
    let bytes = svg.into_bytes();
    PreviewDiagramAssetCache::put(cache_key.clone(), bytes.clone());
    if let Some(root) = disk_root {
        PreviewDiagramAssetCache::put_disk(root, &cache_key, &bytes);
    }
    bytes
}
