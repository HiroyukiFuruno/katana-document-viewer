use crate::theme::KdvThemeSnapshot;

use katana_markdown_model::HtmlBlockRole;

use super::support::{graph, node};

use crate::export_surface::{SurfaceBlock, SurfaceBlockFactory};

use crate::export_surface_line::SurfaceLine;

#[test]
fn html_heading_is_rendered_as_body_line_for_export_reference() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node(r#"<h1 align="center">Centered Heading</h1>"#);
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "Centered Heading");
    assert!(!blocks[0].is_heading());
    assert_eq!(
        blocks[0].height(),
        SurfaceLine::body("h1".to_string()).line_height()
    );
    assert!(blocks[0].debug_for_tests().contains("line:[\"centered\"]"));
}

#[test]
fn heading_level_parses_known_levels_and_skips_unknown_tags() {
    assert_eq!(
        SurfaceBlockFactory::heading_level("<h1>Title</h1>"),
        Some(1)
    );
    assert_eq!(
        SurfaceBlockFactory::heading_level("<H6>Title</H6>"),
        Some(6)
    );
    assert_eq!(SurfaceBlockFactory::heading_level("<p>Body</p>"), None);
}

#[test]
fn heading_html_uses_right_alignment_when_requested() -> Result<(), String> {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<h1 align=right>Right Heading</h1>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    let line = if let SurfaceBlock::Line(line) = &blocks[0] {
        line
    } else {
        return Err("expected right-aligned heading to map to a line block".to_string());
    };
    assert!(line.is_right_aligned());
    Ok(())
}

#[test]
fn heading_html_uses_center_alignment_when_requested() -> Result<(), String> {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<h1 align=center>Center Heading</h1>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(1, blocks.len());
    let line = if let SurfaceBlock::Line(line) = &blocks[0] {
        line
    } else {
        return Err("expected centered heading to map to a line block".to_string());
    };
    assert!(line.is_centered());
    Ok(())
}

#[test]
fn heading_html_uses_body_lines_when_no_alignment() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<h1>Body Heading</h1>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(1, blocks.len());
    assert_eq!(blocks[0].text_for_tests(), "Body Heading");
}

#[test]
fn details_html_is_split_into_summary_and_body_lines() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<details><summary>Summary</summary><div>Body</div></details>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "Summary");
    assert_eq!(blocks[1].text_for_tests(), "Body");
}

#[test]
fn details_html_falls_back_to_raw_body_when_markdown_nodes_are_empty() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<details><summary>Summary</summary> </details>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "Summary");
    assert_eq!(blocks[1].text_for_tests(), "");
}

#[test]
fn heading_html_falls_back_to_plain_text_when_spans_empty() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<h1>   </h1>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(1, blocks.len());
    assert_eq!(blocks[0].text_for_tests(), "");
}
