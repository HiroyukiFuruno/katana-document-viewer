use super::support::{first_target_containing, storybook};
use crate::layout::StorybookPreviewArea;
use katana_document_viewer::{
    DiagramControlCommand, KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX,
    VIEWER_DIAGRAM_DISPLAY_MAX_WIDTH, VIEWER_DIAGRAM_DISPLAY_SCALE, ViewerCommand,
    ViewerCommandFactory,
};
use katana_ui_core::render_model::{UiImageSurfaceRenderPlan, UiNode, UiNodeKind};

#[test]
fn diagram_command_updates_viewport_state() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_diagrams.md");
    storybook.update_scene(1000, 900)?;
    let target = first_target_containing(&storybook, "graph TD")?;
    let command = ViewerCommandFactory::diagram_control_from_action(target, "zoom-in", false)
        .ok_or_else(|| std::io::Error::other("missing diagram command"))?;

    assert!(storybook.apply_viewer_command(&command));

    let ViewerCommand::Diagram(DiagramControlCommand::Zoom(zoom)) = command else {
        return Err(std::io::Error::other("expected zoom command").into());
    };
    let state = storybook
        .diagram_viewports
        .get(zoom.target.node_id.0.as_str())
        .ok_or_else(|| std::io::Error::other("missing diagram state"))?;
    assert!(state.zoom > 1.0);
    Ok(())
}

#[test]
fn diagram_command_refreshes_loaded_scene_without_asset_job()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_diagrams.md");
    storybook.update_scene_loaded(1000, 900)?;
    let loaded_count = storybook
        .scene
        .as_ref()
        .ok_or("scene missing")?
        .loaded_asset_count;
    assert!(loaded_count > 0);
    let target = first_target_containing(&storybook, "graph TD")?;
    let command = ViewerCommandFactory::diagram_control_from_action(target, "zoom-in", false)
        .ok_or_else(|| std::io::Error::other("missing diagram command"))?;

    assert!(storybook.apply_viewer_command(&command));
    storybook.update_scene_for_refresh(1000, 900)?;

    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    assert!(storybook.asset_job.is_none());
    assert_eq!(loaded_count, scene.loaded_asset_count);
    assert!(scene.image_surface_count > 0);
    Ok(())
}

#[test]
fn diagram_initial_target_stays_inside_preview_width_without_frame_blowup()
-> Result<(), Box<dyn std::error::Error>> {
    let width = 2048;
    let height = 1536;
    let mut storybook = storybook("katana/sample_diagrams.md");
    storybook.update_scene_loaded(width, height)?;
    let target = first_target_containing(&storybook, "graph TD")?;
    let preview_width = StorybookPreviewArea::for_window(width, height, 0.0).width as f32;
    let plans = UiImageSurfaceRenderPlan::collect_from_tree(
        &storybook.scene.as_ref().ok_or("scene missing")?.tree,
    );
    let max_physical_image_width = plans.iter().map(|plan| plan.width).max().unwrap_or(0);

    assert!(
        target.rect.width <= preview_width,
        "diagram viewer target must not exceed the visible preview width: preview_width={preview_width} target={:?} image_surfaces={} max_physical_image_width={max_physical_image_width}",
        target.rect,
        plans.len(),
    );
    Ok(())
}

#[test]
fn diagram_initial_image_surface_keeps_katana_size_without_zoom_or_upscale()
-> Result<(), Box<dyn std::error::Error>> {
    let width = 2048;
    let height = 1536;
    let mut storybook = storybook("katana/sample_diagrams.md");
    storybook.update_scene_loaded(width, height)?;
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let preview_width = StorybookPreviewArea::for_window(width, height, 0.0).width as u32;

    for image in image_surface_nodes(scene.tree.root()) {
        let props = &image.props().image_surface;
        let display_width = props.display_width_milli.div_ceil(1000);
        let intrinsic_width = if props.content_scale == 0 {
            props.width
        } else {
            (u64::from(props.width) * 100).div_ceil(u64::from(props.content_scale)) as u32
        };

        assert!(
            display_width <= preview_width,
            "diagram image display width must be capped by KatanA preview width: display_width={display_width} preview_width={preview_width}"
        );
        assert!(
            display_width <= intrinsic_width,
            "diagram image must not upscale beyond intrinsic KatanA SVG size: display_width={display_width} intrinsic_width={intrinsic_width}"
        );
        assert_eq!(
            100, props.transform.zoom_percent,
            "initial diagram image must use KatanA default zoom"
        );
        assert_eq!(0, props.transform.pan_x);
        assert_eq!(0, props.transform.pan_y);
    }
    Ok(())
}

#[test]
fn wide_window_diagram_image_surface_keeps_fixed_scale_after_katana_fit_width()
-> Result<(), Box<dyn std::error::Error>> {
    let width = 2048;
    let height = 1536;
    let mut storybook = storybook("katana/sample_diagrams.md");
    storybook.update_scene_loaded(width, height)?;
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let preview_width = StorybookPreviewArea::for_window(width, height, 0.0).width as u32;
    let content_width = preview_width.saturating_sub(u32::from(
        KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX,
    ));
    assert!(preview_width > 1200, "test must exercise a wide preview");

    let mut checked = 0usize;
    let mut found_wide = false;
    for image in image_surface_nodes(scene.tree.root()) {
        let props = &image.props().image_surface;
        let display_width = props.display_width_milli.div_ceil(1000);
        let intrinsic_width = if props.content_scale == 0 {
            props.width
        } else {
            (u64::from(props.width) * 100).div_ceil(u64::from(props.content_scale)) as u32
        };
        if display_width <= 1 {
            continue;
        }
        checked += 1;
        let fixed_scaled_width =
            ((intrinsic_width as f32 * VIEWER_DIAGRAM_DISPLAY_SCALE).ceil() as u32).max(1);
        let expected_width = fixed_scaled_width
            .min(content_width)
            .min(VIEWER_DIAGRAM_DISPLAY_MAX_WIDTH);
        let min_scaled_width = expected_width.saturating_sub(2);
        let max_scaled_width = expected_width.saturating_add(2);
        assert!(
            (min_scaled_width..=max_scaled_width).contains(&display_width),
            "KatanA fixed SVG display scale must be kept after fitting to content width: display_width={display_width} expected={expected_width} content_width={content_width} preview_width={preview_width} intrinsic_width={intrinsic_width}"
        );
        found_wide |= display_width > 640;
    }
    assert!(checked > 0, "expected loaded diagram image surfaces");
    assert!(
        found_wide,
        "expected at least one KatanA-width diagram over 640px"
    );
    Ok(())
}

fn image_surface_nodes(node: &UiNode) -> Vec<&UiNode> {
    let mut nodes = Vec::new();
    collect_image_surface_nodes(node, &mut nodes);
    nodes
}

fn collect_image_surface_nodes<'a>(node: &'a UiNode, nodes: &mut Vec<&'a UiNode>) {
    if node.kind() == UiNodeKind::ImageSurface {
        nodes.push(node);
    }
    for child in node.children() {
        collect_image_surface_nodes(child, nodes);
    }
}
