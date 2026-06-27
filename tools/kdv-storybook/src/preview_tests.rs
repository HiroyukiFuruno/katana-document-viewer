use super::PreviewBuilder;
use crate::catalog::StorybookFixture;
use crate::frame::StorybookFrameRenderer;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use katana_document_viewer::{
    KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX, ViewerInteractionConfig, ViewerMode,
    ViewerSearchState, ViewerTypographyConfig, ViewerViewport,
};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind};
use std::path::PathBuf;

#[test]
fn preview_build_does_not_paint_hover_without_active_target()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "direct/sample.md".to_string(),
        path: fixture_path("assets/fixtures/direct/sample.md"),
    };
    let viewport = ViewerViewport {
        width: 800.0,
        height: 600.0,
    };
    let disabled = PreviewBuilder::default().build_without_preview_surface(
        &fixture,
        viewport,
        true,
        ViewerInteractionConfig {
            hover_highlight_enabled: false,
            ..ViewerInteractionConfig::default()
        },
    )?;
    let enabled = PreviewBuilder::default().build_without_preview_surface(
        &fixture,
        viewport,
        true,
        ViewerInteractionConfig::default(),
    )?;

    assert_eq!(0, style_count(disabled.tree.root(), "kdv-hover-highlight"));
    assert_eq!(0, style_count(enabled.tree.root(), "kdv-hover-highlight"));
    Ok(())
}

#[test]
fn preview_typography_build_does_not_attach_export_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = PreviewBuilder::default().build_with_typography(
        &StorybookFixture {
            label: "katana/sample.md".to_string(),
            path: fixture_path("assets/fixtures/katana/sample.md"),
        },
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        false,
        ViewerTypographyConfig::default(),
    )?;

    assert!(scene.surface.is_none());
    Ok(())
}

#[test]
fn preview_build_keeps_kuc_diagram_controls() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "direct/sample.md".to_string(),
        path: fixture_path("assets/fixtures/direct/sample.md"),
    };
    let scene = PreviewBuilder::default().build_without_preview_surface(
        &fixture,
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;

    assert!(action_count(scene.tree.root(), "pan-up") > 0);
    assert!(action_count(scene.tree.root(), "reset-view") > 0);
    assert!(action_count(scene.tree.root(), "fullscreen") > 0);
    Ok(())
}

#[test]
fn preview_lazy_build_keeps_diagrams_pending_without_blocking()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = PreviewBuilder::default().build_lazy_with_mode_and_search(
        &StorybookFixture {
            label: "katana/sample_diagrams.md".to_string(),
            path: fixture_path("assets/fixtures/katana/sample_diagrams.md"),
        },
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        true,
        ViewerInteractionConfig::default(),
        katana_document_viewer::ViewerMode::Document,
        katana_document_viewer::ViewerSearchState::default(),
    )?;

    assert!(scene.asset_request_count >= 3);
    assert_eq!(0, scene.loaded_asset_count);
    assert_eq!(0, scene.image_surface_count);
    assert!(scene.surface.is_none());
    assert!(role_count(scene.tree.root(), "media-pending") >= 3);
    Ok(())
}

#[test]
fn preview_loaded_build_does_not_count_deferred_diagrams_as_pending()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample_diagrams.md")?;

    assert_eq!(
        0, scene.asset_request_count,
        "loaded Storybook scene must not keep offscreen deferred assets pending",
    );
    assert!(scene.loaded_asset_count > 0);
    Ok(())
}

#[test]
fn preview_loaded_build_exposes_preview_surface_for_score_gate()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = PreviewBuilder::default().build(
        &StorybookFixture {
            label: "katana/sample.md".to_string(),
            path: fixture_path("assets/fixtures/katana/sample.md"),
        },
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;
    let surface = scene.surface.ok_or("preview surface missing")?;

    assert!(surface.width > 0);
    assert!(surface.height > 0);
    assert_eq!(
        surface.width as usize * surface.height as usize * 4,
        surface.rgba.len()
    );
    Ok(())
}

