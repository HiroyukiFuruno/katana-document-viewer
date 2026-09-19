use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::ViewerHtmlRole;
use katana_markdown_model::{HtmlBlockRole, KmmNodeKind};

use super::html_height_test_support::{input_with_font_size, node};

const VALID_DATA_SVG: &str = r#"<p align="center"><img src="data:image/svg+xml,%3Csvg xmlns=%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22 width=%22128%22 height=%22128%22%3E%3Crect width=%22128%22 height=%22128%22 fill=%22%23ddd%22/%3E%3C/svg%3E" width="128" alt="icon"></p>"#;
const NORMALIZABLE_DATA_SVG: &str = r#"<p align="center"><img src="data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22> width=%22128%22 height=%22128%22%3E%3Crect width=%22128%22 height=%22128%22 fill=%22%23ddd%22/%3E%3C/svg%3E" width="128" alt="icon"></p>"#;
const UNREPAIRABLE_IMAGE_SOURCE: &str =
    r#"<p align="center"><img src="custom:payload<broken> visible tail" alt="icon"></p>"#;

#[test]
fn planner_keeps_html_data_svg_image_height_for_viewer_surface() {
    let input = html_input(VALID_DATA_SVG);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_centered_html(&plan.nodes[0].kind);
    assert_eq!(162.0, plan.nodes[0].rect.height);
}

#[test]
fn export_planner_keeps_normalizable_katana_data_svg_image_height() {
    let input = html_input(NORMALIZABLE_DATA_SVG);

    let plan = ViewerNodePlanner::create_export_surface(&input, 0.0);

    assert_eq!(164.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_does_not_promote_a_malformed_quoted_attribute_to_image_height() {
    let input = html_input(UNREPAIRABLE_IMAGE_SOURCE);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_centered_html(&plan.nodes[0].kind);
    assert_eq!(21.0, plan.nodes[0].rect.height);
}

#[test]
fn export_planner_does_not_promote_an_unrepairable_image_source() {
    let input = html_input(UNREPAIRABLE_IMAGE_SOURCE);

    let plan = ViewerNodePlanner::create_export_surface(&input, 0.0);

    assert_centered_html(&plan.nodes[0].kind);
    assert_eq!(23.0, plan.nodes[0].rect.height);
}

fn html_input(source: &str) -> crate::ViewerInput {
    input_with_font_size(
        vec![node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Centered),
            source,
            Vec::new(),
        )],
        14,
    )
}

fn assert_centered_html(kind: &ViewerNodeKind) {
    assert_eq!(
        &ViewerNodeKind::Html {
            role: ViewerHtmlRole::Centered,
        },
        kind
    );
}
