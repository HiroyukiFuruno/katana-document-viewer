use super::super::test_support::{input_with_nodes, node};
use super::builder_graph_test_support::{
    diagram_artifact, diagram_artifact_like, diagram_artifact_raw, diagram_node,
    rejected_svg_and_error_artifacts,
};
use super::{ParagraphLayout, ViewerNodePlanBuilder};
use crate::artifact::ArtifactFormat;
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNodeId, KmmNodeKind};

#[test]
fn build_graph_builds_rendered_diagram_list_for_preserve_source_rows_layout() {
    let mut input = input_with_nodes(vec![diagram_node("diagram-node")]);
    input.artifacts = vec![diagram_artifact(&input, "diagram-node")];

    let graphs = ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::PreserveSourceRows)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, graphs.len());
    let graph = &graphs[0];
    assert_eq!(1, graph.rendered_diagrams.len());
    assert_eq!(
        "node-diagram-node",
        graph.rendered_diagrams[0].node_id.as_str()
    );
    assert_eq!("mermaid", graph.rendered_diagrams[0].kind);
    assert!(graph.rendered_diagrams[0].svg.starts_with("<svg"));
}

#[test]
fn rendered_diagram_is_ignored_when_svg_is_invalid_or_diagnostics_are_present() {
    let mut input = input_with_nodes(vec![diagram_node("diagram-node")]);
    input.artifacts = rejected_svg_and_error_artifacts(&input);

    let graphs = ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::PreserveSourceRows)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, graphs.len());
    assert_eq!(1, graphs[0].rendered_diagrams.len());
    let diagram = &graphs[0].rendered_diagrams[0];
    assert_eq!("node-diagram-node", diagram.node_id);
    assert_eq!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
        diagram.svg
    );
}

#[test]
fn rendered_diagram_is_ignored_for_non_matching_artifact_id_prefix() {
    let mut input = input_with_nodes(vec![diagram_node("diagram-node")]);
    input.artifacts = vec![diagram_artifact_like(
        &input,
        ArtifactFormat::Svg,
        "wrong-prefix",
    )];

    let graphs = ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::PreserveSourceRows)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, graphs.len());
    assert!(graphs[0].rendered_diagrams.is_empty());
}

#[test]
fn rendered_diagram_rejects_non_svg_payload() {
    let mut input = input_with_nodes(vec![diagram_node("diagram-node")]);
    input.artifacts = vec![diagram_artifact_raw(
        &input,
        "diagram-node",
        "<html>not svg</html>",
        Vec::new(),
    )];

    let graphs = ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::PreserveSourceRows)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, graphs.len());
    assert!(graphs[0].rendered_diagrams.is_empty());
}

#[test]
fn rendered_diagram_rejects_missing_and_non_diagram_nodes() {
    let mut input = input_with_nodes(vec![node(KmmNodeKind::Paragraph, "plain", Vec::new())]);
    input.artifacts = vec![
        diagram_artifact_raw(
            &input,
            "missing-node",
            "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
            Vec::new(),
        ),
        diagram_artifact_raw(
            &input,
            "plain",
            "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
            Vec::new(),
        ),
    ];

    let graphs = ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::PreserveSourceRows)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, graphs.len());
    assert!(graphs[0].rendered_diagrams.is_empty());
}

#[test]
fn rendered_diagram_preserves_plantuml_kind() {
    let mut plantuml = node(
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::PlantUml,
        }),
        "```plantuml\nAlice -> Bob\n```",
        Vec::new(),
    );
    plantuml.id = KmmNodeId("node-plantuml".to_string());
    let mut input = input_with_nodes(vec![plantuml]);
    input.artifacts = vec![diagram_artifact(&input, "plantuml")];

    let graphs = ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::PreserveSourceRows)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, graphs.len());
    assert_eq!("plantuml", graphs[0].rendered_diagrams[0].kind);
}

#[test]
fn diagram_kind_for_node_id_prefers_nested_code_block_nodes() {
    let nested = node(
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::DrawIo,
        }),
        "drawio-diagram",
        Vec::new(),
    );
    let mut input = input_with_nodes(vec![node(
        KmmNodeKind::BlockQuote,
        "> block",
        vec![node(KmmNodeKind::Paragraph, "text", vec![nested])],
    )]);
    input.artifacts = vec![diagram_artifact(&input, "drawio-diagram")];
    let graphs = ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::PreserveSourceRows)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, graphs.len());
    assert_eq!(1, graphs[0].rendered_diagrams.len());
    assert_eq!("drawio", graphs[0].rendered_diagrams[0].kind);
}

#[test]
fn build_graph_returns_none_without_preserve_source_rows_layout() {
    let input = input_with_nodes(vec![diagram_node("softwrap")]);
    assert!(ViewerNodePlanBuilder::build_graph(&input, ParagraphLayout::SoftWrap).is_none());
}
