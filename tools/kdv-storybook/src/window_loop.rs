#[cfg(test)]
use super::StorybookFrameCache;
use super::{StorybookError, StorybookWindow};
use crate::canvas::SurfaceArea;
use crate::frame::StorybookFrameRenderer;
use crate::frame_ui_surface::render_ui_tree_for_theme_flag;
use crate::layout::{
    SIDEBAR_CONTENT_INSET, StorybookPreviewArea, sidebar_content_height, sidebar_content_width,
    sidebar_content_x,
};
use crate::media_host_action::StorybookMediaHostAction;
use crate::mouse::{
    StorybookHostActionHits, StorybookHostActionRouter, StorybookMouseButton, StorybookPointer,
};
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use crate::settings_action::StorybookSettingsField;
use crate::sidebar::{StorybookSidebar, StorybookSidebarRequest};
use crate::sidebar_hit::{
    SidebarHit, SidebarHitRequest, SidebarHitResult, SidebarInteractionSurface,
};
use crate::window_host_event::StorybookHostEvent;
use katana_document_viewer::{ViewerMediaControlKind, ViewerMode, ViewerViewport};
use katana_ui_core::molecule::{FileTreeAction, SettingsListAction, SettingsValue};
use katana_ui_core::render_model::UiTextSpanAction;
use katana_ui_core_storybook::Canvas;
#[cfg(test)]
use katana_ui_core_storybook::StorybookPresentation;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::{
    env, thread,
    time::{Duration, Instant},
};

#[path = "window_loop_changes.rs"]
mod changes;
pub(crate) use changes::FrameLoopChanges;

const TITLE: &str = "KDV KUC Storybook";
const STORYBOOK_SCALE_ENV: &str = "KDV_STORYBOOK_SCALE";
const KUC_DARK_ACCENT: u32 = 0x569cd6;
const ACTIVE_FRAME_DELAY: Duration = Duration::from_millis(16);
const INTERACTION_FRAME_DELAY: Duration = Duration::from_millis(8);
const IDLE_FRAME_DELAY: Duration = Duration::from_millis(33);
const CODE_COPY_FIXTURE_LABEL: &str = "katana/sample_basic.md";
const CODE_COPY_ACTION_ID: &str = "copy-code";
const DIAGRAM_FIXTURE_LABEL: &str = "katana/sample_diagrams.md";
const DIAGRAM_SMOKE_DOCUMENT_HEIGHT: f32 = 20_000.0;
const DIAGRAM_SMOKE_ACTIONS: &[DiagramSmokeAction] = &[
    DiagramSmokeAction {
        command: "zoom-in",
        stage: "zoom-in",
        requires_frame_diff: true,
    },
    DiagramSmokeAction {
        command: "pan-right",
        stage: "pan-right",
        requires_frame_diff: true,
    },
    DiagramSmokeAction {
        command: "pan-down",
        stage: "pan-down",
        requires_frame_diff: true,
    },
    DiagramSmokeAction {
        command: "reset-view",
        stage: "reset-view",
        requires_frame_diff: true,
    },
    DiagramSmokeAction {
        command: "pan-left",
        stage: "pan-left",
        requires_frame_diff: false,
    },
    DiagramSmokeAction {
        command: "pan-up",
        stage: "pan-up",
        requires_frame_diff: false,
    },
    DiagramSmokeAction {
        command: "zoom-out",
        stage: "zoom-out",
        requires_frame_diff: false,
    },
    DiagramSmokeAction {
        command: "trackpad-help",
        stage: "trackpad-help",
        requires_frame_diff: true,
    },
    DiagramSmokeAction {
        command: "fullscreen",
        stage: "fullscreen",
        requires_frame_diff: true,
    },
];

struct DiagramSmokeAction {
    command: &'static str,
    stage: &'static str,
    requires_frame_diff: bool,
}

impl StorybookWindow {
    pub fn print_live_dark_toggle_point(mut self) -> Result<(), StorybookError> {
        let (width, height) = interactive_window_size(self.args.width, self.args.height);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        let (x, y) = self.live_dark_toggle_point_for_acceptance(width, height)?;
        println!("{x:.1} {y:.1}");
        Ok(())
    }

    pub fn run_live_acceptance_artifact(mut self) -> Result<(), StorybookError> {
        const ACCEPTANCE_SCALE: f32 = 2.0;

        let width = self.args.width;
        let height = self.args.height;
        let dark_output = self.args.screenshot_output.clone();
        let light_output = self.args.light_screenshot_output.clone();

        self.dark = true;
        self.update_frame_size(width, height);
        self.update_scene_loaded(width, height)?;
        let dark = self.render_canvas_scaled(width, height, ACCEPTANCE_SCALE);
        super::window_clipboard_smoke::write_canvas_png(&dark_output, &dark)?;

        let (x, y) = self.live_dark_toggle_point_for_acceptance(width, height)?;
        let pointer = StorybookPointer::new(x, y, StorybookMouseButton::Left);
        if !self.apply_canvas_click(pointer, width, height)? {
            return Err("live acceptance headless click did not dispatch".into());
        }
        if self.dark {
            return Err("live acceptance headless click did not switch dark off".into());
        }

        self.update_scene_loaded(width, height)?;
        let light = self.render_canvas_scaled(width, height, ACCEPTANCE_SCALE);
        super::window_clipboard_smoke::write_canvas_png(&light_output, &light)?;

        println!(
            "storybook live acceptance headless artifact ready width={width} height={height} scale={ACCEPTANCE_SCALE:.1} dark={} light={}",
            dark_output.display(),
            light_output.display()
        );
        println!(
            "storybook live acceptance clicked dark toggle at x={x:.1} y={y:.1} source=headless-kuc-action"
        );
        Ok(())
    }

