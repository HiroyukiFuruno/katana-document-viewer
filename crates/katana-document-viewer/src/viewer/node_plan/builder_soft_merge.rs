use super::super::media_height::ViewerMediaHeight;
use super::super::planned_node::PlannedNode;
use super::super::types::{ViewerNode, ViewerNodeKind, ViewerTextSpan};
use super::{ParagraphLayout, ViewerNodePlanBuilder};
use katana_markdown_model::{KmmNode, SourceSpan};

impl<'a> ViewerNodePlanBuilder<'a> {
    pub(super) fn try_merge_soft_paragraph(
        &mut self,
        node: &KmmNode,
        planned: &PlannedNode,
    ) -> bool {
        if !self.accepts_soft_paragraph_merge(planned) {
            return false;
        }
        let Some(previous_index) = self.nodes.len().checked_sub(1) else {
            return false;
        };
        if !Self::can_merge_soft_paragraph(&self.nodes[previous_index], planned) {
            return false;
        }
        self.merge_previous_paragraph(previous_index, planned);
        self.refresh_merged_paragraph_height(node, previous_index);
        true
    }

    fn accepts_soft_paragraph_merge(&self, planned: &PlannedNode) -> bool {
        self.paragraph_layout == ParagraphLayout::SoftWrap
            && matches!(planned.kind, ViewerNodeKind::Paragraph)
            && planned.reference.is_none()
    }

    fn merge_previous_paragraph(&mut self, previous_index: usize, planned: &PlannedNode) {
        let previous = &mut self.nodes[previous_index];
        previous.text.push(' ');
        previous.text.push_str(&planned.text);
        previous.spans.push(ViewerTextSpan::plain(" "));
        previous.spans.extend(planned.spans.clone());
        previous.source = Self::merged_source(&previous.source, &planned.source);
    }

    fn refresh_merged_paragraph_height(&mut self, node: &KmmNode, previous_index: usize) {
        let merged = self.merged_planned_node(previous_index);
        let height = ViewerMediaHeight::block_height(
            self.graph.as_ref(),
            &self.input.artifacts,
            node,
            &merged,
            self.input.typography,
            self.nodes[previous_index].rect.width,
            self.height_mode(),
        );
        let previous = &mut self.nodes[previous_index];
        previous.rect.height = height;
        self.y = previous.rect.y + previous.rect.height;
    }

    fn merged_planned_node(&self, previous_index: usize) -> PlannedNode {
        let previous = &self.nodes[previous_index];
        PlannedNode {
            node_id: previous.node_id.clone(),
            kind: previous.kind.clone(),
            source: previous.source.clone(),
            text: previous.text.clone(),
            spans: previous.spans.clone(),
            reference: None,
        }
    }

    fn can_merge_soft_paragraph(previous: &ViewerNode, planned: &PlannedNode) -> bool {
        matches!(previous.kind, ViewerNodeKind::Paragraph)
            && previous.artifact_id.is_none()
            && !Self::is_image_paragraph_source(&previous.source)
            && !Self::is_image_paragraph_source(&planned.source)
            && previous.source.line_column_range.end.line + 1
                == planned.source.line_column_range.start.line
    }

    fn is_image_paragraph_source(source: &SourceSpan) -> bool {
        source.raw.text.trim_start().starts_with("![")
    }

    fn merged_source(previous: &SourceSpan, next: &SourceSpan) -> SourceSpan {
        let mut merged = previous.clone();
        merged.byte_range.end = next.byte_range.end;
        merged.line_column_range.end = next.line_column_range.end;
        previous.raw.text.clone_into(&mut merged.raw.text);
        merged.raw.text.push(' ');
        merged.raw.text.push_str(&next.raw.text);
        merged
    }
}
