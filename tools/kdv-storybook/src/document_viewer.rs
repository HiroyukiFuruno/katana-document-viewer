use katana_document_viewer::PreviewOutput;

#[path = "document_viewer/adapter.rs"]
mod adapter;
#[path = "document_viewer/adapter_layout.rs"]
mod adapter_layout;
#[path = "document_viewer/adapter_slideshow.rs"]
mod adapter_slideshow;
#[path = "document_viewer/adapter_types.rs"]
mod adapter_types;
#[path = "document_viewer/asset_index.rs"]
mod asset_index;
#[path = "document_viewer/config.rs"]
mod config;
#[path = "document_viewer/diagram_control_resolver.rs"]
mod diagram_control_resolver;
#[path = "document_viewer/error.rs"]
mod error;
#[path = "document_viewer/html_details.rs"]
mod html_details;
#[path = "document_viewer/media_control_icons.rs"]
pub(crate) mod media_control_icons;
#[path = "document_viewer/node_factory.rs"]
mod node_factory;
#[path = "document_viewer/node_labels.rs"]
mod node_labels;

pub use adapter_types::{KucViewerAdapter, KucViewerPlan};
pub use config::KucViewerConfig;
pub use diagram_control_resolver::KucDiagramControlResolver;
pub use error::KucViewerError;
#[cfg(test)]
pub(super) use node_factory::KucNodeFactory;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DocumentViewerStorybookHost {
    adapter: KucViewerAdapter,
}

impl DocumentViewerStorybookHost {
    pub fn project(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
    ) -> Result<KucViewerPlan, KucViewerError> {
        Ok(self.adapter.render(output, config))
    }
}

#[cfg(test)]
#[path = "document_viewer/config_tests.rs"]
mod config_tests;
#[cfg(test)]
#[path = "document_viewer_node_contract_tests.rs"]
mod node_contract_tests;
#[cfg(test)]
#[path = "document_viewer/node_labels_tests.rs"]
mod node_labels_tests;
#[cfg(test)]
#[path = "document_viewer_tests.rs"]
mod tests;
