use super::support::{
    MediaActionHit, MediaControlFrameSupport, frame_pixel_for_hit, frame_rect_for_hit,
    frame_rect_for_scrolled_hit, preview_frame_bounds,
};
use crate::canvas::Canvas;
use std::collections::BTreeMap;

const DEFAULT_IMAGE_FIXTURE: &str = "direct/kdv-icon.png";
const KUC_INTERACTIVE_HOVER_BORDERS: [u32; 2] = [0x569cd6, 0x0078d4];

#[test]
fn image_controls_have_rendered_frame_hits_for_all_actions()
-> Result<(), Box<dyn std::error::Error>> {
    for fixture in direct_image_fixtures() {
        let scene = MediaControlFrameSupport::build_image_fixture_scene(fixture, true)?;
        let canvas = MediaControlFrameSupport::render_image_fixture_scene_with_action_hover(
            fixture, &scene, true, None,
        );
        let hits = MediaControlFrameSupport::image_action_hits(&scene)?;
        let by_command = command_map(hits)?;

        for command in ["fit", "open", "copy", "reveal-in-os", "zoom-in", "zoom-out"] {
            let hit = by_command.get(command).ok_or_else(|| {
                std::io::Error::other(format!("{fixture} missing image command: {command}"))
            })?;
            assert_eq!(28, hit.hit.rect.width, "{fixture} {command} width");
            assert_eq!(28, hit.hit.rect.height, "{fixture} {command} height");
            assert_visible_hit_pixel(&canvas, hit, fixture, command);
        }
        assert_eq!(
            6,
            by_command.len(),
            "{fixture} unexpected image control count"
        );
        assert!(
            scene.image_surface_count > 0,
            "{fixture} direct image scene must render an image surface"
        );
        assert_raster_image_pixels_when_applicable(&canvas, fixture, by_command.values());
    }
    Ok(())
}

#[test]
fn image_and_code_controls_hover_draws_kuc_preset_border() -> Result<(), Box<dyn std::error::Error>>
{
    assert_image_hover(true)?;
    assert_image_hover(false)?;
    assert_code_hover(true)?;
    assert_code_hover(false)
}

fn assert_image_hover(dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_image_fixture_scene(DEFAULT_IMAGE_FIXTURE, dark)?;
    let sidebar =
        MediaControlFrameSupport::render_image_fixture_sidebar(DEFAULT_IMAGE_FIXTURE, &scene, dark);
    let normal = MediaControlFrameSupport::render_image_fixture_scene_with_action_hover_and_sidebar(
        DEFAULT_IMAGE_FIXTURE,
        &scene,
        dark,
        None,
        &sidebar,
    );
    for action_hit in MediaControlFrameSupport::image_action_hits(&scene)? {
        let hovered =
            MediaControlFrameSupport::render_image_fixture_scene_with_action_hover_and_sidebar(
                DEFAULT_IMAGE_FIXTURE,
                &scene,
                dark,
                Some(&action_hit.hit.action.target),
                &sidebar,
            );
        assert_hover_pixel(&normal, &hovered, &action_hit);
    }
    Ok(())
}

fn assert_code_hover(dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_code_scene(dark)?;
    let sidebar = MediaControlFrameSupport::render_code_scene_sidebar(&scene, dark);
    let mut normal_by_scroll = BTreeMap::new();
    for action_hit in MediaControlFrameSupport::code_action_hits(&scene)? {
        assert_eq!("copy-code", action_hit.command);
        assert_eq!(28, action_hit.hit.rect.width);
        assert_eq!(28, action_hit.hit.rect.height);
        let scroll_y = scroll_for_hit(&action_hit);
        let normal = normal_by_scroll
            .entry(scroll_y.to_bits())
            .or_insert_with(|| {
                MediaControlFrameSupport::render_code_scene_with_scroll_action_hover_and_sidebar(
                    &scene, dark, scroll_y, None, &sidebar,
                )
            });
        let hovered =
            MediaControlFrameSupport::render_code_scene_with_scroll_action_hover_and_sidebar(
                &scene,
                dark,
                scroll_y,
                Some(&action_hit.hit.action.target),
                &sidebar,
            );
        assert_hover_pixel_at_scroll(normal, &hovered, &action_hit, scroll_y);
    }
    Ok(())
}

fn assert_hover_pixel(normal: &Canvas, hovered: &Canvas, action_hit: &MediaActionHit) {
    assert_hover_pixel_at_scroll(normal, hovered, action_hit, 0.0);
}

fn assert_hover_pixel_at_scroll(
    normal: &Canvas,
    hovered: &Canvas,
    action_hit: &MediaActionHit,
    scroll_y: f32,
) {
    let rect = frame_rect_for_scrolled_hit(&action_hit.hit, scroll_y);
    let normal_count = kuc_hover_border_count(normal, rect);
    let hovered_count = kuc_hover_border_count(hovered, rect);
    assert!(
        hovered_count > normal_count,
        "hovered frame must increase KUC border at {}: target={} normal={normal_count} hovered={hovered_count} rect={rect:?} diff={:?} hover_bounds={:?}",
        action_hit.command,
        action_hit.hit.action.target.as_str(),
        diff_bounds(normal, hovered),
        kuc_hover_border_bounds(hovered),
    );
}

