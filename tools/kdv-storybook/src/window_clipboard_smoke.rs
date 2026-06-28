use super::{StorybookError, StorybookWindow};
use crate::layout::StorybookPreviewArea;
use crate::mouse::{StorybookHostActionHits, StorybookMouseButton, StorybookPointer};
use image::ImageEncoder;
use katana_document_viewer::{ViewerMode, ViewerSlideshowControlAction};
use std::path::{Path, PathBuf};

impl StorybookWindow {
    pub(crate) fn run_clipboard_smoke(mut self) -> Result<(), StorybookError> {
        let width = self.args.width;
        let height = self.args.height;
        self.update_frame_size(width, height);
        self.update_scene(width, height)?;
        self.text_selection_start = Some((0, 0));
        self.text_selection_end = Some((width, height));
        self.frame_cache = None;
        let payload = self
            .selected_text_payload(width, height)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| "clipboard smoke selection payload missing".to_string())?;
        super::window_command::write_clipboard_text(&payload)?;
        println!(
            "storybook-clipboard-smoke: ok bytes={} lines={}",
            payload.len(),
            payload.lines().count()
        );
        Ok(())
    }

    pub(crate) fn run_clipboard_keyboard_smoke(mut self) -> Result<(), StorybookError> {
        let width = self.args.width;
        let height = self.args.height;
        self.update_frame_size(width, height);
        self.update_scene(width, height)?;
        self.text_selection_start = Some((0, 0));
        self.text_selection_end = Some((width, height));
        self.frame_cache = None;
        if !self.copy_selected_text_to_clipboard(width, height) {
            return Err("clipboard keyboard smoke copy path did not write payload".into());
        }
        let payload = self
            .selected_text_payload(width, height)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| "clipboard keyboard smoke selection payload missing".to_string())?;
        println!(
            "storybook-clipboard-keyboard-smoke: ok bytes={} lines={}",
            payload.len(),
            payload.lines().count()
        );
        Ok(())
    }

    pub(crate) fn run_clipboard_drag_smoke(mut self) -> Result<(), StorybookError> {
        let width = self.args.width;
        let height = self.args.height;
        self.update_frame_size(width, height);
        self.update_scene(width, height)?;
        if !self.apply_text_selection_drag_for_smoke((0.0, 0.0), (width as f32, height as f32)) {
            return Err("clipboard drag smoke selection path did not update".into());
        }
        if !self.copy_selected_text_to_clipboard(width, height) {
            return Err("clipboard drag smoke copy path did not write payload".into());
        }
        let payload = self
            .selected_text_payload(width, height)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| "clipboard drag smoke selection payload missing".to_string())?;
        println!(
            "storybook-clipboard-drag-smoke: ok bytes={} lines={}",
            payload.len(),
            payload.lines().count()
        );
        Ok(())
    }

    pub(crate) fn run_selection_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        let width = self.args.width;
        let height = self.args.height;
        let output = self.args.screenshot_output.clone();
        self.update_frame_size(width, height);
        self.update_scene(width, height)?;
        let base = self.render_canvas(width, height);
        if !self.apply_text_selection_drag_for_smoke((0.0, 0.0), (width as f32, height as f32)) {
            return Err("selection screenshot smoke drag path did not update".into());
        }
        let selected = self.render_canvas(width, height);
        let changed = pixel_diff_count(&base, &selected);
        if changed == 0 {
            return Err("selection screenshot smoke did not change frame pixels".into());
        }
        write_canvas_png(&output, &selected)?;
        println!(
            "storybook-selection-screenshot-smoke: ok path={} changed_pixels={changed}",
            output.display()
        );
        Ok(())
    }

    pub(crate) fn run_slideshow_screenshot_smoke(mut self) -> Result<(), StorybookError> {
        let width = self.args.width;
        let height = self.args.height;
        let output = self.args.screenshot_output.clone();
        self.update_frame_size(width, height);
        self.update_scene(width, height)?;
        let document = self.render_canvas(width, height);

        if !self.apply_settings_field(
            crate::settings_action::StorybookSettingsField::Mode,
            width,
            height,
        )? {
            return Err("slideshow screenshot smoke mode switch did not update".into());
        }
        self.update_scene(width, height)?;
        let mode = self.render_canvas(width, height);
        require_frame_diff(
            &document,
            &mode,
            "slideshow screenshot smoke mode frame did not change",
        )?;
        write_stage_canvas_png(&output, "mode", &mode)?;

        self.apply_slideshow_control_for_smoke(
            ViewerSlideshowControlAction::NextPage,
            width,
            height,
        )?;
        let next = self.render_canvas(width, height);
        require_frame_diff(
            &mode,
            &next,
            "slideshow screenshot smoke next frame did not change",
        )?;
        write_stage_canvas_png(&output, "next", &next)?;

        self.apply_slideshow_control_for_smoke(
            ViewerSlideshowControlAction::PreviousPage,
            width,
            height,
        )?;
        let previous = self.render_canvas(width, height);
        require_frame_diff(
            &next,
            &previous,
            "slideshow screenshot smoke previous frame did not change",
        )?;
        write_stage_canvas_png(&output, "previous", &previous)?;

        self.apply_slideshow_control_for_smoke(ViewerSlideshowControlAction::Close, width, height)?;
        self.update_scene(width, height)?;
        let closed = self.render_canvas(width, height);
        require_frame_diff(
            &previous,
            &closed,
            "slideshow screenshot smoke close frame did not change",
        )?;
        write_stage_canvas_png(&output, "close", &closed)?;
        write_canvas_png(&output, &closed)?;
        println!(
            "storybook-slideshow-screenshot-smoke: ok path={} stages=mode,next,previous,close",
            output.display()
        );
        Ok(())
    }

    pub(super) fn apply_slideshow_control_for_smoke(
        &mut self,
        action: ViewerSlideshowControlAction,
        width: usize,
        height: usize,
    ) -> Result<(), StorybookError> {
        if self.mode != ViewerMode::Slideshow {
            return Err("slideshow screenshot smoke expected slideshow mode".into());
        }
        let pointer = self.slideshow_control_pointer_for_smoke(action, width, height)?;
        if !self.apply_canvas_click(pointer, width, height)? {
            return Err(format!(
                "slideshow screenshot smoke action did not dispatch: {}",
                action.host_action_id()
            )
            .into());
        }
        Ok(())
    }

    fn slideshow_control_pointer_for_smoke(
        &self,
        action: ViewerSlideshowControlAction,
        width: usize,
        height: usize,
    ) -> Result<StorybookPointer, StorybookError> {
        let scene = self
            .scene
            .as_ref()
            .ok_or_else(|| "slideshow screenshot smoke scene missing".to_string())?;
        let action_id = action.host_action_id();
        let hit = StorybookHostActionHits::hits(scene, width)
            .into_iter()
            .find(|hit| hit.action.action_id == action_id)
            .ok_or_else(|| format!("slideshow screenshot smoke action hit missing: {action_id}"))?;
        let (document_x, document_y) = hit.center_point();
        let area = StorybookPreviewArea::for_window(width, height, 0.0);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        Ok(StorybookPointer::new(x, y, StorybookMouseButton::Left))
    }
}

