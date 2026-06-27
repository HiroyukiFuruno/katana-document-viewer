use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::frame_pixel_guard::StorybookFramePixelGuard;
use crate::frame_preview_pixels::FramePreviewPixels;
use crate::preview::{PreviewBuilder, PreviewScene};
use crate::preview_theme_bridge::KucThemeBridge;
use katana_document_viewer::KdvThemeSnapshot;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use std::path::PathBuf;

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 12000;
const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 11800.0;

#[test]
fn scaled_frame_keeps_logical_size_and_uses_physical_buffer() {
    let fixture = FrameFeatureTestSupport::fixture("katana/sample_basic.md");
    let canvas = StorybookFrameRenderer::render_scaled(
        FrameRenderRequest {
            width: FRAME_WIDTH,
            height: 720,
            fixtures: &[fixture],
            selected_index: 0,
            scene: None,
            scroll_y: 0.0,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &Default::default(),
            dark: true,
            interaction: &ViewerInteractionConfig::default(),
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: None,
            hovered_action_node_id: None,
            animation_phase: 0,
        },
        2.0,
    );

    assert_eq!(FRAME_WIDTH, canvas.logical_width());
    assert_eq!(720, canvas.logical_height());
    assert_eq!(FRAME_WIDTH * 2, canvas.width());
    assert_eq!(1440, canvas.height());
}

#[test]
fn katana_feature_matrix_reaches_storybook_frame_pixels() -> Result<(), Box<dyn std::error::Error>>
{
    let table_row_color = FrameFeatureTestSupport::dark_theme_rgb("table-row-background")?;
    let alert_warning_color = FrameFeatureTestSupport::dark_theme_rgb("alert-warning")?;
    let border_color = FrameFeatureTestSupport::dark_theme_rgb("border")?;
    let quote_background_color = FrameFeatureTestSupport::dark_theme_rgb("quote-background")?;
    let footnote_background_color = FrameFeatureTestSupport::dark_theme_rgb("footnote-background")?;
    for case in FrameFeatureTestSupport::feature_pixel_cases(
        table_row_color,
        alert_warning_color,
        border_color,
        quote_background_color,
        footnote_background_color,
    ) {
        let rendered = FrameFeatureTestSupport::render_fixture_with_scene(case.fixture)?;

        assert!(
            FrameFeatureTestSupport::preview_content_pixel_count(&rendered.canvas)
                >= case.minimum_preview_pixels,
            "{}",
            case.fixture
        );
        assert!(
            rendered.scene.image_surface_count >= case.minimum_image_surfaces,
            "{}",
            case.fixture
        );
        for color in case.required_colors {
            assert!(
                FramePreviewPixels::count_color(&rendered.canvas, color) > 0,
                "{} missing #{color:06x}",
                case.fixture
            );
        }
    }
    Ok(())
}

#[test]
fn direct_media_frames_cover_all_supported_visual_inputs() -> Result<(), Box<dyn std::error::Error>>
{
    for path in FrameFeatureTestSupport::direct_visual_fixtures() {
        let rendered = FrameFeatureTestSupport::render_fixture_with_scene(path)?;

        assert!(rendered.scene.image_surface_count > 0, "{path}");
        assert_eq!(0, rendered.scene.failed_asset_count, "{path}");
        assert!(
            FrameFeatureTestSupport::preview_content_pixel_count(&rendered.canvas) > 256,
            "{path}"
        );
    }
    Ok(())
}

#[test]
fn direct_svg_reaches_storybook_frame_as_retina_image_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FrameFeatureTestSupport::fixture("direct/kdv-icon.svg");
    let scene = PreviewBuilder::default().build(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;
    let image = FrameFeatureTestSupport::first_image_surface(scene.tree.root())
        .ok_or("svg image missing")?;

    assert_eq!(200, image.props().image_surface.content_scale);
    assert_eq!(192, image.props().image_surface.width);
    assert_eq!(192, image.props().image_surface.height);

    let canvas = StorybookFrameRenderer::render_scaled(
        FrameRenderRequest {
            width: FRAME_WIDTH,
            height: 720,
            fixtures: &[fixture],
            selected_index: 0,
            scene: Some(&scene),
            scroll_y: 0.0,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &Default::default(),
            dark: true,
            interaction: &ViewerInteractionConfig::default(),
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: None,
            hovered_action_node_id: None,
            animation_phase: 0,
        },
        2.0,
    );

    assert_eq!(FRAME_WIDTH * 2, canvas.width());
    assert_eq!(720 * 2, canvas.height());
    assert!(FrameFeatureTestSupport::preview_content_pixel_count(&canvas) > 512);
    Ok(())
}

