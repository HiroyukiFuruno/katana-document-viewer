use super::support::{MediaActionHit, MediaControlFrameSupport, color, frame_rect_for_hit};
use crate::KucDiagramControlResolver;
use crate::canvas::Canvas;
use crate::layout::preview_content_width;
use katana_ui_core_storybook::UiTreeHostActionHit;
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeSurfaceHost};

#[test]
fn diagram_control_buttons_keep_transparent_base_in_dark_and_light()
-> Result<(), Box<dyn std::error::Error>> {
    assert_transparent_control_base(true)?;
    assert_transparent_control_base(false)
}

#[test]
fn image_control_buttons_keep_transparent_base_in_dark_and_light()
-> Result<(), Box<dyn std::error::Error>> {
    assert_transparent_image_control_base(true)?;
    assert_transparent_image_control_base(false)
}

fn assert_transparent_control_base(dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_scene(dark)?;
    let canvas = MediaControlFrameSupport::render_scene(&scene, dark);
    let hits = MediaControlFrameSupport::diagram_action_hits(&scene)?;
    let fill = color(&scene.theme, "selection")?;

    assert_eq!(
        1,
        hits.len(),
        "KatanA normal diagram top control exposes fullscreen only"
    );
    for action_hit in hits {
        assert_transparent_hit(&canvas, &action_hit, fill, dark);
    }
    let hits = MediaControlFrameSupport::diagram_action_hits(&scene)?;
    assert_no_selection_fill_between_controls(&canvas, &hits, fill, dark);
    let internal_hits = internal_diagram_control_hits(&scene);
    assert_eq!(
        [
            "pan-up",
            "zoom-in",
            "pan-left",
            "reset-view",
            "pan-right",
            "trackpad-help",
            "pan-down",
            "zoom-out"
        ],
        internal_hits
            .iter()
            .map(|hit| hit.command.as_str())
            .collect::<Vec<_>>()
            .as_slice(),
        "diagram internal controls must keep the full KatanA control set"
    );
    for action_hit in &internal_hits {
        assert_transparent_internal_hit(&canvas, action_hit, fill, dark);
    }
    assert_no_selection_fill_between_internal_controls(&canvas, &internal_hits, fill, dark);
    Ok(())
}

fn assert_transparent_image_control_base(dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_image_fixture_scene("direct/kdv-icon.png", dark)?;
    let canvas = MediaControlFrameSupport::render_image_fixture_scene_with_action_hover(
        "direct/kdv-icon.png",
        &scene,
        dark,
        None,
    );
    let hits = MediaControlFrameSupport::image_action_hits(&scene)?;
    let fill = color(&scene.theme, "selection")?;

    assert_eq!(
        6,
        hits.len(),
        "image fixture must expose every controller button"
    );
    for action_hit in hits {
        assert_transparent_hit(&canvas, &action_hit, fill, dark);
    }
    let hits = MediaControlFrameSupport::image_action_hits(&scene)?;
    assert_no_selection_fill_between_controls(&canvas, &hits, fill, dark);
    Ok(())
}

fn assert_transparent_hit(canvas: &Canvas, action_hit: &MediaActionHit, fill: u32, dark: bool) {
    let filled = count_color_inside_control(canvas, &action_hit.hit, fill);
    let rect = &action_hit.hit.rect;
    assert_eq!(
        0, filled,
        "media control base must stay transparent and not use selection fill: dark={dark} action={} rect=({}, {}, {}, {})",
        action_hit.command, rect.x, rect.y, rect.width, rect.height
    );
}

fn count_color_inside_control(canvas: &Canvas, hit: &UiTreeHostActionHit, color: u32) -> usize {
    let (left, top, right, bottom) = frame_rect_for_hit(hit);
    let left = left + 2;
    let top = top + 2;
    let right = right.saturating_sub(2);
    let bottom = bottom.saturating_sub(2);
    let mut count = 0usize;
    for y in top..bottom.min(canvas.height()) {
        for x in left..right.min(canvas.width()) {
            count += usize::from(canvas.pixels()[y * canvas.width() + x] == color);
        }
    }
    count
}

fn assert_transparent_internal_hit(
    canvas: &Canvas,
    action_hit: &InternalControlHit,
    fill: u32,
    dark: bool,
) {
    let filled = count_color_inside_internal_control(canvas, action_hit, fill);
    assert_eq!(
        0, filled,
        "diagram internal control base must stay transparent and not use selection fill: dark={dark} action={} rect=({}, {}, {}, {})",
        action_hit.command, action_hit.left, action_hit.top, action_hit.right, action_hit.bottom
    );
}

