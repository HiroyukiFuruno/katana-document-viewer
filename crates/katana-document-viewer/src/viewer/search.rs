use crate::artifact::ArtifactId;
use crate::viewer::commands::ViewerScrollCommand;
use crate::viewer::types::{ViewerRect, ViewerTarget};
use katana_markdown_model::{KmmNodeId, SourceSpan};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerSearchMatchId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerSearchDirection {
    Previous,
    Next,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerTextRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerSearchMatch {
    pub id: ViewerSearchMatchId,
    pub node_id: KmmNodeId,
    pub source: SourceSpan,
    pub range: ViewerTextRange,
    pub text: String,
    pub artifact_id: Option<ArtifactId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerSearchTarget {
    pub index: usize,
    pub matched: ViewerSearchMatch,
    pub rect: ViewerRect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerSearchState {
    pub query: String,
    pub matches: Vec<ViewerSearchTarget>,
    pub current_index: Option<usize>,
}

impl Default for ViewerSearchState {
    fn default() -> Self {
        ViewerSearchEngine::state("", Vec::new(), None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerSearchHighlightKind {
    Match,
    Current,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerSearchHighlight {
    pub target: ViewerSearchTarget,
    pub kind: ViewerSearchHighlightKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerSearchCommand {
    pub direction: ViewerSearchDirection,
    pub target: ViewerSearchTarget,
    pub scroll: ViewerScrollCommand,
}

pub struct ViewerSearchEngine;

impl ViewerSearchEngine {
    pub fn state(
        query: impl Into<String>,
        matches: Vec<ViewerSearchTarget>,
        current_index: Option<usize>,
    ) -> ViewerSearchState {
        ViewerSearchState {
            query: query.into(),
            matches,
            current_index,
        }
    }

    pub fn navigate(
        state: &ViewerSearchState,
        direction: ViewerSearchDirection,
    ) -> Option<ViewerSearchCommand> {
        let next_index = Self::next_index(state, direction)?;
        let target = state.matches.get(next_index)?.clone();
        let scroll = ViewerScrollCommand {
            target: Self::scroll_target(&target),
        };
        Some(ViewerSearchCommand {
            direction,
            target,
            scroll,
        })
    }

    pub fn highlights(state: &ViewerSearchState) -> Vec<ViewerSearchHighlight> {
        state
            .matches
            .iter()
            .cloned()
            .map(|target| ViewerSearchHighlight {
                kind: Self::highlight_kind(state, target.index),
                target,
            })
            .collect()
    }

    pub fn next_index(
        state: &ViewerSearchState,
        direction: ViewerSearchDirection,
    ) -> Option<usize> {
        let len = state.matches.len();
        if len == 0 {
            return None;
        }
        let current = state.current_index.unwrap_or(match direction {
            ViewerSearchDirection::Next => len - 1,
            ViewerSearchDirection::Previous => 0,
        });
        Some(match direction {
            ViewerSearchDirection::Next => (current + 1) % len,
            ViewerSearchDirection::Previous => (current + len - 1) % len,
        })
    }

    fn highlight_kind(state: &ViewerSearchState, target_index: usize) -> ViewerSearchHighlightKind {
        match state.current_index {
            Some(current_index) if current_index == target_index => {
                ViewerSearchHighlightKind::Current
            }
            _ => ViewerSearchHighlightKind::Match,
        }
    }

    fn scroll_target(target: &ViewerSearchTarget) -> ViewerTarget {
        ViewerTarget {
            node_id: target.matched.node_id.clone(),
            source: target.matched.source.clone(),
            artifact_id: target
                .matched
                .artifact_id
                .clone()
                .unwrap_or_else(|| ArtifactId(format!("search:{}", target.index))),
            rect: target.rect,
        }
    }
}
