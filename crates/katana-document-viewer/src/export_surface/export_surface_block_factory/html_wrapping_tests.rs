use crate::theme::KdvThemeSnapshot;

use katana_markdown_model::HtmlBlockRole;

use super::support::{graph, node};

use crate::export_surface::{SurfaceBlock, SurfaceBlockFactory};

use crate::export_surface_helpers::BODY_MAX_CHARS;

#[test]
fn generic_html_is_treated_as_wrapped_text() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<div>hello</div>");
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
    assert!(!blocks[0].text_for_tests().is_empty());
}

#[test]
fn centered_html_is_rendered_into_centered_line() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<center>Center</center>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Centered,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "Center");
}

#[test]
fn long_centered_html_falls_back_to_wrapped_centered_lines() {
    let graph = graph();
    let mut blocks = Vec::new();
    let long_text = "a".repeat(BODY_MAX_CHARS + 1);
    let source = format!("<center>{long_text}</center>");
    let node = node(&source);

    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Centered,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "a".repeat(BODY_MAX_CHARS));
    assert_eq!(blocks[1].text_for_tests(), "a");
}

#[test]
fn right_aligned_html_uses_right_line() -> Result<(), String> {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<p align=\"right\">Right</p>");
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
    assert_eq!(blocks[0].text_for_tests(), "Right");
    let line = if let SurfaceBlock::Line(line) = &blocks[0] {
        line
    } else {
        return Err("right aligned html should be exported as a line block".to_string());
    };
    assert!(line.is_right_aligned());
    Ok(())
}

#[test]
fn badge_row_html_creates_badge_block() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<div class=\"badge-row\">A</div>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::BadgeRow,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(!blocks[0].text_for_tests().is_empty());
}
