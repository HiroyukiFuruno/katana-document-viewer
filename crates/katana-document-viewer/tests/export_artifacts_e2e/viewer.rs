use katana_document_viewer::{
    BuildGraph, KdvThemeSnapshot, ViewerHtmlRole, ViewerInput, ViewerInteractionConfig, ViewerMode,
    ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner, ViewerSearchState, ViewerTypographyConfig,
    ViewerViewport,
};

pub(crate) fn assert_viewer_plan_integration(graph: &BuildGraph) {
    let plan = ViewerNodePlanner::create(&viewer_input(graph), 0.0);
    assert_alert_text(&plan);
    assert_footnote_reference_link(&plan);
    assert_no_raw_footnote_marker(&plan);
    assert_commonmark_blocks_are_visible(&plan);
    assert_html_alignment_roles(&plan);
    assert_footnotes_are_last(&plan);
}

fn viewer_input(graph: &BuildGraph) -> ViewerInput {
    ViewerInput {
        snapshot: graph.snapshot.clone(),
        artifacts: Vec::new(),
        theme: KdvThemeSnapshot::katana_light(),
        mode: ViewerMode::Document,
        interaction: ViewerInteractionConfig::default(),
        typography: ViewerTypographyConfig::default(),
        viewport: ViewerViewport {
            width: 960.0,
            height: 720.0,
        },
        search: ViewerSearchState::default(),
    }
}

fn assert_alert_text(plan: &ViewerNodePlan) {
    assert!(
        plan.nodes.iter().any(|node| {
            matches!(&node.kind, ViewerNodeKind::Alert { label } if label == "WARNING")
                && node.text == "WARNING: 危険です。"
        }),
        "viewer plan did not render alert text"
    );
}

fn assert_footnote_reference_link(plan: &ViewerNodePlan) {
    assert!(
        plan.nodes
            .iter()
            .flat_map(|node| node.spans.iter())
            .any(|span| span.text == "[note]" && span.link_target == "#fn-note"),
        "viewer plan did not render footnote reference as an internal link"
    );
}

fn assert_no_raw_footnote_marker(plan: &ViewerNodePlan) {
    assert!(
        plan.nodes.iter().all(|node| {
            !node.text.contains("[^note]")
                && node.spans.iter().all(|span| !span.text.contains("[^note]"))
        }),
        "viewer plan leaked raw footnote markdown"
    );
}

fn assert_commonmark_blocks_are_visible(plan: &ViewerNodePlan) {
    assert!(
        plan.nodes.iter().any(|node| {
            matches!(node.kind, ViewerNodeKind::BlockQuote)
                && node.spans.iter().any(|span| span.text.contains("通常引用"))
        }),
        "viewer plan did not render a normal blockquote"
    );
    assert!(
        plan.nodes
            .iter()
            .any(|node| matches!(node.kind, ViewerNodeKind::Rule)),
        "viewer plan did not render a thematic break"
    );
    assert!(
        plan.nodes
            .iter()
            .any(|node| matches!(node.kind, ViewerNodeKind::List)
                && node.text.contains("用語: 説明")),
        "viewer plan did not render a description list"
    );
    assert!(
        plan.nodes.iter().any(|node| {
            node.spans
                .iter()
                .any(|span| span.text == "削除" && span.style.strikethrough)
        }),
        "viewer plan did not render strikethrough spans"
    );
}

fn assert_html_alignment_roles(plan: &ViewerNodePlan) {
    assert_html_role(plan, ViewerHtmlRole::Left, "left aligned HTML");
    assert_html_role(plan, ViewerHtmlRole::Generic, "generic HTML");
}

fn assert_html_role(plan: &ViewerNodePlan, expected: ViewerHtmlRole, label: &str) {
    assert!(
        plan.nodes
            .iter()
            .any(|node| matches!(&node.kind, ViewerNodeKind::Html { role } if *role == expected)),
        "viewer plan did not render {label}"
    );
}

fn assert_footnotes_are_last(plan: &ViewerNodePlan) {
    assert!(
        footnote_position(plan) > last_non_footnote_position(plan),
        "viewer plan did not move footnote definitions to the end"
    );
}

fn footnote_position(plan: &ViewerNodePlan) -> Option<usize> {
    plan.nodes
        .iter()
        .position(|node| matches!(node.kind, ViewerNodeKind::FootnoteDefinition { .. }))
}

fn last_non_footnote_position(plan: &ViewerNodePlan) -> Option<usize> {
    plan.nodes
        .iter()
        .rposition(|node| !matches!(node.kind, ViewerNodeKind::FootnoteDefinition { .. }))
}
