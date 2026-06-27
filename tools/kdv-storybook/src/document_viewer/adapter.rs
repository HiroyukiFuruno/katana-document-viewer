use super::adapter_slideshow;
use super::adapter_types::{KucViewerAdapter, KucViewerPlan};
use crate::document_viewer::config::KucViewerConfig;
use crate::document_viewer::node_factory::KucNodeFactory;
use katana_document_viewer::{PreviewOutput, ViewerNode, ViewerNodeKind};
use katana_document_viewer::{ViewerNodePlan, ViewerNodePlanner};
use katana_ui_core::atom::Text;
use katana_ui_core::layout::{Column, ScrollArea, ScrollAxis};
use katana_ui_core::render_model::{UiDimension, UiNode, UiTree};
use katana_ui_core::surface::{PaintRequest, SurfaceMetrics};

const EMPTY_VIEWER_TEXT: &str = "empty document";

impl KucViewerAdapter {
    pub fn render(&self, output: &PreviewOutput, config: &KucViewerConfig) -> KucViewerPlan {
        let node_plan = Self::node_plan(output, config);
        let content_height = Self::content_height(output, &node_plan, config);
        let paint_request = Self::paint_request(config, output, &node_plan, content_height);
        KucViewerPlan {
            paint_request,
            node_plan,
            content_height,
        }
    }

    fn node_plan(output: &PreviewOutput, config: &KucViewerConfig) -> ViewerNodePlan {
        if output.input.viewport == config.viewport {
            return Self::node_plan_for_input(output, &output.input, config.export_surface);
        }
        let mut input = output.input.clone();
        input.viewport = config.viewport;
        Self::node_plan_for_input(output, &input, config.export_surface)
    }

    fn node_plan_for_input(
        output: &PreviewOutput,
        input: &katana_document_viewer::ViewerInput,
        export_surface: bool,
    ) -> ViewerNodePlan {
        if export_surface {
            return ViewerNodePlanner::create_export_surface(input, output.scroll_offset);
        }
        ViewerNodePlanner::create(input, output.scroll_offset)
    }

    fn paint_request(
        config: &KucViewerConfig,
        output: &PreviewOutput,
        node_plan: &ViewerNodePlan,
        content_height: f32,
    ) -> PaintRequest {
        PaintRequest::new(config.window_id.clone(), Self::metrics(config)).with_tree(Self::tree(
            config,
            output,
            node_plan,
            content_height,
        ))
    }

    fn metrics(config: &KucViewerConfig) -> SurfaceMetrics {
        SurfaceMetrics::new(
            config.viewport.width,
            config.viewport.height,
            config.scale_factor,
            config.dpi,
        )
    }

    fn tree(
        config: &KucViewerConfig,
        output: &PreviewOutput,
        node_plan: &ViewerNodePlan,
        content_height: f32,
    ) -> UiTree {
        let scroll: UiNode = ScrollArea::new()
            .axis(ScrollAxis::Vertical)
            .viewport(
                Self::logical_extent(config.viewport.width),
                Self::logical_extent(config.viewport.height),
            )
            .content_extent(
                Self::logical_extent(config.viewport.width),
                Self::logical_extent(content_height),
            )
            .offset(0, Self::logical_extent(output.scroll_offset))
            .child(Self::viewer_child(output, node_plan, config))
            .into();
        let viewer = Self::fullscreen_diagram_child(config, output, node_plan).unwrap_or(scroll);
        UiTree::new(
            adapter_slideshow::viewer_root_with_controls(viewer, output, config)
                .theme(&config.theme),
        )
    }

    fn viewer_child(
        output: &PreviewOutput,
        node_plan: &ViewerNodePlan,
        config: &KucViewerConfig,
    ) -> UiNode {
        if node_plan.nodes.is_empty() {
            return Text::new(EMPTY_VIEWER_TEXT).into();
        }
        let node_factory = Self::node_factory(
            output,
            config,
            Self::content_width(config.viewport.width, config),
            Self::media_content_width(config.viewport.width, config),
        );
        let mut column = Column::new();
        let mut cursor_y = 0.0;
        for node in &node_plan.nodes {
            column = Self::append_gap_from_plan(column, node.rect.y - cursor_y);
            column = column.child(node_factory.viewer_node(node));
            cursor_y = node.rect.y + node.rect.height;
        }
        Self::viewer_content_node(column, config)
    }

