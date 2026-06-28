use crate::mouse::mouse_test_support::{
    WINDOW_HEIGHT, WINDOW_WIDTH, pointer_for_accordion, sample_basic_scene,
};
use katana_ui_core::render_model::{UiNode, UiNodeKind};

use super::interaction_matrix_support::storybook_with_label;

#[test]
fn storybook_window_closed_accordion_click_opens_body_pixels()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let accordion = pointer_for_accordion(&scene)?;
    let accordion_node_id =
        first_accordion_node_id(scene.tree.root()).ok_or("accordion node missing")?;
    assert_eq!(
        Some(false),
        accordion_open_state(scene.tree.root(), accordion_node_id.as_str()),
        "sample_basic details must start closed"
    );

    storybook.scene = Some(scene);
    storybook.scroll_y = accordion.scroll_y;
    let before = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert!(storybook.apply_canvas_click(accordion.pointer, WINDOW_WIDTH, WINDOW_HEIGHT,)?);
    assert_eq!("accordion", storybook.last_command_label);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let after_scene = storybook.scene.as_ref().ok_or("scene missing")?;
    assert_eq!(
        Some(true),
        accordion_open_state(after_scene.tree.root(), accordion_node_id.as_str()),
        "accordion click must rebuild the KUC scene into open state"
    );

    let after = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let body_region_x = accordion.pointer.x.round() as usize;
    let body_region_y = accordion.pointer.y.round() as usize + 16;
    let body_region_width = WINDOW_WIDTH.saturating_sub(body_region_x + 24);
    let body_region_height = WINDOW_HEIGHT.saturating_sub(body_region_y);
    assert!(
        pixel_diff_count_in_rect(
            &before,
            &after,
            body_region_x,
            body_region_y,
            body_region_width,
            body_region_height,
        ) > 64,
        "accordion open must change pixels in the body area below the header"
    );
    assert!(
        pixel_diff_count(&before, &after) > 64,
        "accordion open must change rendered body pixels"
    );
    Ok(())
}

fn accordion_open_state(node: &UiNode, node_id: &str) -> Option<bool> {
    if node.kind() == UiNodeKind::Accordion && node.id().as_str() == node_id {
        return Some(node.props().interaction.open);
    }
    node.children()
        .iter()
        .find_map(|child| accordion_open_state(child, node_id))
}

fn first_accordion_node_id(node: &UiNode) -> Option<String> {
    if node.kind() == UiNodeKind::Accordion {
        return Some(node.id().as_str().to_string());
    }
    node.children().iter().find_map(first_accordion_node_id)
}

fn pixel_diff_count(left: &crate::canvas::Canvas, right: &crate::canvas::Canvas) -> usize {
    left.pixels()
        .iter()
        .zip(right.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}

fn pixel_diff_count_in_rect(
    left: &crate::canvas::Canvas,
    right: &crate::canvas::Canvas,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> usize {
    let end_x = x.saturating_add(width).min(left.width());
    let end_y = y.saturating_add(height).min(left.height());
    let mut diff = 0usize;
    for y in y..end_y {
        for x in x..end_x {
            diff += usize::from(
                left.pixels()[y * left.width() + x] != right.pixels()[y * right.width() + x],
            );
        }
    }
    diff
}
