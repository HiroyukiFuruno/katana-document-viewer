use super::contract_test_fixtures::{
    code_markdown, contract_markdown, diagram_markdown, first_code_line_y, first_diagram_sample_y,
    math_markdown, red_rect_svg, styled_svg, surface_markdown,
};
use crate::KdvThemeSnapshot;
use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;
use crate::export_surface_helpers::PAGE_PADDING;
#[test]
fn surface_lines_use_evaluated_content_instead_of_raw_markdown_html_or_diagram_source()
-> Result<(), Box<dyn std::error::Error>> {
    let joined = SurfaceTestSupport::surface_text(
        &SurfaceTestSupport::graph_with_rendered_diagram(surface_markdown())?,
    );

    SurfaceTestSupport::assert_contains_all(&joined, surface_expected_text());
    SurfaceTestSupport::assert_not_contains_any(&joined, surface_forbidden_raw_text());
    Ok(())
}

#[test]
fn surface_contract_covers_markdown_alert_katana_math_and_diagram_categories()
-> Result<(), Box<dyn std::error::Error>> {
    let joined = SurfaceTestSupport::surface_text(
        &SurfaceTestSupport::graph_with_rendered_diagram(contract_markdown())?,
    );

    SurfaceTestSupport::assert_contains_all(&joined, contract_expected_text());
    SurfaceTestSupport::assert_not_contains_any(&joined, contract_forbidden_raw_text());
    Ok(())
}

fn surface_expected_text() -> &'static [&'static str] {
    &[
        "装飾",
        "太字",
        "リンク",
        "アイコン",
        "English",
        "コンポーネント",
        "PreviewPane",
        "Rendered diagram",
    ]
}

fn surface_forbidden_raw_text() -> &'static [&'static str] {
    &[
        "**太字**",
        "[リンク](",
        "![アイコン]",
        "<p",
        "<img",
        "| コンポーネント |",
        "```mermaid",
        "graph TD",
    ]
}

fn contract_expected_text() -> &'static [&'static str] {
    &[
        "太字",
        "斜体",
        "取り消し",
        "code",
        "リンク",
        "アイコン",
        "Warning",
        "危険",
        "進行中",
        "保留",
        "詳細",
        "刀",
        "math-svg:inline",
        "Rendered diagram",
    ]
}

fn contract_forbidden_raw_text() -> &'static [&'static str] {
    &[
        "**太字**",
        "*斜体*",
        "~~取り消し~~",
        "`code`",
        "[リンク](",
        "![アイコン]",
        "[!WARNING]",
        "[/]",
        "[-]",
        "<details>",
        "<summary>",
        "$a^2",
        "$$",
        "```math",
        "```mermaid",
        "graph TD",
    ]
}

#[test]
fn surface_diagram_does_not_leak_svg_style_source() -> Result<(), Box<dyn std::error::Error>> {
    let joined = SurfaceTestSupport::surface_text(
        &SurfaceTestSupport::graph_with_rendered_diagram_svg(diagram_markdown(), styled_svg())?,
    );

    SurfaceTestSupport::assert_contains_all(&joined, &["Rendered diagram"]);
    SurfaceTestSupport::assert_not_contains_any(
        &joined,
        &[
            "#katana-mermaid-svg",
            "@keyframes",
            "edge-animation",
            "stroke-dasharray",
            "font-family",
            "<style",
        ],
    );
    Ok(())
}

#[test]
fn surface_diagram_svg_is_rasterized_into_native_surface() -> Result<(), Box<dyn std::error::Error>>
{
    let graph =
        SurfaceTestSupport::graph_with_rendered_diagram_svg(diagram_markdown(), red_rect_svg())?;
    let theme = KdvThemeSnapshot::katana_light();
    let surface = DocumentSurfaceFactory::create(&graph, &theme);

    let sampled = surface.image.get_pixel(640, first_diagram_sample_y());

    assert_eq!(*sampled, image::Rgba([255, 0, 0, 255]));
    Ok(())
}

#[test]
fn surface_code_block_is_painted_with_code_background() -> Result<(), Box<dyn std::error::Error>> {
    let graph = SurfaceTestSupport::graph_from_markdown("code.md", code_markdown())?;
    let theme = KdvThemeSnapshot::katana_light();
    let code_y = first_code_line_y();
    let surface = DocumentSurfaceFactory::create(&graph, &theme);

    let sampled = surface.image.get_pixel(PAGE_PADDING + 10, code_y);

    assert_eq!(*sampled, image::Rgba([246, 248, 250, 255]));
    Ok(())
}

#[test]
fn surface_math_uses_shared_svg_blocks_instead_of_raw_tex() -> Result<(), Box<dyn std::error::Error>>
{
    let joined = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        "math.md",
        math_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(&joined, &["math-svg:rendered", "math-svg:inline"]);
    SurfaceTestSupport::assert_not_contains_any(
        &joined,
        &["\\int", "\\frac", "mc^2", "\\sum_", "```math", "$$"],
    );
    Ok(())
}
