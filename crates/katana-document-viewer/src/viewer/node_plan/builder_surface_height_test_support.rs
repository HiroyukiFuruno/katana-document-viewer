use crate::export_surface::{SurfaceBlock, SurfaceBlockFactory};
use crate::export_surface_helpers::SurfaceHelpers;
use crate::export_surface_line::SurfaceTypographyConfig;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, KdvThemeSnapshot, MarkdownSource, PreviewConfig,
    PreviewOutputFactory, ViewerNode, ViewerNodePlan, ViewerNodePlanner, ViewerViewport,
};

#[path = "builder_surface_height_test_debug.rs"]
mod debug;

const CONTENT_HEIGHT: f32 = 30_000.0;
const SURFACE_WIDTH: f32 = 1280.0;
const MAX_NODE_HEIGHT_DRIFT: f32 = 1.0;

pub(super) struct SurfaceHeightCase {
    pub(super) plan: ViewerNodePlan,
    graph: BuildGraph,
    theme: KdvThemeSnapshot,
    blocks: Vec<SurfaceBlock>,
    preview_font_size: u16,
}

impl SurfaceHeightCase {
    pub(super) fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_source(
            include_str!("../../../../../assets/fixtures/katana/sample.md"),
            "assets/fixtures/katana/sample.md",
        )
    }

    pub(super) fn load_direct_sample() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_source(
            include_str!("../../../../../assets/fixtures/direct/sample.md"),
            "assets/fixtures/direct/sample.md",
        )
    }

    fn from_source(content: &str, document_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let output = PreviewOutputFactory::from_source(
            &MarkdownSource {
                content: content.to_string(),
                document_id: Some(document_id.to_string()),
            },
            &Self::config(),
            CONTENT_HEIGHT,
        )?;
        let plan = ViewerNodePlanner::create_export_surface(&output.input, 0.0);
        let theme = output.input.theme.clone();
        let graph = BuildGraph::from_request(&BuildRequest {
            snapshot: output.input.snapshot.clone(),
            profile: BuildProfile::markdown_export(),
            theme: theme.clone(),
        });
        let preview_font_size = output.input.typography.preview_font_size;
        let blocks = Self::blocks(&graph, &theme, preview_font_size);
        Ok(Self {
            plan,
            graph,
            theme,
            blocks,
            preview_font_size,
        })
    }

    pub(super) fn node_height_failures(&self) -> Vec<String> {
        self.graph
            .snapshot
            .document
            .nodes
            .iter()
            .filter_map(|node| self.node_height_failure(node))
            .collect()
    }

    pub(super) fn expected_content_height(&self) -> u32 {
        SurfaceHelpers::block_stack_height(self.blocks.iter().map(SurfaceBlock::height))
    }

    pub(super) fn plan_height_failure_message(&self) -> String {
        debug::plan_height_failure_message(&self.plan, &self.blocks)
    }

    pub(super) fn plan_y_for_source(&self, source: &str) -> Option<i32> {
        self.plan
            .nodes
            .iter()
            .find(|node| node.source.raw.text.contains(source))
            .map(|node| node.rect.y.round() as i32)
    }

    pub(super) fn plan_height_for_source(&self, source: &str) -> Option<i32> {
        self.plan
            .nodes
            .iter()
            .find(|node| node.source.raw.text.contains(source))
            .map(|node| node.rect.height.round() as i32)
    }

    pub(super) fn surface_y_for_text(&self, text: &str) -> Option<i32> {
        let mut y = 0_u32;
        for block in &self.blocks {
            if block.text_for_tests().contains(text) {
                return Some(y as i32);
            }
            y = y.saturating_add(block.height());
        }
        None
    }

    pub(super) fn nested_code_node(&self) -> Result<&ViewerNode, Box<dyn std::error::Error>> {
        self.plan
            .nodes
            .iter()
            .find(|node| node.text.contains("let x = 42"))
            .ok_or_else(|| std::io::Error::other("nested list code block must be planned").into())
    }

    pub(super) const fn surface_width(&self) -> f32 {
        SURFACE_WIDTH
    }

    fn node_height_failure(&self, node: &katana_markdown_model::KmmNode) -> Option<String> {
        let planned = self.planned_node_for(node)?;
        let expected = self.expected_node_height(node);
        let drift = (planned.rect.height - expected).abs();
        (drift > MAX_NODE_HEIGHT_DRIFT).then(|| self.node_failure_message(planned, expected, drift))
    }

    fn planned_node_for(&self, node: &katana_markdown_model::KmmNode) -> Option<&ViewerNode> {
        self.plan.nodes.iter().find(|planned| {
            planned.source.byte_range == node.source.byte_range
                && !planned.node_id.0.ends_with("-footnote-separator")
        })
    }

    fn expected_node_height(&self, node: &katana_markdown_model::KmmNode) -> f32 {
        SurfaceBlockFactory::node_height_with_typography(
            &self.graph,
            node,
            &self.theme,
            Self::typography(self.preview_font_size),
        ) as f32
    }

    fn node_failure_message(&self, planned: &ViewerNode, expected: f32, drift: f32) -> String {
        format!(
            "{} {:?}: plan={} surface={} drift={} raw={:?}",
            planned.node_id.0,
            planned.kind,
            planned.rect.height,
            expected,
            drift,
            debug::first_line(&planned.source.raw.text)
        )
    }

    fn blocks(
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        preview_font_size: u16,
    ) -> Vec<SurfaceBlock> {
        SurfaceBlockFactory::create_with_typography(
            graph,
            theme,
            Self::typography(preview_font_size),
        )
    }

    fn typography(preview_font_size: u16) -> SurfaceTypographyConfig {
        SurfaceTypographyConfig::from_body_font_size(f32::from(preview_font_size))
    }

    fn config() -> PreviewConfig {
        PreviewConfig {
            viewport: ViewerViewport {
                width: SURFACE_WIDTH,
                height: CONTENT_HEIGHT,
            },
            ..PreviewConfig::default()
        }
    }
}
