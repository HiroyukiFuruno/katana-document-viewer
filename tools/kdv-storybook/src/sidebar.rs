use crate::catalog::StorybookFixture;
use crate::preview::PreviewScene;
use crate::sidebar_settings_state::StorybookSettingsState;
use katana_document_viewer::{ViewerInteractionConfig, ViewerTypographyConfig};
use katana_ui_core::layout::{Column, ScrollArea, ScrollAxis};
use katana_ui_core::molecule::{FileTree, FileTreeItem, FileTreeState, SettingsList};
use katana_ui_core::render_model::UiTree;

#[path = "sidebar_settings.rs"]
pub(crate) mod sidebar_settings;

pub struct StorybookSidebar;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StorybookSidebarScroll {
    pub(crate) tree_y: u32,
    pub(crate) settings_y: u32,
}

pub(crate) struct StorybookSidebarRequest<'a> {
    pub(crate) fixtures: &'a [StorybookFixture],
    pub(crate) selected_index: usize,
    pub(crate) scene: Option<&'a PreviewScene>,
    pub(crate) dark: bool,
    pub(crate) interaction: &'a ViewerInteractionConfig,
    pub(crate) typography: ViewerTypographyConfig,
    pub(crate) file_tree_state: FileTreeState,
    pub(crate) settings_state: &'a StorybookSettingsState,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) preview_width: usize,
    pub(crate) preview_height: usize,
    pub(crate) scroll: StorybookSidebarScroll,
}

pub(crate) struct SettingsListHitContractRequest<'a> {
    pub(crate) scene: Option<&'a PreviewScene>,
    pub(crate) dark: bool,
    pub(crate) interaction: &'a ViewerInteractionConfig,
    pub(crate) typography: ViewerTypographyConfig,
    pub(crate) settings_state: &'a StorybookSettingsState,
    pub(crate) preview_width: usize,
    pub(crate) preview_height: usize,
}

pub(crate) struct StorybookSidebarBoundsRequest<'a> {
    pub(crate) fixtures: &'a [StorybookFixture],
    pub(crate) selected_index: usize,
    pub(crate) scene: Option<&'a PreviewScene>,
    pub(crate) dark: bool,
    pub(crate) interaction: &'a ViewerInteractionConfig,
    pub(crate) typography: ViewerTypographyConfig,
    pub(crate) file_tree_state: &'a FileTreeState,
    pub(crate) settings_state: &'a StorybookSettingsState,
    pub(crate) height: usize,
    pub(crate) preview_width: usize,
    pub(crate) preview_height: usize,
    pub(crate) scroll: StorybookSidebarScroll,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StorybookSidebarPaneBounds {
    pub(crate) offset_y: f32,
    pub(crate) viewport_height: f32,
    pub(crate) content_height: f32,
}

impl StorybookSidebar {
    pub(crate) fn render(request: StorybookSidebarRequest<'_>) -> UiTree {
        let top_height = split_height(request.height);
        let bottom_height = request.height.saturating_sub(top_height);
        let tree = file_tree(
            request.fixtures,
            request.selected_index,
            &request.file_tree_state,
            request.width,
            top_height,
            request.scroll.tree_y,
        );
        let mut settings = settings_list(StorybookSettingsListRequest {
            scene: request.scene,
            dark: request.dark,
            interaction: request.interaction,
            typography: request.typography,
            settings_state: request.settings_state,
            width: request.width,
            height: bottom_height,
            preview_width: request.preview_width,
            preview_height: request.preview_height,
            offset_y: request.scroll.settings_y,
        });
        if let Some(hovered_id) = request.settings_state.hovered_node_id() {
            settings = settings.with_hovered_node_id(Some(&hovered_id));
        }
        UiTree::new(
            Column::new()
                .child(tree.root().clone())
                .child(settings.root().clone()),
        )
    }

    pub(crate) fn settings_content_height(list: &SettingsList, viewport_height: usize) -> usize {
        viewport_height.max(list.content_height() as usize)
    }

    pub(crate) fn settings_list_for_hit_contract(
        request: SettingsListHitContractRequest<'_>,
    ) -> SettingsList {
        sidebar_settings::settings_list(
            request.scene,
            request.dark,
            request.interaction,
            request.typography,
            request.settings_state,
            request.preview_width,
            request.preview_height,
        )
    }

    pub(crate) fn scroll_bounds(
        request: StorybookSidebarBoundsRequest<'_>,
    ) -> (StorybookSidebarPaneBounds, StorybookSidebarPaneBounds) {
        let top_height = split_height(request.height);
        let bottom_height = request.height.saturating_sub(top_height);
        let selected = &request.fixtures[request.selected_index].label;
        let items = fixture_items(request.fixtures, selected);
        let tree_content_height = FileTree::content_height_with_state(
            &items,
            top_height.max(1) as u32,
            request.file_tree_state,
        );
        let list = sidebar_settings::settings_list(
            request.scene,
            request.dark,
            request.interaction,
            request.typography,
            request.settings_state,
            request.preview_width,
            request.preview_height,
        );
        let settings_content_height = Self::settings_content_height(&list, bottom_height);
        (
            StorybookSidebarPaneBounds {
                offset_y: clamped_offset(request.scroll.tree_y, tree_content_height, top_height),
                viewport_height: top_height.max(1) as f32,
                content_height: tree_content_height as f32,
            },
            StorybookSidebarPaneBounds {
                offset_y: clamped_offset(
                    request.scroll.settings_y,
                    settings_content_height as u32,
                    bottom_height,
                ),
                viewport_height: bottom_height.max(1) as f32,
                content_height: settings_content_height.max(1) as f32,
            },
        )
    }
}

