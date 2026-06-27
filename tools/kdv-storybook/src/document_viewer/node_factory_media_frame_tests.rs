use super::KucNodeFactory;
use super::media_fixture::*;
use super::media_frame_tests_support::find_image_surface;
use crate::test_assert::KucTestExpect;
use katana_document_viewer::{
    ArtifactBytes, ArtifactDiagnostics, ArtifactFactory, ArtifactFormat, ArtifactId, DocumentId,
    SourceRevision, ViewerImageSurface, ViewerInteractionConfig,
};
use katana_ui_core::render_model::{UiDimension, UiNodeKind, UiVisualRole};

#[test]
fn diagram_controls_keep_katana_min_container_height_for_short_surface() {
    let factory =
        KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).interaction(ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: false,
            diagram_controls_enabled: true,
            code_controls_enabled: true,
        });
    let node = diagram_node();
    let media =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    let framed = factory.media_with_controls(&node, media);

    assert_eq!(
        UiDimension::Px(KATANA_MIN_CONTROL_CONTAINER_HEIGHT_PX),
        framed.props().common.height
    );
}

#[test]
fn diagram_controls_pass_katana_container_height_to_image_surface() {
    let factory =
        KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).interaction(ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: false,
            diagram_controls_enabled: true,
            code_controls_enabled: true,
        });
    let node = diagram_node();
    let media =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    let framed = factory.media_with_controls(&node, media);
    let image = find_image_surface(&framed);
    assert!(image.is_some(), "diagram image surface");
    let Some(image) = image else {
        return;
    };

    assert_eq!(
        UiDimension::Px(KATANA_MIN_CONTROL_CONTAINER_HEIGHT_PX),
        image.props().common.height,
        "controls-on diagrams must render the image inside the same KatanA container rect as controls-off diagrams"
    );
}

#[test]
fn fullscreen_diagram_controls_pass_viewport_height_to_centered_image_surface() {
    let mut viewports = viewport_states();
    viewports
        .get_mut("diagram")
        .kuc_expect("diagram viewport fixture")
        .fullscreen_open = true;
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH)
        .interaction(ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: false,
            diagram_controls_enabled: true,
            code_controls_enabled: true,
        })
        .diagram_viewports(&viewports)
        .fullscreen_viewport_size(640, 600);
    let node = diagram_node();
    let media =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    let framed = factory.media_with_controls(&node, media);
    let image = find_image_surface(&framed);
    assert!(image.is_some(), "fullscreen diagram image surface");
    let Some(image) = image else {
        return;
    };

    assert_eq!(
        UiDimension::Px(640),
        framed.props().common.width,
        "fullscreen media frame must use the full viewer width so controls and backdrop share the same KatanA fullscreen rect"
    );
    assert_eq!(
        UiDimension::Px(600),
        framed.props().common.height,
        "fullscreen media frame must use the viewport height so controls and body share one KatanA fullscreen rect"
    );
    assert_eq!(
        UiDimension::Px(600),
        image.props().common.height,
        "fullscreen diagram image must be centered inside the same viewport-height media frame"
    );
}

#[test]
fn diagram_frame_keeps_katana_min_container_height_when_controls_are_hidden() {
    let factory =
        KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).interaction(ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: true,
        });
    let node = diagram_node();
    let media =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    let framed = factory.media_with_controls(&node, media);

    assert_eq!(UiNodeKind::ImageSurface, framed.kind());
    assert_eq!(
        UiDimension::Px(KATANA_MIN_CONTROL_CONTAINER_HEIGHT_PX),
        framed.props().common.height
    );
}

