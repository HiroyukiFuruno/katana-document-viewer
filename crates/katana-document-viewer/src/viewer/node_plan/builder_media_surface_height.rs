use super::super::metrics::ViewerNodeMetrics;
use super::super::planned_node::PlannedNode;
use super::super::types::ViewerNodeKind;
use super::{ViewerHeightMode, ViewerMediaHeight};
use crate::export_surface::SurfaceBlockFactory;
use crate::export_surface_line::SurfaceTypographyConfig;
use crate::forge::BuildGraph;
use crate::viewer::settings_update::ViewerTypographyConfig;
use katana_markdown_model::{KmmNode, KmmNodeKind};

#[cfg(test)]
#[path = "builder_media_surface_height_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "builder_media_surface_height_test_support.rs"]
mod tests_support;

impl ViewerMediaHeight {
    pub(super) fn preferred_surface_height(
        graph: Option<&BuildGraph>,
        node: &KmmNode,
        planned: &PlannedNode,
        typography: ViewerTypographyConfig,
    ) -> Option<f32> {
        let graph = graph?;
        if !Self::should_use_surface_height_first(node, planned) {
            return None;
        }
        Self::positive_surface_height(graph, node, typography)
    }

    pub(super) fn fallback_node_height(
        graph: Option<&BuildGraph>,
        node: &KmmNode,
        planned: &PlannedNode,
        typography: ViewerTypographyConfig,
        content_width: u32,
        height_mode: ViewerHeightMode,
        kind: &ViewerNodeKind,
    ) -> f32 {
        if height_mode == ViewerHeightMode::ExportSurface
            && let Some(height) = Self::positive_surface_height_for_graph(graph, node, typography)
        {
            return height;
        }
        ViewerNodeMetrics::block_height_with_width(
            kind,
            &planned.text,
            typography,
            content_width as usize,
        )
    }

    fn should_use_surface_height_first(node: &KmmNode, planned: &PlannedNode) -> bool {
        Self::uses_surface_height_for_table_fallback(node, planned)
            || Self::uses_surface_height_for_viewer_node(&planned.kind)
    }

    fn positive_surface_height_for_graph(
        graph: Option<&BuildGraph>,
        node: &KmmNode,
        typography: ViewerTypographyConfig,
    ) -> Option<f32> {
        Self::positive_surface_height(graph?, node, typography)
    }

    fn positive_surface_height(
        graph: &BuildGraph,
        node: &KmmNode,
        typography: ViewerTypographyConfig,
    ) -> Option<f32> {
        let surface_height = Self::surface_node_height(graph, node, typography);
        (surface_height > 0.0).then_some(surface_height)
    }

    fn uses_surface_height_for_table_fallback(node: &KmmNode, planned: &PlannedNode) -> bool {
        matches!(node.kind, KmmNodeKind::Table(_))
            && matches!(planned.kind, ViewerNodeKind::Paragraph)
    }

    fn uses_surface_height_for_viewer_node(kind: &ViewerNodeKind) -> bool {
        !matches!(kind, ViewerNodeKind::Diagram { .. } | ViewerNodeKind::Image)
    }

    fn surface_node_height(
        graph: &BuildGraph,
        node: &KmmNode,
        typography: ViewerTypographyConfig,
    ) -> f32 {
        SurfaceBlockFactory::node_height_with_typography(
            graph,
            node,
            &graph.theme,
            SurfaceTypographyConfig::from_body_font_size(f32::from(typography.preview_font_size)),
        ) as f32
    }
}
