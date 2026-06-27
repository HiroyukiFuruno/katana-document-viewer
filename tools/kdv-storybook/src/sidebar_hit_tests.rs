use super::{SidebarHit, SidebarHitRequest, SidebarHitResult};
use crate::canvas::{Canvas, SurfaceArea};
use crate::catalog::StorybookFixture;
use crate::frame_ui_surface::render_ui_tree_for_theme_flag;
use crate::layout::{
    SIDEBAR_CONTENT_INSET, SIDEBAR_WIDTH, sidebar_content_height, sidebar_content_width,
    sidebar_content_x,
};
use crate::sidebar::{StorybookSidebar, StorybookSidebarRequest, StorybookSidebarScroll};
use crate::sidebar_test_support::{
    StorybookFileTreeItemPointRequest, StorybookSettingsFieldHitRequest,
    StorybookSettingsHitTarget, StorybookSettingsSectionHitRequest,
};
use katana_document_viewer::{ViewerInteractionConfig, ViewerTypographyConfig};
use katana_ui_core::molecule::{
    FileTree, FileTreeAction, FileTreeState, SettingsListAction, SettingsValue,
};
use katana_ui_core::render_model::UiHostActionPlan;
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::UiTreeStorybookHost;
use std::collections::HashSet;
use std::path::PathBuf;

const DARK_ACCENT: u32 = 0x569cd6;
const DARK_SELECTION: u32 = 0x264f78;
const MIN_TOGGLE_TRACK_PIXELS_PER_ROW: usize = 20;
const MIN_FILE_ROW_SELECTION_PIXELS_PER_ROW: usize = 80;

#[test]
fn sidebar_hit_selects_kuc_file_tree_fixture() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let (x, y) = file_tree_item_canvas_point(&fixtures, &interaction, "direct/sample.md", 900);
    let hit = SidebarHit::hit(x, y, request(&fixtures, &interaction, 0, 900));

    assert_eq!(
        Some(SidebarHitResult::FileTree(FileTreeAction::SelectFile {
            file_id: "direct/sample.md".to_string()
        })),
        hit
    );
}

#[test]
fn sidebar_hit_returns_kuc_file_tree_directory_toggle() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let (x, y) = file_tree_item_canvas_point(&fixtures, &interaction, "direct", 900);
    let hit = SidebarHit::hit(x, y, request(&fixtures, &interaction, 0, 900));

    assert_eq!(
        Some(SidebarHitResult::FileTree(
            FileTreeAction::ToggleDirectory {
                directory_id: "direct".to_string()
            }
        )),
        hit
    );
}

#[test]
fn sidebar_hit_selects_rendered_kuc_file_row_center() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let (x, y) = rendered_file_row_canvas_point_at_scale(&fixtures, &interaction, 900, 2.0);
    let hit = SidebarHit::hit(x, y, request(&fixtures, &interaction, 0, 900));

    assert_eq!(
        Some(SidebarHitResult::FileTree(FileTreeAction::SelectFile {
            file_id: "direct/sample.md".to_string()
        })),
        hit
    );
}

#[test]
fn sidebar_hit_selects_rendered_kuc_file_row_host_action_rect() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let (x, y) = rendered_host_action_canvas_point(&fixtures, &interaction, 900, |plan| {
        matches!(
            FileTree::action_from_host_plan(plan),
            Some(FileTreeAction::SelectFile { file_id }) if file_id == "direct/sample.md"
        )
    });
    let hit = SidebarHit::hit(x, y, request(&fixtures, &interaction, 0, 900));

    assert_eq!(
        Some(SidebarHitResult::FileTree(FileTreeAction::SelectFile {
            file_id: "direct/sample.md".to_string()
        })),
        hit
    );
}

