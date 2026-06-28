use katana_document_viewer::{Artifact, ArtifactId};

pub(crate) struct KucArtifactIndex<'a> {
    artifacts: &'a [Artifact],
}

impl<'a> KucArtifactIndex<'a> {
    pub(crate) fn new(artifacts: &'a [Artifact]) -> Self {
        Self { artifacts }
    }

    pub(crate) fn find(&self, artifact_id: &ArtifactId) -> Option<&'a Artifact> {
        self.artifacts
            .iter()
            .find(|artifact| &artifact.manifest.id == artifact_id)
    }
}
