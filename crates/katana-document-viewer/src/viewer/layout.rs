use crate::viewer::types::{ViewerRect, ViewerTarget, ViewerViewport};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerVisibleRange {
    pub start_y: f32,
    pub end_y: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerRenderedAnchor {
    pub target: ViewerTarget,
    pub anchor_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerLayoutResult {
    pub anchors: Vec<ViewerRenderedAnchor>,
    pub visible_range: ViewerVisibleRange,
    pub content_height: f32,
    pub scrollable_content_height: f32,
    pub bottom_spacer_height: f32,
}

pub struct ViewerLayoutEngine;

impl ViewerLayoutEngine {
    pub fn from_anchors(
        viewport: ViewerViewport,
        anchors: Vec<ViewerRenderedAnchor>,
        content_height: f32,
        scroll_y: f32,
    ) -> ViewerLayoutResult {
        let bottom_spacer_height = Self::bottom_spacer_height(viewport, &anchors, content_height);
        ViewerLayoutResult {
            anchors,
            visible_range: Self::visible_range(viewport, scroll_y),
            content_height,
            scrollable_content_height: content_height + bottom_spacer_height,
            bottom_spacer_height,
        }
    }

    pub fn scroll_y_for_rect(
        result: &ViewerLayoutResult,
        viewport: ViewerViewport,
        rect: ViewerRect,
    ) -> f32 {
        let max_scroll_y = (result.scrollable_content_height - viewport.height).max(0.0);
        rect.y.min(max_scroll_y)
    }

    fn visible_range(viewport: ViewerViewport, scroll_y: f32) -> ViewerVisibleRange {
        ViewerVisibleRange {
            start_y: scroll_y,
            end_y: scroll_y + viewport.height,
        }
    }

    fn bottom_spacer_height(
        viewport: ViewerViewport,
        anchors: &[ViewerRenderedAnchor],
        content_height: f32,
    ) -> f32 {
        let Some(anchor) = anchors.last() else {
            return 0.0;
        };
        let content_below_anchor = (content_height - anchor.target.rect.y).max(0.0);
        (viewport.height - content_below_anchor).max(0.0)
    }
}
