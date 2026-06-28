use crate::preview::PreviewScene;
use katana_document_viewer::ViewerMode;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct SidebarSceneStats {
    pub(crate) has_scene: bool,
    pub(crate) mode: Option<ViewerMode>,
    pub(crate) slide_current: usize,
    pub(crate) slide_max: usize,
    pub(crate) scene_font: i64,
    pub(crate) nodes: i64,
    pub(crate) loaded: i64,
    pub(crate) failed: i64,
    pub(crate) images: i64,
    pub(crate) surface_label: String,
}

pub(crate) fn scene_stats(scene: Option<&PreviewScene>) -> SidebarSceneStats {
    let Some(scene) = scene else {
        return SidebarSceneStats::default();
    };
    SidebarSceneStats {
        has_scene: true,
        mode: Some(scene.mode.clone()),
        slide_current: scene.slideshow_current_page,
        slide_max: scene.slideshow_max_page,
        scene_font: i64::from(scene.typography.preview_font_size),
        nodes: scene.node_count as i64,
        loaded: scene.loaded_asset_count as i64,
        failed: scene.failed_asset_count as i64,
        images: scene.image_surface_count as i64,
        surface_label: surface_label(scene),
    }
}

pub(crate) fn slide_label(stats: &SidebarSceneStats) -> String {
    if !stats.has_scene {
        return "0/0".to_string();
    }
    format!("{}/{}", stats.slide_current + 1, stats.slide_max + 1)
}

pub(crate) fn mode_label(mode: &Option<ViewerMode>) -> &'static str {
    match mode {
        Some(ViewerMode::Document) | None => "document",
        Some(ViewerMode::Slideshow) => "slideshow",
    }
}

fn surface_label(scene: &PreviewScene) -> String {
    scene.surface.as_ref().map_or_else(
        || "lazy".to_string(),
        |surface| format!("{}x{}", surface.width, surface.height),
    )
}
