use super::adapter_types::KucViewerAdapter;
use super::config::KucViewerConfig;
use katana_document_viewer::{
    KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX,
    KDV_INTERACTIVE_PREVIEW_SURFACE_PADDING_PX, KDV_VIEWER_SURFACE_PADDING_PX, PreviewOutput,
    ViewerNodePlan,
};
use katana_ui_core::atom::Spacer;
use katana_ui_core::layout::Column;
use katana_ui_core::render_model::{UiDimension, UiEdgeInsets, UiNode};

const MIN_VISIBLE_PLAN_GAP_PX: f32 = 0.5;
const EXPORT_SURFACE_PADDING_TOP: u16 = KDV_VIEWER_SURFACE_PADDING_PX;
const EXPORT_SURFACE_PADDING_HORIZONTAL: u16 = KDV_VIEWER_SURFACE_PADDING_PX;
const EXPORT_SURFACE_PADDING_BOTTOM: u16 = KDV_VIEWER_SURFACE_PADDING_PX;
const KATANA_DARK_PREVIEW_TOP_PADDING_ADJUSTMENT_PX: u16 = 12;
const KATANA_LIGHT_PREVIEW_TOP_PADDING_ADJUSTMENT_PX: u16 = 2;
const PREVIEW_SURFACE_PADDING_HORIZONTAL: u16 =
    KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX;
const PREVIEW_SURFACE_PADDING_RIGHT: u16 = 0;
const PREVIEW_SURFACE_PADDING_BOTTOM: u16 = KDV_INTERACTIVE_PREVIEW_SURFACE_PADDING_PX;
const RGB_LUMINANCE_RED_WEIGHT: u32 = 299;
const RGB_LUMINANCE_GREEN_WEIGHT: u32 = 587;
const RGB_LUMINANCE_BLUE_WEIGHT: u32 = 114;
const RGB_LUMINANCE_WEIGHT_TOTAL: u32 = 1000;

impl KucViewerAdapter {
    pub(super) fn append_gap_from_plan(column: Column, gap: f32) -> Column {
        if gap <= MIN_VISIBLE_PLAN_GAP_PX {
            return column;
        }
        column.child(Self::height_spacer(gap))
    }

    pub(super) fn viewer_content_node(column: Column, config: &KucViewerConfig) -> UiNode {
        let node: UiNode = column.into();
        let common = node.props().common.clone().padding(UiEdgeInsets {
            top: UiDimension::Px(Self::padding_top(config)),
            right: UiDimension::Px(Self::padding_right(config)),
            bottom: UiDimension::Px(Self::padding_bottom(config)),
            left: UiDimension::Px(Self::padding_horizontal(config)),
        });
        node.common(common)
    }

    pub(super) fn content_height(
        output: &PreviewOutput,
        node_plan: &ViewerNodePlan,
        config: &KucViewerConfig,
    ) -> f32 {
        if config.export_surface {
            return output.content_height.max(0.0);
        }
        let content_height = Self::node_plan_content_height(node_plan, config);
        Self::scrollable_content_height(
            content_height,
            config.viewport.height,
            Self::last_anchor_y(node_plan, config),
        )
    }

    pub(super) fn logical_extent(value: f32) -> u32 {
        if value <= 0.0 {
            return 0;
        }
        value.round() as u32
    }

    pub(super) fn content_width(viewport_width: f32, config: &KucViewerConfig) -> u32 {
        let deducted_padding = if config.export_surface {
            u32::from(Self::padding_horizontal(config)) + u32::from(Self::padding_right(config))
        } else {
            u32::from(Self::padding_horizontal(config)).saturating_mul(2)
        };
        Self::logical_extent(viewport_width).saturating_sub(deducted_padding)
    }

    pub(super) fn media_content_width(viewport_width: f32, config: &KucViewerConfig) -> u32 {
        if config.export_surface {
            return Self::content_width(viewport_width, config);
        }
        Self::logical_extent(viewport_width)
            .saturating_sub(u32::from(Self::padding_horizontal(config)))
    }

    pub(super) fn logical_height(value: f32) -> UiDimension {
        UiDimension::Px(Self::logical_extent(value).min(u32::from(u16::MAX)) as u16)
    }

    pub(super) fn padding_horizontal(config: &KucViewerConfig) -> u16 {
        if config.export_surface {
            return EXPORT_SURFACE_PADDING_HORIZONTAL;
        }
        PREVIEW_SURFACE_PADDING_HORIZONTAL
    }

    pub(super) fn padding_right(config: &KucViewerConfig) -> u16 {
        if config.export_surface {
            return EXPORT_SURFACE_PADDING_HORIZONTAL;
        }
        PREVIEW_SURFACE_PADDING_RIGHT
    }

    fn height_spacer(height: f32) -> UiNode {
        UiNode::from(Spacer::new("")).height(Self::logical_height(height))
    }

    fn scrollable_content_height(
        content_height: f32,
        viewport_height: f32,
        last_anchor_y: Option<f32>,
    ) -> f32 {
        let viewport_height = viewport_height.max(0.0);
        if content_height <= viewport_height {
            return content_height;
        }
        let Some(last_anchor_y) = last_anchor_y else {
            return content_height;
        };
        let content_below_anchor = (content_height - last_anchor_y).max(0.0);
        content_height + (viewport_height - content_below_anchor).max(0.0)
    }

    fn node_plan_content_height(node_plan: &ViewerNodePlan, config: &KucViewerConfig) -> f32 {
        node_plan.content_height
            + f32::from(Self::padding_top(config))
            + f32::from(PREVIEW_SURFACE_PADDING_BOTTOM)
    }

    fn last_anchor_y(node_plan: &ViewerNodePlan, config: &KucViewerConfig) -> Option<f32> {
        let padding_top = f32::from(Self::padding_top(config));
        node_plan
            .nodes
            .iter()
            .map(|node| node.rect.y + padding_top)
            .max_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub(in crate::document_viewer) fn padding_top(config: &KucViewerConfig) -> u16 {
        if config.export_surface {
            return EXPORT_SURFACE_PADDING_TOP;
        }
        KDV_INTERACTIVE_PREVIEW_SURFACE_PADDING_PX + Self::preview_top_adjustment(config)
    }

    fn preview_top_adjustment(config: &KucViewerConfig) -> u16 {
        if Self::uses_dark_background(config) {
            return KATANA_DARK_PREVIEW_TOP_PADDING_ADJUSTMENT_PX;
        }
        KATANA_LIGHT_PREVIEW_TOP_PADDING_ADJUSTMENT_PX
    }

    fn uses_dark_background(config: &KucViewerConfig) -> bool {
        const DARK_LUMINANCE_THRESHOLD: u32 = 128;
        let Some([red, green, blue, _]) = config.theme.color("background") else {
            return false;
        };
        let luminance = (u32::from(red) * RGB_LUMINANCE_RED_WEIGHT
            + u32::from(green) * RGB_LUMINANCE_GREEN_WEIGHT
            + u32::from(blue) * RGB_LUMINANCE_BLUE_WEIGHT)
            / RGB_LUMINANCE_WEIGHT_TOTAL;
        luminance < DARK_LUMINANCE_THRESHOLD
    }

    fn padding_bottom(config: &KucViewerConfig) -> u16 {
        if config.export_surface {
            return EXPORT_SURFACE_PADDING_BOTTOM;
        }
        PREVIEW_SURFACE_PADDING_BOTTOM
    }
}
