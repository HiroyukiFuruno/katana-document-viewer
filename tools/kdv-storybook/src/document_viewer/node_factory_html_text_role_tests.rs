use super::KucNodeFactory;
use super::node_factory_tests_support::viewer_node;
use katana_document_viewer::{ViewerHtmlAlignment, ViewerHtmlRole, ViewerNodeKind};
use katana_ui_core::render_model::UiDimension;

#[test]
fn html_centered_h1_uses_heading_html_role() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Heading {
                level: 1,
                alignment: ViewerHtmlAlignment::Center,
            },
        },
        "KatanA Desktop",
    );

    let ui_node = factory.viewer_node(&node);

    assert_eq!("document-body", ui_node.props().font_role);
    assert_eq!("heading-html-centered", ui_node.props().text.role);
}

#[test]
fn tall_markdown_heading_2_uses_katana_long_preview_role() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node(
        ViewerNodeKind::Heading { level: 2 },
        "1. HTML Centering (Past Bug: Elements Left-Aligned Instead of Centered)",
    );
    node.rect.height = 47.0;

    let ui_node = factory.viewer_node(&node);

    assert_eq!("document-body", ui_node.props().font_role);
    assert_eq!("heading-2-long", ui_node.props().text.role);
}

#[test]
fn export_surface_keeps_plain_heading_2_role_even_when_row_is_tall() {
    let factory = KucNodeFactory::new(&[], 120).export_surface(true);
    let mut node = viewer_node(
        ViewerNodeKind::Heading { level: 2 },
        "1. HTML Centering (Past Bug: Elements Left-Aligned Instead of Centered)",
    );
    node.rect.height = 47.0;

    let ui_node = factory.viewer_node(&node);

    assert_eq!("document-export-body", ui_node.props().font_role);
    assert_eq!("heading-2-export", ui_node.props().text.role);
}

#[test]
fn export_surface_html_centered_h1_uses_body_alignment_role() {
    let factory = KucNodeFactory::new(&[], 120).export_surface(true);
    let node = viewer_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Heading {
                level: 1,
                alignment: ViewerHtmlAlignment::Center,
            },
        },
        "KatanA Desktop",
    );

    let ui_node = factory.viewer_node(&node);

    assert_eq!("document-export-body", ui_node.props().font_role);
    assert_eq!("html-centered", ui_node.props().text.role);
}

#[test]
fn html_margin_left_reaches_kuc_node_margin_until_document_renderer_uses_planned_rect_x() {
    let factory = KucNodeFactory::new(&[], 320);
    let mut node = viewer_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Generic,
        },
        "indented",
    );
    node.html_margin_left_px = 40;

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiDimension::Px(40), ui_node.props().common.margin.left);
}
