use super::KucNodeFactory;
use super::media_fixture::*;
use crate::test_assert::KucTestExpect;
use katana_document_viewer::{
    ArtifactBytes, ArtifactDiagnostics, ArtifactFactory, ArtifactFormat, ArtifactId, DocumentId,
    SourceRevision, ViewerImageSurface, ViewerInteractionConfig, ViewerNodeKind,
};
use katana_ui_core::render_model::UiImageSurfaceTransform;

#[test]
fn fullscreen_diagram_fits_height_inside_katana_padded_viewport() {
    let mut viewports = viewport_states();
    viewports
        .get_mut("diagram")
        .kuc_expect("diagram viewport fixture")
        .fullscreen_open = true;
    let node = diagram_node();
    let factory = KucNodeFactory::new(&[], KATANA_VIEWER_ROW_MAX_WIDTH)
        .diagram_viewports(&viewports)
        .fullscreen_viewport_height(600);
    let surface = ViewerImageSurface {
        fingerprint: "fullscreen-tall-diagram".to_string(),
        width: 2000,
        height: 2000,
        display_width: 1000.0,
        display_height: 1000.0,
        content_scale: 200,
        rgba: [0, 0, 0, OPAQUE_ALPHA].repeat(2000 * 2000),
    };

    let ui_node = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);

    assert_eq!(
        520_000,
        ui_node.props().image_surface.display_width_milli,
        "fullscreen diagram must fit the KatanA 40px top/bottom padding, not only the available width"
    );
    assert_eq!(520_000, ui_node.props().image_surface.display_height_milli);
}

#[test]
fn image_surface_transform_uses_image_viewport_when_available() {
    let viewports = viewport_states();
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).image_viewports(&viewports);
    let node = viewer_node(ViewerNodeKind::Image, "image", "image");
    let ui_node =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    assert_eq!(
        expected_transform(),
        ui_node.props().image_surface.transform
    );
}

#[test]
fn image_surface_transform_ignores_diagram_viewport_state() {
    let viewports = viewport_states();
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).diagram_viewports(&viewports);
    let node = viewer_node(ViewerNodeKind::Image, "image", "image");
    let ui_node =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    assert_eq!(
        UiImageSurfaceTransform::new(100, 0, 0),
        ui_node.props().image_surface.transform
    );
}

#[test]
fn image_surface_node_preserves_kdv_content_scale() {
    let node = diagram_node();
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH);
    let mut surface = image_surface();
    surface.width = SCALED_IMAGE_SURFACE_WIDTH;
    surface.height = SCALED_IMAGE_SURFACE_HEIGHT;
    surface.content_scale = SCALED_IMAGE_SURFACE_CONTENT_SCALE;
    surface.rgba = [0, 0, 0, OPAQUE_ALPHA]
        .repeat((SCALED_IMAGE_SURFACE_WIDTH * SCALED_IMAGE_SURFACE_HEIGHT) as usize);

    let ui_node = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);

    assert_eq!(
        SCALED_IMAGE_SURFACE_CONTENT_SCALE,
        ui_node.props().image_surface.content_scale
    );
}

#[test]
fn image_surface_uses_explicit_artifact_text_extraction_for_selection() {
    let artifact_id = ArtifactId("artifact-png".to_string());
    let artifact = ArtifactFactory::image_asset_with_id(
        artifact_id.clone(),
        ArtifactFormat::Png,
        DocumentId("document".to_string()),
        SourceRevision("rev".to_string()),
        ArtifactBytes {
            bytes: vec![0, 0, 0, OPAQUE_ALPHA],
        },
        "test",
        ArtifactDiagnostics {
            entries: Vec::new(),
        },
    )
    .with_text_extraction("Raster Needle");
    let node = viewer_node(ViewerNodeKind::Image, "image", "image");
    let artifacts = [artifact];
    let factory = KucNodeFactory::new(&artifacts, STORYBOOK_MEDIA_MAX_WIDTH).interaction(
        ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: false,
        },
    );

    let ui_node = factory.image_surface_node(&node, &artifact_id, image_surface());

    assert_eq!(
        "Raster Needle",
        ui_node.props().image_surface.selection_text
    );
}

#[test]
fn diagram_surface_keeps_katana_display_size_without_extra_preview_scale() {
    let node = diagram_node();
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH);
    let ui_node =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    assert_eq!(2_000, ui_node.props().image_surface.display_width_milli);
    assert_eq!(1_000, ui_node.props().image_surface.display_height_milli);
}
