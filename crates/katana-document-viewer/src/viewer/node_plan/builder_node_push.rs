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
const CANONICAL_BODY_ROW_HEIGHT_RATIO: f32 = 1.5;
const HTML_BODY_LINE_HEIGHT_RATIO: f32 = 1.5;
const HTML_HEADING_CONTENT_HEIGHT_RATIO: f32 = 2.25;
const HTML_TOP_ADJUSTMENT_PX: f32 = 7.0;

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
        if self.nodes.is_empty()
            && matches!(self.paragraph_layout, ParagraphLayout::SoftWrap)
            && matches!(planned.kind, ViewerNodeKind::Heading { .. })
        {
            self.y += self.canonical_body_row_height();
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
            height: self.planned_rect_height(node, planned),
        }
    }

    fn planned_rect_height(&self, node: &KmmNode, planned: &PlannedNode) -> f32 {
        self.html_data_image_height(planned).unwrap_or_else(|| {
            self.html_content_height(
                planned,
                self.planned_height(node, planned, self.input.viewport.width),
            )
        })
    }

    fn html_data_image_height(&self, planned: &PlannedNode) -> Option<f32> {
        if !matches!(self.paragraph_layout, ParagraphLayout::SoftWrap)
            || !matches!(
                planned.kind,
                ViewerNodeKind::Html {
                    role: ViewerHtmlRole::Centered
                }
            )
        {
            return None;
        }
        ViewerMediaHeight::html_data_image_height(
            planned,
            self.input.viewport.width,
            self.height_mode(),
        )
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
        if matches!(self.paragraph_layout, ParagraphLayout::SoftWrap) {
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
        self.native_anchor_gap_before(previous, next_kind)
            .unwrap_or_else(|| self.default_block_gap_before(&previous.kind, next_kind))
    }

    fn native_anchor_gap_before(
        &self,
        previous: &super::super::types::ViewerNode,
        next_kind: &ViewerNodeKind,
    ) -> Option<f32> {
        if Self::uses_interactive_html_allocation(&previous.kind) {
            if Self::uses_interactive_html_allocation(next_kind) {
                return Some(self.html_anchor_advance(previous.rect.height) - previous.rect.height);
            }
            if matches!(next_kind, ViewerNodeKind::Paragraph) {
                return Some(self.html_following_paragraph_gap(previous.rect.height));
            }
        }
        if matches!(previous.kind, ViewerNodeKind::Heading { .. })
            && Self::uses_interactive_html_allocation(next_kind)
        {
            return Some(self.html_body_line_height() - self.html_top_adjustment());
        }
        None
    }

    fn default_block_gap_before(
        &self,
        previous_kind: &ViewerNodeKind,
        next_kind: &ViewerNodeKind,
    ) -> f32 {
        self.html_rule_gap_before(previous_kind, next_kind)
            .or_else(|| self.canonical_block_gap_before(previous_kind, next_kind))
            .unwrap_or_else(|| self.fixed_block_gap_before(previous_kind, next_kind))
    }

    fn html_rule_gap_before(
        &self,
        previous_kind: &ViewerNodeKind,
        next_kind: &ViewerNodeKind,
    ) -> Option<f32> {
        if matches!(previous_kind, ViewerNodeKind::Paragraph)
            && matches!(next_kind, ViewerNodeKind::Rule)
        {
            return Some(self.html_body_line_height());
        }
        if matches!(previous_kind, ViewerNodeKind::Rule)
            && matches!(next_kind, ViewerNodeKind::Heading { .. })
        {
            return Some(self.html_body_line_height());
        }
        None
    }

    fn canonical_block_gap_before(
        &self,
        previous_kind: &ViewerNodeKind,
        next_kind: &ViewerNodeKind,
    ) -> Option<f32> {
        match (previous_kind, next_kind) {
            (
                ViewerNodeKind::Heading { .. } | ViewerNodeKind::Paragraph,
                ViewerNodeKind::Heading { .. } | ViewerNodeKind::Paragraph,
            ) => Some(self.canonical_body_row_height()),
            (ViewerNodeKind::Heading { .. }, ViewerNodeKind::List)
            | (ViewerNodeKind::List, ViewerNodeKind::Heading { .. }) => {
                Some(self.canonical_body_row_height())
            }
            (ViewerNodeKind::Html { .. }, ViewerNodeKind::Rule) => {
                Some(self.canonical_body_row_height())
            }
            _ => None,
        }
    }

    fn fixed_block_gap_before(
        &self,
        previous_kind: &ViewerNodeKind,
        next_kind: &ViewerNodeKind,
    ) -> f32 {
        match (previous_kind, next_kind) {
            (ViewerNodeKind::Paragraph, ViewerNodeKind::Html { .. }) => {
                self.html_body_line_height() - self.html_top_adjustment()
            }
            (ViewerNodeKind::Heading { .. }, ViewerNodeKind::Diagram { .. }) => 6.0,
            _ => PREVIEW_BLOCK_GAP,
        }
    }

    fn html_content_height(&self, planned: &PlannedNode, planned_height: f32) -> f32 {
        if !matches!(self.paragraph_layout, ParagraphLayout::SoftWrap) {
            return planned_height;
        }
        match &planned.kind {
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::Heading { .. },
            } => {
                f32::from(self.input.typography.preview_font_size)
                    * HTML_HEADING_CONTENT_HEIGHT_RATIO
            }
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::BadgeRow,
            } => self.html_parent_line_height(),
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::Centered,
            } => self.html_centered_content_height(planned),
            ViewerNodeKind::Rule => self.html_body_line_height(),
            _ => planned_height,
        }
    }

    fn html_centered_content_height(&self, planned: &PlannedNode) -> f32 {
        if planned
            .spans
            .iter()
            .any(|span| !span.link_target.is_empty())
        {
            return self.html_parent_line_height();
        }
        self.html_body_line_height()
    }

    fn uses_interactive_html_allocation(kind: &ViewerNodeKind) -> bool {
        matches!(
            kind,
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::Heading { .. }
                    | ViewerHtmlRole::Centered
                    | ViewerHtmlRole::BadgeRow
            }
        )
    }

    fn html_body_line_height(&self) -> f32 {
        f32::from(self.input.typography.preview_font_size) * HTML_BODY_LINE_HEIGHT_RATIO
    }

    fn html_top_adjustment(&self) -> f32 {
        HTML_TOP_ADJUSTMENT_PX
    }

    fn html_parent_line_height(&self) -> f32 {
        self.html_body_line_height() + self.html_top_adjustment()
    }

    fn html_anchor_advance(&self, content_height: f32) -> f32 {
        self.html_parent_line_height().max(content_height) + self.html_body_line_height()
            - self.html_top_adjustment()
    }

    fn html_following_paragraph_gap(&self, content_height: f32) -> f32 {
        self.html_anchor_advance(content_height) + self.html_top_adjustment() - content_height
    }

    fn canonical_body_row_height(&self) -> f32 {
        f32::from(self.input.typography.preview_font_size) * CANONICAL_BODY_ROW_HEIGHT_RATIO
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
