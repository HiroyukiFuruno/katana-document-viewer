use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::layout::{KATANA_ACTIVITY_RAIL_WIDTH, SIDEBAR_CONTENT_INSET};
use katana_document_viewer::ViewerInteractionConfig;
use std::path::PathBuf;

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 900;
const SIDEBAR_TEXT: u32 = 0xd4d4d4;
const SIDEBAR_CONTROL_BORDER: u32 = 0x3c3c3c;
const SIDEBAR_CONTENT_X: usize = KATANA_ACTIVITY_RAIL_WIDTH + SIDEBAR_CONTENT_INSET;
const DIRECTORY_ICON_X: usize = SIDEBAR_CONTENT_X + 24;
const DEEP_FILE_ICON_X: usize = SIDEBAR_CONTENT_X + 56;
const TREE_ROW_START_Y: usize = SIDEBAR_CONTENT_INSET + 22;

#[test]
fn sidebar_frame_file_tree_draws_kuc_icons_and_indent_guides() {
    let fixtures = [fixture("katana/drawio/basic/05-edge-variants.drawio")];
    let canvas = StorybookFrameRenderer::render(FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: &fixtures,
        selected_index: 0,
        scene: None,
        scroll_y: 0.0,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &Default::default(),
        dark: true,
        interaction: &ViewerInteractionConfig::default(),
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 0,
    });

    let directory_icon_pixels = rect_pixel_count(
        &canvas,
        SIDEBAR_TEXT,
        DIRECTORY_ICON_X,
        TREE_ROW_START_Y,
        24,
        88,
    );
    let file_icon_pixels = rect_pixel_count(
        &canvas,
        SIDEBAR_TEXT,
        DEEP_FILE_ICON_X,
        TREE_ROW_START_Y + 40,
        30,
        96,
    );
    let indent_guide_pixels = rect_pixel_count(
        &canvas,
        SIDEBAR_CONTROL_BORDER,
        SIDEBAR_CONTENT_X + 12,
        TREE_ROW_START_Y,
        40,
        112,
    );

    assert!(
        directory_icon_pixels > 16,
        "KUC FileTree must render folder/disclosure icon pixels in the sidebar: {directory_icon_pixels}"
    );
    assert!(
        file_icon_pixels > 8,
        "KUC FileTree must render file icon pixels before labels: {file_icon_pixels}"
    );
    assert!(
        indent_guide_pixels > 16,
        "KUC FileTree must render TreeView indent guide pixels: {indent_guide_pixels}"
    );
}

fn rect_pixel_count(
    canvas: &Canvas,
    color: u32,
    x_start: usize,
    y_start: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut count = 0;
    for y in y_start..y_start.saturating_add(height).min(canvas.height()) {
        for x in x_start..x_start.saturating_add(width).min(canvas.width()) {
            if canvas.pixels()[y * canvas.width() + x] == color {
                count += 1;
            }
        }
    }
    count
}

fn fixture(path: &str) -> StorybookFixture {
    StorybookFixture {
        label: path.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{path}")),
    }
}
