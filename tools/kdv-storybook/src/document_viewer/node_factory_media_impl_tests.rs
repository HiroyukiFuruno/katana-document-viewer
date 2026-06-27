use super::KucNodeFactory;
use super::media_fixture::*;
use crate::test_assert::KucTestExpect;
use katana_document_viewer::{
    ArtifactBytes, ArtifactDiagnostics, ArtifactFactory, ArtifactFormat, ArtifactId, DocumentId,
    SourceRevision, ViewerImageSurface, ViewerNodeKind, ViewerRect,
};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVisualRole};

#[test]
fn image_surface_node_uses_default_transform_without_viewport_state() {
    let node = viewer_node(ViewerNodeKind::Image, "image", "image");
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH);

    let ui_node = factory.image_surface_node(
        &node,
        &ArtifactId("artifact".to_string()),
        ViewerImageSurface {
            fingerprint: "fingerprint".to_string(),
            width: IMAGE_SURFACE_WIDTH,
            height: IMAGE_SURFACE_HEIGHT,
            display_width: (IMAGE_SURFACE_WIDTH * 100 / IMAGE_SURFACE_CONTENT_SCALE) as f32,
            display_height: (IMAGE_SURFACE_HEIGHT * 100 / IMAGE_SURFACE_CONTENT_SCALE) as f32,
            content_scale: IMAGE_SURFACE_CONTENT_SCALE,
            rgba: opaque_rgba_surface(),
        },
    );

    assert_eq!(UiNodeKind::ImageSurface, ui_node.kind());
    assert_eq!(UiVisualRole::MediaFrame, ui_node.props().visual_role);
    assert!(!has_style_class(&ui_node, "kdv-document-media"));
    assert_eq!(
        IMAGE_SURFACE_CONTENT_SCALE,
        ui_node.props().image_surface.transform.zoom_percent
    );
    assert_eq!(0, ui_node.props().image_surface.transform.pan_x);
    assert_eq!(0, ui_node.props().image_surface.transform.pan_y);
}

#[test]
fn media_max_width_uses_katana_viewer_row_width_for_interactive_diagram() {
    let factory = KucNodeFactory::new(&[], KATANA_VIEWER_ROW_MAX_WIDTH);

    assert_eq!(
        KATANA_VIEWER_ROW_MAX_WIDTH,
        factory.media_max_width(&viewer_node(ViewerNodeKind::Image, "image", "image"))
    );
    assert_eq!(
        KATANA_VIEWER_ROW_MAX_WIDTH,
        factory.media_max_width(&diagram_node())
    );
    assert_eq!(
        MATH_MEDIA_MAX_WIDTH,
        factory.media_max_width(&viewer_node(ViewerNodeKind::Math, "math", "math"))
    );
}

#[test]
fn interactive_diagram_keeps_intrinsic_body_width_while_row_stays_full_width() {
    let mut node = diagram_node();
    node.rect = ViewerRect {
        x: 0.0,
        y: 0.0,
        width: KATANA_VIEWER_ROW_MAX_WIDTH as f32,
        height: 507.0,
    };
    let factory = KucNodeFactory::new(&[], KATANA_VIEWER_ROW_MAX_WIDTH);
    let surface = ViewerImageSurface {
        fingerprint: "wide-diagram".to_string(),
        width: 2400,
        height: 1200,
        display_width: 1200.0,
        display_height: 600.0,
        content_scale: 200,
        rgba: [0, 0, 0, OPAQUE_ALPHA].repeat(2400 * 1200),
    };

    let media = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);
    let row = factory.node_with_viewer_height(media, &node);
    let image = find_image_surface(&row).kuc_expect("diagram image surface");

    assert_eq!(
        UiDimension::Px(KATANA_VIEWER_ROW_MAX_WIDTH as u16),
        row.props().common.width
    );
    assert_eq!(
        UiDimension::Auto,
        image.props().common.width,
        "diagram image body must keep its exact intrinsic display size; the full row width belongs to the wrapper"
    );
    assert_eq!(
        1_200_000,
        image.props().image_surface.display_width_milli,
        "diagram body keeps SVG intrinsic display size and the renderer downscales to the row width when needed"
    );
    assert_eq!(600_000, image.props().image_surface.display_height_milli);
}

