use super::PreviewBuilder;
use crate::KucViewerAdapter;
use crate::catalog::StorybookFixture;
use crate::preview_build_support::PreviewBuildSupport;
use crate::preview_search_targets::StorybookSearchTargets;
use katana_document_viewer::{
    DiagramRenderEngine, DiagramRenderRequest, KdvThemeSnapshot, MarkdownSource,
    PreviewAssetLoader, PreviewConfig, PreviewOutputFactory, RenderedDiagram,
    ViewerInteractionConfig, ViewerSearchEngine, ViewerViewport,
};
use katana_ui_core::render_model::UiNode;
use std::path::PathBuf;

#[test]
fn preview_build_with_search_reaches_kuc_highlight_spans() -> Result<(), Box<dyn std::error::Error>>
{
    let scene = PreviewBuilder::default().build_with_mode_and_search(
        &StorybookFixture {
            label: "direct/sample.md".to_string(),
            path: fixture_path("assets/fixtures/direct/sample.md"),
        },
        ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        true,
        ViewerInteractionConfig::default(),
        katana_document_viewer::ViewerMode::Document,
        ViewerSearchEngine::state("Direct", Vec::new(), None),
    )?;

    assert!(highlight_span_count(scene.tree.root()) > 0);
    Ok(())
}

#[test]
fn diagram_artifact_text_reaches_storybook_search_targets() -> Result<(), Box<dyn std::error::Error>>
{
    let search = ViewerSearchEngine::state("Needle", Vec::new(), None);
    let config = PreviewConfig {
        viewport: ViewerViewport {
            width: 800.0,
            height: 600.0,
        },
        search: search.clone(),
        ..PreviewConfig::default()
    };
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: "```mermaid\ngraph TD\n  A[Start] --> B[End]\n```".to_string(),
            document_id: Some("artifact-search.md".to_string()),
        },
        &config,
        320.0,
    )?;
    let (output, _) = PreviewAssetLoader::new(FakeDiagramEngine)
        .load_requested(&output, &KdvThemeSnapshot::katana_light())?;
    let theme = PreviewBuildSupport::kdv_theme(false);
    let kuc_theme = crate::preview_theme_bridge::KucThemeBridge::from_kdv(&theme)?;
    let kuc_plan = KucViewerAdapter.render(
        &output,
        &PreviewBuildSupport::kuc_config(
            &config,
            kuc_theme,
            Default::default(),
            Default::default(),
        )?,
    );

    let targets = StorybookSearchTargets::collect(
        &kuc_plan.node_plan,
        &output.input.artifacts,
        &search.query,
    );

    assert_eq!(1, targets.len());
    assert_eq!("Needle", targets[0].matched.text);
    assert!(targets[0].matched.artifact_id.is_some());
    Ok(())
}

fn highlight_span_count(node: &UiNode) -> usize {
    node.props()
        .text
        .spans
        .iter()
        .filter(|span| span.style.highlight)
        .count()
        + node
            .children()
            .iter()
            .map(highlight_span_count)
            .sum::<usize>()
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}

struct FakeDiagramEngine;

impl DiagramRenderEngine for FakeDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: "mermaid".to_string(),
            svg: r#"<svg xmlns="http://www.w3.org/2000/svg"><text>Artifact Needle</text></svg>"#
                .to_string(),
        })
    }
}
