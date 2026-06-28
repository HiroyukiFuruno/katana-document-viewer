use super::media_control_icons::KucMediaControlIconSet;
use katana_document_viewer::{
    DiagramViewportState, ViewerInteractionConfig, ViewerTaskState, ViewerViewport,
};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core::window::WindowId;
use std::collections::{BTreeMap, BTreeSet};

const DEFAULT_SCALE_FACTOR: f32 = 1.0;
const DEFAULT_DPI: f32 = 96.0;

#[derive(Debug, Clone, PartialEq)]
pub struct KucViewerConfig {
    pub window_id: WindowId,
    pub theme: ThemeSnapshot,
    pub viewport: ViewerViewport,
    pub interaction: ViewerInteractionConfig,
    pub diagram_viewports: BTreeMap<String, DiagramViewportState>,
    pub image_viewports: BTreeMap<String, DiagramViewportState>,
    pub task_state_overrides: BTreeMap<String, ViewerTaskState>,
    pub hovered_node_id: Option<String>,
    pub accordion_open_overrides: BTreeMap<String, bool>,
    pub copied_code_node_ids: BTreeSet<String>,
    pub media_control_icons: KucMediaControlIconSet,
    pub export_surface: bool,
    pub scale_factor: f32,
    pub dpi: f32,
}

impl KucViewerConfig {
    #[must_use]
    pub fn new(window_id: impl Into<String>, viewport: ViewerViewport) -> Self {
        Self {
            window_id: WindowId::new(window_id),
            theme: ThemeSnapshot::light(),
            viewport,
            interaction: ViewerInteractionConfig {
                hover_highlight_enabled: false,
                selection_enabled: false,
                image_controls_enabled: false,
                diagram_controls_enabled: false,
                code_controls_enabled: false,
            },
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            hovered_node_id: None,
            accordion_open_overrides: BTreeMap::new(),
            copied_code_node_ids: BTreeSet::new(),
            media_control_icons: KucMediaControlIconSet::katana_default(),
            export_surface: false,
            scale_factor: DEFAULT_SCALE_FACTOR,
            dpi: DEFAULT_DPI,
        }
    }

    #[must_use]
    pub fn theme(mut self, value: ThemeSnapshot) -> Self {
        self.theme = value;
        self
    }

    #[must_use]
    pub fn interaction(mut self, value: ViewerInteractionConfig) -> Self {
        self.interaction = value;
        self
    }

    #[must_use]
    pub fn diagram_viewports(mut self, value: BTreeMap<String, DiagramViewportState>) -> Self {
        self.diagram_viewports = value;
        self
    }

    #[must_use]
    pub fn image_viewports(mut self, value: BTreeMap<String, DiagramViewportState>) -> Self {
        self.image_viewports = value;
        self
    }

    #[must_use]
    pub fn task_state_overrides(mut self, value: BTreeMap<String, ViewerTaskState>) -> Self {
        self.task_state_overrides = value;
        self
    }

    #[must_use]
    pub fn accordion_open_overrides(mut self, value: BTreeMap<String, bool>) -> Self {
        self.accordion_open_overrides = value;
        self
    }

    #[must_use]
    pub fn copied_code_node_ids(mut self, value: BTreeSet<String>) -> Self {
        self.copied_code_node_ids = value;
        self
    }

    #[must_use]
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "KDV keeps the media icon override hook even when the default Storybook binary uses the Katana preset."
        )
    )]
    pub fn media_control_icons(mut self, value: KucMediaControlIconSet) -> Self {
        self.media_control_icons = value;
        self
    }

    #[must_use]
    pub fn export_surface(mut self, value: bool) -> Self {
        self.export_surface = value;
        self
    }
}
