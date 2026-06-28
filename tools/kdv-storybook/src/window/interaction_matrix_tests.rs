use crate::layout::{
    HEADER_HEIGHT, SIDEBAR_CONTENT_INSET, SIDEBAR_WIDTH, StorybookPreviewArea,
    preview_content_width, preview_viewport_height, sidebar_content_x,
};
use crate::mouse::mouse_test_support::{
    PointerHit, WINDOW_HEIGHT, WINDOW_WIDTH, pointer_for_accordion,
    pointer_for_internal_diagram_action, pointer_for_link, pointer_for_media_action,
    pointer_for_task,
};
use crate::mouse::{
    DocumentPoint, StorybookHostActionHits, StorybookHostActionRouter, StorybookMouse,
    StorybookMouseButton, StorybookPointer,
};
use crate::preview::PreviewScene;
use crate::preview_interaction_command_support::build_scene;
use crate::settings_action::StorybookSettingsField;
use katana_document_viewer::{
    DiagramViewportState, HostCommand, ViewerCommand, ViewerInteractionConfig, ViewerMode,
    ViewerTaskState, ViewerVector,
};
use katana_ui_core::molecule::SettingsListAction;
use katana_ui_core::render_model::{UiCursor, UiNode, UiNodeId, UiNodeKind, UiTextSpanAction};
use std::collections::BTreeMap;
use std::sync::OnceLock;

use super::interaction_matrix_support::{
    file_tree_item_point, first_directory_hit, settings_field_point, settings_field_target,
    settings_section_point, settings_section_target, storybook_with_catalog, storybook_with_label,
    storybook_with_minimal_catalog,
};

const SIDEBAR_TREE_HOVER_BACKGROUND: u32 = 0x243041;
const KUC_DARK_HOVER_BORDER: u32 = 0x569cd6;
static SAMPLE_BASIC_SCENE: OnceLock<Result<PreviewScene, String>> = OnceLock::new();
static SAMPLE_SCENE: OnceLock<Result<PreviewScene, String>> = OnceLock::new();
static DIRECT_HTML_ALIGNMENT_SCENE: OnceLock<Result<PreviewScene, String>> = OnceLock::new();
static SAMPLE_DIAGRAM_CONTROLS_SCENE: OnceLock<Result<PreviewScene, String>> = OnceLock::new();

fn sample_basic_scene() -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cached_scene(
        &SAMPLE_BASIC_SCENE,
        "katana/sample_basic.md",
        ViewerInteractionConfig::default(),
    )
}

fn sample_scene() -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cached_scene(
        &SAMPLE_SCENE,
        "katana/sample.md",
        ViewerInteractionConfig::default(),
    )
}

fn direct_html_alignment_scene() -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cached_scene(
        &DIRECT_HTML_ALIGNMENT_SCENE,
        "direct/html-alignment.html",
        ViewerInteractionConfig::default(),
    )
}

fn sample_diagram_controls_scene() -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cached_scene(
        &SAMPLE_DIAGRAM_CONTROLS_SCENE,
        "katana/sample_diagrams.md",
        ViewerInteractionConfig {
            diagram_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )
}

fn cached_scene(
    cache: &'static OnceLock<Result<PreviewScene, String>>,
    path: &'static str,
    interaction: ViewerInteractionConfig,
) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    cache
        .get_or_init(|| build_scene(path, interaction).map_err(|error| error.to_string()))
        .clone()
        .map_err(|error| std::io::Error::other(error).into())
}

#[test]
#[ignore = "covered by per-control window interaction gates; keep as manual aggregate audit"]
fn storybook_window_routes_visible_control_matrix() -> Result<(), Box<dyn std::error::Error>> {
    assert_tree_view_select_and_toggle()?;
    assert_settings_toggle_and_select()?;
    assert_document_link_task_and_accordion()?;
    assert_diagram_control_click_and_hover()?;
    Ok(())
}

#[test]
fn storybook_window_file_tree_hover_draws_kuc_row_background()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_catalog()?;
    let file_id = storybook.catalog.fixtures[1].label.clone();
    let (file_x, file_y) = file_tree_item_point(&storybook, &file_id)?;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_sidebar_tree_hover_for_canvas_point(
        file_x,
        file_y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    ));
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    let normal_count = color_count(&normal, SIDEBAR_TREE_HOVER_BACKGROUND);
    let hovered_count = color_count(&hovered, SIDEBAR_TREE_HOVER_BACKGROUND);
    assert!(
        hovered_count > normal_count,
        "window file tree hover must paint KUC row hover background: normal={normal_count} hovered={hovered_count}"
    );
    Ok(())
}