#[test]
fn sidebar_hit_selects_retina_presented_file_row_without_click_offset() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let (logical_x, logical_y) =
        rendered_file_row_canvas_point_at_scale(&fixtures, &interaction, 900, 2.0);
    let physical_x = logical_x * 2.0;
    let physical_y = logical_y * 2.0;
    let canvas_point = crate::window_coordinates::window_point_to_canvas_point(
        crate::window_coordinates::WindowPoint::new(physical_x, physical_y),
        crate::window_coordinates::SurfaceSize::new(SIDEBAR_WIDTH * 2, 900 * 2),
        crate::window_coordinates::SurfaceSize::new(SIDEBAR_WIDTH, 900),
    );
    assert!(
        canvas_point.is_some(),
        "Retina sidebar physical point must normalize into logical canvas point"
    );
    let Some(canvas_point) = canvas_point else {
        return;
    };
    let hit = SidebarHit::hit(
        canvas_point.x as f32,
        canvas_point.y as f32,
        request(&fixtures, &interaction, 0, 900),
    );

    assert_eq!(
        Some(SidebarHitResult::FileTree(FileTreeAction::SelectFile {
            file_id: "direct/sample.md".to_string()
        })),
        hit
    );
}

#[test]
fn sidebar_hit_selects_large_window_file_and_setting_without_click_offset() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let height = 1496;
    let (file_x, file_y) =
        rendered_host_action_canvas_point(&fixtures, &interaction, height, |plan| {
            matches!(
                FileTree::action_from_host_plan(plan),
                Some(FileTreeAction::SelectFile { file_id }) if file_id == "direct/sample.md"
            )
        });
    let file_hit = SidebarHit::hit(file_x, file_y, request(&fixtures, &interaction, 0, height));

    assert_eq!(
        Some(SidebarHitResult::FileTree(FileTreeAction::SelectFile {
            file_id: "direct/sample.md".to_string()
        })),
        file_hit
    );

    let (setting_x, setting_y) =
        rendered_settings_host_action_canvas_point(&fixtures, &interaction, height, "hover");
    let setting_hit = SidebarHit::hit(
        setting_x,
        setting_y,
        request(&fixtures, &interaction, 0, height),
    );

    assert!(
        matches!(
            setting_hit,
            Some(SidebarHitResult::SettingsAction(SettingsListAction::UpdateField {
                ref field_id,
                ..
            })) if field_id == "hover"
        ),
        "large window settings click did not resolve KUC SettingsList action: {setting_hit:?}"
    );
}

#[test]
fn sidebar_hit_rejects_clicks_outside_rendered_sidebar_content_inset() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();

    assert_eq!(
        None,
        SidebarHit::hit(
            SIDEBAR_CONTENT_INSET.saturating_sub(1) as f32,
            33.0,
            request(&fixtures, &interaction, 0, 120)
        )
    );
    assert_eq!(
        None,
        SidebarHit::hit(
            SIDEBAR_WIDTH.saturating_sub(SIDEBAR_CONTENT_INSET / 2) as f32,
            33.0,
            request(&fixtures, &interaction, 0, 120)
        )
    );
    assert_eq!(
        None,
        SidebarHit::hit(24.0, 2.0, request(&fixtures, &interaction, 0, 120))
    );
    assert_eq!(
        None,
        SidebarHit::hit(24.0, 118.0, request(&fixtures, &interaction, 0, 120))
    );
}

#[test]
fn sidebar_hit_selects_each_file_under_categories() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let selected = ["direct/sample.md", "katana/html-alignment.htm"]
        .into_iter()
        .filter_map(|item_id| {
            let (x, y) = file_tree_item_canvas_point(&fixtures, &interaction, item_id, 900);
            match SidebarHit::hit(x, y, request(&fixtures, &interaction, 0, 900)) {
                Some(SidebarHitResult::FileTree(FileTreeAction::SelectFile { file_id })) => {
                    fixtures.iter().position(|fixture| fixture.label == file_id)
                }
                _ => None,
            }
        })
        .collect::<HashSet<_>>();

    assert!(selected.contains(&0));
    assert!(selected.contains(&1));
}

#[test]
fn sidebar_hit_selects_kuc_settings_toggle_field() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_update_target(&fixtures, &interaction, "dark", 900);
    let hit = settings_field_hit_at(
        &fixtures,
        &interaction,
        target_canvas_x(&target),
        target_canvas_y(&target),
        900,
    );

    assert_eq!(Some(SettingsValue::Bool(false)), hit);
}

