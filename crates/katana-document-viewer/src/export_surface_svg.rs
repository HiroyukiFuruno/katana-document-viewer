use image::RgbaImage;
use resvg::{render, usvg};
use tiny_skia::Pixmap;

const MAX_RASTERIZED_SVG_EDGE: f32 = 8192.0;
const LIGHT_DARK_FUNCTION: &str = "light-dark(";

pub(crate) struct SurfaceSvgImage {
    pub(crate) image: RgbaImage,
}

pub(crate) struct SurfaceSvgRasterizer;

impl SurfaceSvgRasterizer {
    pub(crate) fn rasterize(svg_text: &str, max_width: u32) -> Option<SurfaceSvgImage> {
        let compatible_svg = preprocess_for_rasterizer(svg_text);
        let tree = usvg::Tree::from_str(&compatible_svg, &rasterizer_options()).ok()?;
        let target = RasterTarget::new(tree.size(), max_width);
        let pixmap = target.render(&tree)?;
        let image = RgbaImage::from_raw(target.width, target.height, pixmap.take())?;
        Some(SurfaceSvgImage { image })
    }
}

struct RasterTarget {
    effective_scale: f32,
    width: u32,
    height: u32,
}

impl RasterTarget {
    fn new(size: usvg::Size, max_width: u32) -> Self {
        let display_width = size.width();
        let display_height = size.height();
        let effective_scale = effective_scale(display_width, display_height, max_width);
        Self {
            effective_scale,
            width: ((display_width * effective_scale).ceil() as u32).max(1),
            height: ((display_height * effective_scale).ceil() as u32).max(1),
        }
    }

    fn render(&self, tree: &usvg::Tree) -> Option<Pixmap> {
        let mut pixmap = Pixmap::new(self.width, self.height)?;
        let transform =
            tiny_skia::Transform::from_scale(self.effective_scale, self.effective_scale);
        render(tree, transform, &mut pixmap.as_mut());
        Some(pixmap)
    }
}

fn preprocess_for_rasterizer(svg_text: &str) -> String {
    let with_xml_entities = svg_text.replace("&nbsp;", "&#160;");
    let without_foreign_objects = strip_foreign_objects(&with_xml_entities);
    resolve_light_dark_functions(&without_foreign_objects)
}

fn strip_foreign_objects(svg_text: &str) -> String {
    let mut output = String::with_capacity(svg_text.len());
    let mut remaining = svg_text;
    while let Some(start) = remaining.to_ascii_lowercase().find("<foreignobject") {
        output.push_str(&remaining[..start]);
        let after_open = &remaining[start..];
        let lower_after_open = after_open.to_ascii_lowercase();
        if let Some(self_close) = lower_after_open.find("/>") {
            remaining = &after_open[self_close + "/>".len()..];
            continue;
        }
        let Some(close) = lower_after_open.find("</foreignobject>") else {
            output.push_str(after_open);
            return output;
        };
        remaining = &after_open[close + "</foreignobject>".len()..];
    }
    output.push_str(remaining);
    output
}

fn resolve_light_dark_functions(svg_text: &str) -> String {
    let mut result = String::with_capacity(svg_text.len());
    let mut remaining = svg_text;
    while let Some(start) = find_light_dark_function(remaining) {
        let content_start = start + LIGHT_DARK_FUNCTION.len();
        result.push_str(&remaining[..start]);
        let Some((content_end, light_color)) =
            parse_light_dark_function(&remaining[content_start..])
        else {
            result.push_str(&remaining[start..content_start]);
            remaining = &remaining[content_start..];
            continue;
        };
        result.push_str(light_color.trim());
        remaining = &remaining[content_start + content_end + 1..];
    }
    result.push_str(remaining);
    result
}

fn find_light_dark_function(text: &str) -> Option<usize> {
    text.to_ascii_lowercase().find(LIGHT_DARK_FUNCTION)
}

fn parse_light_dark_function(content: &str) -> Option<(usize, &str)> {
    let mut depth = 0usize;
    let mut comma = None;
    for (index, character) in content.char_indices() {
        match character {
            '(' => depth += 1,
            ')' if depth == 0 => return comma.map(|comma_index| (index, &content[..comma_index])),
            ')' => depth -= 1,
            ',' if depth == 0 && comma.is_none() => comma = Some(index),
            _ => {}
        }
    }
    None
}

fn rasterizer_options() -> usvg::Options<'static> {
    usvg::Options {
        // WHY: SVG内テキストはシステムフォントDBがないと消える場合がある。
        fontdb: font_db(),
        ..usvg::Options::default()
    }
}

fn font_db() -> std::sync::Arc<usvg::fontdb::Database> {
    static FONT_DB: std::sync::OnceLock<std::sync::Arc<usvg::fontdb::Database>> =
        std::sync::OnceLock::new();
    std::sync::Arc::clone(FONT_DB.get_or_init(|| {
        let mut db = usvg::fontdb::Database::new();
        db.load_system_fonts();
        std::sync::Arc::new(db)
    }))
}

fn effective_scale(width: f32, height: f32, max_width: u32) -> f32 {
    let requested_scale = (max_width as f32 / width.max(1.0)).min(1.0);
    let width_scale = MAX_RASTERIZED_SVG_EDGE / width.max(1.0);
    let height_scale = MAX_RASTERIZED_SVG_EDGE / height.max(1.0);
    requested_scale
        .min(width_scale)
        .min(height_scale)
        .max(f32::MIN_POSITIVE)
}
