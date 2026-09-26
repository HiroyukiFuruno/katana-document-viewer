use super::support::{MediaControlFrameSupport, color, frame_rect_for_hit};
use crate::KucDiagramControlResolver;
use crate::canvas::Canvas;
use crate::layout::preview_content_width;
use crate::preview_theme_bridge::KucThemeBridge;
use katana_ui_core_storybook::UiTreeRenderArea;

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
        assert_hover_border_within_hit(
            &normal,
            &hovered,
            frame_rect_for_hit(&action_hit.hit),
            hover_border,
            normal_count,
            action_hit.command.as_str(),
        )?;
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
        assert_hover_border_within_hit(
            &normal,
            &hovered,
            (
                super::support::FRAME_PREVIEW_LEFT + action_hit.host_x,
                super::support::FRAME_PREVIEW_TOP + action_hit.host_y,
                super::support::FRAME_PREVIEW_LEFT + action_hit.host_x + action_hit.host_width,
                super::support::FRAME_PREVIEW_TOP + action_hit.host_y + action_hit.host_height,
            ),
            hover_border,
            normal_count,
            action_hit.command.as_str(),
        )?;
    }
    Ok(())
}

fn assert_hover_border_within_hit(
    normal: &Canvas,
    hovered: &Canvas,
    hit: (usize, usize, usize, usize),
    hover_border: u32,
    normal_count: usize,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hovered_count = color_count(hovered, hover_border);
    assert!(
        hovered_count > normal_count,
        "hover must increase KUC hover border pixels: command={command} normal={normal_count} hovered={hovered_count}"
    );
    let Some((left, top, right, bottom)) = changed_pixel_bounds(normal, hovered) else {
        return Err(format!("hovered KUC node id must change pixels: command={command}").into());
    };
    assert!(
        left >= hit.0 && top >= hit.1 && right < hit.2 && bottom < hit.3,
        "hovered KUC node id must only change pixels inside its host hit: command={command} diff=({left}, {top}, {right}, {bottom}) hit={hit:?}"
    );
    Ok(())
}

fn color_count(canvas: &Canvas, color: u32) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel == color)
        .count()
}

fn changed_pixel_bounds(normal: &Canvas, hovered: &Canvas) -> Option<(usize, usize, usize, usize)> {
    let mut bounds: Option<(usize, usize, usize, usize)> = None;
    for (index, (normal_pixel, hovered_pixel)) in
        normal.pixels().iter().zip(hovered.pixels()).enumerate()
    {
        if normal_pixel == hovered_pixel {
            continue;
        }
        let x = index % normal.width();
        let y = index / normal.width();
        bounds = Some(match bounds {
            Some((left, top, right, bottom)) => {
                (left.min(x), top.min(y), right.max(x), bottom.max(y))
            }
            None => (x, y, x, y),
        });
    }
    bounds
}

fn internal_diagram_control_hits(scene: &crate::preview::PreviewScene) -> Vec<InternalControlHit> {
    KucThemeBridge::document_host(scene.theme.clone(), scene.typography)
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
                host_x: hit.rect.x,
                host_y: hit.rect.y,
                host_width: hit.rect.width,
                host_height: hit.rect.height,
            })
        })
        .collect()
}

struct InternalControlHit {
    command: String,
    node_id: katana_ui_core::render_model::UiNodeId,
    host_x: usize,
    host_y: usize,
    host_width: usize,
    host_height: usize,
}
