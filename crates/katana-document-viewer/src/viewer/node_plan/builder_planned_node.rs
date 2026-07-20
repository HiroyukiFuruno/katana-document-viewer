use super::types::{ViewerNode, ViewerNodeKind, ViewerTextSpan};
use crate::artifact::ArtifactId;
use crate::viewer::asset::ViewerAssetReference;
use crate::viewer::types::ViewerRect;
use katana_markdown_model::{KmmNodeId, SourceSpan};

pub(super) struct PlannedNode {
    pub(super) node_id: KmmNodeId,
    pub(super) kind: ViewerNodeKind,
    pub(super) source: SourceSpan,
    pub(super) text: String,
    pub(super) spans: Vec<ViewerTextSpan>,
    pub(super) reference: Option<ViewerAssetReference>,
}

impl PlannedNode {
    pub(super) fn into_node(
        self,
        rect: ViewerRect,
        artifact_id: Option<ArtifactId>,
        rule_line_offset_px: u16,
    ) -> ViewerNode {
        let html_margin_left_px = Self::html_margin_left_px(&self.source.raw.text);
        ViewerNode {
            node_id: self.node_id,
            kind: self.kind,
            source: self.source,
            text: self.text,
            spans: self.spans,
            html_margin_left_px,
            rule_line_offset_px,
            rect,
            artifact_id,
        }
    }

    pub(super) fn html_margin_left(&self) -> u16 {
        Self::html_margin_left_px(&self.source.raw.text)
    }

    fn html_margin_left_px(raw: &str) -> u16 {
        let lower = raw.to_ascii_lowercase();
        let Some(index) = lower.find("margin-left") else {
            return 0;
        };
        let after_name = &lower[index + "margin-left".len()..];
        let Some(colon_index) = after_name.find(':') else {
            return 0;
        };
        let value = after_name[colon_index + 1..].trim_start();
        let number = value
            .chars()
            .take_while(|character| character.is_ascii_digit() || *character == '.')
            .collect::<String>();
        let Ok(parsed) = number.parse::<f32>() else {
            return 0;
        };
        if value[number.len()..].trim_start().starts_with("rem") {
            return (parsed * 16.0).round().clamp(0.0, f32::from(u16::MAX)) as u16;
        }
        if value[number.len()..].trim_start().starts_with("em") {
            return (parsed * 16.0).round().clamp(0.0, f32::from(u16::MAX)) as u16;
        }
        parsed.round().clamp(0.0, f32::from(u16::MAX)) as u16
    }
}

#[cfg(test)]
#[path = "builder_planned_node_tests.rs"]
mod tests;
