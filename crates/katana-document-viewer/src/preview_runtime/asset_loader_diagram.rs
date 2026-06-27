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
        let cache_key = PreviewDiagramAssetCache::key(
            self.engine.cache_namespace(),
            context.output,
            context.node,
            &context.kind,
            &context.source,
            context.theme,
            &self.engine.cache_options(),
        );
        if let Some(artifact) = self.cached_diagram_artifact(&context, &cache_key) {
            return artifact;
        }
        self.render_fresh_diagram_artifact(&context, cache_key)
    }

    fn cached_diagram_artifact(
        &self,
        context: &DiagramArtifactContext<'_>,
        cache_key: &PreviewDiagramAssetCacheKey,
    ) -> Option<Artifact> {
        let cached = self.cached_diagram_svg(cache_key)?;
        Some(Self::image_artifact(
            context.output,
            context.request.artifact_id.clone(),
            ArtifactFormat::Svg,
            cached.svg,
            PreviewAssetDiagnostics::empty(),
        ))
    }

    fn cached_diagram_svg(
        &self,
        cache_key: &PreviewDiagramAssetCacheKey,
    ) -> Option<crate::preview_runtime::asset_loader_cache::PreviewDiagramAssetCacheValue> {
        if let Some(cached) = PreviewDiagramAssetCache::get(cache_key) {
            return Some(cached);
        }
        let root = self.diagram_cache_root()?;
        let cached = PreviewDiagramAssetCache::get_disk(root, cache_key)?;
        PreviewDiagramAssetCache::put(cache_key.clone(), cached.svg.clone());
        Some(cached)
    }

    fn render_fresh_diagram_artifact(
        &self,
        context: &DiagramArtifactContext<'_>,
        cache_key: PreviewDiagramAssetCacheKey,
    ) -> Artifact {
        match self.engine.render(Self::diagram_request(context)) {
            Ok(diagram) => self.cache_diagram_artifact(context, cache_key, diagram.svg),
            Err(message) => Self::image_artifact(
                context.output,
                context.request.artifact_id.clone(),
                ArtifactFormat::Svg,
                context.node.source.raw.text.as_bytes().to_vec(),
                PreviewAssetLoaderSupport::error_diagnostics(message),
            ),
        }
    }

    fn diagram_request<'b>(context: &'b DiagramArtifactContext<'_>) -> DiagramRenderRequest<'b> {
        DiagramRenderRequest {
            node_id: &context.node.id.0,
            document_id: &context.output.input.snapshot.id.0,
            kind: context.kind.clone(),
            source: context.source.clone(),
            theme: context.theme,
        }
    }

    fn cache_diagram_artifact(
        &self,
        context: &DiagramArtifactContext<'_>,
        cache_key: PreviewDiagramAssetCacheKey,
        svg: String,
    ) -> Artifact {
        let bytes = svg.into_bytes();
        PreviewDiagramAssetCache::put(cache_key, bytes.clone());
        if let Some(root) = self.diagram_cache_root() {
            let disk_key = PreviewDiagramAssetCache::key(
                self.engine.cache_namespace(),
                context.output,
                context.node,
                &context.kind,
                &context.source,
                context.theme,
                &self.engine.cache_options(),
            );
            PreviewDiagramAssetCache::put_disk(root, &disk_key, &bytes);
        }
        Self::image_artifact(
            context.output,
            context.request.artifact_id.clone(),
            ArtifactFormat::Svg,
            bytes,
            PreviewAssetDiagnostics::empty(),
        )
    }
}
