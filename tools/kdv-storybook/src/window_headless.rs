use super::{StorybookError, StorybookWindow};
use crate::frame_pixel_guard::StorybookFramePixelGuard;
use crate::layout::preview_viewport_height;
use crate::smoke_assertions::StorybookSmokeAssertions;

const BOTTOM_SCROLL_FIXTURE: &str = "katana/sample_basic.md";
const LAZY_ASSET_SCROLL_FIXTURE: &str = "katana/sample_diagrams.md";

impl StorybookWindow {
    pub(super) fn render_headless(&mut self) -> Result<(), StorybookError> {
        for index in 0..self.catalog.fixtures.len() {
            self.selected_index = index;
            self.scroll_y = 0.0;
            self.update_scene_loaded(self.args.width, self.args.height)?;
            let canvas = self.render_canvas(self.args.width, self.args.height);
            if canvas.pixels().iter().all(|pixel| *pixel == 0) {
                return Err(format!("storybook rendered a blank frame: {index}").into());
            }
            let label = self.catalog.fixtures[index].label.clone();
            StorybookFramePixelGuard::assert_fixture_content(&label, &canvas, self.dark)?;
            let scene = self.scene.as_ref().ok_or("storybook scene missing")?;
            StorybookSmokeAssertions::assert_fixture_visible(&label, scene)?;
            self.assert_scroll_regression_contracts(&label)?;
        }
        println!(
            "storybook-window-smoke: ok fixtures={} checked={}",
            self.catalog.fixtures.len(),
            self.catalog.fixtures.len(),
        );
        Ok(())
    }

    fn assert_scroll_regression_contracts(&mut self, label: &str) -> Result<(), StorybookError> {
        if label == BOTTOM_SCROLL_FIXTURE {
            self.assert_bottom_scroll_tail_space(label)?;
        }
        if label == LAZY_ASSET_SCROLL_FIXTURE {
            self.assert_lazy_asset_scroll_scope(label)?;
        }
        Ok(())
    }

    fn assert_bottom_scroll_tail_space(&mut self, label: &str) -> Result<(), StorybookError> {
        let content_height = self
            .scene
            .as_ref()
            .ok_or("storybook bottom scroll scene missing")?
            .content_height;
        let viewport_height = preview_viewport_height(self.args.height) as f32;
        if content_height <= viewport_height {
            return Err(format!("bottom scroll fixture is not scrollable: {label}").into());
        }
        if !self.apply_preview_scroll(-10_000.0, self.args.height) {
            return Err(format!("bottom scroll did not move fixture: {label}").into());
        }
        let canvas = self.render_canvas(self.args.width, self.args.height);
        StorybookSmokeAssertions::assert_bottom_tail_space(label, &canvas, self.dark)
    }

    fn assert_lazy_asset_scroll_scope(&mut self, label: &str) -> Result<(), StorybookError> {
        self.scroll_y = 0.0;
        self.loaded_asset_job_keys.clear();
        self.loaded_asset_scenes.clear();
        self.asset_job = None;
        self.scene = None;
        self.update_scene(self.args.width, self.args.height)?;
        let scene = self.scene.as_ref().ok_or("lazy asset scene missing")?;
        let asset_request_key = scene.asset_request_key.clone();
        let asset_job_key = self
            .asset_job
            .as_ref()
            .ok_or_else(|| format!("lazy asset job missing: {label}"))?
            .key()
            .clone();
        if !self.apply_preview_scroll(-10_000.0, self.args.height) {
            return Err(format!("lazy asset scroll did not move fixture: {label}").into());
        }
        let _canvas = self.render_canvas(self.args.width, self.args.height);
        let scrolled_scene = self
            .scene
            .as_ref()
            .ok_or("scrolled lazy asset scene missing")?;
        let scrolled_asset_job_key = self
            .asset_job
            .as_ref()
            .ok_or_else(|| format!("lazy asset job missing after scroll: {label}"))?
            .key();
        if asset_request_key != scrolled_scene.asset_request_key {
            return Err(format!("lazy asset scope changed after scroll: {label}").into());
        }
        if &asset_job_key != scrolled_asset_job_key {
            return Err(format!("lazy asset job key changed after scroll: {label}").into());
        }
        Ok(())
    }
}
