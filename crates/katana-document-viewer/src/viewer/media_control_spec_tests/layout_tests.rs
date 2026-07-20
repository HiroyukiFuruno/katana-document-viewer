use crate::viewer::{
    ViewerDiagramControlSlot, ViewerMediaControlAction, ViewerMediaControlKind,
    ViewerMediaControlSet,
};

#[test]
fn diagram_fullscreen_control_uses_diagram_contract() {
    let control = ViewerMediaControlSet::diagram_fullscreen_control();
    assert_eq!(ViewerMediaControlKind::Diagram, control.kind);
    assert_eq!("fullscreen", control.command);
    assert_eq!("fullscreen", control.accessibility_label);
    assert_eq!((28, 28), (control.width_px, control.height_px));
}

#[test]
fn image_controls_use_overlay_slots() -> Result<(), String> {
    let top_slots = ViewerMediaControlSet::image_top_slots();
    let grid_rows = ViewerMediaControlSet::image_grid_rows();

    assert_image_control(top_slots[0], "open")?;
    assert_image_gap(top_slots[1], 2, 28)?;
    assert_image_control(top_slots[2], "copy")?;
    assert_image_gap(top_slots[3], 2, 28)?;
    assert_image_control(top_slots[4], "reveal-in-os")?;
    assert_image_control(grid_rows[0][2], "zoom-in")?;
    assert_image_control(grid_rows[1][2], "fit")?;
    assert_image_control(grid_rows[2][2], "zoom-out")?;
    Ok(())
}

#[test]
fn media_control_set_uses_expected_slot_layouts_and_ignored_defaults() -> Result<(), String> {
    let diagram_slots = ViewerMediaControlSet::diagram_grid_rows();
    let image_slots = ViewerMediaControlSet::image_grid_rows();
    let image_top_slots = ViewerMediaControlSet::image_top_slots();
    let top_slots = ViewerMediaControlSet::diagram_top_slots();
    let expected_spacer = ViewerDiagramControlSlot::Spacer {
        width_px: 28,
        height_px: 28,
    };

    assert_image_gap(diagram_slots[0][1], 2, 28)?;
    assert_image_gap(image_slots[0][1], 2, 28)?;
    assert_eq!(expected_spacer, image_slots[1][0]);
    assert_eq!(expected_spacer, diagram_slots[0][0]);
    assert_eq!(
        "viewer.diagram.fullscreen",
        ViewerMediaControlAction::host_action_id_for(
            crate::viewer::ViewerMediaControlKind::Diagram,
            "fullscreen"
        )
    );
    assert!(matches!(top_slots[0], ViewerDiagramControlSlot::Control(_)));
    assert!(
        matches!(image_top_slots[1], ViewerDiagramControlSlot::Gap { .. }),
        "image top slot gap branch should remain documented"
    );

    Ok(())
}

fn assert_image_control(slot: ViewerDiagramControlSlot, command: &str) -> Result<(), String> {
    let ViewerDiagramControlSlot::Control(spec) = slot else {
        return Err("expected image control slot".to_string());
    };
    assert_eq!(ViewerMediaControlKind::Image, spec.kind);
    assert_eq!(command, spec.command);
    assert_eq!(28, spec.width_px);
    assert_eq!(28, spec.height_px);
    assert!(
        spec.icon_svg.contains("<svg"),
        "KDV core must own image slot icon SVG: {command}"
    );
    assert_eq!(
        format!("viewer.image.{command}"),
        ViewerMediaControlAction::host_action_id_for(spec.kind, spec.command)
    );
    Ok(())
}

fn assert_image_gap(
    slot: ViewerDiagramControlSlot,
    width_px: u16,
    height_px: u16,
) -> Result<(), String> {
    let ViewerDiagramControlSlot::Gap {
        width_px: actual_width,
        height_px: actual_height,
    } = slot
    else {
        return Err("expected image gap slot".to_string());
    };
    assert_eq!(width_px, actual_width);
    assert_eq!(height_px, actual_height);
    Ok(())
}
