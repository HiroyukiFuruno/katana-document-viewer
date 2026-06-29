use crate::KdvThemeSnapshot;
use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface::SurfaceBlock;
use crate::export_surface::SurfaceBlockFactory;
use crate::export_surface::export_surface_blocks::SurfaceTableLayout;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;
use crate::export_surface_helpers::{SURFACE_CONTENT_WIDTH, SurfaceHelpers};
use katana_markdown_model::TableAlignment;

const TABLE_CELL_PADDING: u32 = 16;

#[test]
fn pdf_surface_table_is_a_table_block() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "table.md",
        table_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "table:3x2",
            "コンポーネント  役割",
            "PreviewPane  セクション管理",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(&debug, &["| コンポーネント |", "|---|"]);
    Ok(())
}

#[test]
fn pdf_surface_table_right_alignment_keeps_cell_padding() {
    let cell_x = 120;
    let cell_width = 360;
    let text = "テキスト";
    let text_x = SurfaceTableLayout::cell_text_x(text, &TableAlignment::Right, cell_x, cell_width);
    let text_right = text_x + SurfaceTableLayout::estimated_cell_text_width(text);

    assert!(
        text_right <= cell_x + cell_width - 16,
        "right-aligned table text must not run into the cell border"
    );
}

#[test]
fn pdf_surface_table_uses_html_like_padding_and_theme_colors()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = KdvThemeSnapshot::katana_light();
    assert_eq!(theme.table_header_background, "#f3f3f3");
    assert_eq!(theme.table_even_row_background, "#ffffff");

    let (row_height, text_y) =
        surface_table_metrics_from_markdown(table_markdown()).ok_or("table fixture must parse")?;

    assert!(row_height >= 66, "table rows need vertical breathing room");
    assert!(
        text_y >= TABLE_CELL_PADDING,
        "table text must be vertically centered with padding"
    );
    Ok(())
}

#[test]
fn pdf_surface_table_paints_active_theme_table_colors() -> Result<(), Box<dyn std::error::Error>> {
    let theme = active_table_theme();
    let surface = themed_table_surface(&theme)?;

    assert_surface_has_theme_color(&surface.image, &theme.table_header_background, 500);
    assert_surface_has_theme_color(&surface.image, &theme.table_even_row_background, 500);
    assert_surface_has_theme_color(&surface.image, &theme.table_border, 100);
    Ok(())
}

fn active_table_theme() -> KdvThemeSnapshot {
    let mut theme = KdvThemeSnapshot::katana_dark();
    theme.table_border = "#112233".to_string();
    theme.table_header_background = "#223344".to_string();
    theme.table_even_row_background = "#334455".to_string();
    theme
}

fn themed_table_surface(
    theme: &KdvThemeSnapshot,
) -> Result<crate::export_surface::DocumentSurface, Box<dyn std::error::Error>> {
    let graph = SurfaceTestSupport::graph_from_markdown("table-theme.md", table_markdown())?;
    Ok(DocumentSurfaceFactory::create(&graph, theme))
}

fn assert_surface_has_theme_color(image: &image::RgbaImage, color: &str, min_pixels: usize) {
    let pixel_count = count_exact_pixels(image, SurfaceHelpers::parse_color(color));
    assert!(
        pixel_count > min_pixels,
        "table surface must use active theme color {color}: {pixel_count}"
    );
}

#[test]
fn pdf_surface_empty_code_block_keeps_visible_code_area() -> Result<(), Box<dyn std::error::Error>>
{
    let box_height = surface_code_box_height_from_markdown(empty_code_markdown())
        .ok_or("code fixture must parse")?;

    assert!(
        box_height >= 56,
        "empty code block must not collapse into a horizontal line"
    );
    Ok(())
}

#[test]
fn pdf_surface_empty_code_block_paints_visible_code_area() -> Result<(), Box<dyn std::error::Error>>
{
    let graph = SurfaceTestSupport::graph_from_markdown("empty-code.md", empty_code_markdown())?;
    let theme = KdvThemeSnapshot::katana_light();
    let surface = DocumentSurfaceFactory::create(&graph, &theme);
    let code_background = SurfaceHelpers::parse_color(&theme.code_background);
    let painted_rows = count_rows_with_code_background(&surface.image, code_background);

    assert!(
        painted_rows >= 48,
        "empty code block must paint a visible rectangular area, not a thin line: {painted_rows}"
    );
    Ok(())
}

fn table_markdown() -> String {
    [
        "| コンポーネント | 役割 |",
        "|---|---|",
        "| `PreviewPane` | **セクション管理** |",
        "| `show_content` | UI描画 |",
    ]
    .join("\n")
}

fn empty_code_markdown() -> String {
    ["# empty code", "", "```", "```"].join("\n")
}

fn surface_table_metrics_from_markdown(markdown: String) -> Option<(u32, u32)> {
    let graph = SurfaceTestSupport::graph_from_markdown("table.md", markdown).ok()?;
    let blocks = SurfaceBlockFactory::create(&graph, &graph.theme);
    blocks.iter().find_map(|block| match block {
        SurfaceBlock::Table(table) => {
            let column_width = SURFACE_CONTENT_WIDTH / table.column_count() as u32;
            let row_height = table.row_height(0, column_width);
            Some((row_height, SurfaceTableLayout::cell_text_y(row_height, 1)))
        }
        _ => None,
    })
}

fn surface_code_box_height_from_markdown(markdown: String) -> Option<u32> {
    let graph = SurfaceTestSupport::graph_from_markdown("code.md", markdown).ok()?;
    let blocks = SurfaceBlockFactory::create(&graph, &graph.theme);
    blocks.iter().find_map(|block| match block {
        SurfaceBlock::Code(code) => Some(code.box_height()),
        _ => None,
    })
}

fn count_rows_with_code_background(image: &image::RgbaImage, color: image::Rgba<u8>) -> usize {
    (0..image.height())
        .filter(|y| {
            image
                .pixels()
                .skip((*y * image.width()) as usize)
                .take(image.width() as usize)
                .filter(|pixel| **pixel == color)
                .count()
                > image.width() as usize / 2
        })
        .count()
}

fn count_exact_pixels(image: &image::RgbaImage, color: image::Rgba<u8>) -> usize {
    image.pixels().filter(|pixel| **pixel == color).count()
}
