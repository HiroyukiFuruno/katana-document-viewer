use crate::document::DocumentOutline;
use crate::viewer::layout::{ViewerLayoutEngine, ViewerLayoutResult, ViewerRenderedAnchor};
use crate::viewer::types::{ViewerTocItem, ViewerViewport};

pub struct ViewerTocModel;

impl ViewerTocModel {
    pub fn from_outline(
        outline: &DocumentOutline,
        layout: &ViewerLayoutResult,
    ) -> Vec<ViewerTocItem> {
        outline
            .items
            .iter()
            .filter_map(|item| {
                let anchor = layout
                    .anchors
                    .iter()
                    .find(|anchor| anchor.target.node_id == item.node_id)?;
                Some(ViewerTocItem::from_outline_item(
                    item.clone(),
                    anchor.target.rect,
                    anchor.anchor_index,
                ))
            })
            .collect()
    }

    pub fn scroll_y_for_item(
        layout: &ViewerLayoutResult,
        viewport: ViewerViewport,
        item: &ViewerTocItem,
    ) -> f32 {
        ViewerLayoutEngine::scroll_y_for_rect(layout, viewport, item.anchor_rect)
    }

    pub fn active_anchor(
        layout: &ViewerLayoutResult,
        scroll_y: f32,
    ) -> Option<ViewerRenderedAnchor> {
        layout
            .anchors
            .iter()
            .rfind(|anchor| anchor.target.rect.y <= scroll_y)
            .cloned()
    }
}
