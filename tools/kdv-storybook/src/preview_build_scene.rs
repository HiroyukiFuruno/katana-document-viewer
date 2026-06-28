use super::{PreviewBuilder, PreviewScene};
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use crate::preview_build_support::PreviewBuildSupport;
use katana_document_viewer::{
    ArtifactFormat, MarkdownSource, PreviewConfig, PreviewOutput, ViewerAssetLoadRequest,
    ViewerNodePlan,
};
use std::io;

impl PreviewBuilder {
    pub(crate) fn build_scene(
        &self,
        request: PreviewBuildRequest<'_>,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        let source = self.source_for_fixture(request.fixture)?;
        if let Some(scene) = self.lazy_scene(&source, &request)? {
            return Ok(scene);
        }
        let theme = request
            .theme
            .clone()
            .unwrap_or_else(|| PreviewBuildSupport::kdv_theme(request.dark));
        let config = PreviewBuildSupport::preview_config_for_theme(
            request.viewport,
            request.scene_scroll_y(),
            theme.clone(),
            request.interaction.clone(),
            request.mode.clone(),
            request.typography,
            request.search.clone(),
        );
        let output = self.render_output(&source, &config)?;
        let (output, asset_report) = match request.asset_mode {
            PreviewBuildAssetMode::Lazy => (output, Default::default()),
            PreviewBuildAssetMode::VisibleAndNearViewport => {
                let output =
                    self.output_with_cached_artifacts(&source, &config, request.dark, output)?;
                self.loader.load_requested_parallel(&output, &theme)?
            }
        };
        if request.asset_mode != PreviewBuildAssetMode::Lazy {
            self.store_artifacts(&source, &config, request.dark, &output)?;
        }
        let scene = self.scene_from_output(&source, &request, output, asset_report)?;
        self.store_lazy_scene(&source, &request, &scene)?;
        Ok(scene)
    }

    pub(crate) fn source_for_fixture(
        &self,
        fixture: &crate::catalog::StorybookFixture,
    ) -> Result<MarkdownSource, Box<dyn std::error::Error>> {
        self.source_cache
            .lock()
            .map_err(|_| Self::cache_lock_error("preview source"))?
            .source_for_fixture(fixture)
    }

    fn lazy_scene(
        &self,
        source: &MarkdownSource,
        request: &PreviewBuildRequest<'_>,
    ) -> Result<Option<PreviewScene>, Box<dyn std::error::Error>> {
        Ok(self
            .cache
            .lock()
            .map_err(|_| Self::cache_lock_error("preview builder"))?
            .lazy_scene(source, request))
    }

    pub(crate) fn render_output(
        &self,
        source: &MarkdownSource,
        config: &PreviewConfig,
    ) -> Result<katana_document_viewer::PreviewOutput, katana_document_viewer::PreviewError> {
        self.cache
            .lock()
            .map_err(|_| {
                katana_document_viewer::PreviewError::Render(
                    "preview builder cache lock poisoned".to_string(),
                )
            })?
            .parsed_output(&self.engine, source, config)
    }

    pub(crate) fn output_with_cached_artifacts(
        &self,
        source: &MarkdownSource,
        config: &PreviewConfig,
        dark: bool,
        output: katana_document_viewer::PreviewOutput,
    ) -> Result<katana_document_viewer::PreviewOutput, katana_document_viewer::PreviewError> {
        Ok(self
            .cache
            .lock()
            .map_err(|_| {
                katana_document_viewer::PreviewError::Render(
                    "preview builder cache lock poisoned".to_string(),
                )
            })?
            .output_with_artifacts(source, config, dark, output))
    }

    pub(crate) fn store_artifacts(
        &self,
        source: &MarkdownSource,
        config: &PreviewConfig,
        dark: bool,
        output: &PreviewOutput,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.cache
            .lock()
            .map_err(|_| Self::cache_lock_error("preview builder"))?
            .store_artifacts(source, config, dark, output);
        Ok(())
    }

    fn store_lazy_scene(
        &self,
        source: &MarkdownSource,
        request: &PreviewBuildRequest<'_>,
        scene: &PreviewScene,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.cache
            .lock()
            .map_err(|_| Self::cache_lock_error("preview builder"))?
            .store_lazy_scene(source, request, scene);
        Ok(())
    }

    fn cache_lock_error(name: &str) -> io::Error {
        io::Error::other(format!("{name} cache lock poisoned"))
    }

    pub(crate) fn pending_asset_count(output: &PreviewOutput, plan: &ViewerNodePlan) -> usize {
        plan.asset_requests
            .iter()
            .filter(|request| Self::loads_in_storybook_scope(request))
            .filter(|request| {
                !output
                    .input
                    .artifacts
                    .iter()
                    .any(|artifact| artifact.manifest.id == request.artifact_id)
            })
            .count()
    }

    pub(crate) fn pending_asset_key(output: &PreviewOutput, plan: &ViewerNodePlan) -> String {
        plan.asset_requests
            .iter()
            .filter(|request| Self::loads_in_storybook_scope(request))
            .filter(|request| {
                !output
                    .input
                    .artifacts
                    .iter()
                    .any(|artifact| artifact.manifest.id == request.artifact_id)
            })
            .map(|request| request.artifact_id.0.clone())
            .collect::<Vec<_>>()
            .join(";")
    }

    pub(crate) fn loads_in_storybook_scope(request: &ViewerAssetLoadRequest) -> bool {
        request.format != ArtifactFormat::Html
    }
}
