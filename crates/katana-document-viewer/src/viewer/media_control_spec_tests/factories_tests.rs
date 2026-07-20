use crate::viewer::{
    ViewerDiagramControlSlot, ViewerMediaControlKind, ViewerMediaControlSpec,
    media_control_spec::ViewerMediaControlSet,
};
use std::hint::black_box;

#[test]
fn unknown_media_control_icon_falls_back_to_default_circle() {
    assert!(ViewerMediaControlSet::surface_control_svg("not-listed").contains("r=\"4\""));
}

#[test]
fn runtime_control_factories_preserve_slots_and_icons() {
    assert_runtime_factories_preserve_spec_kind();
    assert_runtime_control_slot_variants();
    assert_runtime_control_icons_for_known_commands();
}

fn assert_runtime_factories_preserve_spec_kind() {
    let image_factory: fn(&'static str, &'static str) -> ViewerMediaControlSpec =
        ViewerMediaControlSet::image_control;
    let code_factory: fn(&'static str, &'static str) -> ViewerMediaControlSpec =
        ViewerMediaControlSet::code_control;
    let diagram_factory: fn(&'static str, &'static str) -> ViewerMediaControlSpec =
        ViewerMediaControlSet::diagram_control;

    let image = image_factory(black_box("copy"), black_box("copy"));
    let code = code_factory(black_box("copy-code"), black_box("copy"));
    let diagram = diagram_factory(black_box("pan-up"), black_box("up"));
    assert_eq!(ViewerMediaControlKind::Image, image.kind);
    assert_eq!(ViewerMediaControlKind::Code, code.kind);
    assert_eq!(ViewerMediaControlKind::Diagram, diagram.kind);

    let image_slot_factory: fn(&'static str, &'static str) -> ViewerDiagramControlSlot =
        ViewerMediaControlSet::image_slot;
    let diagram_slot_factory: fn(&'static str, &'static str) -> ViewerDiagramControlSlot =
        ViewerMediaControlSet::diagram_slot;

    assert!(matches!(
        image_slot_factory(black_box("open"), black_box("open")),
        ViewerDiagramControlSlot::Control(spec) if spec.kind == ViewerMediaControlKind::Image
    ));
    assert!(matches!(
        diagram_slot_factory(black_box("pan-down"), black_box("down")),
        ViewerDiagramControlSlot::Control(spec) if spec.kind == ViewerMediaControlKind::Diagram
    ));
}

fn assert_runtime_control_slot_variants() {
    assert!(matches!(
        ViewerMediaControlSet::grid_gap(),
        ViewerDiagramControlSlot::Gap { .. }
    ));
    assert!(matches!(
        ViewerMediaControlSet::image_gap(),
        ViewerDiagramControlSlot::Gap { .. }
    ));
    assert!(matches!(
        ViewerMediaControlSet::spacer(),
        ViewerDiagramControlSlot::Spacer { .. }
    ));
    assert!(matches!(
        ViewerMediaControlSet::image_spacer(),
        ViewerDiagramControlSlot::Spacer { .. }
    ));
}

fn assert_runtime_control_icons_for_known_commands() {
    for command in [
        "copy",
        "fit",
        "open",
        "pan-up",
        "pan-down",
        "pan-left",
        "pan-right",
        "zoom-in",
        "zoom-out",
        "reset-view",
        "trackpad-help",
        "not-listed",
    ] {
        assert!(ViewerMediaControlSet::surface_control_svg(black_box(command)).starts_with("<svg"));
    }
}
