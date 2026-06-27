use crate::artifact::{Artifact, ArtifactId};
use crate::document::{DocumentOutlineItem, DocumentSnapshot};
use crate::theme::KdvThemeSnapshot;
use crate::viewer::search::ViewerSearchState;
use crate::viewer::settings_update::ViewerTypographyConfig;
use katana_markdown_model::{KmmNodeId, SourceSpan};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ViewerPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ViewerVector {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ViewerRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ViewerViewport {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerMode {
    #[default]
    Document,
    Slideshow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerInteractionConfig {
    pub hover_highlight_enabled: bool,
    pub selection_enabled: bool,
    pub image_controls_enabled: bool,
    pub diagram_controls_enabled: bool,
    pub code_controls_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerTarget {
    pub node_id: KmmNodeId,
    pub source: SourceSpan,
    pub artifact_id: ArtifactId,
    pub rect: ViewerRect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerInput {
    pub snapshot: DocumentSnapshot,
    pub artifacts: Vec<Artifact>,
    pub theme: KdvThemeSnapshot,
    pub mode: ViewerMode,
    pub interaction: ViewerInteractionConfig,
    pub typography: ViewerTypographyConfig,
    pub viewport: ViewerViewport,
    pub search: ViewerSearchState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerStateSnapshot {
    pub mode: ViewerMode,
    pub viewport: ViewerViewport,
    pub scroll_y: f32,
    pub content_height: f32,
    pub slideshow: SlideshowState,
    pub interaction: ViewerInteractionConfig,
    pub search: ViewerSearchState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlideshowState {
    pub current_page_index: usize,
    pub max_page_index: usize,
    pub viewport_height: f32,
    pub content_height: f32,
    pub controls_visible: bool,
    pub close_requested: bool,
    pub hover_highlight_enabled: bool,
    pub diagram_controls_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerTocItem {
    pub node_id: KmmNodeId,
    pub level: u8,
    pub text: String,
    pub source: SourceSpan,
    pub anchor_rect: ViewerRect,
    pub anchor_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewerHitTestResponse {
    Hit(ViewerTarget),
    Miss(ViewerPoint),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DiagramViewportState {
    pub zoom: f32,
    pub pan: ViewerVector,
    pub fullscreen_open: bool,
    pub help_requested: bool,
}

impl Default for ViewerInteractionConfig {
    fn default() -> Self {
        Self {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: true,
            diagram_controls_enabled: true,
            code_controls_enabled: true,
        }
    }
}

impl Default for SlideshowState {
    fn default() -> Self {
        Self {
            current_page_index: 0,
            max_page_index: 0,
            viewport_height: 0.0,
            content_height: 0.0,
            controls_visible: true,
            close_requested: false,
            hover_highlight_enabled: false,
            diagram_controls_enabled: false,
        }
    }
}

impl Default for DiagramViewportState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan: ViewerVector { x: 0.0, y: 0.0 },
            fullscreen_open: false,
            help_requested: false,
        }
    }
}

impl ViewerRect {
    pub fn contains(&self, point: ViewerPoint) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

impl ViewerTocItem {
    pub fn from_outline_item(
        item: DocumentOutlineItem,
        anchor_rect: ViewerRect,
        anchor_index: usize,
    ) -> Self {
        Self {
            node_id: item.node_id,
            level: item.level,
            text: item.text,
            source: item.source,
            anchor_rect,
            anchor_index,
        }
    }
}
