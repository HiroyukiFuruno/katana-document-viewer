use super::*;
use crate::export_surface_line::SurfaceTaskMarker;
use crate::theme::KdvThemeSnapshot;
use image::Rgba;

const TASK_MARKER_FRAME_WIDTH: u32 = 80;
const TASK_MARKER_FRAME_HEIGHT: u32 = 40;
const TASK_MARKER_X: u32 = 4;
const TASK_MARKER_Y: u32 = 8;
const TASK_MARKER_TEXT_SIZE: f32 = 20.0;
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);

fn palette() -> SurfacePaintPalette {
    SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light())
}

#[test]
fn paint_material_bullet_cycles_variants() {
    let mut painter = None;

    for indent_depth in 0..3 {
        let mut frame = image::RgbaImage::from_pixel(80, 40, Rgba([255, 255, 255, 255]));
        let request = SurfaceMarkerPaintRequest {
            marker: &SurfaceLineMarker::Bullet,
            x: 4,
            y: 8,
            indent_depth,
            size: 20.0,
        };
        SurfacePainter::paint_line_marker(&mut frame, request, &mut painter, &palette());
        assert!(
            frame
                .pixels()
                .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
        );
    }
}

#[test]
fn paint_task_marker_variants_render_distinct_marks() {
    let image_done = paint_task_marker_image(SurfaceTaskMarker::Done);
    let image_blocked = paint_task_marker_image(SurfaceTaskMarker::Blocked);
    let image_progress = paint_task_marker_image(SurfaceTaskMarker::InProgress);
    let image_empty = paint_task_marker_image(SurfaceTaskMarker::Empty);

    assert_ne!(image_done.get_pixel(10, 20).0, [255, 255, 255, 255]);
    assert_ne!(image_blocked.get_pixel(10, 20).0, [255, 255, 255, 255]);
    assert_ne!(image_progress.get_pixel(10, 20).0, [255, 255, 255, 255]);
    assert_ne!(image_empty.get_pixel(8, 16).0, [255, 255, 255, 255]);
    assert_ne!(image_done, image_blocked);
    assert_ne!(image_empty, image_done);
}

fn paint_task_marker_image(marker: SurfaceTaskMarker) -> image::RgbaImage {
    let mut image =
        image::RgbaImage::from_pixel(TASK_MARKER_FRAME_WIDTH, TASK_MARKER_FRAME_HEIGHT, WHITE);
    let mut painter = None;
    SurfacePainter::paint_line_marker(
        &mut image,
        SurfaceMarkerPaintRequest {
            marker: &SurfaceLineMarker::Task(marker),
            x: TASK_MARKER_X,
            y: TASK_MARKER_Y,
            indent_depth: 0,
            size: TASK_MARKER_TEXT_SIZE,
        },
        &mut painter,
        &palette(),
    );
    image
}

#[test]
fn paint_ordered_marker_uses_painter_when_available() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = image::RgbaImage::from_pixel(80, 40, Rgba([255, 255, 255, 255]));
    let mut painter = Some(
        crate::export_surface_font::SurfaceTextPainter::from_system_fonts()
            .ok_or("system font should be available")?,
    );
    SurfacePainter::paint_line_marker(
        &mut image,
        SurfaceMarkerPaintRequest {
            marker: &SurfaceLineMarker::Ordered("12.".to_string()),
            x: 2,
            y: 2,
            indent_depth: 0,
            size: 16.0,
        },
        &mut painter,
        &palette(),
    );

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
    Ok(())
}

#[test]
fn paint_ordered_marker_fallback_path() {
    let mut image = image::RgbaImage::from_pixel(80, 40, Rgba([255, 255, 255, 255]));
    let mut painter = None;
    SurfacePainter::paint_line_marker(
        &mut image,
        SurfaceMarkerPaintRequest {
            marker: &SurfaceLineMarker::Ordered("12.".to_string()),
            x: 2,
            y: 2,
            indent_depth: 0,
            size: 16.0,
        },
        &mut painter,
        &palette(),
    );

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}
