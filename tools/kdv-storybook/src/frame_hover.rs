use crate::canvas::{Canvas, SurfaceArea};
use crate::frame_ui_surface::render_ui_tree_with_theme;
use crate::preview::PreviewScene;
use katana_document_viewer::{ViewerRect, ViewerTarget};
use katana_ui_core::layout::Column;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVisualRole};

const HOVER_CLASS: &str = "kdv-hover-highlight";

pub(crate) struct StorybookFrameHover;

impl StorybookFrameHover {
    pub(crate) fn draw_at(
        canvas: &mut Canvas,
        scene: &PreviewScene,
        area: SurfaceArea,
        hovered_node_id: Option<&str>,
    ) {
        let Some(target) = Self::target(scene, hovered_node_id) else {
            return;
        };
        let node = Self::hover_tree(target.rect, area.scroll_y, area.width);
        let surface = SurfaceArea {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height,
            scroll_y: 0.0,
        };
        render_ui_tree_with_theme(canvas, &node, surface, &scene.theme);
    }

    fn target<'a>(
        scene: &'a PreviewScene,
        hovered_node_id: Option<&str>,
    ) -> Option<&'a ViewerTarget> {
        let id = hovered_node_id?;
        scene.target_for_node_id(id)
    }

    fn hover_tree(rect: ViewerRect, scroll_y: f32, surface_width: usize) -> UiNode {
        let visible_offset = (rect.y - scroll_y).max(0.0);
        let visible_height = Self::visible_height(rect, scroll_y);
        let mut column = Column::new();
        if visible_offset > 0.0 {
            column = column.child(Self::height_spacer(visible_offset));
        }
        column
            .child(Self::hover_row(visible_height, surface_width))
            .into()
    }

    fn height_spacer(height: f32) -> UiNode {
        UiNode::new(UiNodeKind::Stack, "").height(Self::height(height))
    }

    fn hover_row(visible_height: f32, surface_width: usize) -> UiNode {
        UiNode::new(UiNodeKind::Stack, "")
            .width(Self::width(surface_width as f32))
            .height(Self::height(visible_height))
            .visual_role(UiVisualRole::HoverSurface)
            .style_class(HOVER_CLASS)
    }

    fn visible_height(rect: ViewerRect, scroll_y: f32) -> f32 {
        let clipped_top = (scroll_y - rect.y).max(0.0);
        (rect.height - clipped_top).max(0.0)
    }

    fn height(value: f32) -> UiDimension {
        UiDimension::Px(value.round().max(1.0) as u16)
    }

    fn width(value: f32) -> UiDimension {
        UiDimension::Px(value.round().max(0.0) as u16)
    }
}