#[test]
fn preview_surface_attach_does_not_double_scroll_extent_when_viewport_covers_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_basic.md".to_string(),
        path: fixture_path("assets/fixtures/katana/sample_basic.md"),
    };
    let scene = PreviewBuilder::default().build_scene(PreviewBuildRequest {
        fixture: &fixture,
        viewport: ViewerViewport {
            width: 800.0,
            height: 20_000.0,
        },
        dark: true,
        theme: None,
        interaction: ViewerInteractionConfig::default(),
        mode: ViewerMode::Document,
        typography: ViewerTypographyConfig::default(),
        search: ViewerSearchState::default(),
        diagram_viewports: Default::default(),
        image_viewports: Default::default(),
        task_state_overrides: Default::default(),
        accordion_open_overrides: Default::default(),
        copied_code_node_ids: Default::default(),
        asset_mode: PreviewBuildAssetMode::Lazy,
        attach_surface: true,
        export_surface: true,
    })?;
    let surface = scene.surface.ok_or("preview surface missing")?;

    let scaled_surface_height = surface.content_height as f32 * 800.0 / surface.width as f32;
    assert!(
        scaled_surface_height <= 20_000.0,
        "test fixture must fit in viewport to validate no-tail contract"
    );
    assert_eq!(scaled_surface_height, scene.content_height);
    Ok(())
}

#[test]
fn preview_lazy_build_keeps_diagram_fixture_pending_without_blocking()
-> Result<(), Box<dyn std::error::Error>> {
    StorybookFrameRenderer::prewarm();
    let scene = PreviewBuilder::default().build_lazy_with_mode_and_search(
        &StorybookFixture {
            label: "katana/sample_diagrams.md".to_string(),
            path: fixture_path("assets/fixtures/katana/sample_diagrams.md"),
        },
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        true,
        ViewerInteractionConfig::default(),
        katana_document_viewer::ViewerMode::Document,
        katana_document_viewer::ViewerSearchState::default(),
    )?;

    assert!(scene.asset_request_count >= 3);
    Ok(())
}

#[test]
fn preview_build_katana_sample_keeps_markdown_roles() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample_basic.md")?;

    for role in [
        "heading",
        "code",
        "table",
        "alert",
        "blockquote",
        "footnote",
        "list-marker",
        "list-item",
    ] {
        assert!(
            role_count(scene.tree.root(), role) > 0,
            "missing KUC text role: {role}",
        );
    }
    assert!(indented_list_node_count(scene.tree.root()) > 0);
    Ok(())
}

#[test]
fn preview_build_katana_sample_keeps_heading_inline_code_spans_without_raw_backticks()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample.md")?;
    let heading = first_node_with_label_fragment(scene.tree.root(), "Centered Heading")
        .ok_or("heading node missing")?;

    assert!(
        heading
            .props()
            .text
            .spans
            .iter()
            .any(|span| span.text == r#"<h1 align="center">"# && span.style.inline_code)
    );
    assert!(!heading.props().label.contains('`'));
    assert!(!heading.props().label.starts_with('#'));
    assert!(
        !heading
            .props()
            .text
            .spans
            .iter()
            .any(|span| span.text.contains('`'))
    );
    assert!(
        !heading
            .props()
            .text
            .spans
            .iter()
            .any(|span| span.text.contains('#'))
    );
    Ok(())
}

#[test]
fn preview_build_katana_sample_preserves_heading_spaces_in_kuc_tree()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample.md")?;
    let h1 = first_node_with_label_fragment(scene.tree.root(), "H1 Heading")
        .ok_or("H1 heading node missing")?;
    let numbered = first_node_with_label_fragment(scene.tree.root(), "2.1 Heading Levels")
        .ok_or("numbered heading node missing")?;

    assert_eq!("H1 Heading", h1.props().label);
    assert_eq!("H1 Heading", text_span_label(h1));
    assert_eq!(vec!["H1 Heading"], text_span_texts(h1));
    assert_eq!("2.1 Heading Levels", numbered.props().label);
    assert_eq!("2.1 Heading Levels", text_span_label(numbered));
    assert_eq!(vec!["2.1 Heading Levels"], text_span_texts(numbered));
    Ok(())
}

#[test]
fn preview_build_katana_sample_renders_readme_header_data_svg_without_raw_uri()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample.md")?;

    assert_eq!(
        0,
        label_fragment_count(scene.tree.root(), "data:image/svg+xml"),
        "README header SVG data URI must be rendered as an image surface, not visible raw text"
    );
    for fragment in ["width=%22128", "dominant-baseline", "text-anchor=%22middle"] {
        assert_eq!(
            0,
            label_fragment_count(scene.tree.root(), fragment),
            "README header SVG data URI fragment must not be visible raw text: {fragment}"
        );
    }
    assert!(
        kind_count(scene.tree.root(), UiNodeKind::ImageSurface) > 0,
        "README header SVG data URI must produce a KUC image surface"
    );
    Ok(())
}

