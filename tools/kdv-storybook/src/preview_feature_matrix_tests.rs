use super::PreviewBuilder;
use crate::KucDiagramControlResolver;
use crate::catalog::StorybookFixture;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind};
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeSurfaceHost};
use std::path::PathBuf;

#[test]
fn katana_preview_feature_matrix_by_fixture() -> Result<(), Box<dyn std::error::Error>> {
    for case in feature_cases() {
        let scene = build_scene(case.fixture)?;

        for role in case.roles {
            assert!(
                count_role(scene.tree.root(), role) > 0,
                "{} {role}",
                case.fixture
            );
        }
        for style in case.styles {
            assert!(
                count_style(scene.tree.root(), style) > 0,
                "{} {style}",
                case.fixture
            );
        }
        for kind in case.kinds {
            assert!(
                count_kind(scene.tree.root(), *kind) > 0,
                "{} {kind:?}",
                case.fixture
            );
        }
        for action in case.host_actions {
            assert!(
                count_action(scene.tree.root(), action) > 0,
                "{} {action}",
                case.fixture
            );
        }
        for action in case.internal_diagram_actions {
            assert!(
                count_internal_diagram_action(&scene, action) > 0,
                "{} {action}",
                case.fixture
            );
        }
        assert!(
            count_indented_list_nodes(scene.tree.root()) >= case.minimum_indented_list_nodes,
            "{} indented list nodes",
            case.fixture
        );
        assert!(
            scene.loaded_asset_count >= case.minimum_loaded_assets,
            "{} loaded assets {} < {}",
            case.fixture,
            scene.loaded_asset_count,
            case.minimum_loaded_assets
        );
        assert!(
            scene.image_surface_count >= case.minimum_image_surfaces,
            "{} image surfaces {} < {}",
            case.fixture,
            scene.image_surface_count,
            case.minimum_image_surfaces
        );
    }
    Ok(())
}

struct FeatureCase {
    fixture: &'static str,
    roles: &'static [&'static str],
    styles: &'static [&'static str],
    kinds: &'static [UiNodeKind],
    host_actions: &'static [&'static str],
    internal_diagram_actions: &'static [&'static str],
    minimum_indented_list_nodes: usize,
    minimum_loaded_assets: usize,
    minimum_image_surfaces: usize,
}

fn feature_cases() -> Vec<FeatureCase> {
    vec![
        FeatureCase {
            fixture: "katana/sample_basic.md",
            roles: &[
                "heading",
                "code",
                "table",
                "alert",
                "blockquote",
                "footnote",
                "list-marker",
                "list-item",
            ],
            styles: &[],
            kinds: &[],
            host_actions: &[],
            internal_diagram_actions: &[],
            minimum_indented_list_nodes: 1,
            minimum_loaded_assets: 0,
            minimum_image_surfaces: 0,
        },
        FeatureCase {
            fixture: "katana/sample_diagrams.md",
            roles: &[],
            styles: &[],
            kinds: &[UiNodeKind::ImageSurface],
            host_actions: &["fullscreen"],
            internal_diagram_actions: &["pan-up", "reset-view"],
            minimum_indented_list_nodes: 0,
            minimum_loaded_assets: 3,
            minimum_image_surfaces: 3,
        },
        FeatureCase {
            fixture: "direct/html-alignment.html",
            roles: &[
                "html-centered-preview",
                "html-right-preview",
                "html-left-preview",
                "table",
            ],
            styles: &[],
            kinds: &[UiNodeKind::Accordion],
            host_actions: &[],
            internal_diagram_actions: &[],
            minimum_indented_list_nodes: 0,
            minimum_loaded_assets: 0,
            minimum_image_surfaces: 0,
        },
        FeatureCase {
            fixture: "direct/sample.md",
            roles: &["heading", "list-marker", "list-item"],
            styles: &[],
            kinds: &[UiNodeKind::ImageSurface],
            host_actions: &["fullscreen"],
            internal_diagram_actions: &["pan-up", "reset-view"],
            minimum_indented_list_nodes: 0,
            minimum_loaded_assets: 1,
            minimum_image_surfaces: 1,
        },
        direct_image_case("direct/kdv-icon.bmp"),
        direct_image_case("direct/kdv-icon.gif"),
        direct_image_case("direct/kdv-icon.jpeg"),
        direct_image_case("direct/kdv-icon.jpg"),
        direct_image_case("direct/kdv-icon.png"),
        direct_image_case("direct/kdv-icon.svg"),
        direct_image_case("direct/kdv-icon.webp"),
    ]
}

fn direct_image_case(fixture: &'static str) -> FeatureCase {
    FeatureCase {
        fixture,
        roles: &[],
        styles: &[],
        kinds: &[UiNodeKind::ImageSurface],
        host_actions: &[],
        internal_diagram_actions: &[],
        minimum_indented_list_nodes: 0,
        minimum_loaded_assets: 1,
        minimum_image_surfaces: 1,
    }
}

fn build_scene(path: &str) -> Result<super::PreviewScene, Box<dyn std::error::Error>> {
    PreviewBuilder::default().build_without_preview_surface(
        &StorybookFixture {
            label: path.to_string(),
            path: fixture_path(&format!("assets/fixtures/{path}")),
        },
        ViewerViewport {
            width: 800.0,
            height: 12000.0,
        },
        true,
        ViewerInteractionConfig::default(),
    )
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}

fn count_role(node: &UiNode, role: &str) -> usize {
    usize::from(node.props().text.role == role)
        + node
            .children()
            .iter()
            .map(|child| count_role(child, role))
            .sum::<usize>()
}

fn count_style(node: &UiNode, style: &str) -> usize {
    usize::from(
        node.props()
            .style_classes
            .iter()
            .any(|value| value == style),
    ) + node
        .children()
        .iter()
        .map(|child| count_style(child, style))
        .sum::<usize>()
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

fn count_internal_diagram_action(scene: &super::PreviewScene, action: &str) -> usize {
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

fn surface_width(scene: &super::PreviewScene) -> usize {
    scene
        .surface
        .as_ref()
        .map_or(800, |surface| surface.width.max(1) as usize)
}

fn count_indented_list_nodes(node: &UiNode) -> usize {
    usize::from(matches!(
        node.props().common.margin.left,
        UiDimension::Px(value) if value > 0
    )) + node
        .children()
        .iter()
        .map(count_indented_list_nodes)
        .sum::<usize>()
}
