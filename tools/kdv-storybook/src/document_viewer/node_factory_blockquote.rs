use super::KucNodeFactory;
use super::code::QUOTED_CODE_PADDING_TOP_PX;
use crate::document_viewer::node_labels::KucNodeLabels;
use katana_document_viewer::{ViewerCodeHighlighter, ViewerNode};
use katana_ui_core::atom::Text;
use katana_ui_core::layout::Column;
use katana_ui_core::render_model::{
    UiCommonProps, UiDimension, UiEdgeInsets, UiNode, UiTextWrapMode,
};

const QUOTE_LINE_ROLE: &str = "blockquote";
const QUOTE_INDENT_PX: u16 = 32;
const QUOTE_BULLET_TEXT_OFFSET_PX: u16 = 28;

pub(super) struct QuoteLine {
    depth: usize,
    text: String,
    code: bool,
    bullet: bool,
    language: Option<String>,
}

impl<'a> KucNodeFactory<'a> {
    pub(super) fn blockquote_node(&self, node: &ViewerNode) -> UiNode {
        let lines = QuoteLineParser::parse(&node.source.raw.text);
        if lines.is_empty() {
            return self.text_node(node);
        }
        let mut column = Column::new();
        for line in lines {
            column = column.child(self.blockquote_line_node(line));
        }
        UiNode::from(column)
    }

    fn blockquote_line_node(&self, line: QuoteLine) -> UiNode {
        let label = line.label();
        let text = self.blockquote_text_node(&line, &label);
        Self::decorate_blockquote_line(text.into(), &line)
            .height(UiDimension::Px(self.blockquote_line_height_px(&line)))
    }

    fn blockquote_text_node(&self, line: &QuoteLine, label: &str) -> Text {
        let mut text = Text::new(label.to_string())
            .font_role(line.font_role())
            .text_role(line.text_role())
            .wrap(line.wrap_mode())
            .selectable(self.interaction.selection_enabled);
        if line.code {
            text = text.text_spans(Self::text_spans(&ViewerCodeHighlighter::highlight(
                line.language.as_deref(),
                label,
            )));
        }
        text
    }

    fn decorate_blockquote_line(node: UiNode, line: &QuoteLine) -> UiNode {
        node.common(Self::quote_line_common(line))
    }

    fn quote_line_common(line: &QuoteLine) -> UiCommonProps {
        let mut common = if line.code {
            KucNodeFactory::code_body_common(QUOTED_CODE_PADDING_TOP_PX)
        } else {
            UiCommonProps::default()
        };
        common = common.margin(UiEdgeInsets {
            left: UiDimension::Px(Self::quote_depth_indent(line.depth)),
            ..UiEdgeInsets::default()
        });
        common = common.theme_slot("quote-background");
        if line.bullet {
            return common.padding(UiEdgeInsets {
                left: UiDimension::Px(QUOTE_BULLET_TEXT_OFFSET_PX),
                ..UiEdgeInsets::default()
            });
        }
        common
    }

    fn quote_depth_indent(depth: usize) -> u16 {
        let max_depth = usize::from(u16::MAX / QUOTE_INDENT_PX);
        let capped_depth = depth.min(max_depth);
        let capped_depth = capped_depth as u16;
        capped_depth.saturating_mul(QUOTE_INDENT_PX)
    }

    fn blockquote_line_height_px(&self, line: &QuoteLine) -> u16 {
        if line.code {
            return self.quoted_code_block_height_from_line_count_px(1);
        }
        self.body_line_height_px()
    }
}

impl QuoteLine {
    fn label(&self) -> String {
        if self.text.is_empty() {
            return " ".to_string();
        }
        self.text.clone()
    }

    fn text_role(&self) -> &'static str {
        if self.code {
            return "code";
        }
        QUOTE_LINE_ROLE
    }

    fn wrap_mode(&self) -> UiTextWrapMode {
        if self.code {
            return UiTextWrapMode::NoWrap;
        }
        UiTextWrapMode::Wrap
    }

    fn font_role(&self) -> &'static str {
        if self.code {
            return "code";
        }
        KucNodeLabels::font_role(&katana_document_viewer::ViewerNodeKind::BlockQuote)
    }
}

pub(super) use parser::QuoteLineParser;

#[path = "node_factory_blockquote_parser.rs"]
mod parser;
