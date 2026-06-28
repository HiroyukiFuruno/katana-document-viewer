use katana_document_viewer::{
    Artifact, ViewerArtifactSearchResolver, ViewerNode, ViewerNodePlan, ViewerSearchMatch,
    ViewerSearchMatchId, ViewerSearchTarget, ViewerTextRange,
};

pub struct StorybookSearchTargets;

impl StorybookSearchTargets {
    pub fn collect(
        plan: &ViewerNodePlan,
        artifacts: &[Artifact],
        query: &str,
    ) -> Vec<ViewerSearchTarget> {
        let mut targets = Vec::new();
        for node in &plan.nodes {
            Self::push_node_targets(&mut targets, node);
        }
        for target in ViewerArtifactSearchResolver::resolve_targets(query, &plan.nodes, artifacts) {
            targets.push(Self::reindexed_target(targets.len(), target));
        }
        targets
    }

    fn push_node_targets(targets: &mut Vec<ViewerSearchTarget>, node: &ViewerNode) {
        let mut offset = 0;
        for span in &node.spans {
            let end = offset + span.text.len();
            if span.style.highlight {
                targets.push(Self::target(targets.len(), node, offset, end, &span.text));
            }
            offset = end;
        }
    }

    fn target(
        index: usize,
        node: &ViewerNode,
        start: usize,
        end: usize,
        text: &str,
    ) -> ViewerSearchTarget {
        ViewerSearchTarget {
            index,
            matched: ViewerSearchMatch {
                id: ViewerSearchMatchId(format!("storybook:{index}")),
                node_id: node.node_id.clone(),
                source: node.source.clone(),
                range: ViewerTextRange { start, end },
                text: text.to_string(),
                artifact_id: node.artifact_id.clone(),
            },
            rect: node.rect,
        }
    }

    fn reindexed_target(index: usize, mut target: ViewerSearchTarget) -> ViewerSearchTarget {
        target.index = index;
        target
    }
}

#[cfg(test)]
mod tests {
    use super::StorybookSearchTargets;
    use katana_document_viewer::{
        ArtifactBytes, ArtifactFactory, ArtifactFormat, ArtifactId, ByteRange, DocumentId,
        KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceRevision, SourceSpan,
    };
    use katana_document_viewer::{
        ViewerNode, ViewerNodeKind, ViewerNodePlan, ViewerRect, ViewerTextSpan, ViewerTextStyle,
    };

    #[test]
    fn highlighted_spans_become_ordered_search_targets() {
        let plan = ViewerNodePlan {
            nodes: vec![node("Direct fixture", 64.0)],
            visible_artifact_ids: Vec::new(),
            near_viewport_artifact_ids: Vec::new(),
            asset_requests: Vec::new(),
            content_height: 96.0,
        };

        let targets = StorybookSearchTargets::collect(&plan, &[], "Direct");

        assert_eq!(1, targets.len());
        assert_eq!(0, targets[0].index);
        assert_eq!(64.0, targets[0].rect.y);
        assert_eq!("Direct", targets[0].matched.text);
    }

    #[test]
    fn artifact_text_matches_are_included_after_node_targets() {
        let mut artifact_node = node("diagram", 96.0);
        artifact_node.spans = Vec::new();
        artifact_node.artifact_id = Some(ArtifactId("diagram-svg".to_string()));
        let plan = ViewerNodePlan {
            nodes: vec![node("Direct fixture", 64.0), artifact_node],
            visible_artifact_ids: Vec::new(),
            near_viewport_artifact_ids: Vec::new(),
            asset_requests: Vec::new(),
            content_height: 140.0,
        };
        let artifact = ArtifactFactory::image_asset_with_id(
            ArtifactId("diagram-svg".to_string()),
            ArtifactFormat::Svg,
            DocumentId("document".to_string()),
            SourceRevision("rev".to_string()),
            ArtifactBytes {
                bytes: br#"<svg><text>Artifact Needle</text></svg>"#.to_vec(),
            },
            "test",
            katana_document_viewer::ArtifactDiagnostics {
                entries: Vec::new(),
            },
        );

        let targets = StorybookSearchTargets::collect(&plan, &[artifact], "Needle");

        assert_eq!(2, targets.len());
        assert_eq!(0, targets[0].index);
        assert_eq!(1, targets[1].index);
        assert_eq!("Needle", targets[1].matched.text);
        assert_eq!(
            Some(ArtifactId("diagram-svg".to_string())),
            targets[1].matched.artifact_id
        );
    }

    fn node(text: &str, y: f32) -> ViewerNode {
        ViewerNode {
            node_id: KmmNodeId("node-direct".to_string()),
            kind: ViewerNodeKind::Paragraph,
            source: source(text),
            text: text.to_string(),
            spans: vec![
                ViewerTextSpan {
                    text: "Direct".to_string(),
                    style: ViewerTextStyle::default().highlight(),
                    link_target: String::new(),
                },
                ViewerTextSpan::plain(" fixture"),
            ],
            html_margin_left_px: 0,
            rule_line_offset_px: 0,
            rect: ViewerRect {
                x: 0.0,
                y,
                width: 640.0,
                height: 32.0,
            },
            artifact_id: None,
        }
    }

    fn source(text: &str) -> SourceSpan {
        SourceSpan {
            byte_range: ByteRange {
                start: 0,
                end: text.len(),
            },
            line_column_range: LineColumnRange {
                start: LineColumn { line: 1, column: 1 },
                end: LineColumn {
                    line: 1,
                    column: text.len() + 1,
                },
            },
            raw: RawSnippet {
                text: text.to_string(),
            },
        }
    }
}
