use crate::KdvThemeSnapshot;
use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_table_inline_code_does_not_paint_light_code_background()
-> Result<(), Box<dyn std::error::Error>> {
    let surface = dark_table_surface()?;
    let light_inline_code_background = image::Rgba([239, 242, 246, 255]);
    let themed_inline_code_background = image::Rgba([40, 40, 40, 255]);

    assert_eq!(
        count_pixels(&surface.image, light_inline_code_background),
        0,
        "table-cell inline code must not paint the light inline-code background in dark PDF tables"
    );
    assert!(
        count_pixels(&surface.image, themed_inline_code_background) > 0,
        "table-cell inline code must keep code decoration using the dark theme code background"
    );
    Ok(())
}

fn dark_table_surface() -> Result<crate::export_surface::DocumentSurface, Box<dyn std::error::Error>>
{
    let mut theme = KdvThemeSnapshot::katana_dark();
    theme.table_header_background = "#1f2937".to_string();
    theme.table_even_row_background = "#111827".to_string();
    let graph = SurfaceTestSupport::graph_from_markdown(
        "issue-17-table-code.md",
        architecture_table_markdown(),
    )?;
    Ok(DocumentSurfaceFactory::create(&graph, &theme))
}

fn count_pixels(image: &image::RgbaImage, color: image::Rgba<u8>) -> usize {
    image.pixels().filter(|pixel| **pixel == color).count()
}

fn architecture_table_markdown() -> String {
    [
        "### Architecture Overview",
        "",
        "| Component | Role |",
        "| --- | --- |",
        "| `PreviewPane` | Section management |",
        "| `show_content` | UI rendering |",
    ]
    .join("\n")
}
