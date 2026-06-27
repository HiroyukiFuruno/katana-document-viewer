use super::ViewerNodeClassifier;
use katana_markdown_model::{KmmNode, KmmNodeKind};

impl ViewerNodeClassifier {
    pub(super) fn is_html_table(node: &KmmNode) -> bool {
        matches!(node.kind, KmmNodeKind::HtmlBlock(_))
            && node
                .source
                .raw
                .text
                .trim_start()
                .to_ascii_lowercase()
                .starts_with("<table")
    }
}
