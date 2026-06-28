use crate::{
    MarkdownSource, PreviewConfig, PreviewOutputFactory, ViewerDiagramKind, ViewerNodeKind,
    ViewerNodePlan, ViewerNodePlanner, ViewerViewport,
};

const CONTENT_HEIGHT: f32 = 200_000.0;
const MINIMUM_MERMAID_DIAGRAMS: usize = 50;

#[test]
fn katana_mermaid_fixtures_keep_diagram_nodes_without_storybook_sync_load()
-> Result<(), Box<dyn std::error::Error>> {
    for case in mermaid_cases() {
        let plan = plan_for(case.content, case.document_id)?;
        let count = diagram_count(&plan, ViewerDiagramKind::Mermaid);
        assert!(
            count >= MINIMUM_MERMAID_DIAGRAMS,
            "{} mermaid diagrams {} < {}",
            case.document_id,
            count,
            MINIMUM_MERMAID_DIAGRAMS
        );
    }
    Ok(())
}

struct MermaidCase {
    document_id: &'static str,
    content: &'static str,
}

fn mermaid_cases() -> Vec<MermaidCase> {
    vec![
        MermaidCase {
            document_id: "assets/fixtures/katana/sample_mermaid.md",
            content: include_str!("../../../../assets/fixtures/katana/sample_mermaid.md"),
        },
        MermaidCase {
            document_id: "assets/fixtures/katana/sample_mermaid_ja.md",
            content: include_str!("../../../../assets/fixtures/katana/sample_mermaid_ja.md"),
        },
    ]
}

fn plan_for(
    content: &str,
    document_id: &str,
) -> Result<ViewerNodePlan, Box<dyn std::error::Error>> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some(document_id.to_string()),
        },
        &config(),
        CONTENT_HEIGHT,
    )?;
    Ok(ViewerNodePlanner::create(&output.input, 0.0))
}

fn config() -> PreviewConfig {
    PreviewConfig {
        viewport: ViewerViewport {
            width: 1024.0,
            height: CONTENT_HEIGHT,
        },
        ..PreviewConfig::default()
    }
}

fn diagram_count(plan: &ViewerNodePlan, expected: ViewerDiagramKind) -> usize {
    plan.nodes
        .iter()
        .filter(|node| matches!(&node.kind, ViewerNodeKind::Diagram { kind } if *kind == expected))
        .count()
}
