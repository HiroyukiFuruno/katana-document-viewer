use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX, ViewerHtmlRole};
use katana_markdown_model::{HeadingNode, HtmlBlockRole, KmmNodeKind, TableNode};

use super::html_height_test_support::{input_with_font_size, input_with_nodes, node, table_row};

#[test]
fn planner_expands_details_accordion_height_for_multiline_body() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        "<details><summary>Show details</summary><div>\n\n- Swords\n  - Muramasa\n  - Masamune\n  - Kotetsu\n\n</div></details>",
        Vec::new(),
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Accordion
        },
        plan.nodes[0].kind
    );
    assert_eq!(230.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_keeps_export_surface_table_row_height_for_compact_font() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::Table(TableNode {
                alignments: Vec::new(),
                rows: vec![
                    table_row(&["Feature", "Status"]),
                    table_row(&["---", "---"]),
                    table_row(&["HTML", "covered"]),
                ],
            }),
            "Feature | Status\n--- | ---\nHTML | covered",
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(ViewerNodeKind::Table, plan.nodes[0].kind);
    assert_eq!(104.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_uses_rendered_rect_width_for_table_height() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![
                table_row(&["ABCDEFGHIJKLM", "B"]),
                table_row(&["---", "---"]),
                table_row(&["C", "D"]),
            ],
        }),
        "ABCDEFGHIJKLM | B\n--- | ---\nC | D",
        Vec::new(),
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(ViewerNodeKind::Table, plan.nodes[0].kind);
    assert_eq!(
        input.viewport.width
            - f32::from(KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX) * 2.0,
        plan.nodes[0].rect.width
    );
    assert_eq!(
        132.0, plan.nodes[0].rect.height,
        "table height must be measured against the rendered row width, not the wider host viewport"
    );
}

#[test]
fn planner_keeps_html_data_svg_image_height_for_viewer_surface() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Centered),
            r#"<p align="center"><img src="data:image/svg+xml,%3Csvg xmlns=%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22 width=%22128%22 height=%22128%22%3E%3Crect width=%22128%22 height=%22128%22 fill=%22%23ddd%22/%3E%3C/svg%3E" width="128" alt="icon"></p>"#,
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Centered
        },
        plan.nodes[0].kind
    );
    assert_eq!(162.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_preserves_html_margin_left_for_kuc_layout() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
            r#"<p style="margin-left: 40px"><a href="docs.md">日本語</a></p>"#,
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Generic
        },
        plan.nodes[0].kind
    );
    assert_eq!(40, plan.nodes[0].html_margin_left_px);
    assert_eq!(
        52.0, plan.nodes[0].rect.x,
        "HTML margin-left must affect the planned viewer rect, not only the KUC node wrapper margin"
    );
}

#[test]
fn planner_applies_katana_preview_content_padding_to_viewer_nodes() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::Heading(HeadingNode {
                level: 1,
                text: "KatanA Rendering Regression Test".to_string(),
            }),
            "# KatanA Rendering Regression Test",
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        f32::from(KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX),
        plan.nodes[0].rect.x,
        "interactive viewer nodes must carry KatanA preview content padding into KUC layout"
    );
    assert_eq!(
        input.viewport.width
            - f32::from(KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX) * 2.0,
        plan.nodes[0].rect.width
    );
}

#[test]
fn planner_does_not_promote_broken_katana_svg_data_uri_to_image_height() {
    let input = input_with_font_size(
        vec![node(
            KmmNodeKind::HtmlBlock(HtmlBlockRole::Centered),
            r#"<p align="center"><img src="data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22> width=%22128%22 height=%22128%22%3E%3Crect width=%22128%22 height=%22128%22 fill=%22%23ddd%22/%3E%3C/svg%3E" width="128" alt="icon"></p>"#,
            Vec::new(),
        )],
        14,
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Centered
        },
        plan.nodes[0].kind
    );
    assert_eq!(23.0, plan.nodes[0].rect.height);
}