#[test]
fn storybook_window_settings_toggle_hover_draws_kuc_preset_border()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_minimal_catalog();
    let (dark_x, dark_y) = settings_field_point(&storybook, StorybookSettingsField::Dark)?;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(
        storybook.asset_job.is_none(),
        "settings hover render must not start preview asset work"
    );
    assert!(storybook.update_sidebar_settings_hover_for_canvas_point(
        dark_x,
        dark_y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(
        storybook.asset_job.is_none(),
        "settings hover repaint must stay inside the KUC sidebar layer"
    );

    let normal_count = color_count(&normal, KUC_DARK_HOVER_BORDER);
    let hovered_count = color_count(&hovered, KUC_DARK_HOVER_BORDER);
    assert!(
        hovered_count > normal_count,
        "window settings toggle hover must paint KUC interactive preset border: normal={normal_count} hovered={hovered_count}"
    );
    Ok(())
}

#[test]
fn storybook_window_settings_toggle_click_uses_kuc_action_target_center()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_catalog()?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let (dark_x, dark_y) = settings_field_point(&storybook, StorybookSettingsField::Dark)?;
    let pointer = StorybookPointer::new(dark_x, dark_y, StorybookMouseButton::Left);

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(pointer.x, pointer.y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "rendered KUC settings row center must be the SettingsList pointer lane"
    );
    assert!(storybook.apply_canvas_click(pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert!(
        !storybook.dark,
        "clicking the rendered KUC settings row center must toggle the dark setting"
    );
    Ok(())
}

#[test]
fn storybook_window_interaction_toggles_click_use_kuc_action_target_center()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            StorybookSettingsField::Hover,
            is_hover_highlight_enabled as fn(&super::StorybookWindow) -> bool,
        ),
        (
            StorybookSettingsField::Selection,
            is_selection_enabled as fn(&super::StorybookWindow) -> bool,
        ),
        (
            StorybookSettingsField::ImageControls,
            is_image_controls_enabled as fn(&super::StorybookWindow) -> bool,
        ),
        (
            StorybookSettingsField::DiagramControls,
            is_diagram_controls_enabled as fn(&super::StorybookWindow) -> bool,
        ),
        (
            StorybookSettingsField::CodeControls,
            is_code_controls_enabled as fn(&super::StorybookWindow) -> bool,
        ),
    ];

    for (field, is_enabled) in cases {
        let mut storybook = storybook_with_catalog()?;
        storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let (x, y) = settings_field_point(&storybook, field)?;
        let pointer = StorybookPointer::new(x, y, StorybookMouseButton::Left);

        assert_eq!(
            UiCursor::Pointer,
            storybook.cursor_for_canvas_point(pointer.x, pointer.y, WINDOW_WIDTH, WINDOW_HEIGHT),
            "rendered KUC settings row center must be clickable: {field:?}"
        );
        assert!(storybook.apply_canvas_click(pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
        assert!(
            !is_enabled(&storybook),
            "clicking rendered KUC settings row center must toggle off: {field:?}"
        );
    }
    Ok(())
}

#[test]
fn storybook_window_settings_toggle_click_uses_kuc_action_target_edges()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_catalog()?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let target = settings_field_target(&storybook, StorybookSettingsField::Dark)?;
    let half_width = target.center_x - target.left;
    let y = SIDEBAR_CONTENT_INSET as f32 + target.center_y;
    let left = sidebar_content_x() as f32 + target.left + 1.0;
    let right = sidebar_content_x() as f32 + target.center_x + half_width - 1.0;

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(left, y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "rendered KUC settings row left edge must be clickable"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(left, y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(!storybook.dark);

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(right, y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "rendered KUC settings row right edge must be clickable"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(right, y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(storybook.dark);
    Ok(())
}

#[test]
fn storybook_window_settings_toggle_edge_hover_and_click_use_kuc_action_target()
-> Result<(), Box<dyn std::error::Error>> {
    for edge in [ToggleEdge::Left, ToggleEdge::Right] {
        let mut storybook = storybook_with_catalog()?;
        storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let (x, y) = settings_toggle_edge_point(&storybook, edge)?;

        let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
        assert!(storybook.update_sidebar_settings_hover_for_canvas_point(
            x,
            y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT
        ));
        assert_eq!(
            UiCursor::Pointer,
            storybook.cursor_for_canvas_point(x, y, WINDOW_WIDTH, WINDOW_HEIGHT),
            "rendered KUC settings row {edge:?} edge must expose pointer cursor"
        );
        let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
        let normal_count = color_count(&normal, KUC_DARK_HOVER_BORDER);
        let hovered_count = color_count(&hovered, KUC_DARK_HOVER_BORDER);
        assert!(
            hovered_count > normal_count,
            "rendered KUC settings row {edge:?} edge hover must paint preset border: normal={normal_count} hovered={hovered_count}"
        );

        assert!(storybook.apply_canvas_click(
            StorybookPointer::new(x, y, StorybookMouseButton::Left),
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )?);
        assert!(
            !storybook.dark,
            "clicking the rendered KUC settings row {edge:?} edge must toggle the dark setting"
        );
    }
    Ok(())
}

#[test]
fn storybook_window_settings_section_header_hover_and_click_use_kuc_action_target()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_catalog()?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert!(!storybook.settings_state.is_collapsed("display"));

    let target = settings_section_target(&storybook, "display")?;
    let Some(SettingsListAction::ToggleSection { section_id }) = target.action.as_ref() else {
        return Err(std::io::Error::other("expected display section toggle action").into());
    };
    assert_eq!("display", section_id);
    let (x, y) = settings_section_point(&storybook, "display")?;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_sidebar_settings_hover_for_canvas_point(
        x,
        y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    ));
    assert_eq!(
        Some(UiNodeId::new("settings-section:display")),
        storybook.settings_state.hovered_node_id()
    );
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(x, y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "rendered KUC settings section header must expose pointer cursor"
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(
        pixel_diff_count(&normal, &hovered) > 0,
        "settings section header hover must change rendered sidebar pixels"
    );

    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(x, y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(storybook.settings_state.is_collapsed("display"));
    assert_eq!("toggle-settings-section", storybook.last_command_label);
    let collapsed = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(
        pixel_diff_count(&hovered, &collapsed) > 0,
        "settings section click must collapse visible fields"
    );

    let (open_x, open_y) = settings_section_point(&storybook, "display")?;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(open_x, open_y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "collapsed KUC settings section header must remain clickable"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(open_x, open_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(!storybook.settings_state.is_collapsed("display"));
    assert_eq!("toggle-settings-section", storybook.last_command_label);
    Ok(())
}

#[test]
fn storybook_window_settings_section_header_e2e_toggle_matrix()
-> Result<(), Box<dyn std::error::Error>> {
    for section_id in ["display", "interaction", "state"] {
        let mut storybook = storybook_with_minimal_catalog();

        assert_section_header_toggles(&mut storybook, section_id)?;
    }
    Ok(())
}

#[test]
fn storybook_window_settings_section_header_edge_click_matrix()
-> Result<(), Box<dyn std::error::Error>> {
    for (index, section_id) in ["display", "interaction", "state"].iter().enumerate() {
        let mut storybook = storybook_with_minimal_catalog();
        let (collapse_edge, reopen_edge) = if index % 2 == 0 {
            (HorizontalEdge::Left, HorizontalEdge::Right)
        } else {
            (HorizontalEdge::Right, HorizontalEdge::Left)
        };

        assert_section_header_edge_clicks(&mut storybook, section_id, collapse_edge, true)?;
        assert_section_header_edge_clicks(&mut storybook, section_id, reopen_edge, false)?;
    }
    Ok(())
}

fn assert_section_header_edge_clicks(
    storybook: &mut super::super::StorybookWindow,
    section_id: &str,
    edge: HorizontalEdge,
    expected_collapsed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let (x, y) = section_header_edge_point(storybook, section_id, edge)?;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(x, y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "section header {edge:?} edge must expose pointer cursor: {section_id}"
    );
    let _ = storybook.settings_state.set_hovered_node_id(None);
    assert!(storybook.update_sidebar_settings_hover_for_canvas_point(
        x,
        y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    ));
    assert_eq!(
        Some(UiNodeId::new(format!("settings-section:{section_id}"))),
        storybook.settings_state.hovered_node_id()
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(x, y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert_eq!(
        expected_collapsed,
        storybook.settings_state.is_collapsed(section_id),
        "section header {edge:?} edge click must set collapsed={expected_collapsed}: {section_id}"
    );
    assert_eq!("toggle-settings-section", storybook.last_command_label);
    Ok(())
}

fn assert_section_header_toggles(
    storybook: &mut super::super::StorybookWindow,
    section_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(
        !storybook.settings_state.is_collapsed(section_id),
        "section must start open: {section_id}"
    );
    let target = settings_section_target(storybook, section_id)?;
    let Some(SettingsListAction::ToggleSection {
        section_id: action_section_id,
    }) = target.action.as_ref()
    else {
        return Err(
            std::io::Error::other(format!("expected toggle action for {section_id}")).into(),
        );
    };
    assert_eq!(section_id, action_section_id);

    let (x, y) = settings_section_point(storybook, section_id)?;
    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_sidebar_settings_hover_for_canvas_point(
        x,
        y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    ));
    assert_eq!(
        Some(UiNodeId::new(format!("settings-section:{section_id}"))),
        storybook.settings_state.hovered_node_id()
    );
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(x, y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "section header must expose pointer cursor: {section_id}"
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(
        pixel_diff_count(&normal, &hovered) > 0,
        "section header hover must affect rendered pixels: {section_id}"
    );

    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(x, y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(
        storybook.settings_state.is_collapsed(section_id),
        "section click must collapse: {section_id}"
    );
    assert_eq!("toggle-settings-section", storybook.last_command_label);

    let (open_x, open_y) = settings_section_point(storybook, section_id)?;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(open_x, open_y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "collapsed section header must remain clickable: {section_id}"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(open_x, open_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(
        !storybook.settings_state.is_collapsed(section_id),
        "second section click must reopen: {section_id}"
    );
    assert_eq!("toggle-settings-section", storybook.last_command_label);
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum ToggleEdge {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum HorizontalEdge {
    Left,
    Right,
}

fn settings_toggle_edge_point(
    storybook: &super::StorybookWindow,
    edge: ToggleEdge,
) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let target = settings_field_target(storybook, StorybookSettingsField::Dark)?;
    let half_width = target.center_x - target.left;
    let y = SIDEBAR_CONTENT_INSET as f32 + target.center_y;
    let x = match edge {
        ToggleEdge::Left => sidebar_content_x() as f32 + target.left + 1.0,
        ToggleEdge::Right => sidebar_content_x() as f32 + target.center_x + half_width - 1.0,
    };
    Ok((x, y))
}

fn is_hover_highlight_enabled(storybook: &super::StorybookWindow) -> bool {
    storybook.interaction.hover_highlight_enabled
}

fn is_selection_enabled(storybook: &super::StorybookWindow) -> bool {
    storybook.interaction.selection_enabled
}

fn is_image_controls_enabled(storybook: &super::StorybookWindow) -> bool {
    storybook.interaction.image_controls_enabled
}

fn is_diagram_controls_enabled(storybook: &super::StorybookWindow) -> bool {
    storybook.interaction.diagram_controls_enabled
}

fn is_code_controls_enabled(storybook: &super::StorybookWindow) -> bool {
    storybook.interaction.code_controls_enabled
}

fn section_header_edge_point(
    storybook: &super::super::StorybookWindow,
    section_id: &str,
    edge: HorizontalEdge,
) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let target = settings_section_target(storybook, section_id)?;
    if target.right <= target.left + 2.0 {
        return Err(std::io::Error::other(format!(
            "settings section hit rect is too narrow: {section_id}"
        ))
        .into());
    }
    let y = SIDEBAR_CONTENT_INSET as f32 + target.center_y;
    let x = match edge {
        HorizontalEdge::Left => sidebar_content_x() as f32 + target.left + 1.0,
        HorizontalEdge::Right => sidebar_content_x() as f32 + target.right - 1.0,
    };
    Ok((x, y))
}

#[test]
fn storybook_window_preview_font_setting_rebuilds_scene_and_preview_pixels()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert_eq!(14, storybook.typography.preview_font_size);
    assert_eq!(
        14,
        storybook
            .scene
            .as_ref()
            .ok_or("scene missing")?
            .typography
            .preview_font_size
    );
    let before = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    let (font_x, font_y) =
        settings_field_point(&storybook, StorybookSettingsField::PreviewFontSize)?;
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(font_x, font_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert_eq!(16, storybook.typography.preview_font_size);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert_eq!(
        16,
        storybook
            .scene
            .as_ref()
            .ok_or("scene missing")?
            .typography
            .preview_font_size
    );
    let after = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(
        pixel_diff_count_in_rect(
            &before,
            &after,
            SIDEBAR_WIDTH + 16,
            HEADER_HEIGHT + 16,
            WINDOW_WIDTH - SIDEBAR_WIDTH - 16,
            WINDOW_HEIGHT - HEADER_HEIGHT - 16,
        ) > 256,
        "preview font setting must rebuild and change preview pixels, not only sidebar state"
    );
    Ok(())
}

#[test]
fn storybook_window_code_copy_click_returns_host_copy_from_visible_button()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let copy_hit = pointer_for_media_action(&scene, "copy-code")?;
    storybook.scene = Some(scene.clone());
    storybook.scroll_y = copy_hit.scroll_y;
    let canvas = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let code_background = theme_color(&scene, "code-background")?;
    let copy_point = (
        copy_hit.pointer.x.round() as usize,
        copy_hit.pointer.y.round() as usize,
    );
    assert!(
        non_color_count_in_radius(&canvas, code_background, copy_point.0, copy_point.1, 14) > 8,
        "code copy button must be visible on top of the code block"
    );

    let command = StorybookMouse::command_for_click(
        &scene,
        copy_hit.scroll_y,
        copy_hit.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing code copy command"))?;
    let ViewerCommand::Host(HostCommand::CopyText(copy_command)) = command else {
        return Err(std::io::Error::other("expected code copy host command").into());
    };
    assert_eq!(
        katana_document_viewer::CopyTextSource::Code,
        copy_command.source
    );
    assert_eq!(copy_command.target.source.raw.text, copy_command.text);
    assert!(!copy_command.text.trim().is_empty());

    assert!(storybook.apply_canvas_click(copy_hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("host", storybook.last_command_label);
    Ok(())
}

#[test]
fn storybook_window_code_copy_hover_draws_interactive_preset_border()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let copy = pointer_for_media_action(&scene, "copy-code")?;
    storybook.scene = Some(scene);
    storybook.scroll_y = copy.scroll_y;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        copy.pointer.x,
        copy.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            copy.pointer.x,
            copy.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert!(
        pixel_diff_count(&normal, &hovered) > 0,
        "code copy hover must change KUC interactive preset pixels"
    );
    Ok(())
}

fn assert_tree_view_select_and_toggle() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_catalog()?;
    let file_id = storybook.catalog.fixtures[1].label.clone();
    let (file_x, file_y) = file_tree_item_point(&storybook, &file_id)?;

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(file_x, file_y, WINDOW_WIDTH, WINDOW_HEIGHT)
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(file_x, file_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert_eq!(1, storybook.selected_index);
    assert_eq!("select-file", storybook.last_command_label);

    let mut storybook = storybook_with_catalog()?;
    let directory = first_directory_hit(&storybook)?;
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(directory.x, directory.y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(storybook.file_tree_state.is_collapsed(&directory.id));
    assert_eq!("toggle-directory", storybook.last_command_label);
    Ok(())
}

fn assert_settings_toggle_and_select() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_catalog()?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let (dark_x, dark_y) = settings_field_point(&storybook, StorybookSettingsField::Dark)?;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(dark_x, dark_y, WINDOW_WIDTH, WINDOW_HEIGHT)
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(dark_x, dark_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    assert!(!storybook.dark);

    let (mode_x, mode_y) = settings_field_point(&storybook, StorybookSettingsField::Mode)?;
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(mode_x, mode_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert_eq!(ViewerMode::Slideshow, storybook.mode);
    assert_eq!(
        ViewerMode::Slideshow,
        storybook.scene.as_ref().ok_or("scene missing")?.mode
    );
    Ok(())
}

fn assert_document_link_task_and_accordion() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;

    let link = pointer_for_link(&scene, "Normal link", "https://github.com")?;
    storybook.scene = Some(scene.clone());
    storybook.scroll_y = link.scroll_y;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            link.pointer.x,
            link.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT
        )
    );
    assert!(storybook.apply_canvas_click(link.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("link", storybook.last_command_label);

    let accordion = pointer_for_accordion(&scene)?;
    storybook.scroll_y = accordion.scroll_y;
    assert!(storybook.apply_canvas_click(accordion.pointer, WINDOW_WIDTH, WINDOW_HEIGHT,)?);
    let accordion_node_id = first_accordion_override_key(&storybook)?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let opened_scene = storybook.scene.as_ref().ok_or("scene missing")?;
    assert_eq!(
        Some(true),
        accordion_open_state(opened_scene.tree.root(), accordion_node_id.as_str()),
        "accordion click must rebuild the KUC scene into open state"
    );

    storybook.scene = Some(scene.clone());
    let task = pointer_for_task(&scene, "[ ]")?;
    storybook.scroll_y = task.scroll_y;
    assert!(storybook.apply_canvas_click(task.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("task", storybook.last_command_label);
    Ok(())
}

#[test]
fn storybook_window_right_aligned_html_link_uses_rendered_hit_rect()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("direct/html-alignment.html")?;
    let scene = direct_html_alignment_scene()?;
    let link = pointer_for_link(&scene, "Right aligned link", "https://example.com")?;
    storybook.scene = Some(scene);
    storybook.scroll_y = link.scroll_y;

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            link.pointer.x,
            link.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT
        )
    );
    assert!(storybook.apply_canvas_click(link.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("link", storybook.last_command_label);
    Ok(())
}

fn assert_diagram_control_click_and_hover() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_media_action(&scene, "fullscreen")?;

    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            hit.pointer.x,
            hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT
        )
    );
    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    let changed = storybook
        .diagram_viewports
        .values()
        .any(|state| *state != DiagramViewportState::default());
    assert!(changed, "diagram fullscreen click must update viewer state");
    Ok(())
}

#[test]
fn storybook_window_internal_diagram_control_hover_draws_kuc_border()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_internal_diagram_action(&scene, "zoom-in")?;

    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    ));
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    let normal_count = color_count(&normal, KUC_DARK_HOVER_BORDER);
    let hovered_count = color_count(&hovered, KUC_DARK_HOVER_BORDER);
    let diff = pixel_diff_count(&normal, &hovered);
    assert!(
        hovered_count > normal_count && diff > 0,
        "internal diagram hover must paint KUC border: normal={normal_count} hovered={hovered_count} diff={diff} hovered_action={:?}",
        storybook.hovered_action_node_id
    );
    Ok(())
}

#[test]
fn storybook_window_diagram_fullscreen_click_updates_viewport_state()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_media_action(&scene, "fullscreen")?;

    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    let normal_display_width = max_image_surface_display_width_milli(
        storybook
            .scene
            .as_ref()
            .ok_or_else(|| std::io::Error::other("normal diagram scene missing"))?
            .tree
            .root(),
    )
    .ok_or_else(|| std::io::Error::other("normal diagram surface missing"))?;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            hit.pointer.x,
            hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT
        )
    );
    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    let fullscreen_open = storybook
        .diagram_viewports
        .values()
        .any(|state| state.fullscreen_open);
    assert!(
        fullscreen_open,
        "fullscreen click must open diagram viewport"
    );
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("fullscreen scene missing"))?;
    let fullscreen_display_width = max_image_surface_display_width_milli(scene.tree.root())
        .ok_or_else(|| std::io::Error::other("fullscreen diagram surface missing"))?;
    let fullscreen_actions = media_host_action_debug_values(&scene.tree);
    let fullscreen_viewport_key = storybook
        .diagram_viewports
        .iter()
        .find_map(|(node_id, state)| state.fullscreen_open.then_some(node_id.clone()))
        .ok_or_else(|| std::io::Error::other("fullscreen viewport missing"))?;
    let fullscreen_max_width_milli = (WINDOW_WIDTH.saturating_sub(80) as u32) * 1000;
    assert!(
        fullscreen_display_width > 0
            && fullscreen_display_width <= fullscreen_max_width_milli
            && fullscreen_display_width <= normal_display_width,
        "fullscreen scene must use KatanA padded fit without upscaling: normal={normal_display_width} fullscreen={fullscreen_display_width} max={fullscreen_max_width_milli} viewports={:?} actions={:?} widths={:?}",
        storybook.diagram_viewports,
        fullscreen_actions,
        image_surface_debug_values(scene.tree.root())
    );
    assert_eq!(
        vec![
            format!("{fullscreen_viewport_key}:fullscreen"),
            format!("{fullscreen_viewport_key}:fullscreen")
        ],
        fullscreen_actions,
        "fullscreen scene must render only active diagram controls: backdrop close and top-right close"
    );
    assert_fullscreen_close_hit_uses_katana_contract(&scene)?;
    let fullscreen_frame = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let visible_pixels = visible_fullscreen_pixels(&fullscreen_frame);
    assert!(
        visible_pixels > 800,
        "fullscreen scene must render the active diagram and controls inside the viewport, not only a blank backdrop: visible_pixels={visible_pixels} viewports={:?} widths={:?}",
        storybook.diagram_viewports,
        image_surface_debug_values(scene.tree.root())
    );
    let close_hit = pointer_for_fullscreen_close_in_viewport(&scene)?;

    assert!(storybook.apply_canvas_click(close_hit, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    let fullscreen_closed = storybook
        .diagram_viewports
        .values()
        .all(|state| !state.fullscreen_open);
    assert!(
        fullscreen_closed,
        "second fullscreen click must close diagram viewport"
    );
    Ok(())
}

#[test]
fn storybook_window_diagram_fullscreen_close_hit_follows_katana_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    storybook.scene = Some(sample_diagram_controls_scene()?);
    click_diagram_control_from_current_scene(&mut storybook, "fullscreen")?;
    assert!(only_diagram_viewport(&storybook)?.fullscreen_open);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("fullscreen scene missing"))?;

    storybook.scroll_y = 0.0;
    let close_hit = assert_fullscreen_close_hit_uses_katana_contract(&scene)?;
    let (document_x, document_y) = close_hit.center_point();
    let (canvas_x, canvas_y) = fullscreen_canvas_point_for_document_point(document_x, document_y);
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(canvas_x, canvas_y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "fullscreen close center must keep pointer cursor from the KUC host-action hit"
    );
    let command = StorybookMouse::command_for_click(
        &scene,
        storybook.scroll_y,
        StorybookPointer::new(canvas_x, canvas_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );
    assert!(
        matches!(command, Some(ViewerCommand::Diagram(_))),
        "fullscreen close center must resolve to a diagram viewer command"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(canvas_x, canvas_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    )?);
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    assert_eq!(
        2,
        storybook.host_events.len(),
        "fullscreen open and close must be the only propagated diagram host events"
    );
    assert!(!only_diagram_viewport(&storybook)?.fullscreen_open);
    Ok(())
}

#[test]
fn storybook_window_diagram_fullscreen_resize_refreshes_scene_before_control_hit()
-> Result<(), Box<dyn std::error::Error>> {
    const RESIZED_WIDTH: usize = WINDOW_WIDTH + 280;
    const RESIZED_HEIGHT: usize = WINDOW_HEIGHT - 120;

    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    storybook.scene = Some(sample_diagram_controls_scene()?);
    click_diagram_control_from_current_scene(&mut storybook, "fullscreen")?;
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert!(only_diagram_viewport(&storybook)?.fullscreen_open);

    let size_changed = storybook.update_frame_size(RESIZED_WIDTH, RESIZED_HEIGHT);
    assert!(size_changed);
    assert!(storybook.refresh_scene_before_input_if_needed(
        size_changed,
        RESIZED_WIDTH,
        RESIZED_HEIGHT
    )?);
    let scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("resized fullscreen scene missing"))?;
    let close_hit =
        assert_fullscreen_close_hit_uses_katana_contract_for_size(&scene, RESIZED_WIDTH)?;
    let (document_x, document_y) = close_hit.center_point();
    let (canvas_x, canvas_y) = fullscreen_canvas_point_for_document_point(document_x, document_y);

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(canvas_x, canvas_y, RESIZED_WIDTH, RESIZED_HEIGHT),
        "fullscreen close hit must follow the current resized viewport before mouse input"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(canvas_x, canvas_y, StorybookMouseButton::Left),
        RESIZED_WIDTH,
        RESIZED_HEIGHT
    )?);
    assert!(!only_diagram_viewport(&storybook)?.fullscreen_open);
    Ok(())
}

#[test]
fn storybook_window_diagram_fullscreen_backdrop_click_closes_like_katana_input_blocker()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    storybook.scene = Some(sample_diagram_controls_scene()?);
    click_diagram_control_from_current_scene(&mut storybook, "fullscreen")?;
    assert!(only_diagram_viewport(&storybook)?.fullscreen_open);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("fullscreen scene missing"))?;

    storybook.scroll_y = 0.0;
    let backdrop_hit = assert_fullscreen_backdrop_hit_uses_katana_contract(&scene)?;
    let (document_x, document_y) = backdrop_hit.center_point();
    let (canvas_x, canvas_y) = fullscreen_canvas_point_for_document_point(document_x, document_y);

    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(canvas_x, canvas_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    )?);
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    assert_eq!(
        2,
        storybook.host_events.len(),
        "fullscreen open and backdrop close must be propagated host events"
    );
    assert!(!only_diagram_viewport(&storybook)?.fullscreen_open);
    Ok(())
}

