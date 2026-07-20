use crate::viewer::{
    ViewerDiagramControlSlot, ViewerMediaControlAction, ViewerMediaControlKind,
    ViewerMediaControlSet,
};

#[test]
fn media_control_set_owns_diagram_control_contract() -> Result<(), String> {
    let rows = ViewerMediaControlSet::diagram_grid_rows();

    assert_diagram_control(rows[0][2], "pan-up")?;
    assert_diagram_control(rows[0][4], "zoom-in")?;
    assert_diagram_control(rows[1][0], "pan-left")?;
    assert_diagram_control(rows[1][2], "reset-view")?;
    assert_diagram_control(rows[1][4], "pan-right")?;
    assert_diagram_control(rows[2][0], "trackpad-help")?;
    assert_diagram_control(rows[2][2], "pan-down")?;
    assert_diagram_control(rows[2][4], "zoom-out")?;
    Ok(())
}

#[test]
fn diagram_top_controls_use_katana_host_actions() -> Result<(), String> {
    let slots = ViewerMediaControlSet::diagram_top_slots();

    assert_eq!(1, slots.len());
    assert_diagram_control(slots[0], "fullscreen")?;
    Ok(())
}

#[test]
fn media_control_set_owns_image_control_contract() {
    let commands = ViewerMediaControlSet::image_controls()
        .iter()
        .map(|spec| spec.command)
        .collect::<Vec<_>>();

    assert_eq!(
        vec!["fit", "open", "copy", "reveal-in-os", "zoom-in", "zoom-out"],
        commands
    );
    for spec in ViewerMediaControlSet::image_controls() {
        assert_eq!(ViewerMediaControlKind::Image, spec.kind);
        assert_eq!(28, spec.width_px);
        assert_eq!(28, spec.height_px);
        assert!(
            spec.icon_svg.contains("<svg"),
            "KDV core must own image control icon SVG: {}",
            spec.command
        );
    }
}

#[test]
fn media_control_specs_create_host_action_ids() {
    let copy = ViewerMediaControlSet::code_copy_control();

    assert_eq!(ViewerMediaControlKind::Code, copy.kind);
    assert_eq!("copy-code", copy.command);
    assert_eq!(28, copy.width_px);
    assert_eq!(28, copy.height_px);
    assert!(copy.icon_svg.contains("<svg"));
    assert_eq!(
        "viewer.code.copy-code",
        ViewerMediaControlAction::host_action_id_for(copy.kind, copy.command)
    );
}

fn assert_diagram_control(slot: ViewerDiagramControlSlot, command: &str) -> Result<(), String> {
    let ViewerDiagramControlSlot::Control(spec) = slot else {
        return Err("expected diagram control slot".to_string());
    };
    assert_eq!(ViewerMediaControlKind::Diagram, spec.kind);
    assert_eq!(command, spec.command);
    assert_eq!(28, spec.width_px);
    assert_eq!(28, spec.height_px);
    assert!(
        spec.icon_svg.contains("<svg"),
        "KDV core must own diagram control icon SVG: {command}"
    );
    assert_eq!(
        format!("viewer.diagram.{command}"),
        ViewerMediaControlAction::host_action_id_for(spec.kind, spec.command)
    );
    Ok(())
}