    fn fullscreen_diagram_child(
        config: &KucViewerConfig,
        output: &PreviewOutput,
        node_plan: &ViewerNodePlan,
    ) -> Option<UiNode> {
        let node = Self::fullscreen_diagram_node(config, node_plan)?;
        let viewport_width = Self::logical_extent(config.viewport.width);
        let viewport_height = Self::logical_extent(config.viewport.height);
        Some(
            Self::node_factory(output, config, viewport_width, viewport_width)
                .fullscreen_viewport_size(viewport_width, viewport_height)
                .viewer_node(node)
                .width(UiDimension::Px(
                    viewport_width.min(u32::from(u16::MAX)) as u16
                ))
                .height(UiDimension::Px(
                    viewport_height.min(u32::from(u16::MAX)) as u16
                )),
        )
    }

    fn fullscreen_diagram_node<'a>(
        config: &KucViewerConfig,
        node_plan: &'a ViewerNodePlan,
    ) -> Option<&'a ViewerNode> {
        let node_id = config
            .diagram_viewports
            .iter()
            .find_map(|(node_id, state)| state.fullscreen_open.then_some(node_id))?;
        node_plan.nodes.iter().find(|node| {
            node.node_id.0 == *node_id && matches!(node.kind, ViewerNodeKind::Diagram { .. })
        })
    }

    fn node_factory<'a>(
        output: &'a PreviewOutput,
        config: &'a KucViewerConfig,
        content_width: u32,
        media_max_width: u32,
    ) -> KucNodeFactory<'a> {
        KucNodeFactory::new(&output.input.artifacts, content_width)
            .with_media_max_width(media_max_width)
            .typography(output.input.typography)
            .interaction(config.interaction.clone())
            .diagram_viewports(&config.diagram_viewports)
            .image_viewports(&config.image_viewports)
            .task_state_overrides(&config.task_state_overrides)
            .copied_code_node_ids(&config.copied_code_node_ids)
            .media_control_icons(&config.media_control_icons)
            .accordion_open_overrides(&config.accordion_open_overrides)
            .hovered_node_id(config.hovered_node_id.as_deref())
            .viewer_background(config.theme.color("background"))
            .export_surface(config.export_surface)
    }
}

#[cfg(test)]
mod tests {
    use super::{KucViewerAdapter, KucViewerConfig};
    use katana_document_viewer::ViewerViewport;
    use katana_ui_core::theme::{ThemeId, ThemeSnapshot};

    #[test]
    fn preview_content_and_media_width_split_matches_katana_preview_contract() {
        let config = KucViewerConfig::new(
            "preview",
            ViewerViewport {
                width: 1280.0,
                height: 720.0,
            },
        );

        assert_eq!(1256, KucViewerAdapter::content_width(1280.0, &config));
        assert_eq!(1268, KucViewerAdapter::media_content_width(1280.0, &config));
        assert_eq!(12, KucViewerAdapter::padding_horizontal(&config));
        assert_eq!(0, KucViewerAdapter::padding_right(&config));
    }

    #[test]
    fn preview_top_padding_matches_katana_theme_reference_offsets() {
        let light = KucViewerConfig::new(
            "preview-light",
            ViewerViewport {
                width: 1280.0,
                height: 720.0,
            },
        );
        let dark = KucViewerConfig::new(
            "preview-dark",
            ViewerViewport {
                width: 1280.0,
                height: 720.0,
            },
        )
        .theme(ThemeSnapshot::dark());
        let mut katana_dark_theme = ThemeSnapshot::dark();
        katana_dark_theme.id = ThemeId::new("katana-dark");
        let katana_dark = KucViewerConfig::new(
            "preview-katana-dark",
            ViewerViewport {
                width: 1280.0,
                height: 720.0,
            },
        )
        .theme(katana_dark_theme);

        assert_eq!(14, KucViewerAdapter::padding_top(&light));
        assert_eq!(24, KucViewerAdapter::padding_top(&dark));
        assert_eq!(24, KucViewerAdapter::padding_top(&katana_dark));
    }

    #[test]
    fn export_content_width_keeps_document_page_padding() {
        let mut config = KucViewerConfig::new(
            "export",
            ViewerViewport {
                width: 1280.0,
                height: 720.0,
            },
        );
        config.export_surface = true;

        assert_eq!(1168, KucViewerAdapter::content_width(1280.0, &config));
        assert_eq!(56, KucViewerAdapter::padding_right(&config));
    }
}