#[test]
fn storybook_window_diagram_fullscreen_controls_ignore_stale_document_scroll()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    storybook.scene = Some(sample_diagram_controls_scene()?);
    click_diagram_control_from_current_scene(&mut storybook, "fullscreen")?;
    assert!(only_diagram_viewport(&storybook)?.fullscreen_open);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("fullscreen scene missing"))?;

    storybook.scroll_y = 420.0;
    let zoom_hit = pointer_for_internal_diagram_action_in_viewport(&scene, "zoom-in")?;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(zoom_hit.x, zoom_hit.y, WINDOW_WIDTH, WINDOW_HEIGHT,),
        "fullscreen diagram controls must use viewport coordinates even if document scroll remains"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(zoom_hit.x, zoom_hit.y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    )?);
    assert!(
        only_diagram_viewport(&storybook)?.zoom > 1.0,
        "fullscreen zoom-in must dispatch from the KUC internal control hit"
    );

    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("fullscreen scene missing after zoom"))?;
    storybook.scroll_y = 420.0;
    let close_hit = pointer_for_fullscreen_close_in_viewport(&scene)?;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(close_hit.x, close_hit.y, WINDOW_WIDTH, WINDOW_HEIGHT,),
        "fullscreen close must use viewport coordinates even if document scroll remains"
    );
    assert!(storybook.apply_canvas_click(
        StorybookPointer::new(close_hit.x, close_hit.y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    )?);
    assert!(!only_diagram_viewport(&storybook)?.fullscreen_open);
    Ok(())
}

