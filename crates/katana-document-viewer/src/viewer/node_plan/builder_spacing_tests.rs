use super::ViewerNodePlanner;
use super::test_support::{input_with_nodes, node, text_node};
use crate::ViewerNodeKind;
use katana_markdown_model::{CodeBlockRole, DiagramKind, HeadingNode, HtmlBlockRole, KmmNodeKind};

#[test]
fn planner_uses_preview_gap_between_top_level_nodes() {
    let input = input_with_nodes(vec![
        heading_node(),
        node(KmmNodeKind::Paragraph, "body", vec![text_node("body")]),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_eq!(plan.nodes[0].rect.height + 20.0, plan.nodes[1].rect.y);
    assert_eq!(
        plan.nodes[0].rect.height + 20.0 + plan.nodes[1].rect.height,
        plan.content_height
    );
}

#[test]
fn planner_uses_katana_context_gaps_around_html_rule_and_heading() {
    let mut input = input_with_nodes(vec![
        node(KmmNodeKind::Paragraph, "body", vec![text_node("body")]),
        node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Centered),
            r#"<p align="center">English | <a href="sample_diagrams.ja.md">日本語</a></p>"#,
            Vec::new(),
        ),
        node(KmmNodeKind::ThematicBreak, "---", Vec::new()),
        node(
            KmmNodeKind::Heading(HeadingNode {
                level: 2,
                text: "1. Diagrams — Mermaid".to_string(),
            }),
            "## 1. Diagrams — Mermaid",
            Vec::new(),
        ),
    ]);
    input.typography.preview_font_size = 14;
    input.viewport.width = 1280.0;

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(4, plan.nodes.len());
    assert_eq!(0.0, plan.nodes[0].rect.y);
    assert_eq!(
        plan.nodes[0].rect.y + plan.nodes[0].rect.height + 16.0,
        plan.nodes[1].rect.y,
        "paragraph to centered HTML must follow KatanA compact newline spacing"
    );
    assert_eq!(
        plan.nodes[1].rect.y + plan.nodes[1].rect.height + 14.0,
        plan.nodes[2].rect.y,
        "centered HTML to rule must not use the generic block gap"
    );
    assert_eq!(
        9, plan.nodes[2].rule_line_offset_px,
        "only the centered language selector rule needs KatanA's lowered rule line"
    );
    assert_eq!(
        plan.nodes[2].rect.y + plan.nodes[2].rect.height + 14.0,
        plan.nodes[3].rect.y,
        "rule to heading must follow the current KatanA reference crop spacing"
    );
}

#[test]
fn planner_does_not_apply_centered_html_rule_line_offset_to_regular_rules() {
    let mut input = input_with_nodes(vec![
        node(KmmNodeKind::Paragraph, "body", vec![text_node("body")]),
        node(KmmNodeKind::ThematicBreak, "---", Vec::new()),
        node(
            KmmNodeKind::Heading(HeadingNode {
                level: 2,
                text: "5. Tables (GFM)".to_string(),
            }),
            "## 5. Tables (GFM)",
            Vec::new(),
        ),
    ]);
    input.typography.preview_font_size = 14;
    input.viewport.width = 1280.0;

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(3, plan.nodes.len());
    assert_eq!(ViewerNodeKind::Rule, plan.nodes[1].kind);
    assert_eq!(
        0, plan.nodes[1].rule_line_offset_px,
        "regular document separators must stay vertically centered"
    );
}

#[test]
fn planner_uses_katana_context_gap_between_centered_html_and_html_heading() {
    let mut input = input_with_nodes(vec![
        node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Centered),
            r#"<p align="center"><img src="data:image/svg+xml,%3Csvg%3E" width="128"></p>"#,
            Vec::new(),
        ),
        node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
            r#"<h1 align="center">KatanA Desktop</h1>"#,
            Vec::new(),
        ),
    ]);
    input.typography.preview_font_size = 14;
    input.viewport.width = 1280.0;

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_eq!(
        plan.nodes[0].rect.y + plan.nodes[0].rect.height + 17.0,
        plan.nodes[1].rect.y,
        "centered HTML image block to HTML heading must follow KatanA HTML block margin"
    );
}