pub(super) fn require_frame_diff(
    before: &crate::canvas::Canvas,
    after: &crate::canvas::Canvas,
    message: &str,
) -> Result<(), StorybookError> {
    if pixel_diff_count(before, after) <= 64 {
        return Err(message.to_string().into());
    }
    Ok(())
}

pub(super) fn write_stage_canvas_png(
    output: &Path,
    stage: &str,
    canvas: &crate::canvas::Canvas,
) -> std::io::Result<()> {
    write_canvas_png(&stage_path(output, stage), canvas)
}

fn stage_path(output: &Path, stage: &str) -> PathBuf {
    let parent = output.parent().unwrap_or_else(|| Path::new(""));
    let stem = output
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("kdv-storybook-slideshow-smoke");
    let extension = output
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("png");
    parent.join(format!("{stem}-{stage}.{extension}"))
}

pub(super) fn write_canvas_png(path: &Path, canvas: &crate::canvas::Canvas) -> std::io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::File::create(path)?;
    let encoder = image::codecs::png::PngEncoder::new(file);
    encoder
        .write_image(
            &canvas_rgba(canvas),
            canvas.width() as u32,
            canvas.height() as u32,
            image::ColorType::Rgba8.into(),
        )
        .map_err(|error| std::io::Error::other(format!("png encode failed: {error}")))
}

pub(super) fn write_canvas_png_with_current_alias(
    path: &Path,
    canvas: &crate::canvas::Canvas,
) -> std::io::Result<()> {
    write_canvas_png(path, canvas)?;
    write_canvas_png(&current_alias_path(path), canvas)
}

fn current_alias_path(output: &Path) -> PathBuf {
    let parent = output.parent().unwrap_or_else(|| Path::new(""));
    let stem = output
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("kdv-storybook-smoke");
    let extension = output
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("png");
    parent.join(format!("{stem}-current.{extension}"))
}

fn canvas_rgba(canvas: &crate::canvas::Canvas) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(canvas.width() * canvas.height() * 4);
    for color in canvas.pixels() {
        rgba.push(((color >> 16) & 0xff) as u8);
        rgba.push(((color >> 8) & 0xff) as u8);
        rgba.push((color & 0xff) as u8);
        rgba.push(0xff);
    }
    rgba
}

pub(super) fn pixel_diff_count(
    left: &crate::canvas::Canvas,
    right: &crate::canvas::Canvas,
) -> usize {
    left.pixels()
        .iter()
        .zip(right.pixels().iter())
        .filter(|(left, right)| left != right)
        .count()
}

#[cfg(test)]
mod tests {
    use super::current_alias_path;
    use std::path::Path;

    #[test]
    fn current_alias_path_keeps_smoke_name_visible() {
        assert_eq!(
            Path::new("target/kdv-storybook-window-diagram-smoke-current.png"),
            current_alias_path(Path::new("target/kdv-storybook-window-diagram-smoke.png"))
        );
    }
}