#[test]
fn sidebar_hit_selects_rendered_kuc_toggle_track_center() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let (x, y) = rendered_toggle_track_canvas_point(&fixtures, &interaction, 900);
    let hit = settings_field_hit_at(&fixtures, &interaction, x, y, 900);

    assert_eq!(Some(SettingsValue::Bool(false)), hit);
}

#[test]
fn sidebar_hit_selects_retina_rendered_kuc_toggle_track_center() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let (x, y) = rendered_toggle_track_canvas_point_at_scale(&fixtures, &interaction, 900, 2.0);
    let hit = settings_field_hit_at(&fixtures, &interaction, x, y, 900);

    assert_eq!(Some(SettingsValue::Bool(false)), hit);
}

#[test]
fn sidebar_hit_selects_rendered_kuc_toggle_host_action_rect() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let settings = StorybookSidebar::settings_list_for_hit_contract(
        crate::sidebar::SettingsListHitContractRequest {
            scene: None,
            dark: true,
            interaction: &interaction,
            typography: ViewerTypographyConfig::default(),
            settings_state: &Default::default(),
            preview_width: 1000,
            preview_height: 868,
        },
    );
    let (x, y) = rendered_host_action_canvas_point(&fixtures, &interaction, 900, |plan| {
        matches!(
            settings.action_from_host_plan(plan),
            Some(SettingsListAction::UpdateField { field_id, .. }) if field_id == "dark"
        )
    });
    let hit = settings_field_hit_at(&fixtures, &interaction, x, y, 900);

    assert_eq!(Some(SettingsValue::Bool(false)), hit);
}

#[test]
fn sidebar_hit_does_not_use_pre_search_box_settings_row_offsets() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_section_target(&fixtures, &interaction, "display", 900);
    let hit = settings_field_hit_at(
        &fixtures,
        &interaction,
        target_canvas_x(&target),
        target_canvas_y(&target),
        900,
    );

    assert_eq!(None, hit);
}

#[test]
fn sidebar_hit_selects_kuc_settings_toggle_from_label_lane() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_update_target(&fixtures, &interaction, "dark", 900);
    let hit = settings_field_hit_at(
        &fixtures,
        &interaction,
        sidebar_content_x() as f32 + target.left + 1.0,
        target_canvas_y(&target),
        900,
    );

    assert_eq!(Some(SettingsValue::Bool(false)), hit);
}

#[test]
fn sidebar_hit_selects_kuc_settings_toggle_from_control_lane() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_update_target(&fixtures, &interaction, "dark", 900);
    let hit = settings_field_hit_at(
        &fixtures,
        &interaction,
        target_canvas_x(&target),
        target_canvas_y(&target),
        900,
    );

    assert_eq!(Some(SettingsValue::Bool(false)), hit);
}

#[test]
fn sidebar_hit_selects_kuc_settings_select_from_control_lane() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_update_target(&fixtures, &interaction, "theme", 900);
    let hit = match SidebarHit::hit(
        target_canvas_x(&target),
        target_canvas_y(&target),
        request(&fixtures, &interaction, 0, 900),
    ) {
        Some(SidebarHitResult::SettingsAction(SettingsListAction::UpdateField {
            field_id,
            ..
        })) => Some(field_id),
        _ => None,
    };

    assert_eq!(Some("theme".to_string()), hit);
}

#[test]
fn sidebar_hit_selects_kuc_settings_select_from_label_lane() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_update_target(&fixtures, &interaction, "theme", 900);
    let hit = match SidebarHit::hit(
        sidebar_content_x() as f32 + target.left + 1.0,
        target_canvas_y(&target),
        request(&fixtures, &interaction, 0, 900),
    ) {
        Some(SidebarHitResult::SettingsAction(SettingsListAction::UpdateField {
            field_id,
            ..
        })) => Some(field_id),
        _ => None,
    };

    assert_eq!(Some("theme".to_string()), hit);
}

