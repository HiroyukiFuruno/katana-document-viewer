use crate::DocumentViewerStorybookHost;
pub use crate::preview_scene::PreviewScene;
pub(crate) use asset_events::PreviewAssetSceneEvent;
use katana_document_viewer::{
    DiagramRenderEngine, KrrDiagramRenderEngine, PreviewAssetLoader, PreviewRenderEngine,
};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

type StorybookDiagramEngine = Arc<dyn DiagramRenderEngine + Send + Sync>;

pub struct PreviewBuilder {
    engine: PreviewRenderEngine,
    host: DocumentViewerStorybookHost,
    loader: PreviewAssetLoader<StorybookDiagramEngine>,
    source_cache: Arc<Mutex<preview_source_cache::PreviewSourceCache>>,
    cache: Arc<Mutex<preview_cache::PreviewBuilderCache>>,
}

impl Default for PreviewBuilder {
    fn default() -> Self {
        Self {
            engine: PreviewRenderEngine,
            host: DocumentViewerStorybookHost::default(),
            loader: Self::asset_loader(),
            source_cache: Arc::new(Mutex::new(
                preview_source_cache::PreviewSourceCache::default(),
            )),
            cache: Arc::new(Mutex::new(preview_cache::PreviewBuilderCache::default())),
        }
    }
}

impl Clone for PreviewBuilder {
    fn clone(&self) -> Self {
        Self {
            engine: PreviewRenderEngine,
            host: self.host,
            loader: self.loader.clone(),
            source_cache: self.source_cache.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl PreviewBuilder {
    fn asset_loader() -> PreviewAssetLoader<StorybookDiagramEngine> {
        Self::asset_loader_with_engine(Arc::new(KrrDiagramRenderEngine))
    }

    fn asset_loader_with_engine(
        engine: StorybookDiagramEngine,
    ) -> PreviewAssetLoader<StorybookDiagramEngine> {
        PreviewAssetLoader::new(engine).with_diagram_cache_root(Self::diagram_cache_root())
    }

    fn diagram_cache_root() -> PathBuf {
        std::env::var_os("KDV_DIAGRAM_CACHE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("target/kdv-diagram-cache"))
    }
}

#[cfg(test)]
impl PreviewBuilder {
    pub(crate) fn with_diagram_engine(engine: StorybookDiagramEngine) -> Self {
        Self {
            loader: Self::asset_loader_with_engine(engine),
            ..Self::default()
        }
    }

    pub(crate) fn with_diagram_engine_and_cache_root(
        engine: StorybookDiagramEngine,
        root: PathBuf,
    ) -> Self {
        Self {
            loader: PreviewAssetLoader::new(engine).with_diagram_cache_root(root),
            ..Self::default()
        }
    }

    pub(crate) fn source_cache_stats(
        &self,
    ) -> Result<preview_source_cache::PreviewSourceCacheStats, std::io::Error> {
        Ok(self
            .source_cache
            .lock()
            .map_err(|_| std::io::Error::other("preview source cache lock poisoned"))?
            .stats())
    }

    pub(crate) fn builder_cache_stats(
        &self,
    ) -> Result<preview_cache::PreviewBuilderCacheStats, std::io::Error> {
        Ok(self
            .cache
            .lock()
            .map_err(|_| std::io::Error::other("preview builder cache lock poisoned"))?
            .stats())
    }
}

#[path = "preview_cache.rs"]
mod preview_cache;

#[path = "preview_source_cache.rs"]
mod preview_source_cache;

#[cfg(test)]
#[path = "preview_build_methods.rs"]
mod build_methods;

#[path = "preview_build_scene.rs"]
mod build_scene;

#[path = "preview_scene_from_output.rs"]
mod scene_from_output;

#[path = "preview_asset_request_scope.rs"]
mod preview_asset_request_scope;

#[path = "preview_asset_events.rs"]
mod asset_events;

#[cfg(test)]
#[path = "preview_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "preview_cache_tests.rs"]
mod cache_tests;

#[cfg(test)]
#[path = "preview_diagram_disk_cache_tests.rs"]
mod diagram_disk_cache_tests;

#[cfg(test)]
#[path = "preview_source_cache_tests.rs"]
mod source_cache_tests;

#[cfg(test)]
#[path = "preview_feature_matrix_tests.rs"]
mod feature_matrix_tests;
#[cfg(test)]
#[path = "preview_search_tests.rs"]
mod search_tests;
#[cfg(test)]
#[path = "preview_slideshow_tests.rs"]
mod slideshow_tests;
