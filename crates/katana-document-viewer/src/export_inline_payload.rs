use crate::export_html_ops::{escape_html, render_text};
use crate::export_math_payload::MathHtmlWriter;
use crate::html_sanitizer::HtmlFragmentNormalizer;
use katana_markdown_model::{KatanaMarkdownModel, KmmNode, KmmNodeKind, MarkdownInput};

pub(crate) struct InlineHtmlWriter;

impl InlineHtmlWriter {
    pub(crate) fn append_children(html: &mut String, node: &KmmNode) {
        for child in &node.children {
            Self::append_node(html, child);
        }
    }

    pub(crate) fn append_fragment(html: &mut String, markdown: &str) {
        let parsed = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "table-cell.md",
            markdown.to_string(),
        ));
        let Ok(document) = parsed else {
            html.push_str(&escape_html(markdown));
            return;
        };
        if document.nodes.is_empty() {
            html.push_str(&escape_html(markdown));
            return;
        }
        for node in &document.nodes {
            Self::append_fragment_node(html, node);
        }
    }

    pub(crate) fn append_node(html: &mut String, node: &KmmNode) {
        match &node.kind {
            KmmNodeKind::Text(text) => html.push_str(&render_text(&text.text)),
            KmmNodeKind::Strong(span) => Self::append_span(html, node, "strong", &span.text),
            KmmNodeKind::Emphasis(span) => Self::append_span(html, node, "em", &span.text),
            KmmNodeKind::Strikethrough(span) => Self::append_span(html, node, "s", &span.text),
            KmmNodeKind::InlineCode(code) => Self::append_tag(html, "code", &code.code),
            KmmNodeKind::InlineHtml(inline) => {
                html.push_str(&HtmlFragmentNormalizer::normalize(&inline.html))
            }
            KmmNodeKind::Link(link) => Self::append_link(html, link),
            KmmNodeKind::Image(image) => Self::append_image(html, image),
            KmmNodeKind::FootnoteReference(reference) => {
                Self::append_footnote_reference(html, &reference.label)
            }
            KmmNodeKind::InlineMath(math) => Self::append_inline_math(html, &math.expression),
            KmmNodeKind::Emoji(emoji) => html.push_str(&escape_html(&emoji.value)),
            _ => html.push_str(&escape_html(&node.source.raw.text)),
        }
    }

    pub(crate) fn append_footnote_definition(
        html: &mut String,
        node: &KmmNode,
        label: &str,
        text: &str,
    ) {
        html.push_str(&format!(
            "<section id=\"fn-{}\" data-kdv-footnote-definition=\"{}\">",
            escape_html(label),
            escape_html(label)
        ));
        if node.children.is_empty() {
            html.push_str(&escape_html(text));
        } else {
            Self::append_children(html, node);
        }
        html.push_str("</section>\n");
    }

    pub(crate) fn append_dollar_math_block(html: &mut String, expression: &str) {
        MathHtmlWriter::append_block(html, "dollar-block", expression);
    }

    fn append_tag(html: &mut String, tag: &str, text: &str) {
        html.push_str(&format!("<{tag}>{}</{tag}>", escape_html(text)));
    }

    fn append_fragment_node(html: &mut String, node: &KmmNode) {
        match &node.kind {
            KmmNodeKind::Paragraph => {
                if node.children.is_empty() {
                    html.push_str(&render_text(&node.source.raw.text));
                } else {
                    Self::append_children(html, node);
                }
            }
            _ => Self::append_node(html, node),
        }
    }

    fn append_span(html: &mut String, node: &KmmNode, tag: &str, text: &str) {
        html.push_str(&format!("<{tag}>"));
        if node.children.is_empty() {
            html.push_str(&escape_html(text));
        } else {
            Self::append_children(html, node);
        }
        html.push_str(&format!("</{tag}>"));
    }

    fn append_link(html: &mut String, link: &katana_markdown_model::LinkNode) {
        let title = link
            .title
            .as_ref()
            .map(|value| format!(" title=\"{}\"", escape_html(value)))
            .unwrap_or_default();
        let autolink = if link.autolink {
            " data-kdv-autolink=\"true\""
        } else {
            ""
        };
        html.push_str(&format!(
            "<a href=\"{}\"{title}{autolink}>{}</a>",
            escape_html(&link.destination),
            render_text(&link.label)
        ));
    }

    fn append_image(html: &mut String, image: &katana_markdown_model::ImageNode) {
        let title = image
            .title
            .as_ref()
            .map(|value| format!(" title=\"{}\"", escape_html(value)))
            .unwrap_or_default();
        html.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\"{title}>",
            escape_html(&image.src),
            render_text(&image.alt)
        ));
    }

    fn append_footnote_reference(html: &mut String, label: &str) {
        let escaped_label = escape_html(label);
        html.push_str(&format!(
            "<sup id=\"fnref-{escaped_label}\" data-kdv-footnote-ref=\"{escaped_label}\"><a href=\"#fn-{escaped_label}\">[{escaped_label}]</a></sup>"
        ));
    }

    fn append_inline_math(html: &mut String, expression: &str) {
        MathHtmlWriter::append_inline(html, expression);
    }
}