#[test]
fn sidebar_hit_ignores_rendered_settings_search_box_row() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_section_target(&fixtures, &interaction, "display", 900);
    let hit = SidebarHit::hit(
        target_canvas_x(&target),
        SIDEBAR_CONTENT_INSET as f32 + target.top - 1.0,
        request(&fixtures, &interaction, 0, 900),
    );

    assert_eq!(None, hit);
}

#[test]
fn sidebar_hit_returns_kuc_settings_section_toggle() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_section_target(&fixtures, &interaction, "display", 900);
    let hit = match SidebarHit::hit(
        target_canvas_x(&target),
        target_canvas_y(&target),
        request(&fixtures, &interaction, 0, 900),
    ) {
        Some(SidebarHitResult::SettingsAction(SettingsListAction::ToggleSection {
            section_id,
        })) => Some(section_id),
        _ => None,
    };

    assert_eq!(Some("display".to_string()), hit);
}

#[test]
fn sidebar_hit_selects_kuc_settings_select_fields() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let fields = ["theme", "mode"]
        .into_iter()
        .filter_map(|field_id| {
            let target = settings_update_target(&fixtures, &interaction, field_id, 900);
            settings_update_field_at(&fixtures, &interaction, target, 900)
        })
        .collect::<HashSet<_>>();

    assert!(fields.contains("theme"));
    assert!(fields.contains("mode"));
}

#[test]
fn sidebar_hit_does_not_synthesize_readonly_settings_field_action() {
    let fixtures = fixtures();
    let interaction = ViewerInteractionConfig::default();
    let target = settings_field_target_option(&fixtures, &interaction, "viewport", 900);

    assert!(
        target.is_none(),
        "readonly state fields must not become synthesized KDV SettingsAction values"
    );
}

#[test]
fn sidebar_hit_selects_scrolled_file_tree_item() {
    let fixtures = (0..40)
        .map(|index| fixture(&format!("katana/group/file-{index}.md")))
        .collect::<Vec<_>>();
    let interaction = ViewerInteractionConfig::default();
    let scroll = StorybookSidebarScroll {
        tree_y: 560,
        settings_y: 0,
    };
    let (x, y) = file_tree_item_canvas_point_with_scroll(
        &fixtures,
        &interaction,
        "katana/group/file-25.md",
        scroll,
        900,
    );
    let selected = match SidebarHit::hit(
        x,
        y,
        request_with_scroll(&fixtures, &interaction, 0, scroll, 900),
    ) {
        Some(SidebarHitResult::FileTree(FileTreeAction::SelectFile { file_id })) => {
            fixtures.iter().position(|fixture| fixture.label == file_id)
        }
        _ => None,
    };

    assert!(
        matches!(selected, Some(index) if index >= 20),
        "scrolled FileTree did not expose lower fixture items: {selected:?}"
    );
}

#[test]
fn sidebar_hit_uses_rendered_kuc_host_actions_for_file_tree() {
    let source = include_str!("sidebar_hit.rs");

    assert!(source.contains("StorybookSidebar::render"));
    assert!(source.contains("UiTreeSurfaceHost"));
    assert!(source.contains("host_action_hits"));
    assert!(source.contains("FileTree::action_from_host_plan"));
    assert!(!source.contains("FileTreeHitTestInput"));
    assert!(!source.contains("hit_target_with_state"));
}

fn rendered_toggle_track_canvas_point(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    height: usize,
) -> (f32, f32) {
    rendered_toggle_track_canvas_point_at_scale(fixtures, interaction, height, 1.0)
}

fn rendered_toggle_track_canvas_point_at_scale(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    height: usize,
    scale: f32,
) -> (f32, f32) {
    let canvas = rendered_sidebar_canvas(fixtures, interaction, height, scale);
    let Some((local_x, local_y)) = rendered_toggle_track_center(&canvas) else {
        return missing_canvas_point("rendered KUC toggle track not found".to_string());
    };
    (
        sidebar_content_x() as f32 + local_x,
        SIDEBAR_CONTENT_INSET as f32 + local_y,
    )
}

