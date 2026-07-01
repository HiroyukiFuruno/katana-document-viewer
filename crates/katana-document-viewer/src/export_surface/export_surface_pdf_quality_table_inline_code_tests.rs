use crate::KdvThemeSnapshot;
use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_table_inline_code_does_not_paint_light_code_background()
-> Result<(), Box<dyn std::error::Error>> {
    let mut theme = KdvThemeSnapshot::katana_dark();
    theme.table_header_background = "#1f2937".to_string();
    theme.table_even_row_background = "#111827".to_string();
    let graph = SurfaceTestSupport::graph_from_markdown(
        "issue-17-table-code.md",
        architecture_table_markdown(),
    )?;
    let surface = DocumentSurfaceFactory::create(&graph, &theme);
    let light_inline_code_background = image::Rgba([239, 242, 246, 255]);
    let light_code_pixels = surface
        .image
        .pixels()
        .filter(|pixel| **pixel == light_inline_code_background)
        .count();

    assert_eq!(
        light_code_pixels, 0,
        "table-cell inline code must not paint the light inline-code background in dark PDF tables"
    );
    Ok(())
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
