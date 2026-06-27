use crate::catalog::StorybookFixture;
use crate::preview::{PreviewBuilder, PreviewScene};
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use std::path::PathBuf;

const DRAWIO_BASIC_FIXTURES: &[&str] = &[
    "katana/drawio/basic/01-empty-mxfile.drawio",
    "katana/drawio/basic/02-standalone-mxgraphmodel.drawio",
    "katana/drawio/basic/03-basic-flow.drawio",
    "katana/drawio/basic/04-shape-style-matrix.drawio",
    "katana/drawio/basic/05-edge-variants.drawio",
    "katana/drawio/basic/06-multi-page.drawio",
    "katana/drawio/basic/07-html-labels-and-entities.drawio",
    "katana/drawio/basic/08-group-container.drawio",
    "katana/drawio/basic/09-layers-and-swimlane.drawio",
    "katana/drawio/basic/10-userobject-metadata.drawio",
    "katana/drawio/basic/11-japanese-labels.drawio",
    "katana/drawio/basic/12-vars-placeholders.drawio",
];

const DIAGRAM_ACTIONS: &[&str] = &[
    "fullscreen",
    "pan-up",
    "pan-down",
    "pan-left",
    "pan-right",
    "reset-view",
    "trackpad-help",
    "zoom-in",
    "zoom-out",
];

#[test]
fn katana_drawio_basic_fixtures_reach_kuc_image_surface() -> Result<(), Box<dyn std::error::Error>>
{
    for fixture in DRAWIO_BASIC_FIXTURES {
        let scene = DrawioMatrixSupport::build_scene(fixture)?;
        DrawioMatrixSupport::assert_loaded_diagram(fixture, &scene);
        DrawioMatrixSupport::assert_diagram_actions(fixture, scene.tree.root());
    }
    Ok(())
}

struct DrawioMatrixSupport;

impl DrawioMatrixSupport {
    fn build_scene(path: &str) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        PreviewBuilder::default().build_without_preview_surface(
            &StorybookFixture {
                label: path.to_string(),
                path: Self::fixture_path(path),
            },
            ViewerViewport {
                width: 960.0,
                height: 7200.0,
            },
            true,
            ViewerInteractionConfig::default(),
        )
    }

    fn assert_loaded_diagram(fixture: &str, scene: &PreviewScene) {
        assert_eq!(0, scene.failed_asset_count, "{fixture}");
        assert_eq!(0, scene.asset_request_count, "{fixture}");
        assert!(scene.loaded_asset_count > 0, "{fixture}");
        assert!(scene.image_surface_count > 0, "{fixture}");
        assert!(
            Self::count_kind(scene.tree.root(), UiNodeKind::ImageSurface) > 0,
            "{fixture}"
        );
    }

    fn assert_diagram_actions(fixture: &str, node: &UiNode) {
        for action in DIAGRAM_ACTIONS {
            assert!(
                Self::count_action(node, action) > 0,
                "{fixture} missing {action}"
            );
        }
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

    fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
    }
}
