use super::super::planned_node::PlannedNode;
use super::super::types::ViewerNodeKind;
use super::ViewerNodePlanBuilder;

impl ViewerNodePlanBuilder<'_> {
    pub(super) fn should_skip_planned_node(&self, planned: &PlannedNode) -> bool {
        if planned.reference.is_some() || !matches!(planned.kind, ViewerNodeKind::Paragraph) {
            return false;
        }
        let text = planned.text.trim();
        text.is_empty() || Self::is_structural_html_container_text(text)
    }

    fn is_structural_html_container_text(text: &str) -> bool {
        if text.starts_with("<!") {
            return true;
        }
        let Some(tag) = text
            .strip_prefix('<')
            .and_then(|value| value.strip_suffix('>'))
        else {
            return false;
        };
        Self::is_structural_html_tag(tag)
    }

    fn is_structural_html_tag(tag: &str) -> bool {
        let tag = tag
            .trim()
            .trim_start_matches('/')
            .trim_end_matches('/')
            .trim();
        let Some(name) = tag.split_whitespace().next() else {
            return false;
        };
        matches!(
            name.to_ascii_lowercase().as_str(),
            "html" | "head" | "body" | "main" | "section" | "article" | "header" | "footer" | "nav"
        )
    }
}