struct FrameFeatureTestSupport;

impl FrameFeatureTestSupport {
    fn render_fixture_with_scene(
        path: &str,
    ) -> Result<RenderedFixture, Box<dyn std::error::Error>> {
        let fixture = Self::fixture(path);
        let scene = PreviewBuilder::default().build(
            &fixture,
            ViewerViewport {
                width: PREVIEW_WIDTH,
                height: PREVIEW_HEIGHT,
            },
            true,
            ViewerInteractionConfig::default(),
        )?;
        let height = Self::frame_height(&scene);
        let canvas = StorybookFrameRenderer::render(FrameRenderRequest {
            width: FRAME_WIDTH,
            height,
            fixtures: &[fixture],
            selected_index: 0,
            scene: Some(&scene),
            scroll_y: 0.0,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &Default::default(),
            dark: true,
            interaction: &ViewerInteractionConfig::default(),
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: None,
            hovered_action_node_id: None,
            animation_phase: 0,
        });
        Ok(RenderedFixture { canvas, scene })
    }

    fn preview_content_pixel_count(canvas: &Canvas) -> usize {
        StorybookFramePixelGuard::preview_content_pixel_count(canvas, true)
    }

    fn first_image_surface(node: &UiNode) -> Option<&UiNode> {
        if node.kind() == UiNodeKind::ImageSurface {
            return Some(node);
        }
        node.children().iter().find_map(Self::first_image_surface)
    }

    fn frame_height(scene: &PreviewScene) -> usize {
        scene
            .surface
            .as_ref()
            .map(|surface| surface.height as usize + crate::layout::HEADER_HEIGHT + 96)
            .unwrap_or(FRAME_HEIGHT)
            .max(FRAME_HEIGHT)
    }

    fn direct_visual_fixtures() -> &'static [&'static str] {
        &[
            "direct/kdv-icon.bmp",
            "direct/kdv-icon.gif",
            "direct/kdv-icon.jpeg",
            "direct/kdv-icon.jpg",
            "direct/kdv-icon.png",
            "direct/kdv-icon.svg",
            "direct/kdv-icon.webp",
            "direct/sample.drawio",
            "direct/sample.drowio",
            "direct/sample.mermaid",
            "direct/sample.mmd",
            "direct/sample.plantuml",
            "direct/sample.puml",
        ]
    }

    fn feature_pixel_cases(
        table_row_color: u32,
        alert_warning_color: u32,
        border_color: u32,
        quote_background_color: u32,
        footnote_background_color: u32,
    ) -> Vec<FrameFeaturePixelCase> {
        vec![
            FrameFeaturePixelCase {
                fixture: "katana/sample_basic.md",
                required_colors: vec![
                    border_color,
                    table_row_color,
                    alert_warning_color,
                    quote_background_color,
                    footnote_background_color,
                ],
                minimum_preview_pixels: 1024,
                minimum_image_surfaces: 0,
            },
            FrameFeaturePixelCase {
                fixture: "direct/html-alignment.html",
                required_colors: vec![table_row_color],
                minimum_preview_pixels: 256,
                minimum_image_surfaces: 0,
            },
            FrameFeaturePixelCase {
                fixture: "katana/sample_diagrams.md",
                required_colors: vec![],
                minimum_preview_pixels: 1024,
                minimum_image_surfaces: 3,
            },
            FrameFeaturePixelCase {
                fixture: "direct/sample.md",
                required_colors: vec![],
                minimum_preview_pixels: 512,
                minimum_image_surfaces: 1,
            },
            FrameFeaturePixelCase {
                fixture: "direct/kdv-icon.png",
                required_colors: vec![],
                minimum_preview_pixels: 256,
                minimum_image_surfaces: 1,
            },
        ]
    }

    fn dark_theme_rgb(name: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let theme = KucThemeBridge::from_kdv(&KdvThemeSnapshot::katana_dark())?;
        let rgba = theme
            .color(name)
            .ok_or_else(|| format!("missing KUC theme color token: {name}"))?;
        Ok(((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | rgba[2] as u32)
    }

    fn fixture(path: &str) -> StorybookFixture {
        StorybookFixture {
            label: path.to_string(),
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("../../assets/fixtures/{path}")),
        }
    }
}

struct RenderedFixture {
    canvas: Canvas,
    scene: PreviewScene,
}

struct FrameFeaturePixelCase {
    fixture: &'static str,
    required_colors: Vec<u32>,
    minimum_preview_pixels: usize,
    minimum_image_surfaces: usize,
}
