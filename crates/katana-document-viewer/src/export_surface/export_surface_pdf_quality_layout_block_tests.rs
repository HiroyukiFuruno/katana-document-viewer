use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

const FILLER_LINE_COUNT_BEFORE_DIAGRAM: u32 = 42;

#[test]
fn pdf_surface_keeps_structured_blocks_inside_regular_blockquote()
-> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "blockquote-children.md",
        structured_blockquote_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "太字の引用:[\"bold\"]",
            "リスト項目 1:[\"indent=0\", \"list-marker=bullet\", \"marker-column=36\", \"marker-paint=material-dot\"]",
            "let:[\"monospace\", \"color\"]",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(
        &debug,
        &["太字の引用 - リスト項目 1 - リスト項目 2 rust let"],
    );
    Ok(())
}

#[test]
fn pdf_surface_caps_diagram_width() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(
        &SurfaceTestSupport::graph_with_rendered_diagram_svg(diagram_markdown(), wide_svg())?,
    );

    assert!(debug.contains("diagram:860x430"), "{debug}");
    Ok(())
}

#[test]
fn pdf_surface_does_not_upscale_small_diagram_svg() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(
        &SurfaceTestSupport::graph_with_rendered_diagram_svg(diagram_markdown(), small_svg())?,
    );

    assert!(
        debug.contains("diagram:320x160"),
        "small diagrams must keep their natural size instead of expanding to page width: {debug}"
    );
    Ok(())
}

#[test]
fn pdf_surface_does_not_leave_heading_orphan_before_diagram()
-> Result<(), Box<dyn std::error::Error>> {
    let pages = SurfaceTestSupport::surface_page_texts(
        &SurfaceTestSupport::graph_with_rendered_diagram_svg(
            heading_orphan_markdown(),
            small_svg(),
        )?,
    );
    let Some(page) = pages
        .iter()
        .find(|page| page.contains("Heading before diagram"))
    else {
        return Err(format!("heading page is missing: {pages:#?}").into());
    };

    assert!(
        page.contains("Rendered diagram"),
        "heading and following diagram must stay on the same page: {pages:#?}"
    );
    Ok(())
}

fn structured_blockquote_markdown() -> String {
    [
        "> **太字の引用**",
        ">",
        "> - リスト項目 1",
        "> - リスト項目 2",
        "",
        "> ```rust",
        "> let quoted_code = true;",
        "> ```",
    ]
    .join("\n")
}

fn diagram_markdown() -> String {
    [
        "# diagram",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
    .join("\n")
}

fn wide_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="2000" height="1000">"#,
        r##"<rect width="2000" height="1000" fill="#ff0000"/>"##,
        "</svg>",
    ]
    .join("")
}

fn small_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="320" height="160">"#,
        r##"<rect width="320" height="160" fill="#ff0000"/>"##,
        "</svg>",
    ]
    .join("")
}

fn heading_orphan_markdown() -> String {
    let mut lines = vec!["# paged".to_string(), String::new()];
    for index in 1..=FILLER_LINE_COUNT_BEFORE_DIAGRAM {
        lines.push(format!("filler line {index}"));
        lines.push(String::new());
    }
    lines.extend([
        "## Heading before diagram".to_string(),
        String::new(),
        "```mermaid".to_string(),
        "graph TD".to_string(),
        "  A --> B".to_string(),
        "```".to_string(),
    ]);
    lines.join("\n")
}
