use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{ViewerHtmlAlignment, ViewerHtmlRole};
use katana_markdown_model::{HtmlBlockRole, KmmNodeKind};

use super::html_height_test_support::{input_with_font_size, node};

#[test]
fn planner_keeps_aligned_html_heading_height_for_viewer_surface() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
            r#"<h1 align="center">KatanA Desktop</h1>"#,
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_html_heading_node(&plan.nodes[0]);
    assert_eq!(43.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_keeps_paragraph_wrapped_html_heading_height_for_viewer_surface() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::Paragraph,
            r#"<h1 align="center">KatanA Desktop</h1>"#,
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_html_heading_node(&plan.nodes[0]);
    assert_eq!(43.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_keeps_raw_block_html_heading_height_for_viewer_surface() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::RawBlock {
                reason: "html".to_string(),
            },
            r#"<h1 align="center">KatanA Desktop</h1>"#,
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_html_heading_node(&plan.nodes[0]);
    assert_eq!("KatanA Desktop", plan.nodes[0].text);
    assert_eq!(43.0, plan.nodes[0].rect.height);
}

fn assert_html_heading_node(node: &super::ViewerNode) {
    assert_eq!(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Heading {
                level: 1,
                alignment: ViewerHtmlAlignment::Center,
            }
        },
        node.kind
    );
}
