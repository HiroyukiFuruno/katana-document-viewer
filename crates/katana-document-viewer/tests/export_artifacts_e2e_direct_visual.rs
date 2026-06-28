use std::error::Error;

#[path = "export_artifacts_e2e/direct_visual/mod.rs"]
mod direct_visual;

use direct_visual::{DirectVisualCases, ExportedDirectVisual};

#[test]
fn direct_visual_sources_export_html_pdf_png_and_jpeg_media() -> Result<(), Box<dyn Error>> {
    let output_dir = direct_visual::unique_output_dir()?;
    std::fs::create_dir_all(&output_dir)?;
    let cases = DirectVisualCases::create(&output_dir)?;

    for case in cases.items {
        let artifacts = ExportedDirectVisual::export(case.clone(), &output_dir)?;

        case.assert_html_media(std::str::from_utf8(&artifacts.html)?);
        artifacts.assert_quality(&case)?;
    }
    Ok(())
}