#[test]
fn diagram_media_row_wrapper_uses_full_katana_row_width_even_without_extra_height()
-> Result<(), Box<dyn std::error::Error>> {
    let artifact_id = ArtifactId("doc:diagram:row".to_string());
    let artifact = ArtifactFactory::image_asset_with_id(
        artifact_id.clone(),
        ArtifactFormat::Svg,
        DocumentId("document".to_string()),
        SourceRevision("rev".to_string()),
        ArtifactBytes {
            bytes: br#"<svg xmlns="http://www.w3.org/2000/svg" width="320" height="160"><rect width="320" height="160"/></svg>"#.to_vec(),
        },
        "test",
        ArtifactDiagnostics {
            entries: Vec::new(),
        },
    );
    let mut node = diagram_node();
    node.artifact_id = Some(artifact_id);
    let artifacts = [artifact];
    let factory = KucNodeFactory::new(&artifacts, KATANA_VIEWER_ROW_MAX_WIDTH);

    let framed = factory.viewer_node(&node);
    let image = find_image_surface(&framed).kuc_expect("diagram image surface");

    assert_eq!(UiNodeKind::Stack, framed.kind());
    assert_eq!(
        UiVisualRole::MediaFrame,
        framed.props().visual_role,
        "the full-width row owns the media frame background"
    );
    assert_eq!(
        UiDimension::Px(KATANA_VIEWER_ROW_MAX_WIDTH as u16),
        framed.props().common.width
    );
    assert!(
        image.props().image_surface.display_width_milli <= 320_000,
        "the full-width row must not upscale the image body"
    );
    Ok(())
}

#[test]
fn export_surface_diagram_frame_keeps_export_media_role_when_controls_are_hidden() {
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH)
        .export_surface(true)
        .interaction(ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: true,
        });
    let node = diagram_node();
    let media =
        factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), image_surface());

    let framed = factory.media_with_controls(&node, media);

    assert_eq!(UiVisualRole::ExportMediaFrame, framed.props().visual_role);
    assert_eq!(
        UiDimension::Px(37),
        framed.props().common.height,
        "export surface diagram frame follows reference surface margins and does not inherit the interactive controls min-height"
    );
}

#[test]
fn export_surface_diagram_frame_includes_export_vertical_margins_when_controls_are_hidden() {
    let factory = KucNodeFactory::new(&[], DIAGRAM_MEDIA_MAX_WIDTH)
        .export_surface(true)
        .interaction(ViewerInteractionConfig {
            hover_highlight_enabled: false,
            selection_enabled: false,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: false,
        });
    let node = diagram_node();
    let surface = ViewerImageSurface {
        fingerprint: "export-diagram".to_string(),
        width: 860,
        height: 153,
        display_width: 860.0,
        display_height: 153.0,
        content_scale: 100,
        rgba: [0, 0, 0, OPAQUE_ALPHA].repeat(860 * 153),
    };

    let media = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);
    let framed = factory.media_with_controls(&node, media);

    assert_eq!(UiVisualRole::ExportMediaFrame, framed.props().visual_role);
    assert_eq!(
        UiDimension::Px(189),
        framed.props().common.height,
        "export surface diagram frame must include the same top/bottom media margins as the reference surface"
    );
}

#[test]
fn export_surface_short_diagram_frame_does_not_use_interactive_control_min_height() {
    let factory = KucNodeFactory::new(&[], DIAGRAM_MEDIA_MAX_WIDTH)
        .export_surface(true)
        .interaction(ViewerInteractionConfig {
            hover_highlight_enabled: false,
            selection_enabled: false,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: false,
        });
    let node = diagram_node();
    let surface = ViewerImageSurface {
        fingerprint: "export-short-diagram".to_string(),
        width: 860,
        height: 65,
        display_width: 860.0,
        display_height: 65.0,
        content_scale: 100,
        rgba: [0, 0, 0, OPAQUE_ALPHA].repeat(860 * 65),
    };

    let media = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);
    let framed = factory.media_with_controls(&node, media);

    assert_eq!(UiVisualRole::ExportMediaFrame, framed.props().visual_role);
    assert_eq!(
        UiDimension::Px(101),
        framed.props().common.height,
        "export surface capture must match the KDV reference surface height: diagram content plus 18px vertical margins on each side"
    );
}

#[test]
fn diagram_frame_uses_fractional_display_height_when_controls_are_hidden() {
    let factory =
        KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).interaction(ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: true,
        });
    let node = diagram_node();
    let surface = ViewerImageSurface {
        fingerprint: "fractional".to_string(),
        width: 320,
        height: 302,
        display_width: 100.0,
        display_height: 151.2,
        content_scale: 200,
        rgba: [0, 0, 0, OPAQUE_ALPHA].repeat(320 * 302),
    };
    let media = factory.image_surface_node(&node, &ArtifactId("artifact".to_string()), surface);

    let framed = factory.media_with_controls(&node, media);

    assert_eq!(UiNodeKind::ImageSurface, framed.kind());
    assert_eq!(UiDimension::Px(152), framed.props().common.height);
}