#[test]
fn preview_build_direct_html_keeps_alignment_roles() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("direct/html-alignment.html")?;

    assert!(role_count(scene.tree.root(), "html-centered-preview") > 0);
    assert!(role_count(scene.tree.root(), "html-right-preview") > 0);
    assert!(role_count(scene.tree.root(), "html-left-preview") > 0);
    let right = first_node_for_role(scene.tree.root(), "html-right-preview")
        .ok_or("html-right-preview node missing")?;
    let expected_content_width = 800 - KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX * 2;
    assert_eq!(
        UiDimension::Px(expected_content_width),
        right.props().common.width,
        "html-right node width must follow KatanA interactive preview content padding"
    );
    assert!(role_count(scene.tree.root(), "table") > 0);
    assert!(
        kind_count(scene.tree.root(), UiNodeKind::Accordion) > 0,
        "accordion kind missing; html-accordion role count={} label={}",
        role_count(scene.tree.root(), "html-accordion"),
        first_label_for_role(scene.tree.root(), "html-accordion")
    );
    Ok(())
}

fn build_scene(path: &str) -> Result<super::PreviewScene, Box<dyn std::error::Error>> {
    PreviewBuilder::default().build_without_preview_surface(
        &StorybookFixture {
            label: path.to_string(),
            path: fixture_path(&format!("assets/fixtures/{path}")),
        },
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        true,
        ViewerInteractionConfig::default(),
    )
}

#[test]
fn sample_top_description_target_uses_rendered_line_height()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample.md".to_string(),
        path: fixture_path("assets/fixtures/katana/sample.md"),
    };
    let scene = PreviewBuilder::default().build_with_typography(
        &fixture,
        ViewerViewport {
            width: 1280.0,
            height: 2400.0,
        },
        false,
        ViewerTypographyConfig {
            preview_font_size: 14,
        },
    )?;
    let target = scene
        .targets
        .iter()
        .find(|target| {
            target
                .source
                .raw
                .text
                .starts_with("This document is a comprehensive sample")
        })
        .ok_or("sample top description target missing")?;

    assert_eq!(46.0, target.rect.height);
    Ok(())
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}

fn style_count(node: &UiNode, style: &str) -> usize {
    usize::from(
        node.props()
            .style_classes
            .iter()
            .any(|value| value == style),
    ) + node
        .children()
        .iter()
        .map(|child| style_count(child, style))
        .sum::<usize>()
}

fn indented_list_node_count(node: &UiNode) -> usize {
    usize::from(matches!(
        node.props().common.margin.left,
        UiDimension::Px(value) if value > 0
    )) + node
        .children()
        .iter()
        .map(indented_list_node_count)
        .sum::<usize>()
}

fn action_count(node: &UiNode, action: &str) -> usize {
    usize::from(node.props().interaction.value == action)
        + node
            .children()
            .iter()
            .map(|child| action_count(child, action))
            .sum::<usize>()
}

fn role_count(node: &UiNode, role: &str) -> usize {
    usize::from(node.props().text.role == role)
        + node
            .children()
            .iter()
            .map(|child| role_count(child, role))
            .sum::<usize>()
}

fn first_node_for_role<'a>(node: &'a UiNode, role: &str) -> Option<&'a UiNode> {
    if node.props().text.role == role {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| first_node_for_role(child, role))
}

fn first_label_for_role(node: &UiNode, role: &str) -> String {
    if node.props().text.role == role {
        return node.props().label.clone();
    }
    for child in node.children() {
        let label = first_label_for_role(child, role);
        if !label.is_empty() {
            return label;
        }
    }
    String::new()
}

fn first_node_with_label_fragment<'a>(node: &'a UiNode, fragment: &str) -> Option<&'a UiNode> {
    if node.props().label.contains(fragment) {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| first_node_with_label_fragment(child, fragment))
}

fn text_span_label(node: &UiNode) -> String {
    node.props()
        .text
        .spans
        .iter()
        .map(|span| span.text.as_str())
        .collect()
}

fn text_span_texts(node: &UiNode) -> Vec<&str> {
    node.props()
        .text
        .spans
        .iter()
        .map(|span| span.text.as_str())
        .collect()
}

fn kind_count(node: &UiNode, expected: UiNodeKind) -> usize {
    usize::from(node.kind() == expected)
        + node
            .children()
            .iter()
            .map(|child| kind_count(child, expected))
            .sum::<usize>()
}

fn label_fragment_count(node: &UiNode, fragment: &str) -> usize {
    usize::from(node.props().label.contains(fragment))
        + node
            .children()
            .iter()
            .map(|child| label_fragment_count(child, fragment))
            .sum::<usize>()
}
