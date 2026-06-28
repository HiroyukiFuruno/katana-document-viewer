use super::{SidebarHit, SidebarHitRequest, SidebarHitResult};
use crate::canvas::{Canvas, SurfaceArea};
use crate::catalog::StorybookFixture;
use crate::frame_ui_surface::render_ui_tree_for_theme_flag;
use crate::layout::{
    SIDEBAR_CONTENT_INSET, sidebar_content_height, sidebar_content_width, sidebar_content_x,
};
use crate::sidebar::{StorybookSidebar, StorybookSidebarRequest, StorybookSidebarScroll};
use katana_document_viewer::{ViewerInteractionConfig, ViewerTypographyConfig};
use katana_ui_core::molecule::{FileTreeState, SettingsListAction, SettingsValue};
use katana_ui_core::render_model::UiCursor;

const DARK_ACCENT: u32 = 0x569cd6;

#[test]
fn sidebar_hit_accepts_rendered_toggle_track_bounds_at_retina_scale() {
    let fixtures = vec![StorybookFixture {
        label: "direct/sample.md".to_string(),
        path: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/fixtures/direct/sample.md"),
    }];
    let interaction = ViewerInteractionConfig::default();
    let height = 900;
    let canvas = rendered_sidebar_canvas(&fixtures, &interaction, height, 2.0);
    let bounds = rendered_toggle_track_bounds(&canvas);

    for x in [bounds.min_x, bounds.max_x] {
        for y in [bounds.min_y, bounds.max_y] {
            let pointer_x = sidebar_content_x() as f32 + logical_inside(x, &canvas);
            let pointer_y = SIDEBAR_CONTENT_INSET as f32 + logical_inside(y, &canvas);
            let result = SidebarHit::interaction(
                pointer_x,
                pointer_y,
                request(&fixtures, &interaction, height),
            );

            assert_eq!(
                UiCursor::Pointer,
                result.cursor,
                "toggle rendered bound should expose KUC pointer cursor at {pointer_x},{pointer_y}"
            );
            assert!(result.hovered_settings_node_id.is_some());
            assert_eq!(
                Some(SettingsValue::Bool(false)),
                settings_bool_value(result.action),
                "toggle rendered bound should activate KUC SettingsList action at {pointer_x},{pointer_y}"
            );
        }
    }
}

fn rendered_sidebar_canvas(
    fixtures: &[StorybookFixture],
    interaction: &ViewerInteractionConfig,
    height: usize,
    scale: f32,
) -> Canvas {
    let width = sidebar_content_width();
    let content_height = sidebar_content_height(height);
    let tree = StorybookSidebar::render(StorybookSidebarRequest {
        fixtures,
        selected_index: 0,
        scene: None,
        dark: true,
        interaction,
        typography: ViewerTypographyConfig::default(),
        file_tree_state: FileTreeState::default(),
        settings_state: &Default::default(),
        width,
        height: content_height,
        preview_width: width,
        preview_height: content_height,
        scroll: StorybookSidebarScroll::default(),
    });
    let mut canvas = Canvas::new_scaled(width, content_height, scale, 0);
    render_ui_tree_for_theme_flag(
        &mut canvas,
        tree.root(),
        SurfaceArea {
            x: 0,
            y: 0,
            width,
            height: content_height,
            scroll_y: 0.0,
        },
        true,
    );
    canvas
}

fn rendered_toggle_track_bounds(canvas: &Canvas) -> PhysicalBounds {
    let min_y = canvas.height() / 3;
    let mut bounds = PhysicalBounds::empty();
    for y in min_y..canvas.height() {
        for x in 0..canvas.width() {
            if pixel(canvas, x, y) == DARK_ACCENT {
                bounds.include(x, y);
            }
        }
        if bounds.found && y.saturating_sub(bounds.min_y) > scaled_track_search_height(canvas) {
            break;
        }
    }
    assert!(bounds.found, "rendered KUC toggle track was not found");
    bounds
}

fn scaled_track_search_height(canvas: &Canvas) -> usize {
    (28.0 * canvas.scale_factor()).ceil() as usize
}

fn pixel(canvas: &Canvas, x: usize, y: usize) -> u32 {
    canvas.pixels()[y * canvas.width() + x]
}

fn logical_inside(physical: usize, canvas: &Canvas) -> f32 {
    (physical as f32 + 0.5) / canvas.scale_factor()
}

fn settings_bool_value(action: Option<SidebarHitResult>) -> Option<SettingsValue> {
    match action {
        Some(SidebarHitResult::SettingsAction(SettingsListAction::UpdateField {
            field_id,
            value,
        })) if field_id == "dark" => Some(value),
        _ => None,
    }
}

fn request<'a>(
    fixtures: &'a [StorybookFixture],
    interaction: &'a ViewerInteractionConfig,
    height: usize,
) -> SidebarHitRequest<'a> {
    SidebarHitRequest {
        fixtures,
        selected_index: 0,
        scene: None,
        dark: true,
        interaction,
        typography: ViewerTypographyConfig::default(),
        settings_state: Default::default(),
        file_tree_state: FileTreeState::default(),
        scroll: StorybookSidebarScroll::default(),
        width: 1000,
        height,
    }
}

#[derive(Debug, Clone, Copy)]
struct PhysicalBounds {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    found: bool,
}

impl PhysicalBounds {
    const fn empty() -> Self {
        Self {
            min_x: usize::MAX,
            min_y: usize::MAX,
            max_x: 0,
            max_y: 0,
            found: false,
        }
    }

    fn include(&mut self, x: usize, y: usize) {
        self.found = true;
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }
}