fn rendered_sidebar_canvas(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    height: usize,
    scale: f32,
) -> Canvas {
    let content_height = sidebar_content_height(height);
    let content_width = sidebar_content_width();
    let tree = StorybookSidebar::render(StorybookSidebarRequest {
        fixtures,
        selected_index: 0,
        scene: None,
        dark: true,
        interaction,
        typography: ViewerTypographyConfig::default(),
        file_tree_state: FileTreeState::default(),
        settings_state: &Default::default(),
        width: content_width,
        height: content_height,
        preview_width: content_width,
        preview_height: content_height,
        scroll: StorybookSidebarScroll::default(),
    });
    let mut canvas = Canvas::new_scaled(content_width, content_height, scale, 0);
    render_ui_tree_for_theme_flag(
        &mut canvas,
        tree.root(),
        SurfaceArea {
            x: 0,
            y: 0,
            width: content_width,
            height: content_height,
            scroll_y: 0.0,
        },
        true,
    );
    canvas
}

fn rendered_host_action_canvas_point(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    height: usize,
    accepts: impl Fn(&UiHostActionPlan) -> bool,
) -> (f32, f32) {
    let content_height = sidebar_content_height(height);
    let content_width = sidebar_content_width();
    let tree = StorybookSidebar::render(StorybookSidebarRequest {
        fixtures,
        selected_index: 0,
        scene: None,
        dark: true,
        interaction,
        typography: ViewerTypographyConfig::default(),
        file_tree_state: FileTreeState::default(),
        settings_state: &Default::default(),
        width: content_width,
        height: content_height,
        preview_width: content_width,
        preview_height: content_height,
        scroll: StorybookSidebarScroll::default(),
    });
    let host = UiTreeStorybookHost::new(ThemeSnapshot::dark());
    let Some(hit) = host
        .host_action_hits(
            tree.root(),
            SurfaceArea {
                x: 0,
                y: 0,
                width: content_width,
                height: content_height,
                scroll_y: 0.0,
            },
        )
        .into_iter()
        .find(|hit| accepts(&hit.action))
    else {
        return missing_canvas_point("rendered KUC host action rect missing".to_string());
    };
    let (local_x, local_y) = hit.center_point();
    (
        sidebar_content_x() as f32 + local_x,
        SIDEBAR_CONTENT_INSET as f32 + local_y,
    )
}

fn rendered_settings_host_action_canvas_point(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    height: usize,
    field_id: &str,
) -> (f32, f32) {
    let content_height = sidebar_content_height(height);
    let content_width = sidebar_content_width();
    let settings_list = StorybookSidebar::settings_list_for_hit_contract(
        crate::sidebar::SettingsListHitContractRequest {
            scene: None,
            dark: true,
            interaction,
            typography: ViewerTypographyConfig::default(),
            settings_state: &Default::default(),
            preview_width: content_width,
            preview_height: content_height,
        },
    );
    rendered_host_action_canvas_point(fixtures, interaction, height, |plan| {
        matches!(
            settings_list.action_from_host_plan(plan),
            Some(SettingsListAction::UpdateField {
                field_id: ref candidate,
                ..
            }) if candidate == field_id
        )
    })
}

fn rendered_file_row_canvas_point_at_scale(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    height: usize,
    scale: f32,
) -> (f32, f32) {
    let canvas = rendered_sidebar_canvas(fixtures, interaction, height, scale);
    let Some((local_x, local_y)) = rendered_file_row_center(&canvas) else {
        return missing_canvas_point("rendered KUC selected file row not found".to_string());
    };
    (
        sidebar_content_x() as f32 + local_x,
        SIDEBAR_CONTENT_INSET as f32 + local_y,
    )
}

fn rendered_file_row_center(canvas: &Canvas) -> Option<(f32, f32)> {
    let top_y = first_file_row_selection_row(canvas)?;
    let bottom_y = top_y
        .saturating_add((24.0 * canvas.scale_factor()) as usize)
        .min(canvas.height());
    let (min_x, min_y, max_x, max_y) = color_bounds(canvas, DARK_SELECTION, top_y, bottom_y)?;
    Some((
        (min_x + max_x) as f32 / 2.0 / canvas.scale_factor(),
        (min_y + max_y) as f32 / 2.0 / canvas.scale_factor(),
    ))
}

