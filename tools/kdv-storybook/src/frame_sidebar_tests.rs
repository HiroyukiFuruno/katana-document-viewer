use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::layout::{
    SIDEBAR_CONTENT_INSET, SIDEBAR_WIDTH, sidebar_content_width, sidebar_content_x,
};
use katana_document_viewer::ViewerInteractionConfig;
use katana_ui_core_storybook::StorybookPresentation;
use std::path::PathBuf;

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 900;
const SIDEBAR_SPLIT_Y: usize = 430;
const SIDEBAR_SELECTION: u32 = 0x264f78;
const SIDEBAR_TEXT: u32 = 0xd4d4d4;
const SIDEBAR_CONTROL_BORDER: u32 = 0x3c3c3c;

#[test]
fn sidebar_frame_keeps_file_tree_above_settings() {
    let fixtures = [
        fixture("direct/sample.md"),
        fixture("katana/html-alignment.htm"),
    ];
    let canvas = StorybookFrameRenderer::render(FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: &fixtures,
        selected_index: 1,
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

    assert!(pixel_count(&canvas, SIDEBAR_SELECTION, 0, SIDEBAR_SPLIT_Y) > 64);
    assert!(
        pixel_count(
            &canvas,
            SIDEBAR_CONTROL_BORDER,
            SIDEBAR_SPLIT_Y,
            FRAME_HEIGHT
        ) > 64
    );
    assert!(pixel_count(&canvas, SIDEBAR_TEXT, SIDEBAR_SPLIT_Y, FRAME_HEIGHT) > 64);
}

#[test]
fn sidebar_frame_file_tree_selection_uses_kuc_full_row_background() {
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

    let expected = sidebar_content_width().saturating_sub(8);
    let selected_row_pixels = max_row_pixel_count(
        &canvas,
        SIDEBAR_SELECTION,
        SIDEBAR_CONTENT_INSET,
        SIDEBAR_SPLIT_Y,
    );

    assert!(
        selected_row_pixels >= expected,
        "selected FileTree row must be filled across KUC sidebar content: {selected_row_pixels}"
    );
}

#[test]
fn scaled_sidebar_file_tree_selection_uses_physical_sidebar_width() {
    let fixtures = [fixture("katana/drawio/basic/05-edge-variants.drawio")];
    let canvas = StorybookFrameRenderer::render_scaled(
        FrameRenderRequest {
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
        },
        2.0,
    );

    let expected = (sidebar_content_width().saturating_sub(8)) * 2;
    let selected_row_pixels = max_physical_sidebar_row_pixel_count(
        &canvas,
        SIDEBAR_SELECTION,
        SIDEBAR_CONTENT_INSET * 2,
        SIDEBAR_SPLIT_Y * 2,
    );

    assert!(
        selected_row_pixels >= expected,
        "scaled FileTree row must be filled across the physical sidebar: {selected_row_pixels}"
    );
}

#[test]
fn presented_scaled_sidebar_file_tree_selection_keeps_logical_row_width() {
    let fixtures = [fixture("katana/drawio/basic/05-edge-variants.drawio")];
    let scaled = StorybookFrameRenderer::render_scaled(
        FrameRenderRequest {
            width: 2048,
            height: 1496,
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
        },
        2.0,
    );
    let presented = StorybookPresentation::present_frame_for_window(&scaled, 2048, 1496, 0);

    let expected = sidebar_content_width().saturating_sub(8);
    let selected_row_pixels = max_row_pixel_count(
        &presented,
        SIDEBAR_SELECTION,
        SIDEBAR_CONTENT_INSET,
        SIDEBAR_SPLIT_Y,
    );

    assert!(
        selected_row_pixels >= expected,
        "presented scaled FileTree row must not collapse to a half-width sidebar: {selected_row_pixels}"
    );
}

fn pixel_count(canvas: &Canvas, color: u32, y_start: usize, y_end: usize) -> usize {
    let mut count = 0;
    for y in y_start..y_end.min(canvas.height()) {
        count += row_pixel_count(canvas, color, y);
    }
    count
}

fn row_pixel_count(canvas: &Canvas, color: u32, y: usize) -> usize {
    let mut count = 0;
    for x in sidebar_content_x()..SIDEBAR_WIDTH.min(canvas.width()) {
        if canvas.pixels()[y * canvas.width() + x] == color {
            count += 1;
        }
    }
    count
}

fn max_row_pixel_count(canvas: &Canvas, color: u32, y_start: usize, y_end: usize) -> usize {
    let mut max_count = 0;
    for y in y_start..y_end.min(canvas.height()) {
        max_count = max_count.max(row_pixel_count(canvas, color, y));
    }
    max_count
}

fn max_physical_sidebar_row_pixel_count(
    canvas: &Canvas,
    color: u32,
    y_start: usize,
    y_end: usize,
) -> usize {
    let mut max_count = 0;
    let physical_sidebar_width = SIDEBAR_WIDTH * canvas.scale_factor() as usize;
    let physical_content_x = sidebar_content_x() * canvas.scale_factor() as usize;
    for y in y_start..y_end.min(canvas.height()) {
        let mut count = 0;
        for x in physical_content_x..physical_sidebar_width.min(canvas.width()) {
            if canvas.pixels()[y * canvas.width() + x] == color {
                count += 1;
            }
        }
        max_count = max_count.max(count);
    }
    max_count
}

fn fixture(path: &str) -> StorybookFixture {
    StorybookFixture {
        label: path.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{path}")),
    }
}