fn kuc_hover_border_count(canvas: &Canvas, rect: (usize, usize, usize, usize)) -> usize {
    let (left, top, right, bottom) = rect;
    let right = right.min(canvas.width());
    let bottom = bottom.min(canvas.height());
    (top..bottom)
        .flat_map(|y| (left..right).map(move |x| y * canvas.width() + x))
        .filter(|index| KUC_INTERACTIVE_HOVER_BORDERS.contains(&canvas.pixels()[*index]))
        .count()
}

fn kuc_hover_border_bounds(canvas: &Canvas) -> Option<(usize, usize, usize, usize)> {
    let mut min_x = canvas.width();
    let mut max_x = 0usize;
    let mut min_y = canvas.height();
    let mut max_y = 0usize;
    let mut found = false;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        if !KUC_INTERACTIVE_HOVER_BORDERS.contains(pixel) {
            continue;
        }
        let x = index % canvas.width();
        let y = index / canvas.width();
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        found = true;
    }
    found.then_some((min_x, max_x, min_y, max_y))
}

fn diff_bounds(left: &Canvas, right: &Canvas) -> Option<(usize, usize, usize, usize, usize)> {
    let mut min_x = left.width();
    let mut max_x = 0usize;
    let mut min_y = left.height();
    let mut max_y = 0usize;
    let mut count = 0usize;
    for (index, (left_pixel, right_pixel)) in
        left.pixels().iter().zip(right.pixels().iter()).enumerate()
    {
        if left_pixel == right_pixel {
            continue;
        }
        let x = index % left.width();
        let y = index / left.width();
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        count = count.saturating_add(1);
    }
    (count > 0).then_some((min_x, max_x, min_y, max_y, count))
}

fn scroll_for_hit(action_hit: &MediaActionHit) -> f32 {
    (action_hit.hit.rect.y as f32 - 120.0).max(0.0)
}

fn assert_visible_hit_pixel(
    canvas: &Canvas,
    action_hit: &MediaActionHit,
    fixture: &str,
    command: &str,
) {
    let (x, y) = frame_pixel_for_hit(&action_hit.hit);
    let background = canvas.pixels()[0];
    assert_ne!(
        background,
        canvas.pixels()[y * canvas.width() + x],
        "{fixture} control hit point is not visibly rendered: {command}"
    );
}

fn direct_image_fixtures() -> [&'static str; 7] {
    [
        "direct/kdv-icon.bmp",
        "direct/kdv-icon.gif",
        "direct/kdv-icon.jpeg",
        "direct/kdv-icon.jpg",
        "direct/kdv-icon.png",
        "direct/kdv-icon.svg",
        "direct/kdv-icon.webp",
    ]
}

fn assert_raster_image_pixels_when_applicable<'a>(
    canvas: &Canvas,
    fixture: &str,
    control_hits: impl Iterator<Item = &'a MediaActionHit>,
) {
    if fixture == "direct/kdv-icon.svg" {
        return;
    }
    let control_rects = control_hits
        .map(|hit| frame_rect_for_hit(&hit.hit))
        .collect::<Vec<_>>();
    let blue_pixels = kdv_icon_blue_pixel_count(canvas, &control_rects);
    assert!(
        blue_pixels > 4_000,
        "{fixture} did not paint enough KDV icon pixels: {blue_pixels}"
    );
}

fn kdv_icon_blue_pixel_count(
    canvas: &Canvas,
    control_rects: &[(usize, usize, usize, usize)],
) -> usize {
    let (left, top, right, bottom) = preview_frame_bounds();
    let mut count = 0usize;
    for y in top..bottom.min(canvas.height()) {
        for x in left..right.min(canvas.width()) {
            if is_inside_any_rect(x, y, control_rects) {
                continue;
            }
            count += usize::from(is_kdv_icon_blue(canvas.pixels()[y * canvas.width() + x]));
        }
    }
    count
}

fn is_inside_any_rect(x: usize, y: usize, rects: &[(usize, usize, usize, usize)]) -> bool {
    rects
        .iter()
        .any(|(left, top, right, bottom)| x >= *left && x < *right && y >= *top && y < *bottom)
}

fn is_kdv_icon_blue(pixel: u32) -> bool {
    let red = (pixel >> 16) & 0xff;
    let green = (pixel >> 8) & 0xff;
    let blue = pixel & 0xff;
    blue > 120 && green > 70 && blue > red + 32
}

fn command_map(
    hits: Vec<MediaActionHit>,
) -> Result<BTreeMap<String, MediaActionHit>, std::io::Error> {
    let mut by_command = BTreeMap::new();
    for hit in hits {
        if by_command.insert(hit.command.clone(), hit).is_some() {
            return Err(std::io::Error::other("duplicated image command"));
        }
    }
    Ok(by_command)
}
