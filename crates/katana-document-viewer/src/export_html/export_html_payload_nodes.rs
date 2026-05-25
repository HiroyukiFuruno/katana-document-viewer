use crate::export_block_payload::BlockHtmlWriter;
use crate::export_inline_payload::InlineHtmlWriter;
use crate::export_list_payload::ListHtmlWriter;
use crate::export_table_payload::TableHtmlWriter;
use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{KmmNode, KmmNodeKind};

pub(crate) struct RemainingHtmlNodeWriter;

impl RemainingHtmlNodeWriter {
    pub(crate) fn append(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
    ) {
        match &node.kind {
            KmmNodeKind::Text(_)
            | KmmNodeKind::Strong(_)
            | KmmNodeKind::Emphasis(_)
            | KmmNodeKind::Strikethrough(_)
            | KmmNodeKind::InlineCode(_)
            | KmmNodeKind::InlineHtml(_)
            | KmmNodeKind::Link(_)
            | KmmNodeKind::Image(_)
            | KmmNodeKind::FootnoteReference(_)
            | KmmNodeKind::InlineMath(_)
            | KmmNodeKind::Emoji(_) => InlineHtmlWriter::append_node(html, node, theme),
            KmmNodeKind::ThematicBreak => html.push_str("<hr>\n"),
            KmmNodeKind::RawBlock { reason } => {
                BlockHtmlWriter::append_raw_block(html, reason, &node.source.raw.text);
            }
            _ => Self::append_structured(html, graph, theme, node),
        }
    }

    fn append_structured(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
    ) {
        match &node.kind {
            KmmNodeKind::List(list) => ListHtmlWriter::append(
                html,
                graph,
                theme,
                list.ordered,
                &list.items,
                &node.source.raw.text,
            ),
            KmmNodeKind::Table(table) => {
                TableHtmlWriter::append(html, table, &node.source.raw.text, theme);
            }
            KmmNodeKind::BlockQuote => BlockHtmlWriter::append_blockquote(html, graph, theme, node),
            KmmNodeKind::Alert { label } => {
                BlockHtmlWriter::append_alert(html, graph, theme, node, label);
            }
            KmmNodeKind::DescriptionList { items } => {
                BlockHtmlWriter::append_description_list(html, items);
            }
            _ => unreachable!("node was handled by HtmlExportPayloadFactory::append_node"),
        }
    }
}