fn first_file_row_selection_row(canvas: &Canvas) -> Option<usize> {
    let max_y = canvas.height() / 2;
    (0..max_y).find(|y| {
        let selection_pixels = (0..canvas.width())
            .filter(|x| physical_pixel(canvas, *x, *y) == DARK_SELECTION)
            .count();
        selection_pixels >= MIN_FILE_ROW_SELECTION_PIXELS_PER_ROW
    })
}

fn rendered_toggle_track_center(canvas: &Canvas) -> Option<(f32, f32)> {
    let top_y = first_toggle_track_row(canvas)?;
    let bottom_y = top_y
        .saturating_add((22.0 * canvas.scale_factor()) as usize)
        .min(canvas.height());
    let (min_x, min_y, max_x, max_y) = color_bounds(canvas, DARK_ACCENT, top_y, bottom_y)?;
    Some((
        (min_x + max_x) as f32 / 2.0 / canvas.scale_factor(),
        (min_y + max_y) as f32 / 2.0 / canvas.scale_factor(),
    ))
}

fn color_bounds(
    canvas: &Canvas,
    color: u32,
    top_y: usize,
    bottom_y: usize,
) -> Option<(usize, usize, usize, usize)> {
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut found = false;
    for y in top_y..bottom_y {
        for x in 0..canvas.width() {
            if physical_pixel(canvas, x, y) != color {
                continue;
            }
            found = true;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }
    found.then_some((min_x, min_y, max_x, max_y))
}

fn first_toggle_track_row(canvas: &Canvas) -> Option<usize> {
    let min_y = canvas.height() / 3;
    (min_y..canvas.height()).find(|y| {
        let accent_pixels = (0..canvas.width())
            .filter(|x| physical_pixel(canvas, *x, *y) == DARK_ACCENT)
            .count();
        accent_pixels >= MIN_TOGGLE_TRACK_PIXELS_PER_ROW
    })
}

fn physical_pixel(canvas: &Canvas, x: usize, y: usize) -> u32 {
    canvas.pixels()[y * canvas.width() + x]
}

fn settings_update_field_at(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    target: StorybookSettingsHitTarget,
    height: usize,
) -> Option<String> {
    match SidebarHit::hit(
        target_canvas_x(&target),
        target_canvas_y(&target),
        request(fixtures, interaction, 0, height),
    ) {
        Some(SidebarHitResult::SettingsAction(SettingsListAction::UpdateField {
            field_id,
            ..
        })) => Some(field_id),
        _ => None,
    }
}

fn settings_field_hit_at(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    pointer_x: f32,
    pointer_y: f32,
    height: usize,
) -> Option<SettingsValue> {
    match SidebarHit::hit(
        pointer_x,
        pointer_y,
        request(fixtures, interaction, 0, height),
    ) {
        Some(SidebarHitResult::SettingsAction(SettingsListAction::UpdateField {
            field_id,
            value,
        })) if field_id == "dark" => Some(value),
        _ => None,
    }
}

fn settings_update_target(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    expected_field_id: &str,
    height: usize,
) -> StorybookSettingsHitTarget {
    let target = settings_field_target(fixtures, interaction, expected_field_id, height);
    assert!(
        matches!(
            &target.action,
            Some(SettingsListAction::UpdateField { field_id, .. })
                if field_id == expected_field_id
        ),
        "settings field target action missing: {expected_field_id}"
    );
    target
}

fn settings_field_target(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    expected_field_id: &str,
    height: usize,
) -> StorybookSettingsHitTarget {
    let Some(target) =
        settings_field_target_option(fixtures, interaction, expected_field_id, height)
    else {
        return missing_settings_target(format!(
            "settings field target missing: {expected_field_id}"
        ));
    };
    target
}

fn settings_field_target_option(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    expected_field_id: &str,
    height: usize,
) -> Option<StorybookSettingsHitTarget> {
    let _ = fixtures;
    StorybookSidebar::settings_field_hit_target(StorybookSettingsFieldHitRequest {
        scene: None,
        dark: true,
        interaction,
        typography: ViewerTypographyConfig::default(),
        settings_state: &Default::default(),
        width: sidebar_content_width(),
        height: sidebar_content_height(height),
        scroll: StorybookSidebarScroll::default(),
        field_id: expected_field_id,
    })
}

fn settings_section_target(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    expected_section_id: &str,
    height: usize,
) -> StorybookSettingsHitTarget {
    let _ = fixtures;
    let Some(target) =
        StorybookSidebar::settings_section_hit_target(StorybookSettingsSectionHitRequest {
            scene: None,
            dark: true,
            interaction,
            typography: ViewerTypographyConfig::default(),
            settings_state: &Default::default(),
            width: sidebar_content_width(),
            height: sidebar_content_height(height),
            scroll: StorybookSidebarScroll::default(),
            section_id: expected_section_id,
        })
    else {
        return missing_settings_target(format!(
            "settings section target missing: {expected_section_id}"
        ));
    };
    target
}

fn missing_settings_target(message: String) -> StorybookSettingsHitTarget {
    assert!(message.is_empty(), "{message}");
    StorybookSettingsHitTarget {
        left: 0.0,
        top: 0.0,
        right: 0.0,
        center_x: 0.0,
        center_y: 0.0,
        action: None,
    }
}

fn missing_canvas_point(message: String) -> (f32, f32) {
    assert!(message.is_empty(), "{message}");
    (0.0, 0.0)
}

fn file_tree_item_canvas_point(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    item_id: &str,
    height: usize,
) -> (f32, f32) {
    file_tree_item_canvas_point_with_scroll(
        fixtures,
        interaction,
        item_id,
        StorybookSidebarScroll::default(),
        height,
    )
}

fn file_tree_item_canvas_point_with_scroll(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    item_id: &str,
    scroll: StorybookSidebarScroll,
    height: usize,
) -> (f32, f32) {
    let _ = interaction;
    let Some(point) =
        StorybookSidebar::fixture_canvas_point_for_item_id(StorybookFileTreeItemPointRequest {
            fixtures,
            selected_index: 0,
            state: &FileTreeState::default(),
            item_id,
            height: sidebar_content_height(height),
            scroll,
        })
    else {
        return missing_canvas_point(format!("file tree target missing: {item_id}"));
    };
    (point.x, point.y)
}

fn target_canvas_x(target: &StorybookSettingsHitTarget) -> f32 {
    target.canvas_point().x
}

fn target_canvas_y(target: &StorybookSettingsHitTarget) -> f32 {
    target.canvas_point().y
}

fn fixtures() -> Vec<StorybookFixture> {
    vec![
        fixture("direct/sample.md"),
        fixture("katana/html-alignment.htm"),
    ]
}

fn request<'a>(
    fixtures: &'a [StorybookFixture],
    interaction: &'a ViewerInteractionConfig,
    selected_index: usize,
    height: usize,
) -> SidebarHitRequest<'a> {
    request_with_scroll(
        fixtures,
        interaction,
        selected_index,
        StorybookSidebarScroll::default(),
        height,
    )
}

fn request_with_scroll<'a>(
    fixtures: &'a [StorybookFixture],
    interaction: &'a ViewerInteractionConfig,
    selected_index: usize,
    scroll: StorybookSidebarScroll,
    height: usize,
) -> SidebarHitRequest<'a> {
    SidebarHitRequest {
        fixtures,
        selected_index,
        scene: None,
        dark: true,
        interaction,
        typography: Default::default(),
        settings_state: Default::default(),
        file_tree_state: Default::default(),
        scroll,
        width: 1000,
        height,
    }
}

fn fixture(label: &str) -> StorybookFixture {
    StorybookFixture {
        label: label.to_string(),
        path: PathBuf::from(label),
    }
}
