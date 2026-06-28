use crate::canvas::Canvas;
use crate::palette::StorybookPalette;
use crate::preview::PreviewScene;

pub struct StorybookFrameStatus;

impl StorybookFrameStatus {
    pub fn draw(
        canvas: &mut Canvas,
        scene: &PreviewScene,
        x: usize,
        y: usize,
        palette: StorybookPalette,
        last_command_label: &str,
    ) {
        let status = format!(
            "command={} nodes={} targets={} requests={} loaded={} failed={} images={} content={:.0} slide={}/{} warnings={}",
            last_command_label,
            scene.node_count,
            scene.targets.len(),
            scene.asset_request_count,
            scene.loaded_asset_count,
            scene.failed_asset_count,
            scene.image_surface_count,
            scene.content_height,
            scene.slideshow_current_page,
            scene.slideshow_max_page,
            scene.warnings.len()
        );
        canvas.draw_text(x, y, &status, palette.text());
    }

    pub fn viewer_mode_label(scene: Option<&PreviewScene>) -> &'static str {
        let Some(scene) = scene else {
            return "document";
        };
        match scene.mode {
            katana_document_viewer::ViewerMode::Document => "document",
            katana_document_viewer::ViewerMode::Slideshow => "slideshow",
        }
    }
}
