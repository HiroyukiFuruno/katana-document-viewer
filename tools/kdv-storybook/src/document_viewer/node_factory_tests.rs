use super::KucNodeFactory;
use super::node_factory_tests_support::{has_style_class, viewer_node};
use crate::preview_theme_bridge::KucThemeBridge;
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerNodeKind, ViewerTextSpan, ViewerTextStyle,
    ViewerTypographyConfig,
};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTextWrapMode, UiVisualRole};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::UiTreeRenderArea;

#[test]
fn text_node_preserves_viewer_inline_spans() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "bold link");
    node.spans = vec![
        ViewerTextSpan::styled("bold", ViewerTextStyle::default().bold()),
        ViewerTextSpan::linked(
            "link",
            "https://example.com",
            ViewerTextStyle::default().link(),
        ),
    ];

    let ui_node = factory.viewer_node(&node);

    assert!(ui_node.props().text.spans[0].style.bold);
    assert_eq!(
        "https://example.com",
        ui_node.props().text.spans[1].link_target
    );
    assert!(ui_node.props().text.spans[1].style.underline);
}

#[test]
fn paragraph_text_uses_kuc_wrap_contract() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node(ViewerNodeKind::Paragraph, "long body line");

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiTextWrapMode::Wrap, ui_node.props().text.wrap);
}

#[test]
fn heading_node_preserves_viewer_inline_code_spans() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node(
        ViewerNodeKind::Heading { level: 3 },
        r#"1.1 `<h1 align="center">`"#,
    );
    node.spans = vec![
        ViewerTextSpan::plain("1.1 "),
        ViewerTextSpan::styled(
            r#"<h1 align="center">"#,
            ViewerTextStyle::default().inline_code(),
        ),
    ];

    let ui_node = factory.viewer_node(&node);

    assert_eq!("heading-3", ui_node.props().text.role);
    assert_eq!(2, ui_node.props().text.spans.len());
    assert!(ui_node.props().text.spans[1].style.inline_code);
    assert!(!ui_node.props().text.spans[1].text.contains('`'));
}

#[test]
fn interactive_text_without_source_line_count_preserves_viewer_height_and_width() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node(ViewerNodeKind::Paragraph, "Body");

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiDimension::Px(32), ui_node.props().common.height);
    assert_eq!(UiDimension::Px(120), ui_node.props().common.width);
}

#[test]
fn interactive_text_preserves_non_line_quantized_viewer_height() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "Windows native body");
    node.rect.height = 42.0;

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiDimension::Px(42), ui_node.props().common.height);
}

#[test]
fn interactive_soft_wrapped_text_keeps_kdv_planned_semantic_hit_height()
-> Result<(), Box<dyn std::error::Error>> {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "Windows native body");
    node.rect.height = 42.0;
    node.source.line_column_range.end.line = 3;

    let ui_node = factory.viewer_node(&node);
    assert_eq!(UiNodeKind::Row, ui_node.kind());
    let hit =
        KucThemeBridge::document_host(ThemeSnapshot::light(), ViewerTypographyConfig::default())
            .document_node_hits(
                &ui_node,
                UiTreeRenderArea {
                    x: 0,
                    y: 0,
                    width: 120,
                    height: 42,
                    scroll_y: 0.0,
                },
            )
            .into_iter()
            .find(|hit| hit.node_id.as_str() == node.node_id.0)
            .ok_or("soft-wrapped paragraph must expose its semantic node hit")?;

    assert_eq!(42, hit.rect.height);
    Ok(())
}

#[test]
fn interactive_paragraph_uses_source_line_count_with_kuc_body_baseline()
-> Result<(), Box<dyn std::error::Error>> {
    let factory = KucNodeFactory::new(&[], 120).typography(ViewerTypographyConfig {
        preview_font_size: 14,
    });
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "Two source lines");
    node.rect.height = 46.0;

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiNodeKind::Row, ui_node.kind());
    assert_eq!(UiDimension::Px(42), ui_node.props().common.height);
    assert_eq!(UiDimension::Px(120), ui_node.props().common.width);
    let hit = KucThemeBridge::document_host(
        ThemeSnapshot::light(),
        ViewerTypographyConfig {
            preview_font_size: 14,
        },
    )
    .document_node_hits(
        &ui_node,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 42,
            scroll_y: 0.0,
        },
    )
    .into_iter()
    .find(|hit| hit.node_id.as_str() == node.node_id.0)
    .ok_or("interactive paragraph row must expose its semantic node hit")?;
    assert_eq!(42, hit.rect.height);
    Ok::<(), Box<dyn std::error::Error>>(())
}

#[test]
fn interactive_paragraph_uses_base_typography_source_line_height() {
    let factory = KucNodeFactory::new(&[], 120).typography(ViewerTypographyConfig {
        preview_font_size: 24,
    });
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "Two source lines");
    node.rect.height = 92.0;

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiDimension::Px(72), ui_node.props().common.height);
}

#[test]
fn export_text_height_comes_from_viewer_rect() {
    let factory = KucNodeFactory::new(&[], 120).export_surface(true);
    let node = viewer_node(ViewerNodeKind::Paragraph, "Body");

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiDimension::Px(32), ui_node.props().common.height);
    assert_eq!(UiDimension::Px(120), ui_node.props().common.width);
}

#[test]
fn viewer_text_node_exposes_stable_id_for_host_hover() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node(ViewerNodeKind::Paragraph, "Body");

    let ui_node = factory.viewer_node(&node);

    assert_eq!(node.node_id.0, ui_node.id().as_str());
    assert_eq!(node.node_id.0, ui_node.props().state_id.as_str());
}

