use super::support::{MediaControlFrameSupport, color, frame_pixel_for_hit};
use crate::KucDiagramControlResolver;
use crate::canvas::Canvas;
use crate::layout::preview_content_width;
use katana_ui_core_storybook::UiTreeHostActionHit;
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeSurfaceHost};

#[test]
fn every_diagram_control_hover_draws_kuc_preset_border() -> Result<(), Box<dyn std::error::Error>> {
    assert_every_control_hover(true)?;
    assert_every_control_hover(false)
}

#[test]
fn every_internal_diagram_control_hover_draws_kuc_preset_border()
-> Result<(), Box<dyn std::error::Error>> {
    assert_every_internal_control_hover(true)?;
    assert_every_internal_control_hover(false)
}

fn assert_every_control_hover(dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_scene(dark)?;
    let sidebar = MediaControlFrameSupport::render_scene_sidebar(&scene, dark);
    let normal = MediaControlFrameSupport::render_scene_with_action_hover_and_sidebar(
        &scene, dark, None, &sidebar,
    );
    let hits = MediaControlFrameSupport::diagram_action_hits(&scene)?;
    let hover_border = color(&scene.theme, "accent")?;
    let normal_count = color_count(&normal, hover_border);

    for action_hit in hits {
        let hovered = MediaControlFrameSupport::render_scene_with_action_hover_and_sidebar(
            &scene,
            dark,
            Some(&action_hit.hit.action.target),
            &sidebar,
        );
        assert_hover_border_pixel(
            &normal,
            &hovered,
            &action_hit.hit,
            hover_border,
            normal_count,
            action_hit.command.as_str(),
        );
    }
    Ok(())
}

fn assert_every_internal_control_hover(dark: bool) -> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_scene(dark)?;
    let sidebar = MediaControlFrameSupport::render_scene_sidebar(&scene, dark);
    let normal = MediaControlFrameSupport::render_scene_with_action_hover_and_sidebar(
        &scene, dark, None, &sidebar,
    );
    let hover_border = color(&scene.theme, "accent")?;
    let normal_count = color_count(&normal, hover_border);
    let hits = internal_diagram_control_hits(&scene);

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
        hits.iter()
            .map(|hit| hit.command.as_str())
            .collect::<Vec<_>>()
            .as_slice(),
        "diagram internal controls must keep the full KatanA control set"
    );

    for action_hit in hits {
        let hovered = MediaControlFrameSupport::render_scene_with_action_hover_and_sidebar(
            &scene,
            dark,
            Some(&action_hit.node_id),
            &sidebar,
        );
        assert_hover_border_pixel_at(
            &normal,
            &hovered,
            action_hit.center_x,
            action_hit.center_y,
            hover_border,
            normal_count,
            action_hit.command.as_str(),
        );
    }
    Ok(())
}

fn assert_hover_border_pixel(
    normal: &Canvas,
    hovered: &Canvas,
    hit: &UiTreeHostActionHit,
    hover_border: u32,
    normal_count: usize,
    command: &str,
) {
    let (x, y) = frame_pixel_for_hit(hit);
    let hovered_count = color_count(hovered, hover_border);
    assert!(
        hovered_count > normal_count,
        "hover must increase KUC hover border pixels: command={command} normal={normal_count} hovered={hovered_count}"
    );
    assert_ne!(
        hover_border,
        normal.pixels()[y * normal.width() + x],
        "normal frame already has hover border at {command}"
    );
    assert_eq!(
        hover_border,
        hovered.pixels()[y * hovered.width() + x],
        "hovered KUC node id must paint the same rect used by host action hit-test: command={command}"
    );
}

fn assert_hover_border_pixel_at(
    normal: &Canvas,
    hovered: &Canvas,
    x: usize,
    y: usize,
    hover_border: u32,
    normal_count: usize,
    command: &str,
) {
    let hovered_count = color_count(hovered, hover_border);
    assert!(
        hovered_count > normal_count,
        "hover must increase KUC hover border pixels: command={command} normal={normal_count} hovered={hovered_count}"
    );
    assert_ne!(
        hover_border,
        normal.pixels()[y * normal.width() + x],
        "normal frame already has hover border at {command}"
    );
    assert_eq!(
        hover_border,
        hovered.pixels()[y * hovered.width() + x],
        "hovered KUC internal control node id must paint the same rect used by node hit-test: command={command}"
    );
}

fn color_count(canvas: &Canvas, color: u32) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel == color)
        .count()
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
                node_id: hit.node_id,
                center_x: super::support::FRAME_PREVIEW_LEFT
                    + hit.rect.x.saturating_add(hit.rect.width.saturating_div(2)),
                center_y: super::support::FRAME_PREVIEW_TOP + hit.rect.y,
            })
        })
        .collect()
}

struct InternalControlHit {
    command: String,
    node_id: katana_ui_core::render_model::UiNodeId,
    center_x: usize,
    center_y: usize,
}
