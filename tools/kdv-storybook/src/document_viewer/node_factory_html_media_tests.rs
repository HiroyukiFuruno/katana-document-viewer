use super::KucNodeFactory;
use super::node_factory_tests_support::viewer_node;
use crate::test_assert::KucTestExpect;
use katana_document_viewer::{ViewerHtmlRole, ViewerNodeKind};
use katana_ui_core::render_model::{UiDimension, UiNodeKind, UiVisualRole};

#[test]
fn badge_row_html_renders_as_image_surface() {
    let factory = KucNodeFactory::new(&[], 320);
    let mut node = viewer_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::BadgeRow,
        },
        r#"<p align="center"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License"></p>"#,
    );
    node.rect.height = 34.0;

    let ui_node = factory.viewer_node(&node);
    let image_node = ui_node
        .children()
        .first()
        .kuc_expect("badge row should wrap image surface");

    assert_eq!(UiNodeKind::AlignCenter, ui_node.kind());
    assert_eq!(UiNodeKind::ImageSurface, image_node.kind());
    assert!(
        image_node
            .props()
            .image_surface
            .fingerprint
            .starts_with("html-badge-row:node:bytes=")
    );
    assert_eq!(200, image_node.props().image_surface.content_scale);
    assert_eq!(UiDimension::Px(34), ui_node.props().common.height);
    assert_eq!(UiVisualRole::Content, ui_node.props().visual_role);
}
