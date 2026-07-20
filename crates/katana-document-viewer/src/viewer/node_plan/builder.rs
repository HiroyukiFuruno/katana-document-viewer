pub(super) use super::classifier::ViewerNodeClassifier;
#[cfg(test)]
pub(super) use super::planned_node::PlannedNode;
pub(super) use super::types::ViewerNodeKind;
use super::types::{ViewerNode, ViewerNodePlan};
use crate::artifact::ArtifactId;
use crate::forge::BuildGraph;
use crate::viewer::asset::ViewerAssetReference;
use crate::viewer::types::ViewerInput;
use context::ViewerNodeContext;
use katana_markdown_model::{KmmNode, KmmNodeKind};

const VIEWPORT_PREFETCH_MULTIPLIER: f32 = 2.0;
const PREVIEW_BLOCK_GAP: f32 = 20.0;

#[cfg(test)]
#[path = "builder_html_height_test_support.rs"]
mod html_height_test_support;

pub struct ViewerNodePlanner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParagraphLayout {
    SoftWrap,
    PreserveSourceRows,
}

impl ViewerNodePlanner {
    pub fn create(input: &ViewerInput, scroll_y: f32) -> ViewerNodePlan {
        Self::create_with_layout(input, scroll_y, ParagraphLayout::SoftWrap)
    }

    pub fn create_export_surface(input: &ViewerInput, scroll_y: f32) -> ViewerNodePlan {
        Self::create_with_layout(input, scroll_y, ParagraphLayout::PreserveSourceRows)
    }

    fn create_with_layout(
        input: &ViewerInput,
        scroll_y: f32,
        paragraph_layout: ParagraphLayout,
    ) -> ViewerNodePlan {
        let mut builder = ViewerNodePlanBuilder::new_with_layout(input, scroll_y, paragraph_layout);
        let mut footnote_separator_inserted = false;
        for (index, node) in
            Self::document_nodes_with_footnotes_last(&input.snapshot.document.nodes)
        {
            if matches!(node.kind, KmmNodeKind::FootnoteDefinition(_))
                && !footnote_separator_inserted
            {
                builder.push_footnote_separator(node);
                footnote_separator_inserted = true;
            }
            let context = ViewerNodeContext::top_level(&input.snapshot.document.nodes, index);
            builder.push_node(node, context);
        }
        builder.finish()
    }
}

struct ViewerNodePlanBuilder<'a> {
    input: &'a ViewerInput,
    graph: Option<BuildGraph>,
    paragraph_layout: ParagraphLayout,
    scroll_y: f32,
    y: f32,
    nodes: Vec<ViewerNode>,
    asset_references: Vec<ViewerAssetReference>,
    visible_artifact_ids: Vec<ArtifactId>,
    near_viewport_artifact_ids: Vec<ArtifactId>,
}

impl<'a> ViewerNodePlanBuilder<'a> {
    #[cfg(test)]
    fn new(input: &'a ViewerInput, scroll_y: f32) -> Self {
        Self::new_with_layout(input, scroll_y, ParagraphLayout::SoftWrap)
    }

    fn new_with_layout(
        input: &'a ViewerInput,
        scroll_y: f32,
        paragraph_layout: ParagraphLayout,
    ) -> Self {
        Self {
            input,
            graph: Self::build_graph(input, paragraph_layout),
            paragraph_layout,
            scroll_y,
            y: 0.0,
            nodes: Vec::new(),
            asset_references: Vec::new(),
            visible_artifact_ids: Vec::new(),
            near_viewport_artifact_ids: Vec::new(),
        }
    }

    fn push_node(&mut self, node: &KmmNode, context: ViewerNodeContext<'_>) {
        if let Some(planned) = self.planned_node(node, context) {
            self.push_planned_node(node, planned, context);
            return;
        }
        for child in &node.children {
            self.push_node(child, ViewerNodeContext::empty());
        }
    }
}

#[cfg(test)]
#[path = "builder_test_support.rs"]
pub(super) mod test_support;

#[cfg(test)]
#[path = "builder_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "builder_image_tests.rs"]
mod image_tests;

#[cfg(test)]
#[path = "builder_search_tests.rs"]
mod search_tests;

#[cfg(test)]
#[path = "builder_footnote_tests.rs"]
mod footnote_tests;

#[cfg(test)]
#[path = "builder_html_height_tests.rs"]
mod html_height_tests;

#[cfg(test)]
#[path = "builder_html_heading_tests.rs"]
mod html_heading_tests;

#[cfg(test)]
#[path = "builder_rich_height_tests.rs"]
mod rich_height_tests;

#[cfg(test)]
#[path = "builder_skip_tests.rs"]
mod skip_tests;

#[cfg(test)]
#[path = "builder_spacing_tests.rs"]
mod spacing_tests;

#[cfg(test)]
#[path = "builder_surface_height_tests.rs"]
mod surface_height_tests;

#[path = "builder_assets.rs"]
mod assets;
#[path = "builder_context.rs"]
mod context;
#[path = "builder_footnotes.rs"]
mod footnotes;
#[path = "builder_graph.rs"]
mod graph;
#[path = "builder_lifecycle.rs"]
mod lifecycle;
#[path = "builder_node_push.rs"]
mod node_push;
#[path = "builder_node_resolve.rs"]
mod node_resolve;
#[path = "builder_skip.rs"]
mod skip;
#[path = "builder_soft_merge.rs"]
mod soft_merge;
