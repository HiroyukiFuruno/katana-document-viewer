use crate::artifact::{Artifact, ArtifactFormat, ArtifactId, ArtifactUri};
use crate::document::SourceRevision;
use katana_markdown_model::KmmNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerAssetLoadPriority {
    Visible,
    NearViewport,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerAssetReference {
    pub node_id: KmmNodeId,
    pub artifact_id: ArtifactId,
    pub uri: ArtifactUri,
    pub format: ArtifactFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerAssetLoadRequest {
    pub document_revision: SourceRevision,
    pub node_id: KmmNodeId,
    pub artifact_id: ArtifactId,
    pub uri: ArtifactUri,
    pub format: ArtifactFormat,
    pub priority: ViewerAssetLoadPriority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerAssetLoadResult {
    pub document_revision: SourceRevision,
    pub artifact_id: ArtifactId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerAssetState {
    pub document_revision: SourceRevision,
    pub requests: Vec<ViewerAssetLoadRequest>,
    pub loaded_artifacts: Vec<ArtifactId>,
}

pub struct ViewerAssetPipeline;

impl ViewerAssetPipeline {
    pub fn initial_state(document_revision: SourceRevision) -> ViewerAssetState {
        ViewerAssetState {
            document_revision,
            requests: Vec::new(),
            loaded_artifacts: Vec::new(),
        }
    }

    pub fn load_requests(
        document_revision: SourceRevision,
        references: &[ViewerAssetReference],
        visible_artifacts: &[ArtifactId],
    ) -> Vec<ViewerAssetLoadRequest> {
        Self::load_requests_for_viewport(document_revision, references, visible_artifacts, &[])
    }

    pub fn load_requests_for_viewport(
        document_revision: SourceRevision,
        references: &[ViewerAssetReference],
        visible_artifacts: &[ArtifactId],
        near_viewport_artifacts: &[ArtifactId],
    ) -> Vec<ViewerAssetLoadRequest> {
        references
            .iter()
            .map(|reference| {
                Self::request(
                    document_revision.clone(),
                    reference,
                    visible_artifacts,
                    near_viewport_artifacts,
                )
            })
            .collect()
    }

    pub fn reference_for_artifact(node_id: KmmNodeId, artifact: &Artifact) -> ViewerAssetReference {
        ViewerAssetReference {
            node_id,
            artifact_id: artifact.manifest.id.clone(),
            uri: artifact.uri.clone(),
            format: artifact.manifest.format,
        }
    }

    pub fn references_for_artifacts(
        node_id: KmmNodeId,
        artifacts: &[Artifact],
    ) -> Vec<ViewerAssetReference> {
        artifacts
            .iter()
            .map(|artifact| Self::reference_for_artifact(node_id.clone(), artifact))
            .collect()
    }

    pub fn accept_result(state: &mut ViewerAssetState, result: ViewerAssetLoadResult) -> bool {
        if result.document_revision != state.document_revision {
            return false;
        }
        if state.loaded_artifacts.contains(&result.artifact_id) {
            return false;
        }
        state.loaded_artifacts.push(result.artifact_id);
        true
    }

    fn request(
        document_revision: SourceRevision,
        reference: &ViewerAssetReference,
        visible_artifacts: &[ArtifactId],
        near_viewport_artifacts: &[ArtifactId],
    ) -> ViewerAssetLoadRequest {
        ViewerAssetLoadRequest {
            document_revision,
            node_id: reference.node_id.clone(),
            artifact_id: reference.artifact_id.clone(),
            uri: reference.uri.clone(),
            format: reference.format,
            priority: Self::priority(
                &reference.artifact_id,
                visible_artifacts,
                near_viewport_artifacts,
            ),
        }
    }

    fn priority(
        artifact_id: &ArtifactId,
        visible_artifacts: &[ArtifactId],
        near_viewport_artifacts: &[ArtifactId],
    ) -> ViewerAssetLoadPriority {
        if visible_artifacts.contains(artifact_id) {
            return ViewerAssetLoadPriority::Visible;
        }
        if near_viewport_artifacts.contains(artifact_id) {
            return ViewerAssetLoadPriority::NearViewport;
        }
        ViewerAssetLoadPriority::Deferred
    }
}
