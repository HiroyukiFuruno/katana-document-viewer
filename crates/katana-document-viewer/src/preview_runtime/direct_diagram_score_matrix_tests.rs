use crate::preview_runtime::fixture_score_matrix_render_support::RenderedExportBytes;
use crate::preview_runtime::fixture_score_matrix_support::FixtureScoreAssertions;
use crate::{
    MarkdownSource, PreviewConfig, PreviewOutputFactory, ViewerDiagramKind, ViewerNodeKind,
    ViewerNodePlanner, ViewerViewport,
};

const CONTENT_HEIGHT: f32 = 20_000.0;

#[test]
fn direct_diagrams_bind_viewer_kind_to_export_score() -> Result<(), Box<dyn std::error::Error>> {
    for case in DIRECT_DIAGRAM_CASES {
        let output = PreviewOutputFactory::from_source(
            &MarkdownSource {
                content: case.content.to_string(),
                document_id: Some(case.document_id.to_string()),
            },
            &config(),
            CONTENT_HEIGHT,
        )?;
        let plan = ViewerNodePlanner::create(&output.input, 0.0);

        assert!(
            has_diagram(&plan, case.kind),
            "{} missing {:?}",
            case.name,
            case.kind
        );
        let export_bytes = RenderedExportBytes::from_output(&output)?;
        FixtureScoreAssertions::assert_export_score(
            case.name,
            &export_bytes.score_report(case.content),
        );
    }
    Ok(())
}

struct DirectDiagramCase {
    name: &'static str,
    document_id: &'static str,
    content: &'static str,
    kind: ViewerDiagramKind,
}

const DIRECT_DIAGRAM_CASES: &[DirectDiagramCase] = &[
    DirectDiagramCase {
        name: "direct/sample.drawio",
        document_id: "assets/fixtures/direct/sample.drawio",
        content: include_str!("../../../../assets/fixtures/direct/sample.drawio"),
        kind: ViewerDiagramKind::DrawIo,
    },
    DirectDiagramCase {
        name: "direct/sample.drowio",
        document_id: "assets/fixtures/direct/sample.drowio",
        content: include_str!("../../../../assets/fixtures/direct/sample.drowio"),
        kind: ViewerDiagramKind::DrawIo,
    },
    DirectDiagramCase {
        name: "direct/sample.mermaid",
        document_id: "assets/fixtures/direct/sample.mermaid",
        content: include_str!("../../../../assets/fixtures/direct/sample.mermaid"),
        kind: ViewerDiagramKind::Mermaid,
    },
    DirectDiagramCase {
        name: "direct/sample.mmd",
        document_id: "assets/fixtures/direct/sample.mmd",
        content: include_str!("../../../../assets/fixtures/direct/sample.mmd"),
        kind: ViewerDiagramKind::Mermaid,
    },
    DirectDiagramCase {
        name: "direct/sample.plantuml",
        document_id: "assets/fixtures/direct/sample.plantuml",
        content: include_str!("../../../../assets/fixtures/direct/sample.plantuml"),
        kind: ViewerDiagramKind::PlantUml,
    },
    DirectDiagramCase {
        name: "direct/sample.puml",
        document_id: "assets/fixtures/direct/sample.puml",
        content: include_str!("../../../../assets/fixtures/direct/sample.puml"),
        kind: ViewerDiagramKind::PlantUml,
    },
];

fn has_diagram(plan: &crate::ViewerNodePlan, expected: ViewerDiagramKind) -> bool {
    FixtureScoreAssertions::has_kind(
        plan,
        |kind| matches!(kind, ViewerNodeKind::Diagram { kind } if *kind == expected),
    )
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
