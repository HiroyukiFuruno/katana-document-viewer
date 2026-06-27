use super::super::metrics::ViewerNodeMetrics;
use super::super::types::{ViewerNode, ViewerNodeKind, ViewerNodePlan};
use super::{VIEWPORT_PREFETCH_MULTIPLIER, ViewerNodePlanBuilder};
use crate::viewer::asset::ViewerAssetPipeline;
use crate::viewer::types::ViewerRect;
use katana_markdown_model::{KmmNode, KmmNodeId};

impl<'a> ViewerNodePlanBuilder<'a> {
    pub(super) fn push_footnote_separator(&mut self, node: &KmmNode) {
        if !self.nodes.is_empty() {
            self.y += self.block_gap();
        }
        let height =
            ViewerNodeMetrics::block_height(&ViewerNodeKind::Rule, "", self.input.typography);
        let rect = ViewerRect {
            x: 0.0,
            y: self.y,
            width: self.input.viewport.width,
            height,
        };
        self.y += rect.height;
        self.nodes.push(Self::footnote_separator_node(node, rect));
    }

    fn footnote_separator_node(node: &KmmNode, rect: ViewerRect) -> ViewerNode {
        ViewerNode {
            node_id: KmmNodeId(format!("{}-footnote-separator", node.id.0)),
            kind: ViewerNodeKind::Rule,
            source: node.source.clone(),
            text: String::new(),
            spans: Vec::new(),
            html_margin_left_px: 0,
            rule_line_offset_px: 0,
            rect,
            artifact_id: None,
        }
    }

    pub(super) fn is_visible(&self, rect: ViewerRect) -> bool {
        rect.y < self.viewport_end() && rect.y + rect.height > self.scroll_y
    }

    pub(super) fn is_near_viewport(&self, rect: ViewerRect) -> bool {
        let margin = self.input.viewport.height * VIEWPORT_PREFETCH_MULTIPLIER;
        rect.y < self.viewport_end() + margin && rect.y + rect.height > self.scroll_y - margin
    }

    fn viewport_end(&self) -> f32 {
        self.scroll_y + self.input.viewport.height
    }

    pub(super) fn finish(self) -> ViewerNodePlan {
        let asset_requests = ViewerAssetPipeline::load_requests_for_viewport(
            self.input.snapshot.revision.clone(),
            &self.asset_references,
            &self.visible_artifact_ids,
            &self.near_viewport_artifact_ids,
        );
        ViewerNodePlan {
            nodes: self.nodes,
            visible_artifact_ids: self.visible_artifact_ids,
            near_viewport_artifact_ids: self.near_viewport_artifact_ids,
            asset_requests,
            content_height: self.y,
        }
    }
}
