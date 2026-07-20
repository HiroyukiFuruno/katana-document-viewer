use super::super::media_height::{ViewerHeightMode, ViewerMediaHeight};
use super::super::planned_node::PlannedNode;
use super::super::types::{ViewerHtmlRole, ViewerNodeKind};
use super::context::ViewerNodeContext;
use super::{PREVIEW_BLOCK_GAP, ParagraphLayout, ViewerNodePlanBuilder};
use crate::export_surface_line::LIST_MARKER_COLUMN_WIDTH;
use crate::preview_surface::{
    KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX, KDV_VIEWER_SURFACE_PADDING_PX,
};
use crate::viewer::types::ViewerRect;
use katana_markdown_model::KmmNode;

const KATANA_RULE_AFTER_CENTERED_HTML_LINE_OFFSET_PX: u16 = 9;

impl<'a> ViewerNodePlanBuilder<'a> {
    pub(super) fn push_planned_node(
        &mut self,
        node: &KmmNode,
        planned: PlannedNode,
        context: ViewerNodeContext<'_>,
    ) {
        if self.try_merge_soft_paragraph(node, &planned) {
            return;
        }
        if self.should_insert_gap_before(&planned.kind) {
            self.y += self.block_gap_before(&planned.kind);
        }
        let rect = self.planned_rect(node, &planned, context);
        self.y += rect.height;
        self.commit_planned_node(planned, rect);
    }

    fn planned_rect(
        &self,
        node: &KmmNode,
        planned: &PlannedNode,
        context: ViewerNodeContext<'_>,
    ) -> ViewerRect {
        let x = self.content_padding_x() + Self::planned_node_x(node, planned, context);
        let width = self.planned_rect_width(x, &planned.kind);
        ViewerRect {
            x,
            y: self.y,
            width,
            height: self.planned_height(node, planned, self.input.viewport.width),
        }
    }

    fn planned_height(&self, node: &KmmNode, planned: &PlannedNode, width: f32) -> f32 {
        ViewerMediaHeight::block_height(
            self.graph.as_ref(),
            &self.input.artifacts,
            node,
            planned,
            self.input.typography,
            width,
            self.height_mode(),
        )
    }

    fn commit_planned_node(&mut self, planned: PlannedNode, rect: ViewerRect) {
        let rule_line_offset_px = self.rule_line_offset_px(&planned.kind);
        let artifact_id = planned
            .reference
            .as_ref()
            .map(|reference| reference.artifact_id.clone());
        if let Some(reference) = &planned.reference {
            self.push_asset_reference(reference, rect);
        }
        self.nodes
            .push(planned.into_node(rect, artifact_id, rule_line_offset_px));
    }

    fn rule_line_offset_px(&self, next_kind: &ViewerNodeKind) -> u16 {
        if !matches!(next_kind, ViewerNodeKind::Rule) {
            return 0;
        }
        match self.nodes.last().map(|node| &node.kind) {
            Some(ViewerNodeKind::Html {
                role: ViewerHtmlRole::Centered,
            }) => KATANA_RULE_AFTER_CENTERED_HTML_LINE_OFFSET_PX,
            _ => 0,
        }
    }

    pub(super) fn should_insert_gap_before(&self, next_kind: &ViewerNodeKind) -> bool {
        let Some(previous) = self.nodes.last() else {
            return false;
        };
        !matches!(
            (&previous.kind, next_kind),
            (
                ViewerNodeKind::Rule,
                ViewerNodeKind::FootnoteDefinition { .. }
            ) | (
                ViewerNodeKind::FootnoteDefinition { .. },
                ViewerNodeKind::FootnoteDefinition { .. }
            )
        )
    }

    pub(super) fn block_gap(&self) -> f32 {
        match self.paragraph_layout {
            ParagraphLayout::SoftWrap => PREVIEW_BLOCK_GAP,
            ParagraphLayout::PreserveSourceRows => 0.0,
        }
    }

    fn block_gap_before(&self, next_kind: &ViewerNodeKind) -> f32 {
        if matches!(self.paragraph_layout, ParagraphLayout::PreserveSourceRows) {
            return 0.0;
        }
        let previous = &self.nodes[self.nodes.len() - 1];
        match (&previous.kind, next_kind) {
            (
                ViewerNodeKind::Heading { .. },
                ViewerNodeKind::Html {
                    role: ViewerHtmlRole::BadgeRow,
                },
            ) => 13.0,
            (ViewerNodeKind::Paragraph, ViewerNodeKind::Html { .. }) => 16.0,
            (ViewerNodeKind::Html { .. }, ViewerNodeKind::Rule) => 14.0,
            (ViewerNodeKind::Rule, ViewerNodeKind::Heading { .. }) => 14.0,
            (
                ViewerNodeKind::Html {
                    role: ViewerHtmlRole::Centered,
                },
                ViewerNodeKind::Html {
                    role: ViewerHtmlRole::Heading { .. },
                },
            ) => 17.0,
            (ViewerNodeKind::Heading { .. }, ViewerNodeKind::Diagram { .. }) => 6.0,
            _ => PREVIEW_BLOCK_GAP,
        }
    }

    fn planned_node_x(
        node: &KmmNode,
        planned: &PlannedNode,
        context: ViewerNodeContext<'_>,
    ) -> f32 {
        let kind = &planned.kind;
        if matches!(kind, ViewerNodeKind::Html { .. }) {
            return f32::from(planned.html_margin_left());
        }
        if !matches!(kind, ViewerNodeKind::Code { .. }) {
            return 0.0;
        }
        if !Self::source_has_indented_fence(node) && !context.is_adjacent_to_list(node) {
            return 0.0;
        }
        LIST_MARKER_COLUMN_WIDTH as f32
    }

    fn source_has_indented_fence(node: &KmmNode) -> bool {
        node.source.line_column_range.start.column > 1
            || node
                .source
                .raw
                .text
                .lines()
                .find(|line| !line.trim().is_empty())
                .is_some_and(|line| line.starts_with(' '))
    }

    pub(super) fn height_mode(&self) -> ViewerHeightMode {
        match self.paragraph_layout {
            ParagraphLayout::SoftWrap => ViewerHeightMode::InteractivePreview,
            ParagraphLayout::PreserveSourceRows => ViewerHeightMode::ExportSurface,
        }
    }

    fn content_width(&self) -> f32 {
        let horizontal_padding = match self.height_mode() {
            ViewerHeightMode::InteractivePreview => {
                f32::from(KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX) * 2.0
            }
            ViewerHeightMode::ExportSurface => f32::from(KDV_VIEWER_SURFACE_PADDING_PX) * 2.0,
        };
        (self.input.viewport.width - horizontal_padding).max(1.0)
    }

    fn content_padding_x(&self) -> f32 {
        match self.height_mode() {
            ViewerHeightMode::InteractivePreview => {
                f32::from(KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX)
            }
            ViewerHeightMode::ExportSurface => 0.0,
        }
    }

    fn planned_rect_width(&self, x: f32, kind: &ViewerNodeKind) -> f32 {
        let content_padding_x = self.content_padding_x();
        let local_indent = (x - content_padding_x).max(0.0);
        let width = match self.paragraph_layout {
            ParagraphLayout::SoftWrap => self.content_width(),
            ParagraphLayout::PreserveSourceRows
                if x > 0.0 && matches!(kind, ViewerNodeKind::Code { .. }) =>
            {
                self.input.viewport.width
            }
            ParagraphLayout::PreserveSourceRows => self.content_width(),
        };
        (width - local_indent).max(1.0)
    }
}
