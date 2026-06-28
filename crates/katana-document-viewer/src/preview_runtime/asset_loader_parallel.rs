use crate::preview_runtime::asset_loader::PreviewAssetLoadScope;
use crate::preview_runtime::asset_loader_support::PreviewAssetLoaderSupport;
use crate::{
    Artifact, DiagramRenderEngine, KdvThemeSnapshot, ViewerAssetLoadRequest, ViewerNodePlanner,
};
use crate::{PreviewAssetLoadReport, PreviewAssetLoader, PreviewError, PreviewOutput};

type AssetWorkerHandle<'scope> =
    std::thread::ScopedJoinHandle<'scope, Result<Option<Artifact>, PreviewError>>;

impl<E: DiagramRenderEngine + Sync> PreviewAssetLoader<E> {
    pub fn load_requested_parallel(
        &self,
        output: &PreviewOutput,
        theme: &KdvThemeSnapshot,
    ) -> Result<(PreviewOutput, PreviewAssetLoadReport), PreviewError> {
        self.load_requested_with_scope_parallel(output, theme, PreviewAssetLoadScope::All)
    }

    pub fn load_visible_requested_parallel(
        &self,
        output: &PreviewOutput,
        theme: &KdvThemeSnapshot,
    ) -> Result<(PreviewOutput, PreviewAssetLoadReport), PreviewError> {
        self.load_requested_with_scope_parallel(output, theme, PreviewAssetLoadScope::VisibleOnly)
    }

    fn load_requested_with_scope_parallel(
        &self,
        output: &PreviewOutput,
        theme: &KdvThemeSnapshot,
        scope: PreviewAssetLoadScope,
    ) -> Result<(PreviewOutput, PreviewAssetLoadReport), PreviewError> {
        let requests = Self::pending_requests(output, scope);
        let artifacts = self.load_artifacts_parallel(output, theme, &requests)?;
        Ok(Self::output_with_artifacts(output, artifacts))
    }

    fn pending_requests(
        output: &PreviewOutput,
        scope: PreviewAssetLoadScope,
    ) -> Vec<ViewerAssetLoadRequest> {
        ViewerNodePlanner::create(&output.input, output.scroll_offset)
            .asset_requests
            .into_iter()
            .filter(|request| Self::should_load(request, scope))
            .filter(|request| !Self::artifact_exists(&output.input.artifacts, &request.artifact_id))
            .collect()
    }

    fn load_artifacts_parallel(
        &self,
        output: &PreviewOutput,
        theme: &KdvThemeSnapshot,
        requests: &[ViewerAssetLoadRequest],
    ) -> Result<Vec<Artifact>, PreviewError> {
        std::thread::scope(|scope| {
            let handles = requests
                .iter()
                .map(|request| scope.spawn(move || self.load_request(output, request, theme)))
                .collect::<Vec<_>>();
            Self::join_asset_workers(handles)
        })
    }

    fn join_asset_workers(
        handles: Vec<AssetWorkerHandle<'_>>,
    ) -> Result<Vec<Artifact>, PreviewError> {
        let mut artifacts = Vec::new();
        for handle in handles {
            if let Some(artifact) = Self::join_asset_worker(handle)? {
                artifacts.push(artifact);
            }
        }
        Ok(artifacts)
    }

    fn join_asset_worker(handle: AssetWorkerHandle<'_>) -> Result<Option<Artifact>, PreviewError> {
        handle
            .join()
            .map_err(|_| PreviewError::Render("asset loader worker panicked".to_string()))?
    }

    fn output_with_artifacts(
        output: &PreviewOutput,
        artifacts: Vec<Artifact>,
    ) -> (PreviewOutput, PreviewAssetLoadReport) {
        let mut loaded = output.clone();
        let report = Self::asset_report(&artifacts);
        loaded.input.artifacts.extend(artifacts);
        (loaded, report)
    }

    fn asset_report(artifacts: &[Artifact]) -> PreviewAssetLoadReport {
        artifacts
            .iter()
            .fold(Default::default(), |mut report, artifact| {
                if PreviewAssetLoaderSupport::has_error(artifact) {
                    report.failed_artifact_count += 1;
                } else {
                    report.loaded_artifact_count += 1;
                }
                report
            })
    }
}
