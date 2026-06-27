use crate::artifact::ArtifactId;
use crate::viewer::layout::ViewerRenderedAnchor;
use crate::viewer::search::{ViewerSearchMatch, ViewerSearchMatchId, ViewerSearchTarget};
use crate::viewer::search_matcher::{ViewerSearchTextMatch, ViewerSearchTextMatcher};
use crate::viewer::types::ViewerRect;
use katana_markdown_model::{KmmNodeId, SourceSpan};
use serde::{Deserialize, Serialize};

const MIN_MATCH_WIDTH: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerArtifactTextExtraction {
    pub artifact_id: ArtifactId,
    pub node_id: KmmNodeId,
    pub source: SourceSpan,
    pub text: String,
}

pub struct ViewerSearchLayoutResolver;

impl ViewerSearchLayoutResolver {
    pub fn resolve_matches(
        matches: &[ViewerSearchMatch],
        anchors: &[ViewerRenderedAnchor],
    ) -> Vec<ViewerSearchTarget> {
        matches
            .iter()
            .enumerate()
            .filter_map(|(index, matched)| Self::target_for_match(index, matched, anchors))
            .collect()
    }

    pub fn matches_from_artifact_text(
        query: &str,
        extractions: &[ViewerArtifactTextExtraction],
    ) -> Vec<ViewerSearchMatch> {
        let mut output = Vec::new();
        for extraction in extractions {
            for matched in ViewerSearchTextMatcher::find(query, &extraction.text) {
                output.push(Self::artifact_match(output.len(), extraction, matched));
            }
        }
        output
    }

    fn target_for_match(
        index: usize,
        matched: &ViewerSearchMatch,
        anchors: &[ViewerRenderedAnchor],
    ) -> Option<ViewerSearchTarget> {
        let anchor = anchors
            .iter()
            .find(|anchor| Self::matches_anchor(matched, anchor))?;
        Some(ViewerSearchTarget {
            index,
            matched: matched.clone(),
            rect: Self::match_rect(matched, anchor),
        })
    }

    fn matches_anchor(matched: &ViewerSearchMatch, anchor: &ViewerRenderedAnchor) -> bool {
        if matched.node_id != anchor.target.node_id {
            return false;
        }
        match &matched.artifact_id {
            Some(artifact_id) => artifact_id == &anchor.target.artifact_id,
            None => true,
        }
    }

    fn match_rect(matched: &ViewerSearchMatch, anchor: &ViewerRenderedAnchor) -> ViewerRect {
        let text_len = matched.source.raw.text.len().max(1) as f32;
        let start = matched.range.start as f32 / text_len;
        let end = matched.range.end as f32 / text_len;
        let x = anchor.target.rect.x + anchor.target.rect.width * start.clamp(0.0, 1.0);
        let width = (anchor.target.rect.width * (end - start).max(0.0))
            .max(MIN_MATCH_WIDTH)
            .min(anchor.target.rect.width);
        ViewerRect {
            x,
            y: anchor.target.rect.y,
            width,
            height: anchor.target.rect.height,
        }
    }

    fn artifact_match(
        index: usize,
        extraction: &ViewerArtifactTextExtraction,
        matched: ViewerSearchTextMatch,
    ) -> ViewerSearchMatch {
        ViewerSearchMatch {
            id: ViewerSearchMatchId(format!("artifact:{}:{index}", extraction.artifact_id.0)),
            node_id: extraction.node_id.clone(),
            source: extraction.source.clone(),
            range: crate::viewer::search::ViewerTextRange {
                start: matched.start,
                end: matched.end,
            },
            text: matched.text,
            artifact_id: Some(extraction.artifact_id.clone()),
        }
    }
}