#[test]
fn storybook_window_every_diagram_control_click_dispatches_from_kuc_hit()
-> Result<(), Box<dyn std::error::Error>> {
    for action in diagram_control_actions() {
        let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
        let scene = sample_diagram_controls_scene()?;
        let hit = pointer_for_media_action(&scene, action)?;
        storybook.scene = Some(scene);
        storybook.scroll_y = hit.scroll_y;

        assert_eq!(
            UiCursor::Pointer,
            storybook.cursor_for_canvas_point(
                hit.pointer.x,
                hit.pointer.y,
                WINDOW_WIDTH,
                WINDOW_HEIGHT
            ),
            "{action} must expose pointer cursor from KUC hit"
        );
        assert!(
            storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?,
            "{action} must dispatch through StorybookWindow click path"
        );
        assert_diagram_window_dispatch(action, &storybook)?;
    }
    Ok(())
}

#[test]
fn storybook_window_diagram_control_click_repaints_viewer_frame()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_media_action(&scene, "fullscreen")?;
    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    let before = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let after = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert!(
        pixel_diff_count(&before, &after) > 64,
        "diagram control click must repaint the visible viewer frame"
    );
    Ok(())
}

#[test]
fn storybook_window_diagram_fullscreen_gestures_update_viewport_state()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_media_action(&scene, "fullscreen")?;

    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert!(storybook.apply_fullscreen_diagram_drag(ViewerVector { x: 12.0, y: -8.0 }));
    let drag_state = only_diagram_viewport(&storybook)?;
    assert_ne!(drag_state.pan, DiagramViewportState::default().pan);

    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert!(storybook.apply_fullscreen_diagram_smooth_scroll(ViewerVector { x: -24.0, y: 18.0 }));
    let smooth_state = only_diagram_viewport(&storybook)?;
    assert_ne!(smooth_state.pan, drag_state.pan);

    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    assert!(storybook.apply_fullscreen_diagram_trackpad_zoom(1.2));
    let zoom_state = only_diagram_viewport(&storybook)?;
    assert!(zoom_state.zoom > DiagramViewportState::default().zoom);
    Ok(())
}

#[test]
fn storybook_window_fullscreen_diagram_held_drag_pans_after_initial_press()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_media_action(&scene, "fullscreen")?;

    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let initial = only_diagram_viewport(&storybook)?;
    assert!(initial.fullscreen_open);

    assert!(!storybook.update_fullscreen_diagram_drag(
        true,
        Some((320.0, 240.0)),
        Some(StorybookMouseButton::Left)
    ));
    assert_eq!(initial.pan, only_diagram_viewport(&storybook)?.pan);

    assert!(storybook.update_fullscreen_diagram_drag(true, Some((352.0, 224.0)), None));
    let dragged = only_diagram_viewport(&storybook)?;
    assert_eq!(32.0, dragged.pan.x);
    assert_eq!(-16.0, dragged.pan.y);

    assert!(!storybook.update_fullscreen_diagram_drag(false, Some((352.0, 224.0)), None));
    assert!(!storybook.update_fullscreen_diagram_drag(true, Some((380.0, 250.0)), None));
    assert_eq!(
        dragged.pan,
        only_diagram_viewport(&storybook)?.pan,
        "re-pressing after release must seed the drag origin before applying pan"
    );
    Ok(())
}

#[test]
fn storybook_window_document_diagram_held_drag_pans_from_kuc_media_hit()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_first_diagram_body(&scene)?;

    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    let start = (hit.pointer.x, hit.pointer.y);
    assert!(storybook.update_document_diagram_drag(
        true,
        Some(start),
        Some(StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    assert!(
        storybook.diagram_viewports.is_empty(),
        "initial press seeds the drag origin but must not pan yet"
    );

    assert!(storybook.update_document_diagram_drag(
        true,
        Some((start.0 + 24.0, start.1 - 10.0)),
        None,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    let dragged = only_diagram_viewport(&storybook)?;
    assert_eq!(24.0, dragged.pan.x);
    assert_eq!(-10.0, dragged.pan.y);
    assert!(!dragged.fullscreen_open);
    Ok(())
}

#[test]
fn storybook_window_document_diagram_drag_uses_scroll_aware_kuc_surface_with_root_offset()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let mut scene = sample_diagram_controls_scene()?;
    let base_hit = pointer_for_first_diagram_body(&scene)?;
    let root_offset = base_hit.scroll_y.round().max(1.0) as u32;
    scene.tree = scene.tree.with_scroll_area_offset_y(root_offset);
    let hit = pointer_for_first_visible_diagram_body(&scene, root_offset as f32)?;

    storybook.scene = Some(scene);
    storybook.scroll_y = root_offset as f32;
    let start = (hit.pointer.x, hit.pointer.y);
    assert!(storybook.update_document_diagram_drag(
        true,
        Some(start),
        Some(StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));

    assert!(storybook.update_document_diagram_drag(
        true,
        Some((start.0 + 18.0, start.1 + 14.0)),
        None,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    let dragged = only_diagram_viewport(&storybook)?;
    assert_eq!(18.0, dragged.pan.x);
    assert_eq!(14.0, dragged.pan.y);
    assert!(!dragged.fullscreen_open);
    Ok(())
}

#[test]
fn storybook_window_document_diagram_trackpad_zoom_uses_kuc_media_hit()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_first_diagram_body(&scene)?;

    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;
    assert!(storybook.apply_document_diagram_trackpad_zoom_at(
        Some((hit.pointer.x, hit.pointer.y)),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        1.2,
    ));
    let zoomed = only_diagram_viewport(&storybook)?;
    assert!(zoomed.zoom > DiagramViewportState::default().zoom);
    assert!(!zoomed.fullscreen_open);
    Ok(())
}

#[test]
fn storybook_window_fullscreen_diagram_overlay_controls_dispatch_from_kuc_hits()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    storybook.scene = Some(sample_diagram_controls_scene()?);
    click_diagram_control_from_current_scene(&mut storybook, "fullscreen")?;
    assert!(only_diagram_viewport(&storybook)?.fullscreen_open);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let fullscreen_scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("fullscreen overlay scene missing"))?;

    for action in fullscreen_diagram_overlay_actions() {
        storybook.scene = Some(fullscreen_scene.clone());
        reset_only_diagram_viewport_for_fullscreen_overlay(&mut storybook)?;
        click_diagram_control_from_current_scene(&mut storybook, action)?;
        assert_diagram_fullscreen_overlay_dispatch(action, &storybook)?;
        assert!(
            storybook.asset_job.is_none(),
            "fullscreen overlay control must not restart asset work: {action}"
        );
    }
    Ok(())
}

#[test]
fn storybook_window_diagram_controls_survive_continuous_kuc_hit_sequence_without_asset_reload()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let base_scene = sample_diagram_controls_scene()?;
    storybook.scene = Some(base_scene.clone());

    click_diagram_control_from_current_scene(&mut storybook, "fullscreen")?;
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    assert_eq!(
        1,
        storybook.host_events.len(),
        "fullscreen open must be the only host event so far"
    );
    assert!(only_diagram_viewport(&storybook)?.fullscreen_open);
    assert!(
        storybook.asset_job.is_none(),
        "diagram controls must update loaded scene state without starting asset work"
    );

    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let fullscreen_scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("fullscreen scene missing"))?;
    storybook.scene = Some(fullscreen_scene);
    click_diagram_control_from_current_scene(&mut storybook, "fullscreen")?;
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    assert_eq!(
        2,
        storybook.host_events.len(),
        "fullscreen close must propagate, internal controls must not"
    );
    assert!(!only_diagram_viewport(&storybook)?.fullscreen_open);
    assert!(storybook.asset_job.is_none());

    storybook.scene = Some(base_scene.clone());
    click_diagram_control_from_current_scene(&mut storybook, "zoom-in")?;
    let zoomed = only_diagram_viewport(&storybook)?;
    assert!(zoomed.zoom > DiagramViewportState::default().zoom);
    assert_eq!("diagram", storybook.last_command_label);
    assert_eq!(2, storybook.host_events.len());
    assert!(storybook.asset_job.is_none());

    storybook.scene = Some(base_scene.clone());
    click_diagram_control_from_current_scene(&mut storybook, "pan-right")?;
    let panned_right = only_diagram_viewport(&storybook)?;
    assert!(panned_right.pan.x > DiagramViewportState::default().pan.x);
    assert_eq!(2, storybook.host_events.len());
    assert!(storybook.asset_job.is_none());

    storybook.scene = Some(base_scene.clone());
    click_diagram_control_from_current_scene(&mut storybook, "pan-down")?;
    let panned_down = only_diagram_viewport(&storybook)?;
    assert!(panned_down.pan.y > DiagramViewportState::default().pan.y);
    assert_eq!(2, storybook.host_events.len());
    assert!(storybook.asset_job.is_none());

    storybook.scene = Some(base_scene.clone());
    click_diagram_control_from_current_scene(&mut storybook, "trackpad-help")?;
    assert!(only_diagram_viewport(&storybook)?.help_requested);
    assert_eq!(2, storybook.host_events.len());
    assert!(storybook.asset_job.is_none());

    storybook.scene = Some(base_scene);
    click_diagram_control_from_current_scene(&mut storybook, "reset-view")?;
    assert_eq!(
        DiagramViewportState::default(),
        only_diagram_viewport(&storybook)?
    );
    assert_eq!(2, storybook.host_events.len());
    assert!(storybook.asset_job.is_none());
    Ok(())
}

#[test]
fn storybook_window_media_control_hover_draws_interactive_preset_border()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_diagrams.md")?;
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_media_action(&scene, "fullscreen")?;
    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    assert!(
        storybook.hovered_node_id.is_some(),
        "plain block hover must resolve a viewer target"
    );
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            hit.pointer.x,
            hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert!(
        pixel_diff_count(&normal, &hovered) > 0,
        "window hover state must change KUC media control pixels"
    );
    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("diagram:fullscreen", storybook.last_command_label);
    Ok(())
}

