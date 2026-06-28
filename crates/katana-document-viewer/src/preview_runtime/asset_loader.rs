use crate::preview_runtime::asset_loader_support::PreviewAssetLoaderSupport;
use crate::{
    Artifact, ArtifactId, DiagramRenderEngine, KdvThemeSnapshot, KrrDiagramRenderEngine,
    ViewerAssetLoadPriority, ViewerAssetLoadRequest, ViewerNodePlanner,
};
use crate::{PreviewError, PreviewOutput};
use std::path::{Path, PathBuf};

pub struct PreviewAssetLoader<E = KrrDiagramRenderEngine> {
    pub(crate) engine: E,
    pub(crate) diagram_cache_root: Option<PathBuf>,
}

impl<E: Clone> Clone for PreviewAssetLoader<E> {
    fn clone(&self) -> Self {
        Self {
            engine: self.engine.clone(),
            diagram_cache_root: self.diagram_cache_root.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PreviewAssetLoadReport {
    pub loaded_artifact_count: usize,
    pub failed_artifact_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewAssetLoadScope {
    All,
    VisibleOnly,
}

impl PreviewAssetLoader<KrrDiagramRenderEngine> {
    #[must_use]
    pub fn krr() -> Self {
        Self {
            engine: KrrDiagramRenderEngine,
            diagram_cache_root: None,
        }
    }
}

impl<E> PreviewAssetLoader<E> {
    #[must_use]
    pub fn new(engine: E) -> Self {
        Self {
            engine,
            diagram_cache_root: None,
        }
    }

    #[must_use]
    pub fn with_diagram_cache_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.diagram_cache_root = Some(root.into());
        self
    }

    pub(crate) fn diagram_cache_root(&self) -> Option<&Path> {
        self.diagram_cache_root.as_deref()
    }
}

impl Default for PreviewAssetLoader<KrrDiagramRenderEngine> {
    fn default() -> Self {
        Self::krr()
    }
}

impl<E: DiagramRenderEngine> PreviewAssetLoader<E> {
    pub fn load_asset_request(
        &self,
        output: &PreviewOutput,
        request: &ViewerAssetLoadRequest,
        theme: &KdvThemeSnapshot,
    ) -> Result<Option<Artifact>, PreviewError> {
        self.load_request(output, request, theme)
    }

    pub fn load_requested(
        &self,
        output: &PreviewOutput,
        theme: &KdvThemeSnapshot,
    ) -> Result<(PreviewOutput, PreviewAssetLoadReport), PreviewError> {
        self.load_requested_with_scope(output, theme, PreviewAssetLoadScope::All)
    }

    pub fn load_visible_requested(
        &self,
        output: &PreviewOutput,
        theme: &KdvThemeSnapshot,
    ) -> Result<(PreviewOutput, PreviewAssetLoadReport), PreviewError> {
        self.load_requested_with_scope(output, theme, PreviewAssetLoadScope::VisibleOnly)
    }

    fn load_requested_with_scope(
        &self,
        output: &PreviewOutput,
        theme: &KdvThemeSnapshot,
        scope: PreviewAssetLoadScope,
    ) -> Result<(PreviewOutput, PreviewAssetLoadReport), PreviewError> {
        let mut loaded = output.clone();
        let mut report = PreviewAssetLoadReport::default();
        let plan = ViewerNodePlanner::create(&loaded.input, loaded.scroll_offset);
        for request in plan
            .asset_requests
            .iter()
            .filter(|request| Self::should_load(request, scope))
        {
            if Self::artifact_exists(&loaded.input.artifacts, &request.artifact_id) {
                continue;
            }
            if let Some(artifact) = self.load_request(&loaded, request, theme)? {
                if PreviewAssetLoaderSupport::has_error(&artifact) {
                    report.failed_artifact_count += 1;
                } else {
                    report.loaded_artifact_count += 1;
                }
                loaded.input.artifacts.push(artifact);
            }
        }
        Ok((loaded, report))
    }

    pub(crate) fn should_load(
        request: &ViewerAssetLoadRequest,
        scope: PreviewAssetLoadScope,
    ) -> bool {
        match scope {
            PreviewAssetLoadScope::All => true,
            PreviewAssetLoadScope::VisibleOnly => {
                request.priority == ViewerAssetLoadPriority::Visible
            }
        }
    }

    pub(crate) fn artifact_exists(artifacts: &[Artifact], artifact_id: &ArtifactId) -> bool {
        artifacts
            .iter()
            .any(|artifact| artifact.manifest.id == *artifact_id)
    }
}
