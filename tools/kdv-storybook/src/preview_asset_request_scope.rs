use katana_document_viewer::{
    Artifact, ArtifactFormat, PreviewAssetLoadReport, PreviewOutput, ViewerAssetLoadRequest,
    ViewerNodePlanner,
};

pub(crate) struct PreviewAssetRequestScope;

impl PreviewAssetRequestScope {
    pub(crate) fn pending_asset_requests(output: &PreviewOutput) -> Vec<ViewerAssetLoadRequest> {
        ViewerNodePlanner::create(&output.input, output.scroll_offset)
            .asset_requests
            .into_iter()
            .filter(Self::loads_in_incremental_scope)
            .filter(|request| {
                !output
                    .input
                    .artifacts
                    .iter()
                    .any(|artifact| artifact.manifest.id == request.artifact_id)
            })
            .collect()
    }

    pub(crate) fn append_artifact(
        output: &mut PreviewOutput,
        artifact: Artifact,
        report: &mut PreviewAssetLoadReport,
    ) {
        if artifact.manifest.diagnostics.entries.is_empty() {
            report.loaded_artifact_count += 1;
        } else {
            report.failed_artifact_count += 1;
        }
        output.input.artifacts.push(artifact);
    }

    fn loads_in_incremental_scope(request: &ViewerAssetLoadRequest) -> bool {
        request.format != ArtifactFormat::Html
    }
}

#[cfg(test)]
mod tests {
    use super::PreviewAssetRequestScope;
    use katana_document_viewer::{
        MarkdownSource, PreviewConfig, PreviewOutputFactory, ViewerAssetLoadPriority,
        ViewerNodePlanner,
    };

    #[test]
    fn pending_asset_requests_include_deferred_document_assets()
    -> Result<(), Box<dyn std::error::Error>> {
        let body = "paragraph\n\n".repeat(80);
        let source = MarkdownSource {
            content: format!(
                "# Diagrams\n\n```mermaid\ngraph TD\nA --> B\n```\n\n{body}```mermaid\ngraph TD\nC --> D\n```"
            ),
            document_id: Some("document-scope.md".to_string()),
        };
        let config = PreviewConfig::default();
        let output = PreviewOutputFactory::from_source(&source, &config, 4000.0)?;
        let plan = ViewerNodePlanner::create(&output.input, 0.0);

        assert!(
            plan.asset_requests
                .iter()
                .any(|request| request.priority == ViewerAssetLoadPriority::Deferred)
        );
        assert_eq!(
            plan.asset_requests.len(),
            PreviewAssetRequestScope::pending_asset_requests(&output).len()
        );
        Ok(())
    }
}