#[test]
fn storybook_window_link_hover_draws_block_hover_surface() -> Result<(), Box<dyn std::error::Error>>
{
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let link = pointer_for_link(&scene, "Normal link", "https://github.com")?;
    storybook.scene = Some(scene);
    storybook.scroll_y = link.scroll_y;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        link.pointer.x,
        link.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert_hover_surface_increased(&normal, &hovered, "link");
    Ok(())
}

#[test]
fn storybook_window_plain_block_hover_draws_viewer_hover_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let hit = pointer_for_plain_block(&scene)?;
    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    assert!(
        storybook.hovered_node_id.is_some(),
        "plain block hover must resolve a viewer target"
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert!(
        pixel_diff_count(&normal, &hovered) > 0,
        "plain block hover must change viewer pixels"
    );
    Ok(())
}

#[test]
fn storybook_window_plain_block_hover_uses_kuc_node_hit_cache()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let hit = pointer_for_plain_block(&scene)?;
    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;

    assert!(storybook.update_document_hover_for_canvas_point(
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    let hovered_node_id = storybook
        .hovered_node_id
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing hovered node id"))?;
    let cache = storybook
        .document_interaction_surface_cache
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing interaction cache"))?;

    assert!(
        cache
            .node_hits
            .iter()
            .any(|hit| hit.node_id.as_str() == hovered_node_id),
        "window hover must resolve viewer hover from KUC node hit cache, hovered={hovered_node_id} cached_hits={}",
        cache.node_hits.len()
    );
    Ok(())
}

#[test]
fn storybook_window_footnote_reference_click_jumps_to_definition()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let hit = pointer_for_link(&scene, "[1]", "#fn-1")?;
    let target = scene
        .target_for_internal_anchor("#fn-1")
        .ok_or_else(|| std::io::Error::other("missing footnote definition target"))?
        .clone();
    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            hit.pointer.x,
            hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT,)?);

    assert_eq!("toc", storybook.last_command_label);
    let visible_bottom = storybook.scroll_y + preview_viewport_height(WINDOW_HEIGHT) as f32;
    assert!(storybook.scroll_y <= target.rect.y);
    assert!(
        target.rect.y < visible_bottom,
        "footnote click must bring definition into view: scroll={} target={} visible_bottom={visible_bottom}",
        storybook.scroll_y,
        target.rect.y
    );
    Ok(())
}

#[test]
fn storybook_window_footnote_backlink_click_jumps_to_reference()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let hit = pointer_for_link(&scene, "↩", "#fnref-1")?;
    let target = scene
        .target_for_internal_anchor("#fnref-1")
        .ok_or_else(|| std::io::Error::other("missing footnote reference target"))?
        .clone();
    storybook.scene = Some(scene);
    storybook.scroll_y = hit.scroll_y;

    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            hit.pointer.x,
            hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    assert!(storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT,)?);

    assert_eq!("toc", storybook.last_command_label);
    let visible_bottom = storybook.scroll_y + preview_viewport_height(WINDOW_HEIGHT) as f32;
    assert!(storybook.scroll_y <= target.rect.y);
    assert!(
        target.rect.y < visible_bottom,
        "footnote backlink must bring reference into view: scroll={} target={} visible_bottom={visible_bottom}",
        storybook.scroll_y,
        target.rect.y
    );
    Ok(())
}

#[test]
fn storybook_window_sample_footnote_reference_click_jumps_after_loaded_assets()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample.md")?;
    storybook.update_scene_loaded(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook
        .scene
        .as_ref()
        .ok_or_else(|| std::io::Error::other("sample scene missing after loaded assets"))?
        .clone();
    assert!(
        scene.loaded_asset_count > 0 || scene.failed_asset_count > 0,
        "sample.md must exercise the loaded-asset coordinate path"
    );
    assert_sample_footnote_click(&mut storybook, &scene, "[1]", "#fn-1")?;
    assert_sample_footnote_click(&mut storybook, &scene, "↩", "#fnref-1")?;
    Ok(())
}

#[test]
fn storybook_window_sample_footnote_reference_click_uses_kuc_hit_rect_without_loaded_assets()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample.md")?;
    let scene = sample_scene()?;
    storybook.scene = Some(scene.clone());

    assert_sample_footnote_click(&mut storybook, &scene, "[1]", "#fn-1")?;
    assert_sample_footnote_click(&mut storybook, &scene, "↩", "#fnref-1")?;
    Ok(())
}

#[test]
fn storybook_window_paragraph_link_hover_and_click_use_same_kuc_action_rect()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let link = paragraph_link_pointer(&scene)?;
    storybook.scene = Some(scene.clone());
    storybook.scroll_y = link.scroll_y;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        link.pointer.x,
        link.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            link.pointer.x,
            link.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert_hover_surface_increased(&normal, &hovered, "paragraph link");

    let command = StorybookMouse::command_for_click(
        &scene,
        link.scroll_y,
        link.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing paragraph link command"))?;
    let ViewerCommand::Link(command) = command else {
        return Err(std::io::Error::other("expected paragraph link command").into());
    };
    assert!(
        command.target.source.raw.text.contains("日本語"),
        "test must exercise the paragraph HTML link"
    );
    assert!(
        !command.target.source.raw.text.contains("- [Normal link]"),
        "test must not exercise the list item link"
    );

    assert!(storybook.apply_canvas_click(link.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("link", storybook.last_command_label);
    Ok(())
}

fn assert_sample_footnote_click(
    storybook: &mut super::StorybookWindow,
    scene: &PreviewScene,
    label: &str,
    target_anchor: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hit = pointer_for_link(scene, label, target_anchor)?;
    let target = scene
        .target_for_internal_anchor(target_anchor)
        .ok_or_else(|| std::io::Error::other(format!("missing target: {target_anchor}")))?
        .clone();
    storybook.scroll_y = hit.scroll_y;
    storybook.frame_cache = None;
    storybook.document_interaction_surface_cache = None;

    let (document_x, document_y) =
        StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, storybook.scroll_y)
            .document_point(hit.pointer.x, hit.pointer.y)
            .ok_or_else(|| std::io::Error::other("footnote pointer outside preview area"))?;
    assert!(
        hit.pointer.x.is_finite() && hit.pointer.y.is_finite(),
        "footnote pointer must be finite"
    );
    assert!(
        target_anchor.starts_with('#'),
        "footnote target must stay an internal anchor"
    );
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            hit.pointer.x,
            hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ),
        "KUC hit center must map to a pointer cursor at document=({document_x:.1},{document_y:.1})"
    );
    assert!(
        storybook.apply_canvas_click(hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?,
        "KUC hit center click must dispatch for {label} -> {target_anchor}"
    );

    assert_eq!("toc", storybook.last_command_label);
    let visible_bottom = storybook.scroll_y + preview_viewport_height(WINDOW_HEIGHT) as f32;
    assert!(
        storybook.scroll_y <= target.rect.y && target.rect.y < visible_bottom,
        "footnote click must bring target into view: label={label} target={target_anchor} scroll={} target_y={} visible_bottom={visible_bottom}",
        storybook.scroll_y,
        target.rect.y
    );
    Ok(())
}

