use super::*;
use crate::{DocumentSnapshotFactory, DocumentSource, SourceKind, SourceRevision, SourceUri};
use e2e_diagram_controls::supported_diagram_controls;
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

#[path = "e2e_diagram_controls.rs"]
mod e2e_diagram_controls;

const SAMPLE_JA: &str = include_str!("../../../../assets/fixtures/katana/sample.ja.md");
const CONTENT_HEIGHT: f32 = 2_400.0;
const FIXTURE_OUTLINE_ANCHOR_COUNT: usize = 6;
const ANCHOR_Y_STEP: f32 = 120.0;
const ANCHOR_WIDTH: f32 = 640.0;
const ANCHOR_HEIGHT: f32 = 32.0;
const SEARCH_MATCH_END: usize = 4;

#[test]
fn katana_fixture_runs_document_slideshow_toc_search_and_diagram_controls()
-> Result<(), Box<dyn std::error::Error>> {
    let snapshot = fixture_snapshot()?;
    let viewport = ViewerViewport {
        width: 900.0,
        height: 600.0,
    };
    let flow = fixture_flow(snapshot, viewport);

    assert_fixture_flow(flow);
    Ok(())
}

struct FixtureFlow {
    toc_items: Vec<ViewerTocItem>,
    slideshow: SlideshowState,
    search_targets: Vec<ViewerSearchTarget>,
    diagram_state: DiagramViewportState,
    supported_diagram_controls: Vec<DiagramControlRequirement>,
}

fn fixture_flow(snapshot: crate::DocumentSnapshot, viewport: ViewerViewport) -> FixtureFlow {
    let anchors = anchors_for_outline(&snapshot.outline);
    let layout = ViewerLayoutEngine::from_anchors(viewport, anchors, CONTENT_HEIGHT, 0.0);
    let toc_items = ViewerTocModel::from_outline(&snapshot.outline, &layout);
    let slideshow = ViewerStateEngine::apply_slideshow_command(
        ViewerStateEngine::slideshow_state(viewport, CONTENT_HEIGHT, 0, true, true),
        SlideshowCommand::NextPage,
    );
    let search_targets = ViewerSearchLayoutResolver::resolve_matches(
        &[search_match_from_toc(&toc_items[0])],
        &layout.anchors,
    );
    let diagram_state = ViewerStateEngine::apply_diagram_command(
        DiagramViewportState::default(),
        &DiagramControlCommand::FullscreenOpen(layout.anchors[0].target.clone()),
    );
    let supported_diagram_controls = supported_diagram_controls(&layout.anchors[0].target);
    FixtureFlow {
        toc_items,
        slideshow,
        search_targets,
        diagram_state,
        supported_diagram_controls,
    }
}

fn assert_fixture_flow(flow: FixtureFlow) {
    assert!(!flow.toc_items.is_empty());
    assert_eq!(flow.slideshow.current_page_index, 1);
    assert_eq!(flow.search_targets.len(), 1);
    assert!(flow.diagram_state.fullscreen_open);
    assert!(DiagramControlParity::is_complete(
        &flow.supported_diagram_controls
    ));
}

fn fixture_snapshot() -> Result<crate::DocumentSnapshot, Box<dyn std::error::Error>> {
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "sample.ja.md",
        SAMPLE_JA.to_string(),
    ))?;
    Ok(DocumentSnapshotFactory::from_kmm(
        DocumentSource {
            uri: SourceUri("fixture://sample.ja.md".to_string()),
            kind: SourceKind::Markdown,
            revision: SourceRevision(document.fingerprint.value.clone()),
            content: SAMPLE_JA.to_string(),
        },
        document,
    ))
}

fn anchors_for_outline(outline: &crate::DocumentOutline) -> Vec<ViewerRenderedAnchor> {
    outline
        .items
        .iter()
        .take(FIXTURE_OUTLINE_ANCHOR_COUNT)
        .enumerate()
        .map(|(index, item)| ViewerRenderedAnchor {
            target: ViewerTarget {
                node_id: item.node_id.clone(),
                source: item.source.clone(),
                artifact_id: crate::ArtifactId(format!("heading-{index}")),
                rect: ViewerRect {
                    x: 0.0,
                    y: index as f32 * ANCHOR_Y_STEP,
                    width: ANCHOR_WIDTH,
                    height: ANCHOR_HEIGHT,
                },
            },
            anchor_index: index,
        })
        .collect()
}

fn search_match_from_toc(item: &ViewerTocItem) -> ViewerSearchMatch {
    ViewerSearchMatch {
        id: ViewerSearchMatchId("fixture-search".to_string()),
        node_id: item.node_id.clone(),
        source: item.source.clone(),
        range: ViewerTextRange {
            start: 0,
            end: SEARCH_MATCH_END,
        },
        text: item.text.clone(),
        artifact_id: Some(crate::ArtifactId(format!("heading-{}", item.anchor_index))),
    }
}
