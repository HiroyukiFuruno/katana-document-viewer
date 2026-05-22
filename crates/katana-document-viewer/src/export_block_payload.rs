use crate::export_html_ops::{alert_body, escape_html, render_text};
use crate::export_html_payload::append_node;
use crate::export_inline_payload::InlineHtmlWriter;
use crate::export_legacy_note_payload::LegacyNoteHtmlWriter;
use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{DescriptionItem, KmmNode};

pub(crate) struct BlockHtmlWriter;

impl BlockHtmlWriter {
    pub(crate) fn append_blockquote(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
    ) {
        if NestedBlockquoteHtml::is_nested(&node.source.raw.text) {
            NestedBlockquoteHtml::append(html, &node.source.raw.text);
            return;
        }
        html.push_str("<blockquote data-kdv-blockquote=\"quote\">");
        if node.children.is_empty() {
            html.push_str(&render_text(&node.source.raw.text));
        } else {
            Self::append_children(html, graph, theme, node);
        }
        html.push_str("</blockquote>\n");
    }

    pub(crate) fn append_alert(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
        label: &str,
    ) {
        if !Self::is_gfm_alert(&node.source.raw.text) {
            LegacyNoteHtmlWriter::append(html, &node.source.raw.text);
            return;
        }
        html.push_str(&format!(
            "<aside data-github-alert=\"{}\" data-kdv-blockquote=\"alert\">",
            escape_html(label)
        ));
        Self::append_alert_title(html, label);
        if node.children.is_empty() {
            html.push_str(&format!(
                "<p>{}</p>",
                render_text(&alert_body(&node.source.raw.text))
            ));
        } else {
            Self::append_alert_children(html, graph, theme, node, label);
        }
        html.push_str("</aside>\n");
    }

    pub(crate) fn append_description_list(html: &mut String, items: &[DescriptionItem]) {
        html.push_str("<dl>\n");
        for item in items {
            html.push_str(&format!(
                "<dt>{}</dt><dd>{}</dd>\n",
                render_text(&item.term),
                render_text(&item.description)
            ));
        }
        html.push_str("</dl>\n");
    }

    pub(crate) fn append_raw_block(html: &mut String, reason: &str, text: &str) {
        html.push_str(&format!(
            "<pre data-kdv-raw-reason=\"{}\">{}</pre>\n",
            escape_html(reason),
            escape_html(text)
        ));
    }

    fn append_children(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
    ) {
        for child in &node.children {
            append_node(html, graph, theme, child);
        }
    }

    fn append_alert_children(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
        label: &str,
    ) {
        let mut children = node.children.iter();
        if let Some(first) = children.next()
            && !Self::is_alert_title(first, label)
        {
            append_node(html, graph, theme, first);
        }
        for child in children {
            append_node(html, graph, theme, child);
        }
    }

    fn append_alert_title(html: &mut String, label: &str) {
        let title = Self::alert_title(label);
        html.push_str(&format!(
            "<p data-kdv-alert-title=\"{}\"><span data-kdv-alert-icon=\"{}\" aria-hidden=\"true\">{}</span><strong>{}</strong></p>",
            escape_html(label),
            escape_html(label),
            Self::alert_icon_svg(label),
            title
        ));
    }

    fn is_gfm_alert(raw: &str) -> bool {
        raw.lines()
            .next()
            .map(|line| {
                line.trim_start()
                    .strip_prefix('>')
                    .unwrap_or(line)
                    .trim_start()
                    .starts_with("[!")
            })
            .unwrap_or(false)
    }

    fn is_alert_title(node: &KmmNode, label: &str) -> bool {
        let expected = Self::alert_title(label);
        node.children.iter().any(|child| {
            matches!(&child.kind, katana_markdown_model::KmmNodeKind::Strong(span) if span.text == expected)
        })
    }

    fn alert_title(label: &str) -> &'static str {
        match label {
            "NOTE" => "Note",
            "TIP" => "Tip",
            "IMPORTANT" => "Important",
            "WARNING" => "Warning",
            "CAUTION" => "Caution",
            _ => "Note",
        }
    }

    fn alert_icon_svg(label: &str) -> &'static str {
        match label {
            "NOTE" => {
                r#"<svg data-kdv-alert-icon-svg="NOTE" viewBox="0 0 16 16"><circle cx="8" cy="8" r="6"></circle><path d="M8 7v4"></path><path d="M8 5h.01"></path></svg>"#
            }
            "TIP" => {
                r#"<svg data-kdv-alert-icon-svg="TIP" viewBox="0 0 16 16"><path d="M6 13h4"></path><path d="M6.5 10.5h3"></path><path d="M5 6a3 3 0 1 1 6 0c0 1.1-.55 1.85-1.15 2.45-.45.45-.85.95-.85 1.55H7c0-.6-.4-1.1-.85-1.55C5.55 7.85 5 7.1 5 6z"></path></svg>"#
            }
            "IMPORTANT" => {
                r#"<svg data-kdv-alert-icon-svg="IMPORTANT" viewBox="0 0 16 16"><path d="M3 3.5h10v8H8l-3 2v-2H3z"></path><path d="M8 5.5v3"></path><path d="M8 10.5h.01"></path></svg>"#
            }
            "WARNING" => {
                r#"<svg data-kdv-alert-icon-svg="WARNING" viewBox="0 0 16 16"><path d="M8 2.5 14 13H2z"></path><path d="M8 6v3"></path><path d="M8 11h.01"></path></svg>"#
            }
            "CAUTION" => {
                r#"<svg data-kdv-alert-icon-svg="CAUTION" viewBox="0 0 16 16"><path d="M5 2h6l3 3v6l-3 3H5l-3-3V5z"></path><path d="M8 5v4"></path><path d="M8 11h.01"></path></svg>"#
            }
            _ => {
                r#"<svg data-kdv-alert-icon-svg="NOTE" viewBox="0 0 16 16"><circle cx="8" cy="8" r="6"></circle><path d="M8 7v4"></path><path d="M8 5h.01"></path></svg>"#
            }
        }
    }
}

struct NestedBlockquoteHtml;

impl NestedBlockquoteHtml {
    fn is_nested(text: &str) -> bool {
        text.lines()
            .filter_map(Self::line_parts)
            .any(|line| line.depth > 1)
    }

    fn append(html: &mut String, text: &str) {
        let mut current_depth = 0;
        for line in text.lines().filter_map(Self::line_parts) {
            while current_depth > line.depth {
                html.push_str("</blockquote>\n");
                current_depth -= 1;
            }
            while current_depth < line.depth {
                current_depth += 1;
                html.push_str(&format!(
                    "<blockquote data-kdv-blockquote=\"quote\" data-kdv-quote-depth=\"{current_depth}\">"
                ));
            }
            if !line.text.trim().is_empty() {
                html.push_str("<p>");
                InlineHtmlWriter::append_fragment(html, line.text);
                html.push_str("</p>\n");
            }
        }
        while current_depth > 0 {
            html.push_str("</blockquote>\n");
            current_depth -= 1;
        }
    }

    fn line_parts(line: &str) -> Option<BlockquoteLine<'_>> {
        let mut rest = line.trim_start();
        let mut depth = 0;
        while let Some(stripped) = rest.strip_prefix('>') {
            depth += 1;
            rest = stripped.trim_start();
        }
        (depth > 0).then_some(BlockquoteLine { depth, text: rest })
    }
}

struct BlockquoteLine<'a> {
    depth: usize,
    text: &'a str,
}