#[test]
fn storybook_window_list_link_hover_and_click_use_same_kuc_action_rect()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let link = pointer_for_link(&scene, "Normal link", "https://github.com")?;
    storybook.scene = Some(scene.clone());
    storybook.scroll_y = link.scroll_y;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        link.pointer.x,
        link.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            link.pointer.x,
            link.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert_hover_surface_increased(&normal, &hovered, "list link");

    let command = StorybookMouse::command_for_click(
        &scene,
        link.scroll_y,
        link.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing list link command"))?;
    let ViewerCommand::Link(command) = command else {
        return Err(std::io::Error::other("expected list link command").into());
    };
    assert!(
        command.target.source.raw.text.contains("- [Normal link]"),
        "test must exercise the list item link, not a paragraph link"
    );

    assert!(storybook.apply_canvas_click(link.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("link", storybook.last_command_label);
    Ok(())
}

fn paragraph_link_pointer(scene: &PreviewScene) -> Result<LinkPointer, Box<dyn std::error::Error>> {
    let hits = StorybookHostActionHits::hits(scene, WINDOW_WIDTH);
    for hit in hits {
        let Some(UiTextSpanAction::OpenLink { target }) = hit.action.text_span_action() else {
            continue;
        };
        if hit.action.label != "日本語" || target != "sample_basic.ja.md" {
            continue;
        }
        let (document_x, document_y) = hit.center_point();
        let scroll_y = (document_y - 120.0).max(0.0);
        let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        let pointer = StorybookPointer::new(x, y, StorybookMouseButton::Left);
        let Some(ViewerCommand::Link(command)) = StorybookMouse::command_for_click(
            scene,
            scroll_y,
            pointer,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ) else {
            continue;
        };
        if command.target.source.raw.text.contains("- [Normal link]") {
            continue;
        }
        return Ok(LinkPointer { pointer, scroll_y });
    }
    Err(std::io::Error::other("missing paragraph link hit").into())
}

struct LinkPointer {
    pointer: StorybookPointer,
    scroll_y: f32,
}

#[test]
fn storybook_window_accordion_hover_draws_block_hover_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let accordion = pointer_for_accordion(&scene)?;
    storybook.scene = Some(scene);
    storybook.scroll_y = accordion.scroll_y;

    let normal = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        accordion.pointer.x,
        accordion.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert_hover_surface_increased(&normal, &hovered, "accordion");
    Ok(())
}

#[test]
fn storybook_window_accordion_hover_click_open_and_close_use_same_kuc_action_rect()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let accordion_node_id =
        first_accordion_node_id(scene.tree.root()).ok_or("accordion node missing")?;
    assert_eq!(
        Some(false),
        accordion_open_state(scene.tree.root(), accordion_node_id.as_str()),
        "sample_basic details must start closed"
    );
    let accordion = pointer_for_accordion(&scene)?;
    storybook.scene = Some(scene);
    storybook.scroll_y = accordion.scroll_y;

    let closed = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert!(storybook.update_document_hover_for_canvas_point(
        accordion.pointer.x,
        accordion.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            accordion.pointer.x,
            accordion.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    let hovered = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert_hover_surface_increased(&closed, &hovered, "accordion open");

    assert!(storybook.apply_canvas_click(accordion.pointer, WINDOW_WIDTH, WINDOW_HEIGHT,)?);
    assert_eq!("accordion", storybook.last_command_label);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let opened_scene = storybook.scene.as_ref().ok_or("scene missing")?.clone();
    assert_eq!(
        Some(true),
        accordion_open_state(opened_scene.tree.root(), accordion_node_id.as_str()),
        "accordion click must rebuild the KUC scene into open state"
    );
    let opened = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert_accordion_body_changed(&closed, &opened, &accordion, "open");

    let close_hit = pointer_for_accordion(&opened_scene)?;
    storybook.scroll_y = close_hit.scroll_y;
    let opened_for_close = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(
            close_hit.pointer.x,
            close_hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
    );
    assert!(storybook.apply_canvas_click(close_hit.pointer, WINDOW_WIDTH, WINDOW_HEIGHT,)?);
    assert_eq!("accordion", storybook.last_command_label);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let closed_again_scene = storybook.scene.as_ref().ok_or("scene missing")?;
    assert_eq!(
        Some(false),
        accordion_open_state(closed_again_scene.tree.root(), accordion_node_id.as_str()),
        "second accordion click must rebuild the KUC scene into closed state"
    );
    let closed_again = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    assert_accordion_body_changed(&opened_for_close, &closed_again, &close_hit, "close");
    Ok(())
}

#[test]
fn storybook_window_accordion_click_updates_body_visibility_pixels()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("direct/html-alignment.html")?;
    let scene = direct_html_alignment_scene()?;
    let accordion = pointer_for_accordion(&scene)?;
    let accordion_node_id =
        first_accordion_node_id(scene.tree.root()).ok_or("accordion node missing")?;
    assert_eq!(
        Some(true),
        accordion_open_state(scene.tree.root(), accordion_node_id.as_str()),
        "direct html fixture must start with open details"
    );

    storybook.scene = Some(scene);
    storybook.scroll_y = accordion.scroll_y;
    let before = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    assert!(storybook.apply_canvas_click(accordion.pointer, WINDOW_WIDTH, WINDOW_HEIGHT,)?);
    assert_eq!("accordion", storybook.last_command_label);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let after_scene = storybook.scene.as_ref().ok_or("scene missing")?;
    assert_eq!(
        Some(false),
        accordion_open_state(after_scene.tree.root(), accordion_node_id.as_str()),
        "accordion click must rebuild the KUC scene into closed state"
    );
    let after = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let body_region_x = SIDEBAR_WIDTH + 24;
    let body_region_y = accordion.pointer.y.round() as usize + 16;
    let body_region_width = WINDOW_WIDTH.saturating_sub(SIDEBAR_WIDTH + 48);
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
        "accordion close must change pixels in the body area below the header"
    );
    assert!(
        pixel_diff_count(&before, &after) > 64,
        "accordion close must change rendered body pixels"
    );
    Ok(())
}

fn assert_accordion_body_changed(
    before: &crate::canvas::Canvas,
    after: &crate::canvas::Canvas,
    accordion: &crate::mouse::mouse_test_support::PointerHit,
    label: &str,
) {
    let body_region_x = SIDEBAR_WIDTH + 24;
    let body_region_y = accordion.pointer.y.round() as usize + 16;
    let body_region_width = WINDOW_WIDTH.saturating_sub(SIDEBAR_WIDTH + 48);
    let body_region_height = WINDOW_HEIGHT.saturating_sub(body_region_y);
    assert!(
        pixel_diff_count_in_rect(
            before,
            after,
            body_region_x,
            body_region_y,
            body_region_width,
            body_region_height,
        ) > 64,
        "accordion {label} must change pixels in the body area below the header"
    );
    assert!(
        pixel_diff_count(before, after) > 64,
        "accordion {label} must change rendered body pixels"
    );
}

