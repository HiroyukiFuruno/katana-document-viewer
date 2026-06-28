use super::media_geometry::{
    DIAGRAM_EXPORT_MAX_WIDTH, MATH_MAX_WIDTH, capped_diagram_height, capped_diagram_width,
    display_height_for_width, fullscreen_diagram_width,
};
use super::{KucNodeFactory, KucNodeLabels};
use katana_document_viewer::{
    Artifact, ArtifactId, DiagnosticSeverity, ViewerArtifactSearchResolver,
    ViewerImageSurfaceError, ViewerImageSurfaceFactory, ViewerNode, ViewerNodeKind,
};
use katana_ui_core::atom::{ImageSurface, Spinner};
use katana_ui_core::layout::{AlignCenter, Row};
use katana_ui_core::render_model::{
    UiImageSurfaceFit, UiImageSurfaceTransform, UiNode, UiVisualRole,
};

impl<'a> KucNodeFactory<'a> {
    pub(super) fn media_node(&self, node: &ViewerNode) -> UiNode {
        let Some((artifact_id, artifact)) = self.media_artifact(node) else {
            return self.pending_node(node);
        };
        if Self::has_error_diagnostic(artifact) {
            eprintln!("[kdv-kuc] artifact render error for {}", artifact_id.0);
            return self.media_with_controls(node, self.media_error_node());
        }
        self.media_with_controls(
            node,
            self.render_media_artifact(node, artifact_id, artifact),
        )
    }

