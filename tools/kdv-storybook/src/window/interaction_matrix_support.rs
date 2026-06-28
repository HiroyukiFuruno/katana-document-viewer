use super::super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::layout::{sidebar_content_height, sidebar_content_width};
use crate::mouse::mouse_test_support::WINDOW_HEIGHT;
use crate::preview::PreviewBuilder;
use crate::settings_action::StorybookSettingsField;
use crate::sidebar::StorybookSidebar;
use crate::sidebar_test_support::{
    StorybookFileTreeItemPointRequest, StorybookSettingsFieldHitRequest,
    StorybookSettingsHitTarget, StorybookSettingsSectionHitRequest,
};
use std::path::Path;

thread_local! {
    static SHARED_PREVIEW_BUILDER: PreviewBuilder = PreviewBuilder::default();
}

pub(super) fn storybook_with_catalog() -> Result<StorybookWindow, Box<dyn std::error::Error>> {
    Ok(StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog::load(fixtures_root().as_path())?,
        shared_preview_builder(),
    ))
}

pub(super) fn storybook_with_minimal_catalog() -> StorybookWindow {
    let root = fixtures_root();
    StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![StorybookFixture {
                label: "katana/sample_basic.md".to_string(),
                path: root.join("katana/sample_basic.md"),
            }],
        },
        shared_preview_builder(),
    )
}

pub(super) fn storybook_with_label(
    label: &str,
) -> Result<StorybookWindow, Box<dyn std::error::Error>> {
    let mut catalog = FixtureCatalog::load(fixtures_root().as_path())?;
    let index = catalog
        .fixtures
        .iter()
        .position(|fixture| fixture.label == label)
        .ok_or_else(|| format!("fixture missing: {label}"))?;
    catalog.fixtures.swap(0, index);
    Ok(StorybookWindow::new(
        StorybookArgs::default(),
        catalog,
        shared_preview_builder(),
    ))
}

fn shared_preview_builder() -> PreviewBuilder {
    SHARED_PREVIEW_BUILDER.with(Clone::clone)
}

pub(super) fn settings_field_point(
    storybook: &StorybookWindow,
    field: StorybookSettingsField,
) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let point =
        StorybookSidebar::settings_field_canvas_point(settings_field_request(storybook, field))
            .ok_or_else(|| format!("settings field target missing: {}", field.id()))?;
    Ok((point.x, point.y))
}

pub(super) fn settings_field_target(
    storybook: &StorybookWindow,
    field: StorybookSettingsField,
) -> Result<StorybookSettingsHitTarget, Box<dyn std::error::Error>> {
    StorybookSidebar::settings_field_hit_target(settings_field_request(storybook, field))
        .ok_or_else(|| format!("settings field target missing: {}", field.id()).into())
}

pub(super) fn settings_section_point(
    storybook: &StorybookWindow,
    section_id: &str,
) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let point = StorybookSidebar::settings_section_canvas_point(settings_section_request(
        storybook, section_id,
    ))
    .ok_or_else(|| format!("settings section target missing: {section_id}"))?;
    Ok((point.x, point.y))
}

pub(super) fn settings_section_target(
    storybook: &StorybookWindow,
    section_id: &str,
) -> Result<StorybookSettingsHitTarget, Box<dyn std::error::Error>> {
    StorybookSidebar::settings_section_hit_target(settings_section_request(storybook, section_id))
        .ok_or_else(|| format!("settings section target missing: {section_id}").into())
}

fn settings_field_request(
    storybook: &StorybookWindow,
    field: StorybookSettingsField,
) -> StorybookSettingsFieldHitRequest<'_> {
    StorybookSettingsFieldHitRequest {
        scene: storybook.scene.as_ref(),
        dark: storybook.dark,
        interaction: &storybook.interaction,
        typography: storybook.typography,
        settings_state: &storybook.settings_state,
        width: sidebar_content_width(),
        height: sidebar_content_height(WINDOW_HEIGHT),
        scroll: storybook.sidebar_scroll,
        field_id: field.id(),
    }
}

fn settings_section_request<'a>(
    storybook: &'a StorybookWindow,
    section_id: &'a str,
) -> StorybookSettingsSectionHitRequest<'a> {
    StorybookSettingsSectionHitRequest {
        scene: storybook.scene.as_ref(),
        dark: storybook.dark,
        interaction: &storybook.interaction,
        typography: storybook.typography,
        settings_state: &storybook.settings_state,
        width: sidebar_content_width(),
        height: sidebar_content_height(WINDOW_HEIGHT),
        scroll: storybook.sidebar_scroll,
        section_id,
    }
}

pub(super) fn first_directory_hit(
    storybook: &StorybookWindow,
) -> Result<DirectoryHit, Box<dyn std::error::Error>> {
    let id = first_directory_id(storybook)?;
    let point = file_tree_item_point(storybook, id.as_str())?;
    Ok(DirectoryHit {
        id,
        x: point.0,
        y: point.1,
    })
}

pub(super) fn file_tree_item_point(
    storybook: &StorybookWindow,
    item_id: &str,
) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let point =
        StorybookSidebar::fixture_canvas_point_for_item_id(StorybookFileTreeItemPointRequest {
            fixtures: &storybook.catalog.fixtures,
            selected_index: storybook.selected_index,
            state: &storybook.file_tree_state,
            item_id,
            height: sidebar_content_height(WINDOW_HEIGHT),
            scroll: storybook.sidebar_scroll,
        })
        .ok_or_else(|| format!("file tree target missing: {item_id}"))?;
    Ok((point.x, point.y))
}

fn first_directory_id(storybook: &StorybookWindow) -> Result<String, Box<dyn std::error::Error>> {
    storybook
        .catalog
        .fixtures
        .first()
        .and_then(|fixture| fixture.label.split_once('/'))
        .map(|(directory, _)| directory.to_string())
        .ok_or_else(|| "directory id missing".into())
}

fn fixtures_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/fixtures")
}

pub(super) struct DirectoryHit {
    pub(super) id: String,
    pub(super) x: f32,
    pub(super) y: f32,
}