#[test]
fn storybook_window_task_click_rebuilds_scene_with_external_state_override()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let source_before = std::fs::read_to_string(&storybook.catalog.fixtures[0].path)?;
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook
        .scene
        .as_ref()
        .ok_or_else(|| std::io::Error::other("initial scene missing"))?
        .clone();
    let task = pointer_for_task(&scene, "[ ]")?;
    storybook.scene = Some(scene);
    storybook.scroll_y = task.scroll_y;
    let before = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    let command = StorybookMouse::command_for_click(
        storybook.scene.as_ref().ok_or("scene missing")?,
        task.scroll_y,
        task.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing task command from click"))?;
    let state_id = match command {
        katana_document_viewer::ViewerCommand::Task(command) => {
            let task_target = command
                .task_target
                .ok_or_else(|| std::io::Error::other("task command missing typed target"))?;
            assert_ne!(task_target.state_id, command.target.artifact_id.0);
            task_target.state_id
        }
        _ => return Err(std::io::Error::other("expected task command from click").into()),
    };

    assert!(storybook.apply_canvas_click(task.pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    assert_eq!(
        "[x]",
        node_value_for_state(scene.tree.root(), state_id.as_str())?
    );
    let after = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let source_after = std::fs::read_to_string(&storybook.catalog.fixtures[0].path)?;
    assert_eq!(
        source_before, source_after,
        "task click must not edit markdown source in Storybook"
    );

    assert!(
        pixel_diff_count(&before, &after) > 64,
        "task click must change rendered KUC checkbox pixels through external state override"
    );
    Ok(())
}

#[test]
fn storybook_window_task_context_menu_selects_marker_through_external_state()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let task = pointer_for_task(&scene, "[ ]")?;
    storybook.scene = Some(scene);
    storybook.scroll_y = task.scroll_y;
    let before = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    let right_click =
        StorybookPointer::new(task.pointer.x, task.pointer.y, StorybookMouseButton::Right);
    assert!(storybook.apply_canvas_click(right_click, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("task-context", storybook.last_command_label);

    let blocked_pointer = storybook
        .task_context_menu
        .as_ref()
        .and_then(|menu| menu.test_pointer_for_marker("[-]"))
        .ok_or_else(|| std::io::Error::other("missing blocked context item"))?;
    assert!(storybook.apply_canvas_click(blocked_pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("task", storybook.last_command_label);
    assert!(
        storybook
            .task_state_overrides
            .values()
            .any(|state| *state == ViewerTaskState::Blocked)
    );

    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let after = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let task_hit_point = (
        task.pointer.x.round() as usize,
        task.pointer.y.round() as usize,
    );
    assert!(
        pixel_diff_count_in_radius(&before, &after, task_hit_point.0, task_hit_point.1, 12) > 16,
        "context menu marker selection must change local checkbox pixels through external state override"
    );
    Ok(())
}

#[test]
fn storybook_window_task_context_menu_selects_every_marker_through_external_state()
-> Result<(), Box<dyn std::error::Error>> {
    for (marker, expected) in [
        ("[ ]", ViewerTaskState::Empty),
        ("[x]", ViewerTaskState::Done),
        ("[/]", ViewerTaskState::Progress),
        ("[-]", ViewerTaskState::Blocked),
    ] {
        let mut storybook = storybook_with_label("katana/sample_basic.md")?;
        let scene = sample_basic_scene()?;
        let task = pointer_for_task(&scene, "[ ]")?;
        storybook.scene = Some(scene);
        storybook.scroll_y = task.scroll_y;

        let right_click =
            StorybookPointer::new(task.pointer.x, task.pointer.y, StorybookMouseButton::Right);
        assert!(storybook.apply_canvas_click(right_click, WINDOW_WIDTH, WINDOW_HEIGHT)?);
        assert_eq!("task-context", storybook.last_command_label);

        let marker_pointer = storybook
            .task_context_menu
            .as_ref()
            .and_then(|menu| menu.test_pointer_for_marker(marker))
            .ok_or_else(|| std::io::Error::other(format!("missing context item {marker}")))?;
        assert!(storybook.apply_canvas_click(marker_pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
        assert_eq!("task", storybook.last_command_label);
        let state_id = only_task_override(&storybook, expected)?;

        storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
        let scene = storybook.scene.as_ref().ok_or("scene missing")?;
        assert_eq!(
            expected.marker(),
            node_value_for_state(scene.tree.root(), state_id.as_str())?,
            "context menu marker {marker} must rebuild KUC node value through external state"
        );
    }
    Ok(())
}

#[test]
fn storybook_window_task_context_menu_updates_only_right_clicked_row()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook_with_label("katana/sample_basic.md")?;
    let scene = sample_basic_scene()?;
    let task = pointer_for_task(&scene, "[/]")?;
    let before_values = task_state_values(scene.tree.root());
    storybook.scene = Some(scene);
    storybook.scroll_y = task.scroll_y;

    let right_click =
        StorybookPointer::new(task.pointer.x, task.pointer.y, StorybookMouseButton::Right);
    assert!(storybook.apply_canvas_click(right_click, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("task-context", storybook.last_command_label);

    let blocked_pointer = storybook
        .task_context_menu
        .as_ref()
        .and_then(|menu| menu.test_pointer_for_marker("[-]"))
        .ok_or_else(|| std::io::Error::other("missing blocked context item"))?;
    assert!(storybook.apply_canvas_click(blocked_pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?);
    assert_eq!("task", storybook.last_command_label);
    let state_id = only_task_override(&storybook, ViewerTaskState::Blocked)?;
    assert_eq!(
        Some("[/]"),
        before_values.get(state_id.as_str()).map(String::as_str),
        "context menu action must target the right-clicked progress task row"
    );

    storybook.update_scene_for_refresh(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let after_values = task_state_values(scene.tree.root());
    for (id, before) in before_values {
        let after = after_values
            .get(id.as_str())
            .ok_or_else(|| std::io::Error::other(format!("missing task state {id}")))?;
        if id == state_id {
            assert_eq!("[-]", after, "right-clicked task row must change");
        } else {
            assert_eq!(&before, after, "non-target task row must not change: {id}");
        }
    }
    Ok(())
}

fn theme_color(scene: &crate::preview::PreviewScene, token: &str) -> Result<u32, std::io::Error> {
    let color = scene
        .theme
        .color(token)
        .ok_or_else(|| std::io::Error::other(format!("missing theme token: {token}")))?;
    Ok(((u32::from(color[0])) << 16) | ((u32::from(color[1])) << 8) | u32::from(color[2]))
}

fn color_count(canvas: &crate::canvas::Canvas, color: u32) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel == color)
        .count()
}

fn non_color_count_in_radius(
    canvas: &crate::canvas::Canvas,
    color: u32,
    center_x: usize,
    center_y: usize,
    radius: usize,
) -> usize {
    let width = canvas.width();
    let height = canvas.height();
    let min_x = center_x.saturating_sub(radius);
    let min_y = center_y.saturating_sub(radius);
    let max_x = (center_x.saturating_add(radius)).min(width.saturating_sub(1));
    let max_y = (center_y.saturating_add(radius)).min(height.saturating_sub(1));

    let mut count = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if canvas.pixels()[y * width + x] != color {
                count += 1;
            }
        }
    }
    count
}

fn assert_hover_surface_increased(
    normal: &crate::canvas::Canvas,
    hovered: &crate::canvas::Canvas,
    label: &str,
) {
    let normal_count = color_count(normal, SIDEBAR_TREE_HOVER_BACKGROUND);
    let hovered_count = color_count(hovered, SIDEBAR_TREE_HOVER_BACKGROUND);
    assert!(
        hovered_count > normal_count,
        "window {label} hover must paint block hover surface: normal={normal_count} hovered={hovered_count}"
    );
}

fn pointer_for_plain_block(scene: &PreviewScene) -> Result<PointerHit, std::io::Error> {
    let preview_width = crate::layout::preview_content_width(WINDOW_WIDTH);
    let node_hits = katana_ui_core_storybook::UiTreeSurfaceHost::new(scene.theme.clone())
        .document_node_hits(
            scene.tree.root(),
            katana_ui_core_storybook::UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_width,
                height: scene.content_height.ceil().max(1.0) as usize,
                scroll_y: 0.0,
            },
        );
    let (document_x, document_y) = node_hits
        .iter()
        .filter(|hit| hit.rect.width > 0 && hit.rect.height > 0)
        .map(|hit| {
            let (x, y) = hit.rect.center_point();
            (x, y)
        })
        .next()
        .ok_or_else(|| std::io::Error::other("missing plain block target"))?;
    let scroll_y = (document_y - 120.0).max(0.0);
    let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
    let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
    Ok(PointerHit {
        pointer: StorybookPointer::new(x, y, StorybookMouseButton::Left),
        scroll_y,
    })
}

fn node_value_for_state<'a>(node: &'a UiNode, state_id: &str) -> Result<&'a str, std::io::Error> {
    if node.props().state_id.as_str() == state_id {
        return Ok(node.props().interaction.value.as_str());
    }
    node.children()
        .iter()
        .find_map(|child| node_value_for_state(child, state_id).ok())
        .ok_or_else(|| {
            std::io::Error::other(format!(
                "state node missing: requested={state_id} available={:?}",
                task_state_ids(node)
            ))
        })
}

fn task_state_ids(node: &UiNode) -> Vec<String> {
    let mut values = Vec::new();
    collect_task_state_ids(node, &mut values);
    values
}

fn task_state_values(node: &UiNode) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    collect_task_state_values(node, &mut values);
    values
}

fn collect_task_state_values(node: &UiNode, values: &mut BTreeMap<String, String>) {
    if node.props().state_id.as_str().starts_with("ui-task-state:") {
        values.insert(
            node.props().state_id.as_str().to_string(),
            node.props().interaction.value.clone(),
        );
    }
    for child in node.children() {
        collect_task_state_values(child, values);
    }
}

fn collect_task_state_ids(node: &UiNode, values: &mut Vec<String>) {
    if node.props().state_id.as_str().starts_with("ui-task-state:") {
        values.push(node.props().state_id.as_str().to_string());
    }
    for child in node.children() {
        collect_task_state_ids(child, values);
    }
}

fn only_task_override(
    storybook: &super::StorybookWindow,
    expected: ViewerTaskState,
) -> Result<String, std::io::Error> {
    let mut overrides = storybook.task_state_overrides.iter();
    let Some((state_id, state)) = overrides.next() else {
        return Err(std::io::Error::other("task override missing"));
    };
    if overrides.next().is_some() {
        return Err(std::io::Error::other("multiple task overrides"));
    }
    assert_eq!(expected, *state);
    Ok(state_id.clone())
}

fn only_diagram_viewport(
    storybook: &super::StorybookWindow,
) -> Result<DiagramViewportState, std::io::Error> {
    let mut values = storybook.diagram_viewports.values();
    let Some(value) = values.next().copied() else {
        return Err(std::io::Error::other("diagram viewport missing"));
    };
    if values.next().is_some() {
        return Err(std::io::Error::other("multiple diagram viewports"));
    }
    Ok(value)
}

fn pointer_for_first_diagram_body(scene: &PreviewScene) -> Result<PointerHit, std::io::Error> {
    let Some(node_id) = scene.diagram_node_ids.iter().next() else {
        return Err(std::io::Error::other("diagram node id missing"));
    };
    let Some(target) = scene.target_for_node_id(node_id) else {
        return Err(std::io::Error::other("diagram target missing"));
    };
    let document_x = target.rect.x + target.rect.width * 0.5;
    let document_y = target.rect.y + target.rect.height * 0.5;
    let scroll_y = (document_y - 120.0).max(0.0);
    let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
    let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
    Ok(PointerHit {
        pointer: StorybookPointer::new(x, y, StorybookMouseButton::Left),
        scroll_y,
    })
}

fn pointer_for_first_visible_diagram_body(
    scene: &PreviewScene,
    scroll_y: f32,
) -> Result<PointerHit, std::io::Error> {
    let (_, node_hits) = katana_ui_core_storybook::UiTreeSurfaceHost::new(scene.theme.clone())
        .viewport_interaction_hits(
            scene.tree.root(),
            katana_ui_core_storybook::UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_content_width(WINDOW_WIDTH),
                height: preview_viewport_height(WINDOW_HEIGHT),
                scroll_y: render_scroll_delta_for_scene(scene, scroll_y),
            },
        );
    let hit = node_hits
        .into_iter()
        .filter(|hit| diagram_hit_matches_scene(scene, hit))
        .max_by_key(|hit| hit.rect.area())
        .ok_or_else(|| std::io::Error::other("visible diagram KUC hit missing"))?;
    let (viewport_x, viewport_y) = hit.rect.center_point();
    let document_y = viewport_y + DocumentPoint::effective_scroll_y(scene, scroll_y);
    let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
    let (x, y) = area.canvas_point_for_document_point(viewport_x, document_y);
    Ok(PointerHit {
        pointer: StorybookPointer::new(x, y, StorybookMouseButton::Left),
        scroll_y,
    })
}

fn render_scroll_delta_for_scene(scene: &PreviewScene, scroll_y: f32) -> f32 {
    let tree_offset = scene.tree.root().props().scroll_area.offset_y as f32;
    (scroll_y - tree_offset).max(0.0)
}

fn diagram_hit_matches_scene(
    scene: &PreviewScene,
    hit: &katana_ui_core_storybook::UiTreeNodeHit,
) -> bool {
    scene.diagram_node_ids.contains(hit.node_id.as_str())
        || hit
            .semantic_node_id
            .as_ref()
            .is_some_and(|node_id| scene.diagram_node_ids.contains(node_id.as_str()))
}

fn reset_only_diagram_viewport_for_fullscreen_overlay(
    storybook: &mut super::StorybookWindow,
) -> Result<(), std::io::Error> {
    let mut values = storybook.diagram_viewports.values_mut();
    let Some(value) = values.next() else {
        return Err(std::io::Error::other("diagram viewport missing"));
    };
    if values.next().is_some() {
        return Err(std::io::Error::other("multiple diagram viewports"));
    }
    *value = DiagramViewportState {
        fullscreen_open: true,
        ..DiagramViewportState::default()
    };
    Ok(())
}

fn diagram_control_actions() -> [&'static str; 9] {
    [
        "fullscreen",
        "pan-up",
        "pan-down",
        "pan-left",
        "pan-right",
        "zoom-in",
        "zoom-out",
        "reset-view",
        "trackpad-help",
    ]
}

fn fullscreen_diagram_overlay_actions() -> [&'static str; 8] {
    [
        "pan-up",
        "pan-down",
        "pan-left",
        "pan-right",
        "zoom-in",
        "zoom-out",
        "reset-view",
        "trackpad-help",
    ]
}

fn assert_diagram_window_dispatch(
    action: &str,
    storybook: &super::StorybookWindow,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected_label = if action == "fullscreen" {
        "diagram:fullscreen"
    } else {
        "diagram"
    };
    assert_eq!(expected_label, storybook.last_command_label);
    let state = only_diagram_viewport(storybook)?;
    match action {
        "fullscreen" => assert!(state.fullscreen_open, "fullscreen must open viewer"),
        "pan-up" => assert_eq!(-50.0, state.pan.y),
        "pan-down" => assert_eq!(50.0, state.pan.y),
        "pan-left" => assert_eq!(-50.0, state.pan.x),
        "pan-right" => assert_eq!(50.0, state.pan.x),
        "zoom-in" => assert!(state.zoom > 1.0, "zoom-in must enlarge diagram"),
        "zoom-out" => assert!(state.zoom < 1.0, "zoom-out must shrink diagram"),
        "reset-view" => assert_eq!(DiagramViewportState::default(), state),
        "trackpad-help" => assert!(state.help_requested, "help must open help state"),
        other => return Err(format!("unknown diagram control action: {other}").into()),
    }
    Ok(())
}

fn assert_diagram_fullscreen_overlay_dispatch(
    action: &str,
    storybook: &super::StorybookWindow,
) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("diagram", storybook.last_command_label);
    let state = only_diagram_viewport(storybook)?;
    assert!(
        state.fullscreen_open,
        "fullscreen overlay control must keep fullscreen open: {action}"
    );
    match action {
        "pan-up" => assert_eq!(-50.0, state.pan.y),
        "pan-down" => assert_eq!(50.0, state.pan.y),
        "pan-left" => assert_eq!(-50.0, state.pan.x),
        "pan-right" => assert_eq!(50.0, state.pan.x),
        "zoom-in" => assert!(state.zoom > 1.0, "zoom-in must enlarge fullscreen diagram"),
        "zoom-out" => assert!(state.zoom < 1.0, "zoom-out must shrink fullscreen diagram"),
        "reset-view" => assert_eq!(
            DiagramViewportState {
                fullscreen_open: true,
                ..DiagramViewportState::default()
            },
            state
        ),
        "trackpad-help" => assert!(state.help_requested, "help must open help state"),
        other => return Err(format!("unknown fullscreen diagram action: {other}").into()),
    }
    Ok(())
}

