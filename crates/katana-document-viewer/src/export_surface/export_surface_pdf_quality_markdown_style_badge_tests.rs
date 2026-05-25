use crate::ExportFormat;
use crate::export_payload::ExportPayloadFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_evaluates_shields_badges_as_badge_text() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "badges.md",
        badges_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "License=MIT | CI=passing | platform=macOS",
            "platform=macOS | Windows | Linux",
            "[\"centered\"]",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(
        &debug,
        &["img.shields.io", "<img", "License: MIT CI Platform"],
    );
    Ok(())
}

#[test]
fn pdf_surface_renders_local_html_image_from_markdown_directory()
-> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!("kdv-local-image-{}", std::process::id()));
    let asset_dir = root.join("assets");
    std::fs::create_dir_all(&asset_dir)?;
    let image_path = asset_dir.join("icon.png");
    let image = image::RgbaImage::from_pixel(16, 16, image::Rgba([0, 120, 212, 255]));

    image.save(&image_path)?;
    let markdown_path = root.join("README.md");
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        markdown_path.to_string_lossy().as_ref(),
        r#"<p align="center"><img src="assets/icon.png" width="8" alt="icon"></p>"#.to_string(),
    )?);

    SurfaceTestSupport::assert_contains_all(&debug, &["image:8x8:icon"]);
    SurfaceTestSupport::assert_not_contains_any(&debug, &["assets/icon.png"]);
    Ok(())
}

#[test]
fn pdf_surface_creates_link_annotations_for_badges() -> Result<(), Box<dyn std::error::Error>> {
    let graph = SurfaceTestSupport::graph_from_markdown("badges.md", badges_markdown())?;
    let pdf = ExportPayloadFactory::create(
        &graph,
        ExportFormat::Pdf,
        &crate::KdvThemeSnapshot::katana_light(),
    )?;
    let text = String::from_utf8_lossy(&pdf);

    assert_eq!(
        text.matches("/Dest [").count(),
        3,
        "all badge images wrapped by links must keep clickable PDF areas",
    );
    assert_eq!(text.matches("/URI (#fn-1)").count(), 0);
    assert_eq!(text.matches("/URI (#fnref-1)").count(), 0);
    Ok(())
}

fn badges_markdown() -> String {
    [
        r#"<p align="center">"#,
        r##"<a href="#"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>"##,
        r##"<a href="#"><img src="https://img.shields.io/badge/CI-passing-brightgreen.svg" alt="CI"></a>"##,
        r##"<a href="#"><img src="https://img.shields.io/badge/platform-macOS-lightgrey" alt="Platform: macOS"></a>"##,
        r#"<img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey" alt="Platform: macOS | Windows | Linux">"#,
        r#"</p>"#,
    ]
    .join("\n")
}
