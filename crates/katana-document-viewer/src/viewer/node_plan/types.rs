use crate::artifact::ArtifactId;
use crate::viewer::asset::ViewerAssetLoadRequest;
use crate::viewer::types::ViewerRect;
use katana_markdown_model::{KmmNodeId, SourceSpan};
use serde::{Deserialize, Serialize};

pub use self::types_text::{VIEWER_TEXT_COLOR_CHANNELS, ViewerTextSpan, ViewerTextStyle};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerNodePlan {
    pub nodes: Vec<ViewerNode>,
    pub visible_artifact_ids: Vec<ArtifactId>,
    pub near_viewport_artifact_ids: Vec<ArtifactId>,
    pub asset_requests: Vec<ViewerAssetLoadRequest>,
    pub content_height: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerNode {
    pub node_id: KmmNodeId,
    pub kind: ViewerNodeKind,
    pub source: SourceSpan,
    pub text: String,
    #[serde(default)]
    pub spans: Vec<ViewerTextSpan>,
    #[serde(default)]
    pub html_margin_left_px: u16,
    #[serde(default)]
    pub rule_line_offset_px: u16,
    pub rect: ViewerRect,
    pub artifact_id: Option<ArtifactId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerNodeKind {
    Heading { level: u8 },
    Paragraph,
    Code { language: Option<String> },
    Diagram { kind: ViewerDiagramKind },
    Math,
    Html { role: ViewerHtmlRole },
    Table,
    List,
    BlockQuote,
    Alert { label: String },
    FootnoteDefinition { label: String },
    Image,
    Rule,
    Raw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerDiagramKind {
    Mermaid,
    DrawIo,
    PlantUml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerHtmlRole {
    Generic,
    Left,
    Centered,
    Right,
    Heading {
        level: u8,
        alignment: ViewerHtmlAlignment,
    },
    BadgeRow,
    Accordion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerHtmlAlignment {
    Left,
    Center,
    Right,
}

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

#[path = "types_text.rs"]
mod types_text;