fn count_color_inside_internal_control(
    canvas: &Canvas,
    hit: &InternalControlHit,
    color: u32,
) -> usize {
    let left = hit.left + 2;
    let top = hit.top + 2;
    let right = hit.right.saturating_sub(2);
    let bottom = hit.bottom.saturating_sub(2);
    let mut count = 0usize;
    for y in top..bottom.min(canvas.height()) {
        for x in left..right.min(canvas.width()) {
            count += usize::from(canvas.pixels()[y * canvas.width() + x] == color);
        }
    }
    count
}

fn assert_no_selection_fill_between_controls(
    canvas: &Canvas,
    hits: &[MediaActionHit],
    fill: u32,
    dark: bool,
) {
    let mut rows = hits.iter().collect::<Vec<_>>();
    rows.sort_by_key(|hit| (hit.hit.rect.y, hit.hit.rect.x));
    for pair in rows.windows(2) {
        let left = pair[0];
        let right = pair[1];
        if left.hit.rect.y != right.hit.rect.y {
            continue;
        }
        let filled = count_color_between_controls(canvas, &left.hit, &right.hit, fill);
        assert_eq!(
            0, filled,
            "media control row gap must stay transparent: dark={dark} left={} right={}",
            left.command, right.command
        );
    }
}

fn count_color_between_controls(
    canvas: &Canvas,
    left: &UiTreeHostActionHit,
    right: &UiTreeHostActionHit,
    color: u32,
) -> usize {
    let (_, top, left_edge, bottom) = frame_rect_for_hit(left);
    let (right_edge, _, _, _) = frame_rect_for_hit(right);
    if left_edge >= right_edge {
        return 0;
    }
    let mut count = 0usize;
    for y in top..bottom.min(canvas.height()) {
        for x in left_edge..right_edge.min(canvas.width()) {
            count += usize::from(canvas.pixels()[y * canvas.width() + x] == color);
        }
    }
    count
}

fn assert_no_selection_fill_between_internal_controls(
    canvas: &Canvas,
    hits: &[InternalControlHit],
    fill: u32,
    dark: bool,
) {
    let mut rows = hits.iter().collect::<Vec<_>>();
    rows.sort_by_key(|hit| (hit.top, hit.left));
    for pair in rows.windows(2) {
        let left = pair[0];
        let right = pair[1];
        if left.top != right.top {
            continue;
        }
        let filled = count_color_between_internal_controls(canvas, left, right, fill);
        assert_eq!(
            0, filled,
            "diagram internal control row gap must stay transparent: dark={dark} left={} right={}",
            left.command, right.command
        );
    }
}

fn count_color_between_internal_controls(
    canvas: &Canvas,
    left: &InternalControlHit,
    right: &InternalControlHit,
    color: u32,
) -> usize {
    if left.right >= right.left {
        return 0;
    }
    let mut count = 0usize;
    for y in left.top..left.bottom.min(canvas.height()) {
        for x in left.right..right.left.min(canvas.width()) {
            count += usize::from(canvas.pixels()[y * canvas.width() + x] == color);
        }
    }
    count
}

fn internal_diagram_control_hits(scene: &crate::preview::PreviewScene) -> Vec<InternalControlHit> {
    UiTreeSurfaceHost::new(scene.theme.clone())
        .document_node_hits(
            scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_content_width(super::support::FRAME_WIDTH),
                height: scene.content_height.ceil().max(1.0) as usize,
                scroll_y: 0.0,
            },
        )
        .into_iter()
        .filter_map(|hit| {
            let action = KucDiagramControlResolver::internal_action_for_node(
                scene.tree.root(),
                &hit.node_id,
            )?;
            Some(InternalControlHit {
                command: action.command,
                left: super::support::FRAME_PREVIEW_LEFT + hit.rect.x,
                top: super::support::FRAME_PREVIEW_TOP + hit.rect.y,
                right: super::support::FRAME_PREVIEW_LEFT + hit.rect.x + hit.rect.width,
                bottom: super::support::FRAME_PREVIEW_TOP + hit.rect.y + hit.rect.height,
            })
        })
        .collect()
}

struct InternalControlHit {
    command: String,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}