    fn media_artifact<'b>(
        &'b self,
        node: &'b ViewerNode,
    ) -> Option<(&'b ArtifactId, &'b Artifact)> {
        let artifact_id = node.artifact_id.as_ref()?;
        let artifact = self.artifacts.find(artifact_id)?;
        Some((artifact_id, artifact))
    }

    fn render_media_artifact(
        &self,
        node: &ViewerNode,
        artifact_id: &ArtifactId,
        artifact: &Artifact,
    ) -> UiNode {
        let surface = self.media_surface_for_artifact(node, artifact);
        match surface {
            Ok(surface) => {
                self.image_surface_node_for_artifact(node, artifact_id, surface, Some(artifact))
            }
            Err(error) => {
                eprintln!(
                    "[kdv-kuc] artifact rasterize error for {}: {error}",
                    artifact_id.0
                );
                self.media_error_node()
            }
        }
    }

    fn media_surface_for_artifact(
        &self,
        node: &ViewerNode,
        artifact: &Artifact,
    ) -> Result<katana_document_viewer::ViewerImageSurface, ViewerImageSurfaceError> {
        if matches!(node.kind, ViewerNodeKind::Math) {
            return ViewerImageSurfaceFactory::from_math_artifact(artifact);
        }
        if self.export_surface && matches!(node.kind, ViewerNodeKind::Diagram { .. }) {
            return ViewerImageSurfaceFactory::from_export_surface_diagram_artifact(
                artifact,
                self.media_max_width(node),
            );
        }
        if self.diagram_fullscreen_open(node) {
            return self.fullscreen_diagram_surface(node, artifact);
        }
        if matches!(node.kind, ViewerNodeKind::Diagram { .. }) {
            return self.diagram_surface(node, artifact);
        }
        ViewerImageSurfaceFactory::from_artifact(artifact, self.media_max_width(node))
    }

    fn fullscreen_diagram_surface(
        &self,
        node: &ViewerNode,
        artifact: &Artifact,
    ) -> Result<katana_document_viewer::ViewerImageSurface, ViewerImageSurfaceError> {
        if let Some(background) = self.viewer_background {
            return ViewerImageSurfaceFactory::from_fullscreen_diagram_artifact_with_background(
                artifact,
                self.media_max_width(node),
                background,
            );
        }
        ViewerImageSurfaceFactory::from_fullscreen_diagram_artifact(
            artifact,
            self.media_max_width(node),
        )
    }

    fn diagram_surface(
        &self,
        node: &ViewerNode,
        artifact: &Artifact,
    ) -> Result<katana_document_viewer::ViewerImageSurface, ViewerImageSurfaceError> {
        if let Some(background) = self.viewer_background {
            return ViewerImageSurfaceFactory::from_diagram_artifact_with_background(
                artifact,
                self.media_max_width(node),
                background,
            );
        }
        ViewerImageSurfaceFactory::from_diagram_artifact(artifact, self.media_max_width(node))
    }

    pub(super) fn image_surface_node(
        &self,
        node: &ViewerNode,
        artifact_id: &ArtifactId,
        surface: katana_document_viewer::ViewerImageSurface,
    ) -> UiNode {
        self.image_surface_node_for_artifact(
            node,
            artifact_id,
            surface,
            self.artifacts.find(artifact_id),
        )
    }

    fn image_surface_node_for_artifact(
        &self,
        node: &ViewerNode,
        artifact_id: &ArtifactId,
        surface: katana_document_viewer::ViewerImageSurface,
        artifact: Option<&Artifact>,
    ) -> UiNode {
        let label = KucNodeLabels::media_label(node);
        let display_width = self.display_width(node, &surface);
        let display_height = self.display_height(node, &surface);
        match ImageSurface::from_rgba(
            label.clone(),
            surface.fingerprint,
            surface.width,
            surface.height,
            surface.rgba,
        ) {
            Ok(image) => {
                let mut image = image
                    .content_scale(surface.content_scale)
                    .display_size_exact(display_width, display_height)
                    .fit(UiImageSurfaceFit::Contain)
                    .accessibility_label(label);
                if self.interaction.selection_enabled
                    && let Some(text) =
                        artifact.and_then(ViewerArtifactSearchResolver::artifact_text)
                {
                    image = image.selection_text(text);
                }
                let media_node: UiNode = self.image_surface_transform(node, image).into();
                self.export_surface_media_node(media_node, node)
            }
            Err(error) => {
                eprintln!(
                    "[kdv-kuc] invalid image surface for {}: {error}",
                    artifact_id.0
                );
                self.media_error_node()
            }
        }
    }

    fn image_surface_transform(&self, node: &ViewerNode, image: ImageSurface) -> ImageSurface {
        let Some(state) = self.media_viewport_state(node) else {
            return image;
        };
        image.transform(UiImageSurfaceTransform::new(
            (state.zoom * 100.0).round().max(1.0) as u32,
            state.pan.x.round() as i32,
            state.pan.y.round() as i32,
        ))
    }

    fn export_surface_media_node(&self, media: UiNode, node: &ViewerNode) -> UiNode {
        if self.export_surface && matches!(node.kind, ViewerNodeKind::Diagram { .. }) {
            return media.visual_role(UiVisualRole::ExportMediaFrame);
        }
        media.visual_role(UiVisualRole::MediaFrame)
    }

    fn media_viewport_state(
        &self,
        node: &ViewerNode,
    ) -> Option<&katana_document_viewer::DiagramViewportState> {
        match node.kind {
            ViewerNodeKind::Diagram { .. } => self
                .diagram_viewports
                .and_then(|viewports| viewports.get(node.node_id.0.as_str())),
            ViewerNodeKind::Image => self
                .image_viewports
                .and_then(|viewports| viewports.get(node.node_id.0.as_str())),
            _ => None,
        }
    }

    fn display_width(
        &self,
        node: &ViewerNode,
        surface: &katana_document_viewer::ViewerImageSurface,
    ) -> f32 {
        if self.export_surface && matches!(node.kind, ViewerNodeKind::Diagram { .. }) {
            return capped_diagram_width(surface.display_width, DIAGRAM_EXPORT_MAX_WIDTH);
        }
        if self.diagram_fullscreen_open(node) {
            return fullscreen_diagram_width(
                surface,
                self.max_media_width,
                self.fullscreen_viewport_width,
                self.fullscreen_viewport_height,
            );
        }
        surface.display_width
    }

    fn display_height(
        &self,
        node: &ViewerNode,
        surface: &katana_document_viewer::ViewerImageSurface,
    ) -> f32 {
        if self.export_surface && matches!(node.kind, ViewerNodeKind::Diagram { .. }) {
            return capped_diagram_height(
                surface.display_width,
                surface.display_height,
                DIAGRAM_EXPORT_MAX_WIDTH,
            );
        }
        if self.diagram_fullscreen_open(node) {
            return display_height_for_width(
                surface.display_width,
                surface.display_height,
                fullscreen_diagram_width(
                    surface,
                    self.max_media_width,
                    self.fullscreen_viewport_width,
                    self.fullscreen_viewport_height,
                ),
            );
        }
        surface.display_height
    }

    pub(super) fn diagram_fullscreen_open(&self, node: &ViewerNode) -> bool {
        matches!(node.kind, ViewerNodeKind::Diagram { .. })
            && self
                .media_viewport_state(node)
                .is_some_and(|state| state.fullscreen_open)
    }

    pub(super) fn media_max_width(&self, node: &ViewerNode) -> u32 {
        match node.kind {
            ViewerNodeKind::Diagram { .. } => self.diagram_raster_max_width(),
            ViewerNodeKind::Math => MATH_MAX_WIDTH,
            _ => self.max_media_width,
        }
    }

    fn diagram_raster_max_width(&self) -> u32 {
        if self.export_surface {
            return DIAGRAM_EXPORT_MAX_WIDTH;
        }
        self.max_media_width
    }

    fn pending_node(&self, node: &ViewerNode) -> UiNode {
        let pending: UiNode =
            AlignCenter::new()
                .child(Row::new().child(Spinner::new("loading")).child(
                    self.text_with_role(KucNodeLabels::rendering_label(node), "media-pending"),
                ))
                .into();
        self.media_with_controls(node, pending)
    }

    fn media_error_node(&self) -> UiNode {
        self.text_with_role("media-error".to_string(), "media-error")
    }

    fn has_error_diagnostic(artifact: &Artifact) -> bool {
        artifact
            .manifest
            .diagnostics
            .entries
            .iter()
            .any(|entry| entry.severity == DiagnosticSeverity::Error)
    }
}
