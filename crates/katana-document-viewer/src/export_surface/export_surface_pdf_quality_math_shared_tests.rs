use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

const ALPHA_CHANNEL_INDEX: usize = 3;
const FULL_ALPHA: u8 = 255;
const CODE_PIXEL_RED_MIN: u8 = 120;
const CODE_PIXEL_GREEN_MAX: u8 = 80;
const CODE_PIXEL_BLUE_MIN: u8 = 80;
const INLINE_MATH_TEST_MAX_WIDTH: u32 = 240;

#[test]
fn pdf_surface_renders_math_from_shared_svg() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "math.md",
        [
            "# math",
            "",
            "```math",
            r"f(x) = \int_{0}^{x} \frac{t^2}{1 + t^4} \, dt",
            "```",
            "",
            r"$$ \sum_{k=1}^{n} k = \frac{n(n+1)}{2} $$",
        ]
        .join("\n"),
    )?);

    SurfaceTestSupport::assert_contains_all(&debug, &["math-svg:"]);
    SurfaceTestSupport::assert_not_contains_any(
        &debug,
        &[r"\frac", "lower=", "upper=", "math:f(x) = ∫"],
    );
    Ok(())
}

#[test]
fn pdf_surface_keeps_sample_math_fraction_on_same_page() -> Result<(), Box<dyn std::error::Error>> {
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/rendering/sample.ja.md");
    let markdown = std::fs::read_to_string(&fixture)?;
    let pages = SurfaceTestSupport::surface_page_texts(&SurfaceTestSupport::graph_from_markdown(
        &fixture.display().to_string(),
        markdown,
    )?);

    let page = pages
        .iter()
        .find(|page| page.contains("math-svg:"))
        .ok_or("integral page is missing")?;

    assert!(
        !page.contains(r"\frac"),
        "display math fraction must not be split across pages: {pages:#?}"
    );
    Ok(())
}

#[test]
fn pdf_surface_keeps_emoji_sequence_on_one_line() -> Result<(), Box<dyn std::error::Error>> {
    let text = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        "emoji.md",
        emoji_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(&text, &["絵文字: 🦀 ⚡ 📝 🔧 ✅ ❌ ⚠️ 💡 ⭐"]);
    Ok(())
}

#[test]
fn pdf_surface_renders_code_highlight_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let graph = SurfaceTestSupport::graph_from_markdown("code.md", code_markdown())?;
    let surface = crate::export_surface::DocumentSurfaceFactory::create(
        &graph,
        &crate::KdvThemeSnapshot::katana_light(),
    );

    assert!(
        contains_non_black_code_pixel(&surface.image),
        "syntax color pixel missing"
    );
    Ok(())
}

#[test]
fn pdf_inline_math_does_not_exceed_body_line_height_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let inline = crate::render_runtime::KrrRenderRuntimeAdapter::render_math_tex_with_theme(
        r"E = mc^2",
        crate::render_runtime::KrrMathMode::Inline,
        Some(theme.krr_math_theme()),
    )
    .svg_payload()
    .and_then(|svg| {
        crate::export_surface_svg::SurfaceSvgRasterizer::rasterize(svg, INLINE_MATH_TEST_MAX_WIDTH)
    })
    .ok_or("inline math SVG should rasterize")?;
    let body_line_height =
        crate::export_surface_line::SurfaceLine::body("".to_string()).line_height();

    assert!(
        inline.image.height() <= body_line_height,
        "inline math must fit body line height contract: image_height={}, body_line_height={}",
        inline.image.height(),
        body_line_height
    );
    Ok(())
}

fn code_markdown() -> String {
    [
        "# code",
        "",
        "```rust",
        "fn main() {",
        "  println!(\"hi\");",
        "}",
        "```",
    ]
    .join("\n")
}

fn emoji_markdown() -> String {
    ["# emoji", "", "絵文字: 🦀 ⚡ 📝 🔧 ✅ ❌ ⚠️ 💡 ⭐"].join("\n")
}

fn contains_non_black_code_pixel(image: &image::RgbaImage) -> bool {
    image.pixels().any(|pixel| {
        pixel[ALPHA_CHANNEL_INDEX] == FULL_ALPHA
            && pixel[0] > CODE_PIXEL_RED_MIN
            && pixel[1] < CODE_PIXEL_GREEN_MAX
            && pixel[2] > CODE_PIXEL_BLUE_MIN
    })
}
