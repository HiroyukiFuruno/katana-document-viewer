use super::super::types::ViewerNodeKind;
use super::ViewerNodeClassifier;
use crate::artifact::ArtifactFormat;
use katana_markdown_model::{ImageNode, KmmNode, KmmNodeKind};
use std::path::Path;

impl ViewerNodeClassifier {
    pub(in crate::viewer::node_plan) fn asset_format(
        node: &KmmNode,
        kind: &ViewerNodeKind,
    ) -> Option<ArtifactFormat> {
        match kind {
            ViewerNodeKind::Diagram { .. } | ViewerNodeKind::Math => Some(ArtifactFormat::Svg),
            ViewerNodeKind::Html { .. } => Some(ArtifactFormat::Html),
            ViewerNodeKind::Image => Self::image_format(node),
            _ => None,
        }
    }

    pub(super) fn standalone_image(node: &KmmNode) -> Option<&ImageNode> {
        if node.kind != KmmNodeKind::Paragraph {
            return None;
        }
        if !Self::source_is_standalone_image(&node.source.raw.text) {
            return None;
        }
        let [child] = node.children.as_slice() else {
            return None;
        };
        match &child.kind {
            KmmNodeKind::Image(image) => Some(image),
            _ => None,
        }
    }

    fn source_is_standalone_image(raw: &str) -> bool {
        let trimmed = raw.trim();
        trimmed.starts_with("![") && trimmed.ends_with(')') && trimmed.find("](").is_some()
    }

    pub(super) fn is_details_html(node: &KmmNode) -> bool {
        matches!(node.kind, KmmNodeKind::HtmlBlock(_))
            && node.source.raw.text.trim_start().starts_with("<details")
            && node.source.raw.text.contains("<summary>")
    }

    fn image_format(node: &KmmNode) -> Option<ArtifactFormat> {
        let source = Self::image_source(node)?;
        match Path::new(Self::strip_query_fragment(source))
            .extension()?
            .to_str()?
            .to_ascii_lowercase()
            .as_str()
        {
            "png" => Some(ArtifactFormat::Png),
            "jpg" | "jpeg" => Some(ArtifactFormat::Jpeg),
            "gif" => Some(ArtifactFormat::Gif),
            "webp" => Some(ArtifactFormat::Webp),
            "bmp" => Some(ArtifactFormat::Bmp),
            "svg" => Some(ArtifactFormat::Svg),
            _ => None,
        }
    }

    pub(in crate::viewer::node_plan) fn image_source(node: &KmmNode) -> Option<&str> {
        match &node.kind {
            KmmNodeKind::Image(image) => Some(&image.src),
            _ => Self::standalone_image(node).map(|image| image.src.as_str()),
        }
    }

    fn strip_query_fragment(value: &str) -> &str {
        value.split(['?', '#']).next().unwrap_or(value)
    }
}