    pub(crate) fn live_dark_toggle_point_for_acceptance(
        &self,
        width: usize,
        height: usize,
    ) -> Result<(f32, f32), StorybookError> {
        let canvas = self.render_sidebar_canvas_for_live_acceptance(width, height);
        let (x, y) = dark_toggle_track_center_from_sidebar_canvas(&canvas)?;
        let interaction = self.compute_sidebar_interaction_for_canvas_point(x, y, width, height);
        match interaction.action {
            Some(SidebarHitResult::SettingsAction(SettingsListAction::UpdateField {
                field_id,
                value: SettingsValue::Bool(false),
            })) if field_id == StorybookSettingsField::Dark.id() => Ok((x, y)),
            other => Err(format!(
                "live dark toggle point did not resolve to dark=false settings action: point=({x},{y}) action={other:?}"
            )
            .into()),
        }
    }

    fn render_sidebar_canvas_for_live_acceptance(&self, width: usize, height: usize) -> Canvas {
        let content_width = sidebar_content_width();
        let content_height = sidebar_content_height(height);
        let tree = StorybookSidebar::render(StorybookSidebarRequest {
            fixtures: &self.catalog.fixtures,
            selected_index: self.selected_index,
            scene: self.scene.as_ref(),
            dark: self.dark,
            interaction: &self.interaction,
            typography: self.typography,
            file_tree_state: self.file_tree_state.clone(),
            settings_state: &self.settings_state,
            width: content_width,
            height: content_height,
            preview_width: width,
            preview_height: height,
            scroll: self.sidebar_scroll,
        });
        let mut canvas = Canvas::new(content_width, content_height, 0);
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
            self.dark,
        );
        canvas
    }

    pub fn run(mut self) -> Result<(), StorybookError> {
        if !self.args.interactive {
            self.render_headless()?;
            return Ok(());
        }
        let mut window = self.create_window()?;
        self.run_window_loop(&mut window)?;
        Ok(())
    }

    pub(crate) fn run_window_selection_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let before = self.presented_frame_for_current_window(width, height)?;
        if !self.apply_text_selection_drag_for_smoke((0.0, 0.0), (width as f32, height as f32)) {
            return Err("window selection screenshot smoke drag path did not update".into());
        }
        self.update_window_buffer(&mut window, width, height, true)?;
        let after = self.presented_frame_for_current_window(width, height)?;
        if super::window_clipboard_smoke::pixel_diff_count(&before, &after) == 0 {
            return Err("window selection screenshot smoke did not change presented frame".into());
        }
        let payload = self
            .selected_text_payload(width, height)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| "window selection screenshot smoke payload missing".to_string())?;
        if !self.copy_selected_text_to_clipboard(width, height) {
            return Err("window selection screenshot smoke clipboard copy did not run".into());
        }
        let clipboard = super::window_command::read_clipboard_text()?;
        if clipboard != payload {
            return Err("window selection screenshot smoke clipboard payload mismatch".into());
        }
        super::window_clipboard_smoke::write_canvas_png(&self.args.screenshot_output, &after)?;
        println!(
            "storybook-window-selection-screenshot-smoke: ok path={} bytes={} lines={}",
            self.args.screenshot_output.display(),
            payload.len(),
            payload.lines().count()
        );
        Ok(())
    }

    pub(crate) fn run_window_hover_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        let fixture_label = self.args.window_smoke_fixture.clone();
        self.select_fixture_for_window_smoke(&fixture_label)?;
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        let (node_id, pointer) = self.document_hover_point_for_window_smoke(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let base = self.presented_frame_for_current_window(width, height)?;

        if !self.update_document_hover_for_canvas_point(pointer.x, pointer.y, width, height) {
            return Err(format!("window hover screenshot smoke target missing: {node_id}").into());
        }
        self.update_window_buffer(&mut window, width, height, true)?;
        let hovered = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &base,
            &hovered,
            "window hover screenshot smoke frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "hover",
            &hovered,
        )?;
        super::window_clipboard_smoke::write_canvas_png(&self.args.screenshot_output, &base)?;
        println!(
            "storybook-window-hover-screenshot-smoke: ok path={} node={node_id} stages=hover",
            self.args.screenshot_output.display()
        );
        Ok(())
    }

    pub(crate) fn run_window_footnote_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        self.select_fixture_for_window_smoke("katana/sample.md")?;
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        let (target_y, pointer) =
            self.footnote_reference_point_for_window_smoke("[1]", "#fn-1", width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let reference = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "reference",
            &reference,
        )?;

        if !self.apply_canvas_click(pointer, width, height)? {
            return Err("window footnote screenshot smoke click did not dispatch".into());
        }
        self.update_window_buffer(&mut window, width, height, true)?;
        let definition = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &reference,
            &definition,
            "window footnote screenshot smoke frame did not change",
        )?;
        self.require_document_y_visible_for_window_smoke(target_y, width, height)?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "definition",
            &definition,
        )?;
        super::window_clipboard_smoke::write_canvas_png(&self.args.screenshot_output, &definition)?;
        println!(
            "storybook-window-footnote-screenshot-smoke: ok path={} stages=reference,definition target_y={target_y:.1}",
            self.args.screenshot_output.display()
        );
        Ok(())
    }

    pub(crate) fn run_window_table_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        self.select_fixture_for_window_smoke("katana/sample.md")?;
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        let table_y = self.scroll_to_table_for_window_smoke(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let table = self.presented_frame_for_current_window(width, height)?;
        self.require_document_y_visible_for_window_smoke(table_y, width, height)?;
        super::window_clipboard_smoke::write_canvas_png(&self.args.screenshot_output, &table)?;
        println!(
            "storybook-window-table-screenshot-smoke: ok path={} table_y={table_y:.1}",
            self.args.screenshot_output.display()
        );
        Ok(())
    }

    pub(crate) fn run_window_code_copy_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        self.select_fixture_for_window_smoke(CODE_COPY_FIXTURE_LABEL)?;
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        self.scroll_to_media_action_for_window_smoke(
            ViewerMediaControlKind::Code,
            CODE_COPY_ACTION_ID,
            width,
            height,
        )?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let base = self.presented_frame_for_current_window(width, height)?;
        let pointer = self.host_media_action_pointer_for_window_smoke(
            ViewerMediaControlKind::Code,
            CODE_COPY_ACTION_ID,
            width,
            height,
        )?;

        if !self.update_document_hover_for_canvas_point(pointer.x, pointer.y, width, height) {
            return Err("window code copy screenshot smoke hover did not resolve action".into());
        }
        self.update_window_buffer(&mut window, width, height, true)?;
        let hovered = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &base,
            &hovered,
            "window code copy screenshot smoke hover frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "hover",
            &hovered,
        )?;

        if !self.apply_canvas_click(pointer, width, height)? {
            return Err("window code copy screenshot smoke click did not dispatch".into());
        }
        if self.copied_code_node_ids.is_empty() {
            return Err(
                "window code copy screenshot smoke did not record copied code target".into(),
            );
        }
        self.update_scene_for_refresh(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let copied = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &hovered,
            &copied,
            "window code copy screenshot smoke copied frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "copied",
            &copied,
        )?;
        super::window_clipboard_smoke::write_canvas_png(&self.args.screenshot_output, &base)?;
        println!(
            "storybook-window-code-copy-screenshot-smoke: ok path={} fixture={} stages=hover,copied copied_targets={}",
            self.args.screenshot_output.display(),
            CODE_COPY_FIXTURE_LABEL,
            self.copied_code_node_ids.len()
        );
        Ok(())
    }

    pub(crate) fn run_window_slideshow_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let document = self.presented_frame_for_current_window(width, height)?;

        if !self.apply_settings_field(
            crate::settings_action::StorybookSettingsField::Mode,
            width,
            height,
        )? {
            return Err("window slideshow screenshot smoke mode switch did not update".into());
        }
        self.update_scene_for_refresh(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let mode = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &document,
            &mode,
            "window slideshow screenshot smoke mode frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "mode",
            &mode,
        )?;

        self.apply_slideshow_control_for_smoke(
            katana_document_viewer::ViewerSlideshowControlAction::NextPage,
            width,
            height,
        )?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let next = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &mode,
            &next,
            "window slideshow screenshot smoke next frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "next",
            &next,
        )?;

        self.apply_slideshow_control_for_smoke(
            katana_document_viewer::ViewerSlideshowControlAction::PreviousPage,
            width,
            height,
        )?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let previous = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &next,
            &previous,
            "window slideshow screenshot smoke previous frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "previous",
            &previous,
        )?;

        self.apply_slideshow_control_for_smoke(
            katana_document_viewer::ViewerSlideshowControlAction::Close,
            width,
            height,
        )?;
        self.update_scene_for_refresh(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let closed = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &previous,
            &closed,
            "window slideshow screenshot smoke close frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "close",
            &closed,
        )?;
        super::window_clipboard_smoke::write_canvas_png(&self.args.screenshot_output, &closed)?;
        println!(
            "storybook-window-slideshow-screenshot-smoke: ok path={} stages=mode,next,previous,close",
            self.args.screenshot_output.display()
        );
        Ok(())
    }

    pub(crate) fn run_window_diagram_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        let fixture_label = self.diagram_smoke_fixture_label();
        self.select_fixture_for_window_smoke(&fixture_label)?;
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_full_height_diagram_scene_for_window_smoke(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let base = self.presented_frame_for_current_window(width, height)?;
        let mut last = base.clone();
        let mut stages = Vec::new();

        for action in DIAGRAM_SMOKE_ACTIONS {
            let next = self.apply_diagram_action_for_window_smoke(
                &mut window,
                action,
                width,
                height,
                &last,
            )?;
            if action.requires_frame_diff {
                last = next;
                stages.push(action.stage);
            }
        }

        super::window_clipboard_smoke::write_canvas_png_with_current_alias(
            &self.args.screenshot_output,
            &base,
        )?;
        println!(
            "storybook-window-diagram-screenshot-smoke: ok path={} fixture={} actions={} stages={}",
            self.args.screenshot_output.display(),
            fixture_label,
            DIAGRAM_SMOKE_ACTIONS
                .iter()
                .map(|action| action.command)
                .collect::<Vec<_>>()
                .join(","),
            stages.join(",")
        );
        Ok(())
    }

    pub(crate) fn run_window_sidebar_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        let mut window = self.create_window()?;
        let (width, height) = render_size_for_minifb_window(&window);
        self.update_frame_size(width, height);
        self.update_scene_for_refresh(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let base = self.presented_frame_for_current_window(width, height)?;

        let (file_id, file_x, file_y) = self.sidebar_file_point_for_window_smoke(width, height)?;
        if !self.update_sidebar_hover(Some((file_x, file_y)), width, height) {
            return Err(
                format!("window sidebar screenshot smoke file hover missing: {file_id}").into(),
            );
        }
        self.update_window_buffer(&mut window, width, height, true)?;
        let file_hover = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &base,
            &file_hover,
            "window sidebar screenshot smoke file hover frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "file-hover",
            &file_hover,
        )?;

        let file_pointer = StorybookPointer::new(file_x, file_y, StorybookMouseButton::Left);
        if !self.apply_canvas_click(file_pointer, width, height)? {
            return Err(format!(
                "window sidebar screenshot smoke file click did not dispatch: {file_id}"
            )
            .into());
        }
        if self.catalog.fixtures[self.selected_index].label != file_id {
            return Err(format!(
                "window sidebar screenshot smoke file click selected `{}`, expected `{file_id}`",
                self.catalog.fixtures[self.selected_index].label
            )
            .into());
        }
        self.update_scene_for_refresh(width, height)?;
        self.update_window_buffer(&mut window, width, height, true)?;
        let file_click = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &file_hover,
            &file_click,
            "window sidebar screenshot smoke file click frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "file-click",
            &file_click,
        )?;

        let (setting_x, setting_y) = self.sidebar_settings_field_point_for_window_smoke(
            StorybookSettingsField::Hover,
            width,
            height,
        )?;
        if !self.update_sidebar_hover(Some((setting_x, setting_y)), width, height) {
            return Err("window sidebar screenshot smoke settings hover missing".into());
        }
        self.update_window_buffer(&mut window, width, height, true)?;
        let settings_hover = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &file_click,
            &settings_hover,
            "window sidebar screenshot smoke settings hover frame did not change",
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "settings-hover",
            &settings_hover,
        )?;

        let before_hover_setting = self.interaction.hover_highlight_enabled;
        let setting_pointer =
            StorybookPointer::new(setting_x, setting_y, StorybookMouseButton::Left);
        if !self.apply_canvas_click(setting_pointer, width, height)? {
            return Err("window sidebar screenshot smoke settings click did not dispatch".into());
        }
        if self.interaction.hover_highlight_enabled == before_hover_setting {
            return Err(
                "window sidebar screenshot smoke settings click did not toggle hover".into(),
            );
        }
        self.update_window_buffer(&mut window, width, height, true)?;
        let settings_click = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &settings_hover,
            &settings_click,
            "window sidebar screenshot smoke settings click frame did not change",
        )?;
        super::window_clipboard_smoke::write_canvas_png(
            &self.args.screenshot_output,
            &settings_click,
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            "settings-click",
            &settings_click,
        )?;
        println!(
            "storybook-window-sidebar-screenshot-smoke: ok path={} stages=file-hover,file-click,settings-hover,settings-click",
            self.args.screenshot_output.display()
        );
        Ok(())
    }

    fn sidebar_file_point_for_window_smoke(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(String, f32, f32), StorybookError> {
        let selected = self.catalog.fixtures[self.selected_index].label.clone();
        self.sidebar_interaction_surface_for_window_smoke(width, height)?
            .file_tree_canvas_point(|action| {
                matches!(action, FileTreeAction::SelectFile { file_id } if file_id != &selected)
            })
            .and_then(|(action, x, y)| match action {
                FileTreeAction::SelectFile { file_id } => Some((file_id, x, y)),
                _ => None,
            })
            .ok_or_else(|| "window sidebar screenshot smoke file action hit missing".into())
    }

    fn document_hover_point_for_window_smoke(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(String, StorybookPointer), StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or("window hover screenshot smoke scene missing")?;
        let target = scene
            .targets
            .iter()
            .find(|target| target.rect.width > 0.0 && target.rect.height > 0.0)
            .ok_or("window hover screenshot smoke target missing")?;
        let document_x = target.rect.x + target.rect.width / 2.0;
        let document_y = target.rect.y + target.rect.height / 2.0;
        let node_id = target.node_id.0.clone();
        self.scroll_y = (document_y - 120.0).max(0.0);
        let area = StorybookPreviewArea::for_window(width, height, self.scroll_y);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        Ok((
            node_id,
            StorybookPointer::new(x, y, StorybookMouseButton::Left),
        ))
    }

    fn footnote_reference_point_for_window_smoke(
        &mut self,
        label: &str,
        target: &str,
        width: usize,
        height: usize,
    ) -> Result<(f32, StorybookPointer), StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or("window footnote screenshot smoke scene missing")?;
        let action = StorybookHostActionHits::hits(scene, width)
            .into_iter()
            .filter(|hit| Self::is_text_open_link_action(hit, label, target))
            .min_by_key(|hit| hit.rect.area())
            .ok_or("window footnote screenshot smoke reference action hit missing")?;
        let definition_y = scene
            .target_for_internal_anchor(target)
            .ok_or("window footnote screenshot smoke definition target missing")?
            .rect
            .y;
        let (document_x, document_y) = action.center_point();
        self.scroll_y = (document_y - 120.0).max(0.0);
        self.frame_cache = None;
        let area = StorybookPreviewArea::for_window(width, height, self.scroll_y);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        Ok((
            definition_y,
            StorybookPointer::new(x, y, StorybookMouseButton::Left),
        ))
    }

    fn scroll_to_table_for_window_smoke(
        &mut self,
        _width: usize,
        height: usize,
    ) -> Result<f32, StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or("window table screenshot smoke scene missing")?;
        let target = scene
            .targets
            .iter()
            .find(|target| {
                target.source.raw.text.contains("Feature | Status | Notes")
                    || target.source.raw.text.contains("Feature\nStatus\nNotes")
            })
            .ok_or("window table screenshot smoke table target missing")?;
        let max_scroll = (scene.content_height - height as f32).max(0.0);
        self.scroll_y = (target.rect.y - 120.0).clamp(0.0, max_scroll);
        self.frame_cache = None;
        Ok(target.rect.y)
    }

    fn is_text_open_link_action(
        hit: &katana_ui_core_storybook::UiTreeHostActionHit,
        label: &str,
        target: &str,
    ) -> bool {
        matches!(
            hit.action.text_span_action(),
            Some(UiTextSpanAction::OpenLink { target: action_target })
                if hit.action.label == label && action_target == target
        )
    }

    fn require_document_y_visible_for_window_smoke(
        &self,
        document_y: f32,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        let visible_bottom = self.scroll_y
            + StorybookPreviewArea::for_window(width, height, self.scroll_y).height as f32;
        if self.scroll_y <= document_y && document_y < visible_bottom {
            return Ok(());
        }
        Err(format!(
            "window footnote screenshot smoke target is not visible: scroll={} target={} bottom={visible_bottom}",
            self.scroll_y, document_y
        )
        .into())
    }

    fn sidebar_settings_field_point_for_window_smoke(
        &mut self,
        field: StorybookSettingsField,
        width: usize,
        height: usize,
    ) -> Result<(f32, f32), StorybookError> {
        self.sidebar_interaction_surface_for_window_smoke(width, height)?
            .settings_field_canvas_point(field.id())
            .ok_or_else(|| {
                format!(
                    "window sidebar screenshot smoke settings action hit missing: {}",
                    field.id()
                )
                .into()
            })
    }

    fn sidebar_interaction_surface_for_window_smoke(
        &self,
        width: usize,
        height: usize,
    ) -> Result<SidebarInteractionSurface, StorybookError> {
        Ok(SidebarHit::interaction_surface(&SidebarHitRequest {
            fixtures: &self.catalog.fixtures,
            selected_index: self.selected_index,
            scene: self.scene.as_ref(),
            dark: self.dark,
            interaction: &self.interaction,
            typography: self.typography,
            settings_state: self.settings_state.clone(),
            file_tree_state: self.file_tree_state.clone(),
            scroll: self.sidebar_scroll,
            width,
            height,
        }))
    }

    fn apply_diagram_action_for_window_smoke(
        &mut self,
        window: &mut Window,
        action: &DiagramSmokeAction,
        width: usize,
        height: usize,
        before: &Canvas,
    ) -> Result<Canvas, StorybookError> {
        self.scroll_to_host_action_for_window_smoke(action.command, width, height)?;
        self.clear_document_hover_state();
        self.update_window_buffer(window, width, height, true)?;
        let unhovered = self.presented_frame_for_current_window(width, height)?;
        let pointer = self.host_action_pointer_for_window_smoke(action.command, width, height)?;
        if !self.update_document_hover_for_canvas_point(pointer.x, pointer.y, width, height) {
            return Err(format!(
                "window diagram screenshot smoke hover did not resolve action: {}",
                action.command
            )
            .into());
        }
        self.update_window_buffer(window, width, height, true)?;
        let hovered = self.presented_frame_for_current_window(width, height)?;
        super::window_clipboard_smoke::require_frame_diff(
            &unhovered,
            &hovered,
            &format!(
                "window diagram screenshot smoke {} hover frame did not change",
                action.command
            ),
        )?;
        super::window_clipboard_smoke::write_stage_canvas_png(
            &self.args.screenshot_output,
            &format!("hover-{}", action.stage),
            &hovered,
        )?;
        let before_viewports = self.diagram_viewports.clone();
        if !self.apply_canvas_click(pointer, width, height)? {
            return Err(format!(
                "window diagram screenshot smoke action did not dispatch: {}",
                action.command
            )
            .into());
        }
        self.update_full_height_diagram_scene_for_window_smoke(width, height)?;
        self.update_window_buffer(window, width, height, true)?;
        let after = self.presented_frame_for_current_window(width, height)?;
        if self.diagram_viewports == before_viewports {
            return Err(format!(
                "window diagram screenshot smoke action did not update viewport state: {}",
                action.command
            )
            .into());
        }
        let requires_frame_diff = self.diagram_smoke_requires_frame_diff(action);
        if requires_frame_diff {
            super::window_clipboard_smoke::require_frame_diff(
                before,
                &after,
                &format!(
                    "window diagram screenshot smoke {} frame did not change",
                    action.command
                ),
            )?;
            super::window_clipboard_smoke::write_stage_canvas_png(
                &self.args.screenshot_output,
                action.stage,
                &after,
            )?;
        }
        if !requires_frame_diff {
            super::window_clipboard_smoke::write_stage_canvas_png(
                &self.args.screenshot_output,
                action.stage,
                &after,
            )?;
        }
        Ok(after)
    }

    fn diagram_smoke_requires_frame_diff(&self, action: &DiagramSmokeAction) -> bool {
        action.requires_frame_diff && !self.args.diagram_smoke_fixture.ends_with(".drawio")
    }

    fn diagram_smoke_fixture_label(&self) -> String {
        if self.args.diagram_smoke_fixture.trim().is_empty() {
            return DIAGRAM_FIXTURE_LABEL.to_string();
        }
        self.args.diagram_smoke_fixture.clone()
    }

    fn update_full_height_diagram_scene_for_window_smoke(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        let fixture = self.catalog.fixtures[self.selected_index].clone();
        let area = StorybookPreviewArea::for_window(width, height, 0.0);
        let viewport = if self.fullscreen_diagram_active() {
            ViewerViewport {
                width: width as f32,
                height: height as f32,
            }
        } else {
            ViewerViewport {
                width: area.width as f32,
                height: DIAGRAM_SMOKE_DOCUMENT_HEIGHT,
            }
        };
        let scene = self.preview.build_scene(PreviewBuildRequest {
            fixture: &fixture,
            viewport,
            dark: self.dark,
            theme: None,
            interaction: self.interaction.clone(),
            mode: self.mode.clone(),
            typography: self.typography,
            search: self.search.clone(),
            diagram_viewports: self.diagram_viewports.clone(),
            image_viewports: self.image_viewports.clone(),
            task_state_overrides: self.task_state_overrides.clone(),
            accordion_open_overrides: self.accordion_open_overrides.clone(),
            copied_code_node_ids: self.copied_code_node_ids.clone(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface: false,
            export_surface: false,
        })?;
        self.scene = Some(scene);
        self.asset_job = None;
        self.deferred_asset_job = false;
        self.frame_cache = None;
        Ok(())
    }

    fn select_fixture_for_window_smoke(&mut self, label: &str) -> Result<(), StorybookError> {
        let index = self
            .catalog
            .fixtures
            .iter()
            .position(|fixture| fixture.label == label)
            .ok_or_else(|| format!("window screenshot smoke fixture missing: {label}"))?;
        self.selected_index = index;
        self.reset_fixture_state();
        Ok(())
    }

    fn scroll_to_host_action_for_window_smoke(
        &mut self,
        action_id: &str,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or_else(|| "window screenshot smoke scene missing".to_string())?;
        let (_, document_y) = Self::diagram_smoke_action_document_point(scene, action_id, width)?;
        let max_scroll = (scene.content_height - height as f32).max(0.0);
        self.scroll_y = (document_y - 120.0).clamp(0.0, max_scroll);
        self.frame_cache = None;
        Ok(())
    }

    fn scroll_to_media_action_for_window_smoke(
        &mut self,
        kind: ViewerMediaControlKind,
        action_id: &str,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or_else(|| "window screenshot smoke scene missing".to_string())?;
        let (_, document_y) =
            Self::media_smoke_action_document_point(scene, kind, action_id, width)?;
        let max_scroll = (scene.content_height - height as f32).max(0.0);
        self.scroll_y = (document_y - 120.0).clamp(0.0, max_scroll);
        self.frame_cache = None;
        Ok(())
    }

    fn host_action_pointer_for_window_smoke(
        &self,
        action_id: &str,
        width: usize,
        height: usize,
    ) -> Result<StorybookPointer, StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or_else(|| "window screenshot smoke scene missing".to_string())?;
        let (document_x, document_y) =
            Self::diagram_smoke_action_document_point(scene, action_id, width)?;
        let area = StorybookPreviewArea::for_window(width, height, self.scroll_y);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        Ok(StorybookPointer::new(x, y, StorybookMouseButton::Left))
    }

    fn host_media_action_pointer_for_window_smoke(
        &self,
        kind: ViewerMediaControlKind,
        action_id: &str,
        width: usize,
        height: usize,
    ) -> Result<StorybookPointer, StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or_else(|| "window screenshot smoke scene missing".to_string())?;
        let (document_x, document_y) =
            Self::media_smoke_action_document_point(scene, kind, action_id, width)?;
        let area = StorybookPreviewArea::for_window(width, height, self.scroll_y);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        Ok(StorybookPointer::new(x, y, StorybookMouseButton::Left))
    }

    fn media_smoke_action_document_point(
        scene: &crate::preview::PreviewScene,
        kind: ViewerMediaControlKind,
        action_id: &str,
        width: usize,
    ) -> Result<(f32, f32), StorybookError> {
        StorybookHostActionHits::hits(scene, width)
            .into_iter()
            .find_map(|hit| {
                let action = StorybookMediaHostAction::from_host_action_plan(&hit.action)?
                    .into_viewer_action();
                (action.kind == kind && action.command == action_id).then(|| hit.center_point())
            })
            .ok_or_else(|| {
                format!("window screenshot smoke media action hit missing: {kind:?}:{action_id}")
                    .into()
            })
    }

    fn diagram_smoke_action_document_point(
        scene: &crate::preview::PreviewScene,
        action_id: &str,
        width: usize,
    ) -> Result<(f32, f32), StorybookError> {
        if let Some(point) = StorybookHostActionHits::hits(scene, width)
            .into_iter()
            .find(|hit| Self::is_diagram_smoke_action(&hit.action, action_id))
            .map(|hit| hit.center_point())
        {
            return Ok(point);
        }
        StorybookHostActionRouter::for_window(scene, width)
            .internal_diagram_point_for_action(action_id)
            .ok_or_else(|| {
                format!("window screenshot smoke action hit missing: {action_id}").into()
            })
    }

    fn is_diagram_smoke_action(
        action: &katana_ui_core::render_model::UiHostActionPlan,
        command: &str,
    ) -> bool {
        StorybookMediaHostAction::from_host_action_plan(action)
            .map(StorybookMediaHostAction::into_viewer_action)
            .is_some_and(|action| {
                action.kind == ViewerMediaControlKind::Diagram && action.command == command
            })
    }

    fn run_window_loop(&mut self, window: &mut Window) -> Result<(), StorybookError> {
        let mut frame_index = 0;
        while self.should_continue(window, frame_index) {
            let frame_started = Instant::now();
            let (window_width, window_height) = window.get_size();
            let (width, height) = render_size_for_window(window_width, window_height);
            let size_changed = self.update_frame_size(width, height);
            let pre_input_scene_changed =
                self.refresh_scene_before_input_if_needed(size_changed, width, height)?;
            let keyboard_changed = self.apply_keyboard(window)?;
            let mouse_changed = self.apply_mouse(window)?;
            let host_changed = self.apply_host_window_events(window);
            let pointer_changed = self.update_pointer_position(window);
            let scroll_changed = self.apply_scroll(window);
            let input_changed = keyboard_changed || mouse_changed || host_changed;
            let post_input_scene_changed = self.scene_refresh_needed(false);
            let scene_changed = pre_input_scene_changed || post_input_scene_changed;
            let cursor_input_changed = scene_changed || input_changed;
            let hover_changed = if scroll_changed {
                let changed = self.clear_document_hover_state();
                self.apply_cursor(window);
                changed
            } else if should_update_document_hover_for_loop(
                cursor_input_changed,
                pointer_changed,
                scroll_changed,
            ) {
                let changed = self.apply_hover(window);
                self.apply_cursor(window);
                changed
            } else {
                false
            };
            let asset_changed = if post_input_scene_changed {
                self.update_scene_for_refresh(width, height)?;
                false
            } else if changes_before_asset_update(scroll_changed, input_changed)
                .should_defer_asset_update()
            {
                false
            } else {
                self.apply_asset_job()?
            };
            let asset_pending = self.asset_job.is_some();
            let changes = FrameLoopChanges {
                scene_changed,
                input_changed,
                scroll_changed,
                hover_changed,
                asset_changed,
            };
            let animation_changed = if changes.should_pause_loading_animation() {
                false
            } else {
                self.update_loading_animation(asset_pending)
            };
            self.update_window_buffer_for_loop(window, width, height, changes, animation_changed)?;
            thread::sleep(changes.delay_after_frame(asset_pending, frame_started.elapsed()));
            frame_index += 1;
        }
        Ok(())
    }

    fn apply_host_window_events(&mut self, _window: &mut Window) -> bool {
        self.drain_diagram_fullscreen_events()
    }

    pub(crate) fn drain_diagram_fullscreen_events(&mut self) -> bool {
        let events = std::mem::take(&mut self.host_events);
        let mut consumed = false;
        for event in events {
            let StorybookHostEvent::DiagramFullscreen { .. } = event;
            consumed = true;
        }
        consumed
    }

    pub(super) fn update_loading_animation(&mut self, asset_pending: bool) -> bool {
        if !asset_pending {
            let changed = self.animation_phase != 0;
            self.animation_phase = 0;
            return changed;
        }
        self.animation_phase = self.animation_phase.wrapping_add(1);
        true
    }

    fn should_continue(&self, window: &Window, frame_index: usize) -> bool {
        let inside_frame_limit = self.args.frames == 0 || frame_index < self.args.frames;
        window.is_open()
            && self.escape_keeps_window_open(window.is_key_down(Key::Escape))
            && inside_frame_limit
    }

    pub(super) fn escape_keeps_window_open(&self, escape_down: bool) -> bool {
        self.mode == ViewerMode::Slideshow || !escape_down
    }

    fn create_window(&self) -> Result<Window, minifb::Error> {
        let (width, height) = interactive_window_size(self.args.width, self.args.height);
        Window::new(
            TITLE,
            width,
            height,
            WindowOptions {
                resize: true,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
    }

    pub(super) fn update_frame_size(&mut self, width: usize, height: usize) -> bool {
        let size = (width, height);
        let changed = self.frame_size.is_some_and(|current| current != size);
        self.frame_size = Some(size);
        if changed {
            self.frame_cache = None;
            self.sidebar_frame_cache.clear();
            self.sidebar_interaction_cache = None;
            self.sidebar_interaction_surface_cache = None;
        }
        changed
    }

    pub(super) fn scene_refresh_needed(&self, size_changed: bool) -> bool {
        size_changed || self.scene.is_none()
    }

    pub(super) fn refresh_scene_before_input_if_needed(
        &mut self,
        size_changed: bool,
        width: usize,
        height: usize,
    ) -> Result<bool, StorybookError> {
        if !self.scene_refresh_needed(size_changed) {
            return Ok(false);
        }
        self.update_scene_for_refresh(width, height)?;
        Ok(true)
    }

    fn update_window_buffer_for_loop(
        &mut self,
        window: &mut Window,
        width: usize,
        height: usize,
        changes: FrameLoopChanges,
        animation_changed: bool,
    ) -> Result<(), minifb::Error> {
        let scale = storybook_scale_factor();
        if self.can_redraw_preview_only(width, height, changes, animation_changed) {
            self.redraw_cached_preview_for_presented_scroll(width, height)?;
        } else if changes.needs_redraw()
            || animation_changed
            || !self.frame_cache_matches_scaled(width, height, scale)
        {
            self.frame_cache = Some(self.render_frame_cache_scaled(width, height, scale));
        }
        let Some(frame) = self.frame_cache.as_mut() else {
            return Err(minifb::Error::UpdateFailed(
                "frame cache missing after rendering".to_string(),
            ));
        };
        let presented = frame.presented_frame(width, height);
        window.update_with_buffer(presented.pixels(), presented.width(), presented.height())
    }

    fn update_window_buffer(
        &mut self,
        window: &mut Window,
        width: usize,
        height: usize,
        redraw_needed: bool,
    ) -> Result<(), minifb::Error> {
        let scale = storybook_scale_factor();
        if redraw_needed || !self.frame_cache_matches_scaled(width, height, scale) {
            self.frame_cache = Some(self.render_frame_cache_scaled(width, height, scale));
        }
        let Some(frame) = self.frame_cache.as_mut() else {
            return Err(minifb::Error::UpdateFailed(
                "frame cache missing after rendering".to_string(),
            ));
        };
        let presented = frame.presented_frame(width, height);
        window.update_with_buffer(presented.pixels(), presented.width(), presented.height())
    }

    fn can_redraw_preview_only(
        &self,
        width: usize,
        height: usize,
        changes: FrameLoopChanges,
        animation_changed: bool,
    ) -> bool {
        self.frame_cache_matches_scaled(width, height, storybook_scale_factor())
            && changes.can_redraw_preview_only(animation_changed)
    }

    #[cfg(test)]
    pub(super) fn redraw_cached_preview(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), minifb::Error> {
        let Some(mut frame) = self.frame_cache.take() else {
            return Err(minifb::Error::UpdateFailed(
                "frame cache missing before preview redraw".to_string(),
            ));
        };
        {
            let request = self.frame_render_request(width, height);
            let rendered_scroll_y = frame.rendered_scroll_y();
            let scroll_redraw =
                if self.text_selection_start.is_none() && self.text_selection_end.is_none() {
                    StorybookFrameRenderer::redraw_preview_scroll_delta_with_result(
                        frame.canvas_mut_preserving_presented(),
                        &request,
                        rendered_scroll_y,
                    )
                } else {
                    None
                };
            let can_scroll_redraw = scroll_redraw
                .is_some_and(|redraw| frame.update_presented_scroll_region(width, height, redraw));
            if scroll_redraw.is_some() && !can_scroll_redraw {
                frame.invalidate_presented();
            }
            if scroll_redraw.is_none() {
                StorybookFrameRenderer::redraw_preview(frame.canvas_mut(), &request);
            }
            frame.set_rendered_scroll_y(self.scroll_y);
        }
        if self.text_selection_start.is_some() || self.text_selection_end.is_some() {
            self.draw_text_selection(frame.canvas_mut());
        }
        self.start_deferred_asset_job_for_current_viewport(width, height);
        self.frame_cache = Some(frame);
        Ok(())
    }

    pub(super) fn redraw_cached_preview_for_presented_scroll(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<(), minifb::Error> {
        let Some(mut frame) = self.frame_cache.take() else {
            return Err(minifb::Error::UpdateFailed(
                "frame cache missing before preview redraw".to_string(),
            ));
        };
        {
            let request = self.frame_render_request(width, height);
            let rendered_scroll_y = frame.rendered_scroll_y();
            let scroll_redraw =
                if self.text_selection_start.is_none() && self.text_selection_end.is_none() {
                    frame.ensure_presented_frame(width, height);
                    StorybookFrameRenderer::redraw_preview_scroll_delta_with_result(
                        frame.canvas_mut_preserving_presented(),
                        &request,
                        rendered_scroll_y,
                    )
                } else {
                    None
                };
            let can_scroll_redraw = scroll_redraw
                .is_some_and(|redraw| frame.update_presented_scroll_region(width, height, redraw));
            if scroll_redraw.is_none() || !can_scroll_redraw {
                frame.invalidate_presented();
                StorybookFrameRenderer::redraw_preview(frame.canvas_mut(), &request);
            }
            frame.set_rendered_scroll_y(self.scroll_y);
        }
        if self.text_selection_start.is_some() || self.text_selection_end.is_some() {
            self.draw_text_selection(frame.canvas_mut());
        }
        self.start_deferred_asset_job_for_current_viewport(width, height);
        self.frame_cache = Some(frame);
        Ok(())
    }

    #[cfg(test)]
    fn presented_frame(frame: &StorybookFrameCache, width: usize, height: usize) -> Canvas {
        let fill = frame.pixels().first().copied().unwrap_or_default();
        StorybookPresentation::present_frame_for_window(frame.canvas(), width, height, fill)
    }

    #[cfg(test)]
    pub(super) fn presented_frame_buffer(
        frame: &StorybookFrameCache,
        width: usize,
        height: usize,
    ) -> Canvas {
        Self::presented_frame(frame, width, height)
    }

    fn presented_frame_for_current_window(
        &mut self,
        width: usize,
        height: usize,
    ) -> Result<Canvas, StorybookError> {
        self.frame_cache
            .as_mut()
            .map(|frame| frame.presented_frame(width, height).clone())
            .ok_or_else(|| "window selection screenshot smoke frame cache missing".into())
    }

    pub(super) fn frame_cache_matches(&self, width: usize, height: usize) -> bool {
        self.frame_cache
            .as_ref()
            .is_some_and(|frame| frame.matches(width, height))
    }

    pub(super) fn frame_cache_matches_scaled(
        &self,
        width: usize,
        height: usize,
        scale: f32,
    ) -> bool {
        self.frame_cache
            .as_ref()
            .is_some_and(|frame| frame.matches_scaled(width, height, scale))
    }
}

pub(super) fn should_update_document_hover_for_loop(
    cursor_input_changed: bool,
    pointer_changed: bool,
    scroll_changed: bool,
) -> bool {
    !scroll_changed && (cursor_input_changed || pointer_changed)
}

fn changes_before_asset_update(scroll_changed: bool, input_changed: bool) -> FrameLoopChanges {
    FrameLoopChanges {
        scene_changed: false,
        input_changed,
        scroll_changed,
        hover_changed: false,
        asset_changed: false,
    }
}

fn storybook_scale_factor() -> f32 {
    env::var(STORYBOOK_SCALE_ENV)
        .ok()
        .and_then(|value| parse_storybook_scale_factor(value.as_str()))
        .unwrap_or_else(default_storybook_scale_factor)
}

fn parse_storybook_scale_factor(value: &str) -> Option<f32> {
    let scale = value.parse::<u32>().ok()?;
    match scale {
        1 | 2 => Some(scale as f32),
        _ => None,
    }
}

pub(super) fn render_size_for_window(width: usize, height: usize) -> (usize, usize) {
    (width, height)
}

fn dark_toggle_track_center_from_sidebar_canvas(
    canvas: &Canvas,
) -> Result<(f32, f32), StorybookError> {
    let min_y = canvas.height() / 3;
    let mut found = false;
    let mut min_x = usize::MAX;
    let mut min_track_y = usize::MAX;
    let mut max_x = 0usize;
    let mut max_track_y = 0usize;
    for y in min_y..canvas.height() {
        let mut row_has_accent = false;
        for x in 0..canvas.width() {
            if canvas.pixels()[y * canvas.width() + x] == KUC_DARK_ACCENT {
                found = true;
                row_has_accent = true;
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_track_y = min_track_y.min(y);
                max_track_y = max_track_y.max(y);
            }
        }
        if found && !row_has_accent && y.saturating_sub(max_track_y) > 2 {
            break;
        }
    }
    if !found {
        return Err("live dark toggle track was not found in rendered KUC sidebar".into());
    }
    let scale = canvas.scale_factor();
    let local_x = ((min_x + max_x) as f32 + 1.0) / (2.0 * scale);
    let local_y = ((min_track_y + max_track_y) as f32 + 1.0) / (2.0 * scale);
    Ok((
        sidebar_content_x() as f32 + local_x,
        SIDEBAR_CONTENT_INSET as f32 + local_y,
    ))
}

fn interactive_window_size(width: usize, height: usize) -> (usize, usize) {
    render_size_for_window(width, height)
}

fn render_size_for_minifb_window(window: &Window) -> (usize, usize) {
    let (width, height) = window.get_size();
    render_size_for_window(width, height)
}

fn default_storybook_scale_factor() -> f32 {
    if cfg!(target_os = "macos") { 2.0 } else { 1.0 }
}

#[cfg(test)]
#[path = "window_loop_tests.rs"]
mod tests;
