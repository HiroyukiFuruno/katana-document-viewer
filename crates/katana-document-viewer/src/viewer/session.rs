use crate::document::SourceRevision;
use crate::viewer::asset::{ViewerAssetPipeline, ViewerAssetState};
use crate::viewer::search::{ViewerSearchEngine, ViewerSearchState};
use crate::viewer::types::{ViewerInput, ViewerViewport};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerConfigRevision(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerSession {
    pub document_revision: SourceRevision,
    pub config_revision: ViewerConfigRevision,
    pub viewport: ViewerViewport,
    pub scroll_y: f32,
    pub assets: ViewerAssetState,
    pub search: ViewerSearchState,
}

impl ViewerSession {
    pub fn new(input: &ViewerInput, config_revision: ViewerConfigRevision) -> Self {
        Self {
            document_revision: input.snapshot.revision.clone(),
            config_revision,
            viewport: input.viewport,
            scroll_y: 0.0,
            assets: ViewerAssetPipeline::initial_state(input.snapshot.revision.clone()),
            search: ViewerSearchEngine::state("", Vec::new(), None),
        }
    }

    pub fn apply_document(&mut self, input: &ViewerInput) {
        if self.document_revision == input.snapshot.revision {
            self.viewport = input.viewport;
            return;
        }
        self.document_revision = input.snapshot.revision.clone();
        self.viewport = input.viewport;
        self.scroll_y = 0.0;
        self.assets = ViewerAssetPipeline::initial_state(input.snapshot.revision.clone());
        self.search = ViewerSearchEngine::state(self.search.query.clone(), Vec::new(), None);
    }

    pub fn apply_config(&mut self, input: &ViewerInput, config_revision: ViewerConfigRevision) {
        self.config_revision = config_revision;
        self.viewport = input.viewport;
    }
}
