use super::{ViewerNodeClassifier, ViewerNodeKind, ViewerNodePlanBuilder};
use crate::artifact::{ArtifactId, ArtifactUri};
use crate::viewer::asset::ViewerAssetReference;
use crate::viewer::types::ViewerRect;
use katana_markdown_model::KmmNode;

impl<'a> ViewerNodePlanBuilder<'a> {
    pub(super) fn asset_reference(
        &self,
        node: &KmmNode,
        kind: &ViewerNodeKind,
    ) -> Option<ViewerAssetReference> {
        let format = ViewerNodeClassifier::asset_format(node, kind)?;
        let artifact_id = ArtifactId(format!(
            "{}:{}:{format:?}",
            self.input.snapshot.id.0, node.id.0
        ));
        let uri = self.asset_uri(node, kind, &artifact_id)?;
        Some(ViewerAssetReference {
            node_id: node.id.clone(),
            uri,
            artifact_id,
            format,
        })
    }

    pub(super) fn push_asset_reference(
        &mut self,
        reference: &ViewerAssetReference,
        rect: ViewerRect,
    ) {
        if self.is_visible(rect) {
            self.visible_artifact_ids
                .push(reference.artifact_id.clone());
        } else if self.is_near_viewport(rect) {
            self.near_viewport_artifact_ids
                .push(reference.artifact_id.clone());
        }
        self.asset_references.push(reference.clone());
    }

    fn asset_uri(
        &self,
        node: &KmmNode,
        kind: &ViewerNodeKind,
        artifact_id: &ArtifactId,
    ) -> Option<ArtifactUri> {
        if *kind == ViewerNodeKind::Image {
            return ViewerNodeClassifier::image_source(node)
                .map(|source| ArtifactUri(source.to_string()));
        }
        Some(ArtifactUri(format!("kdv://viewer-asset/{}", artifact_id.0)))
    }
}