#[test]
fn media_max_width_keeps_export_surface_diagram_contract() {
    let factory = KucNodeFactory::new(&[], EXPORT_MEDIA_MAX_WIDTH).export_surface(true);

    assert_eq!(
        DIAGRAM_MEDIA_MAX_WIDTH,
        factory.media_max_width(&diagram_node())
    );
}

#[test]
fn export_surface_diagram_media_node_uses_export_raster_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let artifact_id = ArtifactId("doc:diagram:wide".to_string());
    let artifact = ArtifactFactory::image_asset_with_id(
        artifact_id.clone(),
        ArtifactFormat::Svg,
        DocumentId("document".to_string()),
        SourceRevision("rev".to_string()),
        ArtifactBytes {
            bytes: br#"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="600"><rect width="1200" height="600"/></svg>"#.to_vec(),
        },
        "test",
        ArtifactDiagnostics {
            entries: Vec::new(),
        },
    );
    let mut node = diagram_node();
    node.artifact_id = Some(artifact_id);
    let artifacts = [artifact];
    let factory = KucNodeFactory::new(&artifacts, EXPORT_MEDIA_MAX_WIDTH).export_surface(true);

    let ui_node = factory.media_node(&node);

    assert_eq!(UiNodeKind::ImageSurface, ui_node.kind());
    assert_eq!(UiVisualRole::ExportMediaFrame, ui_node.props().visual_role);
    assert_eq!(DIAGRAM_MEDIA_MAX_WIDTH, ui_node.props().image_surface.width);
    assert_eq!(430, ui_node.props().image_surface.height);
    assert_eq!(100, ui_node.props().image_surface.content_scale);
    assert_eq!(
        DIAGRAM_MEDIA_MAX_WIDTH * 1000,
        ui_node.props().image_surface.display_width_milli
    );
    assert_eq!(430_000, ui_node.props().image_surface.display_height_milli);
    Ok(())
}

#[test]
fn diagram_surface_transform_uses_diagram_viewport_when_available() {
    let viewports = viewport_states();
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).diagram_viewports(&viewports);
    let node = diagram_node();
    let ui_node =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());
    assert_eq!(UiNodeKind::ImageSurface, ui_node.kind());
    assert_eq!(
        expected_transform(),
        ui_node.props().image_surface.transform
    );
}

#[test]
fn fullscreen_diagram_keeps_katana_original_size_without_upscaling() {
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
        fingerprint: "fullscreen-diagram".to_string(),
        width: KATANA_VIEWER_ROW_MAX_WIDTH * 2,
        height: KATANA_VIEWER_ROW_MAX_WIDTH,
        display_width: 320.0,
        display_height: 160.0,
        content_scale: 200,
        rgba: [0, 0, 0, OPAQUE_ALPHA]
            .repeat((KATANA_VIEWER_ROW_MAX_WIDTH * 2 * KATANA_VIEWER_ROW_MAX_WIDTH) as usize),
    };

    let ui_node = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);

    assert_eq!(
        320_000,
        ui_node.props().image_surface.display_width_milli,
        "fullscreen diagram must use KatanA's min(width-fit, height-fit, 1.0) scale"
    );
    assert_eq!(160_000, ui_node.props().image_surface.display_height_milli);
}

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
        width: 1000,
        height: 1000,
        display_width: 1000.0,
        display_height: 1000.0,
        content_scale: 100,
        rgba: [0, 0, 0, OPAQUE_ALPHA].repeat(1_000_000),
    };

    let ui_node = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);

    assert_eq!(
        520_000,
        ui_node.props().image_surface.display_width_milli,
        "fullscreen diagram must subtract KatanA's 40px top/bottom padding before fitting height"
    );
    assert_eq!(520_000, ui_node.props().image_surface.display_height_milli);
}

fn find_image_surface(node: &UiNode) -> Option<&UiNode> {
    if node.kind() == UiNodeKind::ImageSurface {
        return Some(node);
    }
    node.children().iter().find_map(find_image_surface)
}
