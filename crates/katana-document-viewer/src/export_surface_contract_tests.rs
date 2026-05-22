use super::test_support::{
    assert_contains_all, assert_not_contains_any, graph_from_markdown, graph_with_rendered_diagram,
    graph_with_rendered_diagram_svg, surface_text,
};
use crate::KdvThemeSnapshot;
use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface_helpers::PAGE_PADDING;
use crate::export_surface_line::SurfaceLine;

#[test]
fn surface_lines_use_evaluated_content_instead_of_raw_markdown_html_or_diagram_source()
-> Result<(), Box<dyn std::error::Error>> {
    let joined = surface_text(&graph_with_rendered_diagram(surface_markdown())?);

    assert_contains_all(
        &joined,
        &[
            "装飾",
            "太字",
            "リンク",
            "アイコン",
            "English",
            "コンポーネント",
            "PreviewPane",
            "Rendered diagram",
        ],
    );
    assert_not_contains_any(
        &joined,
        &[
            "**太字**",
            "[リンク](",
            "![アイコン]",
            "<p",
            "<img",
            "| コンポーネント |",
            "```mermaid",
            "graph TD",
        ],
    );
    Ok(())
}

#[test]
fn surface_contract_covers_markdown_alert_katana_math_and_diagram_categories()
-> Result<(), Box<dyn std::error::Error>> {
    let joined = surface_text(&graph_with_rendered_diagram(contract_markdown())?);

    assert_contains_all(
        &joined,
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
            "a² + b² = c²",
            "Rendered diagram",
        ],
    );
    assert_not_contains_any(
        &joined,
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
        ],
    );
    Ok(())
}

#[test]
fn surface_diagram_does_not_leak_svg_style_source() -> Result<(), Box<dyn std::error::Error>> {
    let joined = surface_text(&graph_with_rendered_diagram_svg(
        diagram_markdown(),
        styled_svg(),
    )?);

    assert_contains_all(&joined, &["Rendered diagram"]);
    assert_not_contains_any(
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
    let graph = graph_with_rendered_diagram_svg(diagram_markdown(), red_rect_svg())?;
    let theme = KdvThemeSnapshot::katana_light();
    let surface = DocumentSurfaceFactory::create(&graph, &theme);

    let sampled = surface.image.get_pixel(640, first_diagram_sample_y());

    assert_eq!(*sampled, image::Rgba([255, 0, 0, 255]));
    Ok(())
}

#[test]
fn surface_code_block_is_painted_with_code_background() -> Result<(), Box<dyn std::error::Error>> {
    let graph = graph_from_markdown("code.md", code_markdown())?;
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
    let joined = surface_text(&graph_from_markdown("math.md", math_markdown())?);

    assert_contains_all(&joined, &["math-svg:rendered", "mc²"]);
    assert_not_contains_any(
        &joined,
        &["\\int", "\\frac", "mc^2", "\\sum_", "```math", "$$"],
    );
    Ok(())
}

fn surface_markdown() -> String {
    [
        "# 装飾",
        "",
        "**太字** と [リンク](https://example.com) と ![アイコン](icon.png)",
        "",
        r#"<p align="center"><img alt="English" src="badge.svg"></p>"#,
        "",
        "| コンポーネント | 役割 |",
        "|---|---|",
        "| `PreviewPane` | **セクション管理** |",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
    .join("\n")
}

fn code_markdown() -> String {
    ["# コード", "", "```rust", "fn main() {}", "```"].join("\n")
}

fn math_markdown() -> String {
    [
        "# 数式",
        "",
        "```math",
        r"f(x) = \int_{0}^{x} \frac{t^2}{1 + t^4} \, dt",
        "```",
        "",
        "inline: $ E = mc^2 $",
        "",
        r"$$ \sum_{k=1}^{n} k = \frac{n(n+1)}{2} $$",
    ]
    .join("\n")
}

fn first_code_line_y() -> u32 {
    PAGE_PADDING
        + SurfaceLine::body("source".to_string()).line_height()
        + SurfaceLine::heading(1, "コード".to_string()).line_height()
}

fn first_diagram_sample_y() -> u32 {
    PAGE_PADDING
        + SurfaceLine::body("source".to_string()).line_height()
        + SurfaceLine::heading(1, "図形".to_string()).line_height()
        + 28
}

fn diagram_markdown() -> String {
    ["# 図形", "", "```mermaid", "graph TD", "  A --> B", "```"].join("\n")
}

fn red_rect_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="60">"#,
        r##"<rect width="120" height="60" fill="#ff0000"/>"##,
        "</svg>",
    ]
    .join("")
}

fn styled_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="240" height="120">"#,
        r#"<style>#katana-mermaid-svg{font-family:trebuchet ms;}@keyframes edge-animation{from{stroke-dasharray:0;}}</style>"#,
        r#"<text x="16" y="48">Rendered diagram</text>"#,
        "</svg>",
    ]
    .join("")
}

fn contract_markdown() -> String {
    [
        "# 契約",
        "",
        "**太字** *斜体* ~~取り消し~~ `code` [リンク](https://example.com) ![アイコン](icon.png)",
        "",
        "> [!WARNING]",
        "> **危険** な操作です。",
        "",
        "- [/] 進行中",
        "- [-] 保留",
        "",
        "<details><summary>詳細</summary>",
        "",
        "- 刀",
        "",
        "</details>",
        "",
        "inline math: $a^2 + b^2 = c^2$",
        "",
        "```math",
        "a^2 + b^2 = c^2",
        "```",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
    .join("\n")
}
