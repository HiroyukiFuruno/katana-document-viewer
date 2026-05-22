use crate::export_html_ops::escape_html;
use crate::export_inline_payload::InlineHtmlWriter;
use katana_markdown_model::{KmmNode, KmmNodeKind};

pub(crate) struct FootnoteHtmlWriter;

impl FootnoteHtmlWriter {
    pub(crate) fn is_definition(node: &KmmNode) -> bool {
        matches!(node.kind, KmmNodeKind::FootnoteDefinition(_))
    }

    pub(crate) fn append_definitions(html: &mut String, nodes: &[KmmNode]) {
        let definitions = nodes
            .iter()
            .filter(|node| Self::is_definition(node))
            .collect::<Vec<_>>();
        if definitions.is_empty() {
            return;
        }
        html.push_str("<section data-kdv-footnotes>\n<ol>\n");
        for node in definitions {
            Self::append_definition(html, node);
        }
        html.push_str("</ol>\n</section>\n");
    }

    fn append_definition(html: &mut String, node: &KmmNode) {
        let KmmNodeKind::FootnoteDefinition(definition) = &node.kind else {
            return;
        };
        let escaped_label = escape_html(&definition.label);
        html.push_str(&format!(
            "<li id=\"fn-{escaped_label}\" data-kdv-footnote-definition=\"{escaped_label}\">"
        ));
        if node.children.is_empty() {
            html.push_str(&escape_html(&definition.text));
        } else {
            InlineHtmlWriter::append_children(html, node);
        }
        html.push_str(&format!(
            " <a href=\"#fnref-{escaped_label}\" data-kdv-footnote-backref=\"{escaped_label}\">↩</a></li>\n"
        ));
    }
}
