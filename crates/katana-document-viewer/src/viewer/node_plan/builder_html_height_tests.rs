use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX, ViewerHtmlRole};
use katana_markdown_model::{HeadingNode, HtmlBlockRole, KmmNodeKind, TableAlignment, TableNode};

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
fn planner_preserves_typed_table_cells_and_alignment_without_raw_text_parsing()
-> Result<(), Box<dyn std::error::Error>> {
    let table = typed_table_fixture();
    let projection = crate::ViewerTableProjection::from_kmm(&table);
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Table(table),
        "| unrelated flattened source |",
        Vec::new(),
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(2, projection.rows.len());
    assert_eq!("Typed header", projection.rows[0].cells[0].text);
    assert_eq!("Right body", projection.rows[1].cells[1].text);
    assert_eq!(
        crate::ViewerTableAlignment::Right,
        projection.rows[1].cells[1].alignment
    );
    assert_eq!(
        projection
            .row_heights(plan.nodes[0].rect.width as u32, input.typography)
            .into_iter()
            .sum::<u32>() as f32,
        plan.nodes[0].rect.height
    );
    Ok(())
}

fn typed_table_fixture() -> TableNode {
    TableNode {
        alignments: vec![TableAlignment::Left, TableAlignment::Right],
        rows: vec![
            table_row(&["Typed header", "Aligned header"]),
            table_row(&["---", "---:"]),
            table_row(&["Typed body", "Right body"]),
        ],
    }
}

#[test]
fn typed_table_projection_map_uses_snapshot_node_id() -> Result<(), &'static str> {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Table(typed_table_fixture()),
        "| source text is not the typed table |",
        Vec::new(),
    )]);
    let plan = ViewerNodePlanner::create(&input, 0.0);
    let projections = crate::ViewerTableProjection::from_input(&input);
    let projection = projections
        .get(&plan.nodes[0].node_id.0)
        .ok_or("typed projection by node id")?;

    assert_eq!("Typed header", projection.rows[0].cells[0].text);
    assert_eq!("Right body", projection.rows[1].cells[1].text);
    Ok(())
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
