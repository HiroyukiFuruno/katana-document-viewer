use crate::layout::preview_content_width;
use crate::mouse::StorybookPointer;
use crate::mouse::mouse_test_support::{WINDOW_HEIGHT, WINDOW_WIDTH, pointer_for_task};
use katana_document_viewer::ViewerTaskState;
use katana_ui_core::render_model::{UiCommonProps, UiNode};
use katana_ui_core_storybook::{
    Canvas, UiTreeHostActionHit, UiTreeRenderArea, UiTreeStorybookHost,
};

use super::interaction_matrix_support::storybook_with_label;

#[test]
fn task_click_renders_same_checkbox_crop_as_kuc_component() -> Result<(), Box<dyn std::error::Error>>
{
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let initial_scene = storybook
        .scene
        .as_ref()
        .ok_or("initial scene missing")?
        .clone();
    let task = pointer_for_task(&initial_scene, "[ ]")?;
    storybook.scroll_y = task.scroll_y;

    assert!(storybook.apply_canvas_click(task.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    let state_id = only_task_override(&storybook, ViewerTaskState::Done)?;
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let theme = scene.theme.clone();
    let checkbox = node_for_state(scene.tree.root(), state_id.as_str())
        .ok_or("updated checkbox node missing")?
        .clone();
    assert_eq!("[x]", checkbox.props().interaction.value);
    let task_row = row_for_state(scene.tree.root(), state_id.as_str())
        .ok_or("updated task row node missing")?
        .clone()
        .common(UiCommonProps::default());
    let hit = task_hit_for_state(scene, state_id.as_str()).ok_or("updated task hit missing")?;
    let actual = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let (crop_x, crop_y) = checkbox_crop_origin(&hit, task.pointer);
    let expected = render_kuc_task_row_crop(theme, &task_row, &actual, crop_x, crop_y, &hit)?;
    let expected_non_background = non_background_count(&expected)?;
    let overlap = component_overlap_count(&actual, &expected, crop_x, crop_y)?;
    assert!(
        overlap >= expected_non_background / 2,
        "task row must render through KUC Row + Checkbox components: overlap={overlap} expected_non_background={expected_non_background}"
    );
    Ok(())
}

fn checkbox_crop_origin(hit: &UiTreeHostActionHit, pointer: StorybookPointer) -> (usize, usize) {
    (
        (pointer.x - 4.0).round().max(0.0) as usize,
        (pointer.y - hit.rect.height as f32 / 2.0).round().max(0.0) as usize,
    )
}

fn render_kuc_task_row_crop(
    theme: katana_ui_core::theme::ThemeSnapshot,
    task_row: &UiNode,
    actual: &Canvas,
    crop_x: usize,
    crop_y: usize,
    hit: &UiTreeHostActionHit,
) -> Result<Canvas, Box<dyn std::error::Error>> {
    let background = pixel_at(
        actual,
        crop_x.saturating_add(hit.rect.width.saturating_sub(1)),
        crop_y,
    )?;
    let mut expected = Canvas::new(hit.rect.width, hit.rect.height, background);
    UiTreeStorybookHost::new(theme).render(
        &mut expected,
        task_row,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: hit.rect.width,
            height: hit.rect.height,
            scroll_y: 0.0,
        },
    );
    Ok(expected)
}

fn component_overlap_count(
    actual: &Canvas,
    expected: &Canvas,
    x: usize,
    y: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let actual_background = pixel_at(
        actual,
        x.saturating_add(expected.width().saturating_sub(1)),
        y,
    )?;
    let expected_background = pixel_at(expected, expected.width().saturating_sub(1), 0)?;
    let mut count = 0;
    for current_y in 0..expected.height() {
        for current_x in 0..expected.width() {
            let actual_index = (y + current_y) * actual.width() + x + current_x;
            let expected_index = current_y * expected.width() + current_x;
            if expected.pixels()[expected_index] != expected_background
                && actual.pixels()[actual_index] != actual_background
            {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn non_background_count(canvas: &Canvas) -> Result<usize, Box<dyn std::error::Error>> {
    let background = pixel_at(canvas, canvas.width().saturating_sub(1), 0)?;
    Ok(canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel != background)
        .count())
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Result<u32, Box<dyn std::error::Error>> {
    canvas
        .pixels()
        .get(y.saturating_mul(canvas.width()).saturating_add(x))
        .copied()
        .ok_or_else(|| "pixel outside canvas".into())
}

fn node_for_state<'a>(node: &'a UiNode, state_id: &str) -> Option<&'a UiNode> {
    if node.props().state_id.as_str() == state_id {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| node_for_state(child, state_id))
}

fn row_for_state<'a>(node: &'a UiNode, state_id: &str) -> Option<&'a UiNode> {
    if node
        .children()
        .iter()
        .any(|child| child.props().state_id.as_str() == state_id)
    {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| row_for_state(child, state_id))
}

fn task_hit_for_state(
    scene: &crate::preview::PreviewScene,
    state_id: &str,
) -> Option<UiTreeHostActionHit> {
    UiTreeStorybookHost::new(scene.theme.clone())
        .document_host_action_hits(
            scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_content_width(WINDOW_WIDTH),
                height: scene.content_height.ceil().max(1.0) as usize,
                scroll_y: 0.0,
            },
        )
        .into_iter()
        .find(|hit| {
            hit.action
                .task_control_action_from_root(scene.tree.root())
                .is_some_and(|action| action.state_id == state_id)
        })
}

fn only_task_override(
    storybook: &super::StorybookWindow,
    expected: ViewerTaskState,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut overrides = storybook.task_state_overrides.iter();
    let Some((state_id, state)) = overrides.next() else {
        return Err("task override missing".into());
    };
    if overrides.next().is_some() {
        return Err("multiple task overrides".into());
    }
    assert_eq!(expected, *state);
    Ok(state_id.clone())
}
