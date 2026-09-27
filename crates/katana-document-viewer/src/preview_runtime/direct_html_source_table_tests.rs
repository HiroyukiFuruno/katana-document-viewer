use super::{
    PreviewError, ViewerNodeKind, ViewerNodePlanner, output_for_html, structural_container_html,
};

#[test]
fn direct_html_source_keeps_table_as_table_node() -> Result<(), PreviewError> {
    let output = output_for_html(structural_container_html())?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);

    assert!(
        plan.nodes
            .iter()
            .any(|node| matches!(node.kind, ViewerNodeKind::Table)),
        "{:#?}",
        plan.nodes
    );
    Ok(())
}

#[test]
fn direct_html_source_keeps_header_and_body_in_one_table_node() -> Result<(), PreviewError> {
    let html = [
        "<table>",
        "<thead><tr><th>Feature</th><th>Status</th></tr></thead>",
        "<tbody><tr><td>HTML alignment</td><td>covered</td></tr></tbody>",
        "</table>",
    ]
    .join("\n");
    let output = output_for_html(html)?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);
    let tables = plan
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, ViewerNodeKind::Table))
        .collect::<Vec<_>>();

    assert_eq!(1, tables.len(), "{:#?}", plan.nodes);
    assert_eq!("Feature | Status\nHTML alignment | covered", tables[0].text);
    Ok(())
}
