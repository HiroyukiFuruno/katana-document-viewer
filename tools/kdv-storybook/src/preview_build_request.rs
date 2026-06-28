use crate::catalog::StorybookFixture;
use katana_document_viewer::{
    DiagramViewportState, KdvThemeSnapshot, ViewerInteractionConfig, ViewerMode, ViewerSearchState,
    ViewerTaskState, ViewerTypographyConfig, ViewerViewport,
};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct PreviewBuildRequest<'a> {
    pub(crate) fixture: &'a StorybookFixture,
    pub(crate) viewport: ViewerViewport,
    pub(crate) dark: bool,
    pub(crate) theme: Option<KdvThemeSnapshot>,
    pub(crate) interaction: ViewerInteractionConfig,
    pub(crate) mode: ViewerMode,
    pub(crate) typography: ViewerTypographyConfig,
    pub(crate) search: ViewerSearchState,
    pub(crate) diagram_viewports: BTreeMap<String, DiagramViewportState>,
    pub(crate) image_viewports: BTreeMap<String, DiagramViewportState>,
    pub(crate) task_state_overrides: BTreeMap<String, ViewerTaskState>,
    pub(crate) accordion_open_overrides: BTreeMap<String, bool>,
    pub(crate) copied_code_node_ids: BTreeSet<String>,
    pub(crate) asset_mode: PreviewBuildAssetMode,
    pub(crate) attach_surface: bool,
    pub(crate) export_surface: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreviewBuildAssetMode {
    Lazy,
    VisibleAndNearViewport,
}

impl PreviewBuildRequest<'_> {
    pub(crate) fn scene_scroll_y(&self) -> f32 {
        0.0
    }
}