#[test]
fn planner_uses_katana_html_badge_top_adjustment_after_heading() {
    let mut input = input_with_nodes(vec![
        node(
            KmmNodeKind::Heading(HeadingNode {
                level: 3,
                text: "1.4 Badge Row".to_string(),
            }),
            "### 1.4 Badge Row",
            Vec::new(),
        ),
        node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::BadgeRow),
            r##"<p align="center"><a href="#"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a></p>"##,
            Vec::new(),
        ),
    ]);
    input.typography.preview_font_size = 14;
    input.viewport.width = 1280.0;

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_eq!(
        plan.nodes[0].rect.y + plan.nodes[0].rect.height + 13.0,
        plan.nodes[1].rect.y,
        "Markdown heading to HTML badge row must include KatanA's -7px HTML block top adjustment"
    );
}

#[test]
fn planner_preserves_katana_long_h2_row_height_without_affecting_short_h2() {
    let mut input = input_with_nodes(vec![
        node(
            KmmNodeKind::Heading(HeadingNode {
                level: 2,
                text: "1. Diagrams — Mermaid".to_string(),
            }),
            "## 1. Diagrams — Mermaid",
            Vec::new(),
        ),
        node(
            KmmNodeKind::Heading(HeadingNode {
                level: 2,
                text: "1. HTML Centering (Past Bug: Elements Left-Aligned Instead of Centered)"
                    .to_string(),
            }),
            "## 1. HTML Centering (Past Bug: Elements Left-Aligned Instead of Centered)",
            Vec::new(),
        ),
    ]);
    input.typography.preview_font_size = 14;
    input.viewport.width = 1280.0;

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(34.0, plan.nodes[0].rect.height);
    assert_eq!(
        47.0, plan.nodes[1].rect.height,
        "long Markdown H2 headings must reserve the taller KatanA CommonMark row box"
    );
}

#[test]
fn planner_uses_katana_codeblock_gap_before_diagram_sections() {
    let input = input_with_nodes(vec![heading_node(), diagram_node()]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_eq!(plan.nodes[0].rect.height + 6.0, plan.nodes[1].rect.y);
}

#[test]
fn planner_measures_plain_paragraph_text_without_child_spans() {
    let text = "This document is a comprehensive sample that exercises every rendering feature of KatanA. Open it in KatanA's preview pane to visually verify that all elements render correctly.";
    let mut input = input_with_nodes(vec![node(
        KmmNodeKind::Paragraph,
        text,
        vec![text_node(text)],
    )]);
    input.typography.preview_font_size = 14;
    input.viewport.width = 1280.0;

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(1, plan.nodes.len());
    assert_eq!(46.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_export_surface_matches_reference_without_inserted_gap() {
    let input = input_with_nodes(vec![
        heading_node(),
        node(KmmNodeKind::Paragraph, "body", vec![text_node("body")]),
    ]);

    let plan = ViewerNodePlanner::create_export_surface(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_eq!(plan.nodes[0].rect.height, plan.nodes[1].rect.y);
    assert_eq!(
        plan.nodes[0].rect.height + plan.nodes[1].rect.height,
        plan.content_height
    );
}

fn heading_node() -> katana_markdown_model::KmmNode {
    node(
        KmmNodeKind::Heading(katana_markdown_model::HeadingNode {
            level: 1,
            text: "Title".to_string(),
        }),
        "# Title",
        Vec::new(),
    )
}

fn diagram_node() -> katana_markdown_model::KmmNode {
    node(
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::Mermaid,
        }),
        "```mermaid\ngraph TD\nA-->B\n```",
        Vec::new(),
    )
}
