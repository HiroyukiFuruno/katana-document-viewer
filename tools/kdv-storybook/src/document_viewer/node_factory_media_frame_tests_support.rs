use katana_ui_core::render_model::{UiNode, UiNodeKind};

pub(super) fn find_image_surface(node: &UiNode) -> Option<&UiNode> {
    if node.kind() == UiNodeKind::ImageSurface {
        return Some(node);
    }
    node.children().iter().find_map(find_image_surface)
}
