use super::support::{DiagramActionHit, FRAME_WIDTH, MediaControlFrameSupport};
use crate::KucDiagramControlResolver;
use crate::layout::preview_content_width;
use katana_ui_core_storybook::{UiTreeHostActionHit, UiTreeRenderArea, UiTreeSurfaceHost};
use std::collections::BTreeMap;

const CONTROL_SIZE: usize = 28;
const CONTROL_GAP: usize = 2;
const GRID_SIZE: usize = CONTROL_SIZE * 3 + CONTROL_GAP * 2;

#[test]
fn diagram_controls_follow_katana_top_and_grid_layout() -> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_scene(true)?;
    let hits = MediaControlFrameSupport::diagram_action_hits(&scene)?;
    let by_command = command_map(hits)?;

    assert_expected_commands(&by_command);
    assert_square_buttons(&by_command);
    assert_internal_grid_matches_katana_controller(&scene)?;
    Ok(())
}

fn assert_expected_commands(by_command: &BTreeMap<String, UiTreeHostActionHit>) {
    assert!(by_command.contains_key("fullscreen"), "missing fullscreen");
    assert_eq!(
        1,
        by_command.len(),
        "KatanA normal diagram top control exposes fullscreen only; pan/zoom/reset stay inside KUC"
    );
}

fn assert_square_buttons(by_command: &BTreeMap<String, UiTreeHostActionHit>) {
    for (command, hit) in by_command {
        assert_eq!(CONTROL_SIZE, hit.rect.width, "{command} width");
        assert_eq!(CONTROL_SIZE, hit.rect.height, "{command} height");
    }
}

fn assert_internal_grid_matches_katana_controller(
    scene: &crate::preview::PreviewScene,
) -> Result<(), Box<dyn std::error::Error>> {
    let hits = UiTreeSurfaceHost::new(scene.theme.clone())
        .document_node_hits(
            scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_content_width(FRAME_WIDTH),
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
            Some((action.command, hit.rect))
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        [
            "pan-down",
            "pan-left",
            "pan-right",
            "pan-up",
            "reset-view",
            "trackpad-help",
            "zoom-in",
            "zoom-out"
        ],
        hits.keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice(),
        "fullscreen internal diagram controls must match KatanA controller commands"
    );

    for (command, rect) in &hits {
        assert_eq!(CONTROL_SIZE, rect.width, "{command} width");
        assert_eq!(CONTROL_SIZE, rect.height, "{command} height");
    }

    let grid_left = hits["pan-left"].x;
    let grid_top = hits["pan-up"].y;
    assert_eq!(grid_left + CONTROL_SIZE + CONTROL_GAP, hits["reset-view"].x);
    assert_eq!(
        grid_left + (CONTROL_SIZE + CONTROL_GAP) * 2,
        hits["pan-right"].x
    );
    assert_eq!(grid_left + CONTROL_SIZE + CONTROL_GAP, hits["pan-up"].x);
    assert_eq!(
        grid_left + (CONTROL_SIZE + CONTROL_GAP) * 2,
        hits["zoom-in"].x
    );
    assert_eq!(grid_left, hits["trackpad-help"].x);
    assert_eq!(grid_left + CONTROL_SIZE + CONTROL_GAP, hits["pan-down"].x);
    assert_eq!(
        grid_left + (CONTROL_SIZE + CONTROL_GAP) * 2,
        hits["zoom-out"].x
    );
    assert_eq!(grid_top + CONTROL_SIZE + CONTROL_GAP, hits["reset-view"].y);
    assert_eq!(
        grid_top + (CONTROL_SIZE + CONTROL_GAP) * 2,
        hits["pan-down"].y
    );
    assert_eq!(
        GRID_SIZE,
        hits["zoom-out"].x + CONTROL_SIZE - grid_left,
        "KatanA controller grid width"
    );
    assert_eq!(
        GRID_SIZE,
        hits["zoom-out"].y + CONTROL_SIZE - grid_top,
        "KatanA controller grid height"
    );
    assert!(
        grid_left + GRID_SIZE <= preview_content_width(FRAME_WIDTH) && grid_top > 0,
        "internal controls should be inside the preview frame"
    );
    Ok(())
}

fn command_map(
    hits: Vec<DiagramActionHit>,
) -> Result<BTreeMap<String, UiTreeHostActionHit>, std::io::Error> {
    let mut by_command = BTreeMap::new();
    for action_hit in hits {
        if by_command
            .insert(action_hit.command.clone(), action_hit.hit)
            .is_some()
        {
            return Err(std::io::Error::other(format!(
                "duplicated diagram command: {}",
                action_hit.command
            )));
        }
    }
    Ok(by_command)
}
