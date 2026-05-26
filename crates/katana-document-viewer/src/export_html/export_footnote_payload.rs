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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::KdvThemeSnapshot;
    use katana_markdown_model::{
        ByteRange, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, RawSnippet,
        SourceSpan,
    };

    #[test]
    fn is_definition_works_for_footnote() {
        let node = KmmNode {
            id: KmmNodeId("n".to_string()),
            kind: KmmNodeKind::FootnoteDefinition(katana_markdown_model::FootnoteDefinitionNode {
                label: "1".to_string(),
                text: "note".to_string(),
            }),
            source: source_span("note"),
            children: Vec::new(),
        };
        assert!(FootnoteHtmlWriter::is_definition(&node));
    }

    #[test]
    fn append_definitions_empty_is_noop() {
        let mut html = String::new();
        FootnoteHtmlWriter::append_definitions(&mut html, &[], &KdvThemeSnapshot::katana_light());
        assert_eq!(html, "");
    }

    #[test]
    fn append_definitions_uses_children_for_body() {
        let mut html = String::new();
        let child = KmmNode {
            id: KmmNodeId("child".to_string()),
            kind: KmmNodeKind::Text(katana_markdown_model::TextSpan {
                text: "child".to_string(),
            }),
            source: source_span("child"),
            children: Vec::new(),
        };
        let node = KmmNode {
            id: KmmNodeId("n".to_string()),
            kind: KmmNodeKind::FootnoteDefinition(katana_markdown_model::FootnoteDefinitionNode {
                label: "1".to_string(),
                text: "note".to_string(),
            }),
            source: source_span("note"),
            children: vec![child],
        };
        FootnoteHtmlWriter::append_definitions(
            &mut html,
            std::slice::from_ref(&node),
            &KdvThemeSnapshot::katana_light(),
        );
        assert!(html.contains("data-kdv-footnote-definition=\"1\""));
        assert!(html.contains("child"));
    }

    #[test]
    fn append_definition_with_empty_children_uses_definition_text() {
        let mut html = String::new();
        let node = KmmNode {
            id: KmmNodeId("n".to_string()),
            kind: KmmNodeKind::FootnoteDefinition(katana_markdown_model::FootnoteDefinitionNode {
                label: "1".to_string(),
                text: "note body".to_string(),
            }),
            source: source_span("note body"),
            children: Vec::new(),
        };

        FootnoteHtmlWriter::append_definitions(
            &mut html,
            std::slice::from_ref(&node),
            &KdvThemeSnapshot::katana_light(),
        );

        assert!(html.contains("note body"));
    }

    #[test]
    fn append_definition_ignores_non_definition_nodes() {
        let mut html = String::new();
        let node = KmmNode {
            id: KmmNodeId("n".to_string()),
            kind: KmmNodeKind::Text(katana_markdown_model::TextSpan {
                text: "not a definition".to_string(),
            }),
            source: source_span("not a definition"),
            children: Vec::new(),
        };

        FootnoteHtmlWriter::append_definition(&mut html, &node, &KdvThemeSnapshot::katana_light());

        assert_eq!(html, "");
    }

    fn source_span(text: &str) -> SourceSpan {
        SourceSpan {
            byte_range: ByteRange {
                start: 0,
                end: text.len(),
            },
            line_column_range: LineColumnRange {
                start: LineColumn { line: 1, column: 1 },
                end: LineColumn {
                    line: 1,
                    column: text.len() + 1,
                },
            },
            raw: RawSnippet {
                text: text.to_string(),
            },
        }
    }
}
