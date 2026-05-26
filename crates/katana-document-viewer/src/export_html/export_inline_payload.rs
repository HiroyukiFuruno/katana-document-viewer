use crate::export_html_ops::ExportHtmlOps;
use crate::export_math_payload::MathHtmlWriter;
use crate::export_semantics::EvaluatedMarkdownFragment;
use crate::html_sanitizer::HtmlFragmentNormalizer;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{KmmNode, KmmNodeKind};

pub(crate) struct InlineHtmlWriter;

impl InlineHtmlWriter {
    pub(crate) fn append_children(html: &mut String, node: &KmmNode, theme: &KdvThemeSnapshot) {
        for child in &node.children {
            Self::append_node(html, child, theme);
        }
    }

    pub(crate) fn append_fragment(html: &mut String, markdown: &str, theme: &KdvThemeSnapshot) {
        let fragment = EvaluatedMarkdownFragment::evaluate("table-cell.md", markdown);
        if !fragment.has_nodes() {
            html.push_str(&ExportHtmlOps::escape_html(markdown));
            return;
        }
        for node in fragment.nodes() {
            Self::append_fragment_node(html, node, theme);
        }
    }

    pub(crate) fn append_node(html: &mut String, node: &KmmNode, theme: &KdvThemeSnapshot) {
        match &node.kind {
            KmmNodeKind::Text(text) => Self::append_text(html, &text.text, theme),
            KmmNodeKind::Strong(span) => Self::append_span(html, node, "strong", &span.text, theme),
            KmmNodeKind::Emphasis(span) => Self::append_span(html, node, "em", &span.text, theme),
            KmmNodeKind::Strikethrough(span) => {
                Self::append_span(html, node, "s", &span.text, theme)
            }
            KmmNodeKind::InlineCode(code) => Self::append_tag(html, "code", &code.code),
            KmmNodeKind::InlineHtml(inline) => {
                html.push_str(&HtmlFragmentNormalizer::normalize(&inline.html))
            }
            KmmNodeKind::Link(link) => Self::append_link(html, link),
            KmmNodeKind::Image(image) => Self::append_image(html, image),
            KmmNodeKind::FootnoteReference(reference) => {
                Self::append_footnote_reference(html, &reference.label)
            }
            KmmNodeKind::InlineMath(math) => {
                Self::append_inline_math(html, &math.expression, theme)
            }
            KmmNodeKind::Emoji(emoji) => html.push_str(&ExportHtmlOps::escape_html(&emoji.value)),
            _ => html.push_str(&ExportHtmlOps::escape_html(&node.source.raw.text)),
        }
    }

    pub(crate) fn append_text(html: &mut String, text: &str, theme: &KdvThemeSnapshot) {
        let fragment = EvaluatedMarkdownFragment::evaluate("inline-text.md", text);
        if !fragment.contains_inline_markdown() {
            html.push_str(&ExportHtmlOps::render_text(text));
            return;
        }
        if !Self::try_append_inline_text(html, &fragment, theme) {
            html.push_str(&ExportHtmlOps::render_text(text));
        }
    }

    pub(crate) fn append_footnote_definition(
        html: &mut String,
        node: &KmmNode,
        label: &str,
        text: &str,
        theme: &KdvThemeSnapshot,
    ) {
        html.push_str(&format!(
            "<section id=\"fn-{}\" data-kdv-footnote-definition=\"{}\">",
            ExportHtmlOps::escape_html(label),
            ExportHtmlOps::escape_html(label)
        ));
        if node.children.is_empty() {
            html.push_str(&ExportHtmlOps::escape_html(text));
        } else {
            Self::append_children(html, node, theme);
        }
        html.push_str(&format!(
            " <a href=\"#fnref-{0}\" data-kdv-footnote-backref=\"{0}\">↩</a></section>\n",
            ExportHtmlOps::escape_html(label)
        ));
    }

    pub(crate) fn append_dollar_math_block(
        html: &mut String,
        expression: &str,
        theme: &KdvThemeSnapshot,
    ) {
        MathHtmlWriter::append_block(html, "dollar-block", expression, theme);
    }

    fn append_tag(html: &mut String, tag: &str, text: &str) {
        html.push_str(&format!(
            "<{tag}>{}</{tag}>",
            ExportHtmlOps::escape_html(text)
        ));
    }

    fn append_fragment_node(html: &mut String, node: &KmmNode, theme: &KdvThemeSnapshot) {
        match &node.kind {
            KmmNodeKind::Paragraph => {
                if node.children.is_empty() {
                    html.push_str(&ExportHtmlOps::render_text(&node.source.raw.text));
                } else {
                    Self::append_children(html, node, theme);
                }
            }
            _ => Self::append_node(html, node, theme),
        }
    }

    fn try_append_inline_text(
        html: &mut String,
        fragment: &EvaluatedMarkdownFragment,
        theme: &KdvThemeSnapshot,
    ) -> bool {
        if !fragment.has_nodes() || !fragment.contains_structured_inline() {
            return false;
        }
        for node in fragment.nodes() {
            Self::append_fragment_node(html, node, theme);
        }
        true
    }

    fn append_span(
        html: &mut String,
        node: &KmmNode,
        tag: &str,
        text: &str,
        theme: &KdvThemeSnapshot,
    ) {
        html.push_str(&format!("<{tag}>"));
        if node.children.is_empty() {
            html.push_str(&ExportHtmlOps::escape_html(text));
        } else {
            Self::append_children(html, node, theme);
        }
        html.push_str(&format!("</{tag}>"));
    }

    fn append_link(html: &mut String, link: &katana_markdown_model::LinkNode) {
        let title = link.title.as_ref().map_or_else(String::new, |value| {
            format!(" title=\"{}\"", ExportHtmlOps::escape_html(value))
        });
        let autolink = if link.autolink {
            " data-kdv-autolink=\"true\""
        } else {
            ""
        };
        html.push_str(&format!(
            "<a href=\"{}\"{title}{autolink}>{}</a>",
            ExportHtmlOps::escape_html(&link.destination),
            ExportHtmlOps::render_text(&link.label)
        ));
    }

    fn append_image(html: &mut String, image: &katana_markdown_model::ImageNode) {
        let title = image.title.as_ref().map_or_else(String::new, |value| {
            format!(" title=\"{}\"", ExportHtmlOps::escape_html(value))
        });
        html.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\"{title}>",
            ExportHtmlOps::escape_html(&image.src),
            ExportHtmlOps::render_text(&image.alt)
        ));
    }

    fn append_footnote_reference(html: &mut String, label: &str) {
        let escaped_label = ExportHtmlOps::escape_html(label);
        html.push_str(&format!(
            "<sup id=\"fnref-{escaped_label}\" data-kdv-footnote-ref=\"{escaped_label}\"><a href=\"#fn-{escaped_label}\">[{escaped_label}]</a></sup>"
        ));
    }

    fn append_inline_math(html: &mut String, expression: &str, theme: &KdvThemeSnapshot) {
        MathHtmlWriter::append_inline(html, expression, theme);
    }
}

#[cfg(test)]
#[path = "export_inline_payload_tests.rs"]
mod tests;
