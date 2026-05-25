use crate::export_html_ops::ExportHtmlOps;
use crate::export_inline_payload::InlineHtmlWriter;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{KmmNode, KmmNodeKind};

pub(crate) struct FootnoteHtmlWriter;

impl FootnoteHtmlWriter {
    pub(crate) fn is_definition(node: &KmmNode) -> bool {
        matches!(node.kind, KmmNodeKind::FootnoteDefinition(_))
    }

    pub(crate) fn append_definitions(
        html: &mut String,
        nodes: &[KmmNode],
        theme: &KdvThemeSnapshot,
    ) {
        let definitions = nodes
            .iter()
            .filter(|node| Self::is_definition(node))
            .collect::<Vec<_>>();
        if definitions.is_empty() {
            return;
        }
        html.push_str("<section data-kdv-footnotes>\n<ol>\n");
        for node in definitions {
            Self::append_definition(html, node, theme);
        }
        html.push_str("</ol>\n</section>\n");
    }

    fn append_definition(html: &mut String, node: &KmmNode, theme: &KdvThemeSnapshot) {
        let KmmNodeKind::FootnoteDefinition(definition) = &node.kind else {
            return;
        };
        let escaped_label = ExportHtmlOps::escape_html(&definition.label);
        html.push_str(&format!(
            "<li id=\"fn-{escaped_label}\" data-kdv-footnote-definition=\"{escaped_label}\">"
        ));
        if node.children.is_empty() {
            html.push_str(&ExportHtmlOps::escape_html(&definition.text));
        } else {
            InlineHtmlWriter::append_children(html, node, theme);
        }
        html.push_str(&format!(
            " <a href=\"#fnref-{escaped_label}\" data-kdv-footnote-backref=\"{escaped_label}\">↩</a></li>\n"
        ));
    }
}
