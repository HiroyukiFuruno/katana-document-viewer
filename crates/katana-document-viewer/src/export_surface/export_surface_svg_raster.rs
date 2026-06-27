use tiny_skia::Pixmap;

use resvg::usvg;

const MAX_RASTERIZED_SVG_EDGE: f32 = 8192.0;
const FALLBACK_PROPORTIONAL_FONT_FAMILY: &str = "Ubuntu";
const FALLBACK_MONOSPACE_FONT_FAMILY: &str = "Hack";

pub(super) struct RasterTarget {
    effective_scale: f32,
    width: u32,
    height: u32,
}

impl RasterTarget {
    #[cfg(test)]
    pub(super) fn new(size: usvg::Size, max_width: u32) -> Self {
        Self::new_with_content_scale(size, max_width, 100)
    }

    pub(super) fn new_with_content_scale(
        size: usvg::Size,
        max_width: u32,
        content_scale: u32,
    ) -> Self {
        let display_width = size.width();
        let display_height = size.height();
        let effective_scale = effective_scale(
            display_width,
            display_height,
            max_width,
            content_scale.max(1),
        );
        Self {
            effective_scale,
            width: ((display_width * effective_scale).ceil() as u32).max(1),
            height: ((display_height * effective_scale).ceil() as u32).max(1),
        }
    }

    pub(super) fn render(&self, tree: &usvg::Tree) -> Option<Pixmap> {
        let mut pixmap = Pixmap::new(self.width, self.height)?;
        let transform =
            tiny_skia::Transform::from_scale(self.effective_scale, self.effective_scale);
        resvg::render(tree, transform, &mut pixmap.as_mut());
        Some(pixmap)
    }

    pub(super) fn width(&self) -> u32 {
        self.width
    }

    pub(super) fn height(&self) -> u32 {
        self.height
    }
}

pub(super) fn rasterizer_options() -> usvg::Options<'static> {
    usvg::Options {
        fontdb: font_db(),
        font_family: FALLBACK_PROPORTIONAL_FONT_FAMILY.to_string(),
        ..usvg::Options::default()
    }
}

fn font_db() -> std::sync::Arc<usvg::fontdb::Database> {
    static FONT_DB: std::sync::OnceLock<std::sync::Arc<usvg::fontdb::Database>> =
        std::sync::OnceLock::new();
    std::sync::Arc::clone(FONT_DB.get_or_init(|| {
        let mut db = usvg::fontdb::Database::new();
        db.load_system_fonts();
        load_embedded_fonts(&mut db);
        set_generic_font_families(&mut db);
        std::sync::Arc::new(db)
    }))
}

fn load_embedded_fonts(db: &mut usvg::fontdb::Database) {
    db.load_font_data(epaint_default_fonts::UBUNTU_LIGHT.to_vec());
    db.load_font_data(epaint_default_fonts::HACK_REGULAR.to_vec());
    db.load_font_data(epaint_default_fonts::NOTO_EMOJI_REGULAR.to_vec());
    db.load_font_data(epaint_default_fonts::EMOJI_ICON.to_vec());
}

fn set_generic_font_families(db: &mut usvg::fontdb::Database) {
    db.set_serif_family(FALLBACK_PROPORTIONAL_FONT_FAMILY);
    db.set_sans_serif_family(FALLBACK_PROPORTIONAL_FONT_FAMILY);
    db.set_cursive_family(FALLBACK_PROPORTIONAL_FONT_FAMILY);
    db.set_fantasy_family(FALLBACK_PROPORTIONAL_FONT_FAMILY);
    db.set_monospace_family(FALLBACK_MONOSPACE_FONT_FAMILY);
}

fn effective_scale(width: f32, height: f32, max_width: u32, content_scale: u32) -> f32 {
    let logical_scale = if content_scale > 100 {
        1.0
    } else {
        (max_width as f32 / width.max(1.0)).min(1.0)
    };
    let requested_scale = logical_scale * content_scale as f32 / 100.0;
    let width_scale = MAX_RASTERIZED_SVG_EDGE / width.max(1.0);
    let height_scale = MAX_RASTERIZED_SVG_EDGE / height.max(1.0);
    requested_scale
        .min(width_scale)
        .min(height_scale)
        .max(f32::MIN_POSITIVE)
}

#[cfg(test)]
#[path = "export_surface_svg_raster_tests.rs"]
mod tests;