struct StorybookSettingsListRequest<'a> {
    scene: Option<&'a PreviewScene>,
    dark: bool,
    interaction: &'a ViewerInteractionConfig,
    typography: ViewerTypographyConfig,
    settings_state: &'a StorybookSettingsState,
    width: usize,
    height: usize,
    preview_width: usize,
    preview_height: usize,
    offset_y: u32,
}

pub(crate) fn file_tree(
    fixtures: &[StorybookFixture],
    selected_index: usize,
    state: &FileTreeState,
    width: usize,
    height: usize,
    offset_y: u32,
) -> UiTree {
    let selected = &fixtures[selected_index].label;
    FileTree::render_with_state_and_offset(
        &fixture_items(fixtures, selected),
        selected,
        width as u32,
        (height).max(1) as u32,
        offset_y,
        state,
    )
}

fn settings_list(request: StorybookSettingsListRequest<'_>) -> UiTree {
    let list = sidebar_settings::settings_list(
        request.scene,
        request.dark,
        request.interaction,
        request.typography,
        request.settings_state,
        request.preview_width,
        request.preview_height,
    );
    settings_list_tree(&list, request.width, request.height, request.offset_y)
}

pub(crate) fn settings_list_tree(
    list: &SettingsList,
    width: usize,
    height: usize,
    offset_y: u32,
) -> UiTree {
    let content_height = StorybookSidebar::settings_content_height(list, height);
    UiTree::new(
        ScrollArea::new()
            .axis(ScrollAxis::Vertical)
            .viewport(width as u32, height.max(1) as u32)
            .content_extent(width as u32, content_height.max(1) as u32)
            .offset(0, offset_y)
            .child(list.clone()),
    )
}

pub(crate) fn split_height(height: usize) -> usize {
    (height / 2).max(1)
}

fn clamped_offset(offset_y: u32, content_height: u32, viewport_height: usize) -> f32 {
    let max_offset = content_height.saturating_sub(viewport_height as u32);
    offset_y.min(max_offset) as f32
}

pub(crate) fn fixture_items(fixtures: &[StorybookFixture], selected: &str) -> Vec<FileTreeItem> {
    fixtures
        .iter()
        .filter(|fixture| should_show_fixture(&fixture.label) || fixture.label == selected)
        .map(|fixture| {
            FileTreeItem::new(fixture.label.clone(), fixture_tree_label(&fixture.label))
                .icon(fixture_icon(&fixture.label))
        })
        .collect()
}

fn fixture_icon(label: &str) -> &'static str {
    match extension(label) {
        "markdown" | "md" | "txt" => "markdown",
        "bmp" | "gif" | "jpeg" | "jpg" | "png" | "svg" | "webp" => "image",
        _ => "document",
    }
}

fn should_show_fixture(label: &str) -> bool {
    if !label.starts_with("direct/") {
        return true;
    }
    matches!(
        label,
        "direct/html-alignment.html"
            | "direct/kdv-icon.png"
            | "direct/sample.drawio"
            | "direct/sample.mmd"
            | "direct/sample.puml"
    )
}

fn fixture_tree_label(label: &str) -> String {
    let Some((category, rest)) = label.split_once('/') else {
        return label.to_string();
    };
    match category {
        "direct" => format!("direct/{}/{}", direct_fixture_group(rest), rest),
        "katana" => katana_fixture_label(rest),
        _ => label.to_string(),
    }
}

fn katana_fixture_label(rest: &str) -> String {
    if rest.starts_with("sample_html") {
        return format!("katana/html/{rest}");
    }
    if rest.starts_with("sample_diagrams") || rest.starts_with("sample_mermaid") {
        return format!("katana/diagram/{rest}");
    }
    if rest.starts_with("sample") {
        return format!("katana/markdown/{rest}");
    }
    format!("katana/{rest}")
}

fn direct_fixture_group(path: &str) -> &'static str {
    match extension(path) {
        "htm" | "html" => "html",
        "bmp" | "gif" | "jpeg" | "jpg" | "png" | "svg" | "webp" => "image",
        "drawio" | "drowio" | "mermaid" | "mmd" | "plantuml" | "puml" => "diagram",
        "markdown" | "md" | "txt" => "markdown",
        _ => "other",
    }
}

fn extension(path: &str) -> &str {
    path.rsplit_once('.')
        .map(|(_, value)| value)
        .unwrap_or_default()
}

#[cfg(test)]
#[path = "sidebar_tests.rs"]
mod tests;
