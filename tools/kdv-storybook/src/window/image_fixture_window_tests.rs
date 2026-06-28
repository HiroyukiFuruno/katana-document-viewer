use super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::canvas::Canvas;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::frame_pixel_guard::StorybookFramePixelGuard;
use crate::layout::StorybookPreviewArea;
use crate::preview::PreviewBuilder;
use std::path::PathBuf;

const WINDOW_WIDTH: usize = 1280;
const WINDOW_HEIGHT: usize = 900;
const CONTROL_EXCLUSION_WIDTH: usize = 220;
const MIN_ICON_BLUE_PIXELS: usize = 4_000;

const DIRECT_RASTER_IMAGE_FIXTURES: [&str; 6] = [
    "direct/kdv-icon.bmp",
    "direct/kdv-icon.gif",
    "direct/kdv-icon.jpeg",
    "direct/kdv-icon.jpg",
    "direct/kdv-icon.png",
    "direct/kdv-icon.webp",
];

#[test]
fn direct_raster_image_window_loaded_scenes_render_visible_image_surfaces()
-> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    for label in DIRECT_RASTER_IMAGE_FIXTURES {
        let mut storybook = storybook(label, builder.clone());
        storybook.update_scene_loaded(WINDOW_WIDTH, WINDOW_HEIGHT)?;

        let scene = storybook
            .scene
            .as_ref()
            .ok_or_else(|| format!("{label} scene missing"))?;
        assert_eq!(0, scene.asset_request_count, "{label} pending assets");
        assert_eq!(0, scene.failed_asset_count, "{label} failed assets");
        assert!(scene.loaded_asset_count > 0, "{label} loaded asset count");
        assert!(scene.image_surface_count > 0, "{label} image surface count");

        let canvas = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
        StorybookFramePixelGuard::assert_fixture_content(label, &canvas, storybook.dark)?;
        let blue_pixels = preview_icon_blue_pixel_count(&canvas);
        assert!(
            blue_pixels >= MIN_ICON_BLUE_PIXELS,
            "{label} did not render enough direct image pixels in the Storybook window: {blue_pixels}"
        );
    }
    Ok(())
}

fn storybook(label: &str, builder: PreviewBuilder) -> StorybookWindow {
    StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![StorybookFixture {
                label: label.to_string(),
                path: fixture_path(label),
            }],
        },
        builder,
    )
}

fn preview_icon_blue_pixel_count(canvas: &Canvas) -> usize {
    let area = StorybookPreviewArea::for_window(canvas.width(), canvas.height(), 0.0);
    let right = area
        .x
        .saturating_add(area.width)
        .saturating_sub(CONTROL_EXCLUSION_WIDTH);
    let bottom = area.y.saturating_add(area.height);
    let mut count = 0usize;
    for y in area.y..bottom.min(canvas.height()) {
        for x in area.x..right.min(canvas.width()) {
            count += usize::from(is_kdv_icon_blue(canvas.pixels()[y * canvas.width() + x]));
        }
    }
    count
}

fn is_kdv_icon_blue(pixel: u32) -> bool {
    let red = (pixel >> 16) & 0xff;
    let green = (pixel >> 8) & 0xff;
    let blue = pixel & 0xff;
    blue > 120 && green > 70 && blue > red + 32
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
}