#[test]
fn hover_surface_preserves_natural_host_geometry_and_semantic_id()
-> Result<(), Box<dyn std::error::Error>> {
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "Body");
    node.rect.width = 184.0;
    node.rect.height = 36.0;
    let interaction = ViewerInteractionConfig {
        hover_highlight_enabled: true,
        selection_enabled: false,
        image_controls_enabled: false,
        diagram_controls_enabled: false,
        code_controls_enabled: false,
    };
    let normal_node = KucNodeFactory::new(&[], 240)
        .interaction(interaction.clone())
        .viewer_node(&node);
    let hovered_node = KucNodeFactory::new(&[], 240)
        .interaction(interaction)
        .hovered_node_id(Some(node.node_id.0.as_str()));

    let hovered_node = hovered_node.viewer_node(&node);

    assert_eq!(UiVisualRole::HoverSurface, hovered_node.props().visual_role);
    assert_eq!(
        UiDimension::Px(240),
        hovered_node.props().common.width,
        "hover/click surface must use the full Markdown row width, not the intrinsic text rect"
    );
    assert_eq!(node.node_id.0, hovered_node.props().common.semantic_node_id);
    assert_eq!(node.node_id.0, hovered_node.children()[0].id().as_str());

    let host =
        KucThemeBridge::document_host(ThemeSnapshot::light(), ViewerTypographyConfig::default());
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 240,
        height: 36,
        scroll_y: 0.0,
    };
    let normal_hit = host
        .document_node_hits(&normal_node, area)
        .into_iter()
        .find(|hit| hit.node_id.as_str() == node.node_id.0)
        .ok_or("normal text must expose its semantic node hit")?;
    let hovered_hit = host
        .document_node_hits(&hovered_node, area)
        .into_iter()
        .find(|hit| hit.node_id.as_str() == node.node_id.0)
        .ok_or("hovered text must retain its semantic node hit")?;

    assert_eq!(
        normal_hit.rect, hovered_hit.rect,
        "hover must retain normal natural Text geometry"
    );
    assert_eq!(240, hovered_hit.rect.width);
    let following = UiNode::new(UiNodeKind::Text, "Following").stable_node_id("following");
    let normal_row = UiNode::new(UiNodeKind::Column, "")
        .child(normal_node)
        .child(following.clone());
    let hovered_row = UiNode::new(UiNodeKind::Column, "")
        .child(hovered_node)
        .child(following);
    let normal_following_y = host
        .document_node_hits(&normal_row, area)
        .into_iter()
        .find(|hit| hit.node_id.as_str() == "following")
        .ok_or("normal following node must be hit-testable")?
        .rect
        .y;
    let hovered_following_y = host
        .document_node_hits(&hovered_row, area)
        .into_iter()
        .find(|hit| hit.node_id.as_str() == "following")
        .ok_or("hovered following node must be hit-testable")?
        .rect
        .y;
    assert_eq!(
        normal_following_y, hovered_following_y,
        "hover wrapper must retain the normal Text logical advance"
    );
    assert!(
        hovered_hit
            .semantic_node_id
            .as_ref()
            .is_some_and(|id| id.as_str() == node.node_id.0),
        "hovered text hit must retain its semantic id"
    );
    Ok(())
}

#[test]
fn text_node_preserves_current_search_highlight_style() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "alpha");
    node.spans = vec![ViewerTextSpan::styled(
        "alpha",
        ViewerTextStyle::default().current_highlight(),
    )];

    let ui_node = factory.viewer_node(&node);

    assert!(ui_node.props().text.spans[0].style.highlight);
    assert!(ui_node.props().text.spans[0].style.current_highlight);
}

#[test]
fn inline_code_only_paragraph_uses_code_font_for_width_parity() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node(ViewerNodeKind::Paragraph, "long inline code");
    node.spans = vec![ViewerTextSpan::styled(
        "long inline code",
        ViewerTextStyle::default().inline_code(),
    )];

    let ui_node = factory.viewer_node(&node);

    assert_eq!("document-code", ui_node.props().font_role);
    assert_eq!("paragraph", ui_node.props().text.role);
}

#[test]
fn document_heading_uses_plain_heading_role_and_code_uses_common_props() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut heading_node = viewer_node(ViewerNodeKind::Heading { level: 2 }, "Title With Space");
    heading_node.spans = vec![ViewerTextSpan::plain("Title With Space".to_string())];
    let heading = factory.viewer_node(&heading_node);
    let code_factory = KucNodeFactory::new(&[], 120).interaction(ViewerInteractionConfig {
        hover_highlight_enabled: false,
        selection_enabled: false,
        image_controls_enabled: false,
        diagram_controls_enabled: false,
        code_controls_enabled: true,
    });
    let code = code_factory.viewer_node(&viewer_node(
        ViewerNodeKind::Code {
            language: Some("rust".to_string()),
        },
        "fn main() {}",
    ));

    assert!(!has_style_class(&heading, "kdv-document-heading"));
    assert!(!heading.props().common.border.visible);
    assert_eq!("Title With Space", heading.props().text.spans[0].text);
    assert_eq!("Title With Space", heading.props().label);
    assert_eq!(UiNodeKind::Stack, code.kind());
    assert!(!has_style_class(&code.children()[0], "kdv-document-code"));
    assert!(code.children()[0].props().common.border.visible);
    assert_eq!(
        UiDimension::Px(24),
        code.children()[0].props().common.padding.left
    );
}
