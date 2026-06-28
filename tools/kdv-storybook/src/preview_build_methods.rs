use super::{PreviewBuilder, PreviewScene};
use crate::catalog::StorybookFixture;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use katana_document_viewer::{
    KdvThemeSnapshot, ViewerInteractionConfig, ViewerMode, ViewerSearchState,
    ViewerTypographyConfig, ViewerViewport,
};
use std::collections::BTreeMap;

impl PreviewBuilder {
    #[cfg(test)]
    pub fn build(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        interaction: ViewerInteractionConfig,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_with_mode(fixture, viewport, dark, interaction, ViewerMode::Document)
    }

    #[cfg(test)]
    pub fn build_with_mode(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        interaction: ViewerInteractionConfig,
        mode: ViewerMode,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_with_mode_and_search(
            fixture,
            viewport,
            dark,
            interaction,
            mode,
            ViewerSearchState::default(),
        )
    }

    #[cfg(test)]
    pub fn build_with_mode_and_search(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        interaction: ViewerInteractionConfig,
        mode: ViewerMode,
        search: ViewerSearchState,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_scene(PreviewBuildRequest {
            viewport,
            fixture,
            dark,
            theme: None,
            interaction,
            mode,
            typography: ViewerTypographyConfig::default(),
            search,
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides: BTreeMap::new(),
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface: true,
            export_surface: false,
        })
    }

    #[cfg(test)]
    pub fn build_with_typography(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        typography: ViewerTypographyConfig,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_with_typography_and_interaction(
            fixture,
            viewport,
            dark,
            typography,
            ViewerInteractionConfig::default(),
        )
    }

    #[cfg(test)]
    pub fn build_with_typography_and_interaction(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        typography: ViewerTypographyConfig,
        interaction: ViewerInteractionConfig,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_scene(PreviewBuildRequest {
            viewport,
            fixture,
            dark,
            theme: None,
            interaction,
            mode: ViewerMode::Document,
            typography,
            search: ViewerSearchState::default(),
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides: BTreeMap::new(),
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface: false,
            export_surface: false,
        })
    }

    #[cfg(test)]
    pub fn build_without_preview_surface(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        interaction: ViewerInteractionConfig,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_scene(PreviewBuildRequest {
            viewport,
            fixture,
            dark,
            theme: None,
            interaction,
            mode: ViewerMode::Document,
            typography: ViewerTypographyConfig::default(),
            search: ViewerSearchState::default(),
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides: BTreeMap::new(),
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface: false,
            export_surface: false,
        })
    }

    #[cfg(test)]
    pub fn build_lazy_with_mode_and_search(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        interaction: ViewerInteractionConfig,
        mode: ViewerMode,
        search: ViewerSearchState,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_scene(PreviewBuildRequest {
            viewport,
            fixture,
            dark,
            theme: None,
            interaction,
            mode,
            typography: Default::default(),
            search,
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides: BTreeMap::new(),
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::Lazy,
            attach_surface: false,
            export_surface: false,
        })
    }

    #[cfg(test)]
    pub fn build_surface_with_export_reference_theme(
        &self,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        typography: ViewerTypographyConfig,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        self.build_scene(PreviewBuildRequest {
            viewport,
            fixture,
            dark: false,
            theme: Some(KdvThemeSnapshot::katana_export_reference()),
            interaction: Self::score_comparison_interaction(),
            mode: ViewerMode::Document,
            typography,
            search: ViewerSearchState::default(),
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides: BTreeMap::new(),
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface: true,
            export_surface: true,
        })
    }

    #[cfg(test)]
    fn score_comparison_interaction() -> ViewerInteractionConfig {
        ViewerInteractionConfig {
            hover_highlight_enabled: false,
            selection_enabled: false,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: false,
        }
    }
}