fn click_diagram_control_from_current_scene(
    storybook: &mut super::StorybookWindow,
    action: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let scene = storybook
        .scene
        .clone()
        .ok_or_else(|| std::io::Error::other("diagram scene missing"))?;
    let fullscreen_open = storybook
        .diagram_viewports
        .values()
        .any(|state| state.fullscreen_open);
    let (pointer, scroll_y) = if fullscreen_open {
        let pointer = if action == "fullscreen" {
            pointer_for_fullscreen_close_in_viewport(&scene)?
        } else {
            pointer_for_internal_diagram_action_in_viewport(&scene, action)?
        };
        (pointer, 0.0)
    } else {
        let hit = pointer_for_media_action(&scene, action)?;
        (hit.pointer, hit.scroll_y)
    };
    storybook.scroll_y = scroll_y;
    assert_eq!(
        UiCursor::Pointer,
        storybook.cursor_for_canvas_point(pointer.x, pointer.y, WINDOW_WIDTH, WINDOW_HEIGHT),
        "{action} must keep KUC pointer hit during continuous interaction"
    );
    assert!(
        storybook.apply_canvas_click(pointer, WINDOW_WIDTH, WINDOW_HEIGHT)?,
        "{action} must dispatch from the KUC hit path"
    );
    Ok(())
}

fn first_accordion_override_key(
    storybook: &super::StorybookWindow,
) -> Result<String, Box<dyn std::error::Error>> {
    storybook
        .accordion_open_overrides
        .keys()
        .next()
        .cloned()
        .ok_or_else(|| "accordion override missing".into())
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

fn max_image_surface_display_width_milli(node: &UiNode) -> Option<u32> {
    let current = if node.kind() == UiNodeKind::ImageSurface {
        Some(node.props().image_surface.display_width_milli)
    } else {
        None
    };
    node.children()
        .iter()
        .filter_map(max_image_surface_display_width_milli)
        .chain(current)
        .max()
}

fn visible_fullscreen_pixels(canvas: &crate::canvas::Canvas) -> usize {
    let background = canvas.pixels().first().copied().unwrap_or_default();
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel != background)
        .count()
}

fn image_surface_debug_values(node: &UiNode) -> Vec<String> {
    let mut values = Vec::new();
    collect_image_surface_debug_values(node, &mut values);
    values
}

fn collect_image_surface_debug_values(node: &UiNode, values: &mut Vec<String>) {
    if node.kind() == UiNodeKind::ImageSurface {
        values.push(format!(
            "{}:{}:{}",
            node.id().as_str(),
            node.props().state_id.as_str(),
            node.props().image_surface.display_width_milli
        ));
    }
    for child in node.children() {
        collect_image_surface_debug_values(child, values);
    }
}

fn media_host_action_debug_values(tree: &katana_ui_core::render_model::UiTree) -> Vec<String> {
    katana_ui_core::render_model::UiHostActionPlan::collect_from_tree(tree)
        .into_iter()
        .filter_map(|plan| {
            crate::media_host_action::StorybookMediaHostAction::from_host_action_plan(&plan).map(
                |action| {
                    let viewer_action = action.into_viewer_action();
                    format!("{}:{}", viewer_action.node_id, viewer_action.command)
                },
            )
        })
        .collect()
}

fn assert_fullscreen_close_hit_uses_katana_contract(
    scene: &PreviewScene,
) -> Result<katana_ui_core_storybook::UiTreeHostActionHit, Box<dyn std::error::Error>> {
    assert_fullscreen_close_hit_uses_katana_contract_for_size(scene, WINDOW_WIDTH)
}

fn assert_fullscreen_close_hit_uses_katana_contract_for_size(
    scene: &PreviewScene,
    window_width: usize,
) -> Result<katana_ui_core_storybook::UiTreeHostActionHit, Box<dyn std::error::Error>> {
    let hit = fullscreen_action_hits(scene, window_width)
        .into_iter()
        .min_by_key(|hit| hit.rect.area())
        .ok_or_else(|| std::io::Error::other("fullscreen close host action missing"))?;
    assert_eq!(window_width - 20 - 32, hit.rect.x, "fullscreen close x");
    assert_eq!(20, hit.rect.y, "fullscreen close y");
    assert_eq!(32, hit.rect.width, "fullscreen close width");
    assert_eq!(32, hit.rect.height, "fullscreen close height");
    Ok(hit)
}

fn pointer_for_fullscreen_close_in_viewport(
    scene: &PreviewScene,
) -> Result<StorybookPointer, Box<dyn std::error::Error>> {
    let hit = assert_fullscreen_close_hit_uses_katana_contract(scene)?;
    let (document_x, document_y) = hit.center_point();
    Ok(pointer_for_fullscreen_viewport_document_point(
        document_x, document_y,
    ))
}

fn pointer_for_internal_diagram_action_in_viewport(
    scene: &PreviewScene,
    action: &str,
) -> Result<StorybookPointer, Box<dyn std::error::Error>> {
    let router = StorybookHostActionRouter::for_window(scene, WINDOW_WIDTH);
    let (document_x, document_y) = router
        .internal_diagram_point_for_action(action)
        .ok_or_else(|| std::io::Error::other("missing fullscreen internal diagram hit"))?;
    Ok(pointer_for_fullscreen_viewport_document_point(
        document_x, document_y,
    ))
}

fn pointer_for_fullscreen_viewport_document_point(
    document_x: f32,
    document_y: f32,
) -> StorybookPointer {
    let (canvas_x, canvas_y) = fullscreen_canvas_point_for_document_point(document_x, document_y);
    StorybookPointer::new(canvas_x, canvas_y, StorybookMouseButton::Left)
}

fn fullscreen_canvas_point_for_document_point(document_x: f32, document_y: f32) -> (f32, f32) {
    (document_x, document_y)
}

fn assert_fullscreen_backdrop_hit_uses_katana_contract(
    scene: &PreviewScene,
) -> Result<katana_ui_core_storybook::UiTreeHostActionHit, Box<dyn std::error::Error>> {
    let hit = fullscreen_action_hits(scene, WINDOW_WIDTH)
        .into_iter()
        .max_by_key(|hit| hit.rect.area())
        .ok_or_else(|| std::io::Error::other("fullscreen backdrop host action missing"))?;
    assert_eq!(0, hit.rect.x, "fullscreen backdrop x");
    assert_eq!(0, hit.rect.y, "fullscreen backdrop y");
    assert_eq!(WINDOW_WIDTH, hit.rect.width, "fullscreen backdrop width");
    assert_eq!(WINDOW_HEIGHT, hit.rect.height, "fullscreen backdrop height");
    Ok(hit)
}

fn fullscreen_action_hits(
    scene: &PreviewScene,
    preview_width: usize,
) -> Vec<katana_ui_core_storybook::UiTreeHostActionHit> {
    StorybookHostActionHits::hits_for_preview_width(scene, preview_width)
        .into_iter()
        .filter(|hit| {
            crate::media_host_action::StorybookMediaHostAction::from_host_action_plan(&hit.action)
                .is_some_and(|action| action.into_viewer_action().command == "fullscreen")
        })
        .collect()
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
    let max_x = x.saturating_add(width).min(left.width());
    let max_y = y.saturating_add(height).min(left.height());
    let mut diff = 0;
    for y in y..max_y {
        for x in x..max_x {
            let index = y * left.width() + x;
            if left.pixels()[index] != right.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}

fn pixel_diff_count_in_radius(
    left: &crate::canvas::Canvas,
    right: &crate::canvas::Canvas,
    center_x: usize,
    center_y: usize,
    radius: usize,
) -> usize {
    let width = left.width();
    let height = left.height();
    let min_x = center_x.saturating_sub(radius);
    let min_y = center_y.saturating_sub(radius);
    let max_x = (center_x.saturating_add(radius)).min(width.saturating_sub(1));
    let max_y = (center_y.saturating_add(radius)).min(height.saturating_sub(1));

    let mut diff = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let index = y * width + x;
            if left.pixels()[index] != right.pixels()[index] {
                diff += 1;
            }
        }
    }
    diff
}
