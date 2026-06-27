use crate::KucDiagramControlResolver;
use crate::catalog::StorybookFixture;
use crate::preview::{PreviewBuilder, PreviewScene};
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeSurfaceHost};
use std::path::PathBuf;

const VIEWPORT_HEIGHT: f32 = 6_000.0;
const BASIC_ROLES: &[&str] = &["heading", "code", "list-marker", "list-item"];
const BASIC_STYLES: &[&str] = &[
    "kdv-task-empty",
    "kdv-task-done",
    "kdv-task-progress",
    "kdv-task-blocked",
];
const DIAGRAM_HOST_ACTIONS: &[&str] = &["fullscreen"];
const DIAGRAM_INTERNAL_ACTIONS: &[&str] =
    &["pan-up", "pan-down", "reset-view", "zoom-in", "zoom-out"];

#[test]
fn katana_markdown_fixtures_reach_kuc_feature_matrix() -> Result<(), Box<dyn std::error::Error>> {
    for case in markdown_cases() {
        let scene = MatrixSupport::build_scene(&case)?;
        MatrixSupport::assert_scene(&case, &scene);
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct MatrixCase {
    fixture: &'static str,
    roles: &'static [&'static str],
    styles: &'static [&'static str],
    kinds: &'static [UiNodeKind],
    host_actions: &'static [&'static str],
    internal_diagram_actions: &'static [&'static str],
    minimum_image_surfaces: usize,
}

fn markdown_cases() -> Vec<MatrixCase> {
    vec![
        visible_basic("katana/sample.md"),
        visible_basic("katana/sample.ja.md"),
        visible_basic("katana/sample_basic.md"),
        visible_basic("katana/sample_basic.ja.md"),
        visible_diagram("katana/sample_diagrams.md"),
        visible_diagram("katana/sample_diagrams.ja.md"),
        visible_html("katana/sample_html.md"),
        visible_html("katana/sample_html.ja.md"),
    ]
}

fn visible_basic(fixture: &'static str) -> MatrixCase {
    MatrixCase {
        fixture,
        roles: BASIC_ROLES,
        styles: BASIC_STYLES,
        kinds: &[UiNodeKind::Divider],
        host_actions: &[],
        internal_diagram_actions: &[],
        minimum_image_surfaces: 0,
    }
}

fn visible_diagram(fixture: &'static str) -> MatrixCase {
    MatrixCase {
        fixture,
        roles: &["heading"],
        styles: &[],
        kinds: &[UiNodeKind::ImageSurface],
        host_actions: DIAGRAM_HOST_ACTIONS,
        internal_diagram_actions: DIAGRAM_INTERNAL_ACTIONS,
        minimum_image_surfaces: 1,
    }
}

fn visible_html(fixture: &'static str) -> MatrixCase {
    MatrixCase {
        fixture,
        roles: &["heading", "html-centered-preview"],
        styles: &[],
        kinds: &[UiNodeKind::Divider],
        host_actions: &[],
        internal_diagram_actions: &[],
        minimum_image_surfaces: 0,
    }
}

struct MatrixSupport;

impl MatrixSupport {
    fn build_scene(case: &MatrixCase) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        PreviewBuilder::default().build(
            &StorybookFixture {
                label: case.fixture.to_string(),
                path: Self::fixture_path(case.fixture),
            },
            ViewerViewport {
                width: 960.0,
                height: VIEWPORT_HEIGHT,
            },
            true,
            ViewerInteractionConfig::default(),
        )
    }

    fn assert_scene(case: &MatrixCase, scene: &PreviewScene) {
        assert_eq!(0, scene.failed_asset_count, "{}", case.fixture);
        assert!(scene.node_count > 0, "{}", case.fixture);
        assert!(scene.surface.is_some(), "{}", case.fixture);
        Self::assert_asset_counts(case, scene);
        Self::assert_tree(case, scene);
    }

    fn assert_asset_counts(case: &MatrixCase, scene: &PreviewScene) {
        assert!(
            scene.image_surface_count >= case.minimum_image_surfaces,
            "{} images {} < {}",
            case.fixture,
            scene.image_surface_count,
            case.minimum_image_surfaces
        );
    }

    fn assert_tree(case: &MatrixCase, scene: &PreviewScene) {
        let node = scene.tree.root();
        for role in case.roles {
            assert!(Self::count_role(node, role) > 0, "{} {role}", case.fixture);
        }
        for style in case.styles {
            assert!(
                Self::count_style(node, style) > 0,
                "{} {style}",
                case.fixture
            );
        }
        for kind in case.kinds {
            assert!(
                Self::count_kind(node, *kind) > 0,
                "{} {kind:?}",
                case.fixture
            );
        }
        for action in case.host_actions {
            assert!(
                Self::count_action(node, action) > 0,
                "{} {action}",
                case.fixture
            );
        }
        for action in case.internal_diagram_actions {
            assert!(
                Self::count_internal_diagram_action(scene, action) > 0,
                "{} {action}",
                case.fixture
            );
        }
    }

    fn count_role(node: &UiNode, role: &str) -> usize {
        usize::from(node.props().text.role == role)
            + node
                .children()
                .iter()
                .map(|child| Self::count_role(child, role))
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
            .map(|child| Self::count_style(child, style))
            .sum::<usize>()
    }

    fn count_kind(node: &UiNode, expected: UiNodeKind) -> usize {
        usize::from(node.kind() == expected)
            + node
                .children()
                .iter()
                .map(|child| Self::count_kind(child, expected))
                .sum::<usize>()
    }

    fn count_action(node: &UiNode, action: &str) -> usize {
        usize::from(node.props().interaction.value == action)
            + node
                .children()
                .iter()
                .map(|child| Self::count_action(child, action))
                .sum::<usize>()
    }

    fn count_internal_diagram_action(scene: &PreviewScene, action: &str) -> usize {
        UiTreeSurfaceHost::new(scene.theme.clone())
            .document_node_hits(
                scene.tree.root(),
                UiTreeRenderArea {
                    x: 0,
                    y: 0,
                    width: Self::surface_width(scene),
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

    fn surface_width(scene: &PreviewScene) -> usize {
        scene
            .surface
            .as_ref()
            .map_or(800, |surface| surface.width.max(1) as usize)
    }

    fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
    }
}
