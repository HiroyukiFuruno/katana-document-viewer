use super::KucNodeFactory;
use crate::document_viewer::html_details::DetailsParts;
use katana_document_viewer::{
    MarkdownSource, PreviewConfig, PreviewOutputFactory, ViewerMode, ViewerNode, ViewerNodePlanner,
    ViewerSearchState, ViewerViewport,
};
use katana_ui_core::atom::Text;
use katana_ui_core::layout::Column;
use katana_ui_core::molecule::Accordion;
use katana_ui_core::render_model::{UiNode, UiTextProps, UiTextWrapMode};

impl<'a> KucNodeFactory<'a> {
    pub(crate) fn accordion_open_overrides(
        mut self,
        value: &'a std::collections::BTreeMap<String, bool>,
    ) -> Self {
        self.accordion_open_overrides = Some(value);
        self
    }

    pub(super) fn accordion_node(&self, node: &ViewerNode) -> UiNode {
        let parts = DetailsParts::parse(&node.source.raw.text)
            .or_else(|| DetailsParts::from_plain_text(&node.text));
        let Some(parts) = parts else {
            return self.text_node(node);
        };
        let node_id = node.node_id.0.clone();
        let body = self.accordion_body_node(&node_id, &parts.body);
        let node: UiNode = Accordion::new(parts.summary)
            .open(self.accordion_open(node, parts.open))
            .child(body)
            .into();
        node.font_role("body")
            .text(UiTextProps {
                role: self.accordion_text_role().to_string(),
                wrap: UiTextWrapMode::Wrap,
                ..UiTextProps::default()
            })
            .stable_node_id(node_id.clone())
            .stable_state_id(node_id)
    }

    fn accordion_body_node(&self, node_id: &str, body: &str) -> UiNode {
        self.markdown_body_node(node_id, body)
            .unwrap_or_else(|| self.plain_body_node(body))
    }

    fn markdown_body_node(&self, node_id: &str, body: &str) -> Option<UiNode> {
        if body.trim().is_empty() {
            return None;
        }
        let source = MarkdownSource {
            content: body.to_string(),
            document_id: Some(format!("accordion-body-{node_id}.md")),
        };
        let output =
            PreviewOutputFactory::from_source(&source, &self.body_preview_config(), 0.0).ok()?;
        let plan = ViewerNodePlanner::create(&output.input, 0.0);
        if plan.nodes.is_empty() {
            return None;
        }
        let mut column = Column::new();
        for child in &plan.nodes {
            column = column.child(self.viewer_node(child));
        }
        Some(column.into())
    }

    fn plain_body_node(&self, body: &str) -> UiNode {
        Text::new(body)
            .text_role(self.accordion_body_text_role())
            .wrap(UiTextWrapMode::Wrap)
            .selectable(self.interaction.selection_enabled)
            .into()
    }

    fn body_preview_config(&self) -> PreviewConfig {
        PreviewConfig {
            theme: Default::default(),
            base_font_size: Some(f32::from(self.typography.preview_font_size)),
            line_height: None,
            mode: ViewerMode::Document,
            interaction: self.interaction.clone(),
            viewport: ViewerViewport {
                width: self.content_width as f32,
                height: f32::from(u16::MAX),
            },
            scroll_offset: 0.0,
            search: ViewerSearchState::default(),
        }
    }

    fn accordion_open(&self, node: &ViewerNode, default_open: bool) -> bool {
        self.accordion_open_overrides
            .and_then(|overrides| overrides.get(&node.node_id.0).copied())
            .unwrap_or(default_open)
    }

    fn accordion_text_role(&self) -> &'static str {
        if self.export_surface {
            "html-accordion"
        } else {
            "html-accordion-preview"
        }
    }

    fn accordion_body_text_role(&self) -> &'static str {
        if self.export_surface {
            "html-accordion-body"
        } else {
            "html-accordion-body-preview"
        }
    }
}
