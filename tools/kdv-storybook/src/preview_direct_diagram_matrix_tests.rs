use crate::KucDiagramControlResolver;
use crate::catalog::StorybookFixture;
use crate::preview::PreviewBuilder;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeSurfaceHost};
use std::path::PathBuf;

#[test]
fn direct_diagram_fixtures_reach_kuc_image_surface() -> Result<(), Box<dyn std::error::Error>> {
    for case in direct_diagram_cases() {
        let scene = build_scene(case.fixture)?;

        assert!(
            count_kind(scene.tree.root(), UiNodeKind::ImageSurface) > 0,
            "{} ImageSurface missing",
            case.fixture
        );
        assert!(
            has_accessibility_label(scene.tree.root(), case.accessibility_label),
            "{} {} missing",
            case.fixture,
            case.accessibility_label
        );
        for action in ["pan-up", "reset-view"] {
            assert!(
                count_internal_diagram_action(&scene, action) > 0,
                "{} {action} missing",
                case.fixture
            );
        }
        assert!(
            count_action(scene.tree.root(), "fullscreen") > 0,
            "{} fullscreen missing",
            case.fixture
        );
        assert!(
            scene.loaded_asset_count >= 1,
            "{} loaded assets {} < 1",
            case.fixture,
            scene.loaded_asset_count
        );
        assert!(
            scene.failed_asset_count == 0,
            "{} failed assets {}",
            case.fixture,
            scene.failed_asset_count
        );
    }
    Ok(())
}

struct DirectDiagramCase {
    fixture: &'static str,
    accessibility_label: &'static str,
}

fn direct_diagram_cases() -> Vec<DirectDiagramCase> {
    vec![
        direct_diagram_case("direct/sample.drawio", "diagram:DrawIo"),
        direct_diagram_case("direct/sample.drowio", "diagram:DrawIo"),
        direct_diagram_case("direct/sample.mermaid", "diagram:Mermaid"),
        direct_diagram_case("direct/sample.mmd", "diagram:Mermaid"),
        direct_diagram_case("direct/sample.plantuml", "diagram:PlantUml"),
        direct_diagram_case("direct/sample.puml", "diagram:PlantUml"),
    ]
}

fn direct_diagram_case(
    fixture: &'static str,
    accessibility_label: &'static str,
) -> DirectDiagramCase {
    DirectDiagramCase {
        fixture,
        accessibility_label,
    }
}

fn build_scene(path: &str) -> Result<crate::preview::PreviewScene, Box<dyn std::error::Error>> {
    PreviewBuilder::default().build_without_preview_surface(
        &StorybookFixture {
            label: path.to_string(),
            path: fixture_path(&format!("assets/fixtures/{path}")),
        },
        ViewerViewport {
            width: 800.0,
            height: 12_000.0,
        },
        true,
        ViewerInteractionConfig {
            diagram_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}

fn count_kind(node: &UiNode, expected: UiNodeKind) -> usize {
    usize::from(node.kind() == expected)
        + node
            .children()
            .iter()
            .map(|child| count_kind(child, expected))
            .sum::<usize>()
}

fn count_action(node: &UiNode, action: &str) -> usize {
    usize::from(node.props().interaction.value == action)
        + node
            .children()
            .iter()
            .map(|child| count_action(child, action))
            .sum::<usize>()
}

fn count_internal_diagram_action(scene: &crate::preview::PreviewScene, action: &str) -> usize {
    UiTreeSurfaceHost::new(scene.theme.clone())
        .document_node_hits(
            scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: surface_width(scene),
                height: scene.content_height.ceil().max(1.0) as usize,
                scroll_y: 0.0,
            },
        )
        .iter()
        .filter_map(|hit| {
            KucDiagramControlResolver::internal_action_for_node(scene.tree.root(), &hit.node_id)
        })
        .filter(|resolved| resolved.command == action)
        .count()
}

fn surface_width(scene: &crate::preview::PreviewScene) -> usize {
    scene
        .surface
        .as_ref()
        .map_or(800, |surface| surface.width.max(1) as usize)
}

fn has_accessibility_label(node: &UiNode, label: &str) -> bool {
    node.props().accessibility_label == label
        || node
            .children()
            .iter()
            .any(|child| has_accessibility_label(child, label))
}
