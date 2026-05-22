use crate::export_block_payload::BlockHtmlWriter;
use crate::export_code_payload::CodeHtmlWriter;
use crate::export_details_payload::DetailsHtmlWriter;
use crate::export_footnote_payload::FootnoteHtmlWriter;
use crate::export_heading_payload::HeadingHtmlWriter;
use crate::export_html_ops::{diagram_kind_label, escape_html, fenced_body, render_text};
use crate::export_html_style::HtmlExportStyle;
use crate::export_inline_payload::InlineHtmlWriter;
use crate::export_list_payload::ListHtmlWriter;
use crate::export_math_payload::MathHtmlWriter;
use crate::export_table_payload::TableHtmlWriter;
use crate::forge::{BuildGraph, RenderedDiagram};
use crate::html_sanitizer::HtmlFragmentNormalizer;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};

pub(crate) struct HtmlExportPayloadFactory;

impl HtmlExportPayloadFactory {
    pub(crate) fn create(graph: &BuildGraph, theme: &KdvThemeSnapshot) -> Vec<u8> {
        let mut html = format!(
            "<!doctype html>\n<html lang=\"ja\" data-kdv-theme=\"{}\">\n",
            escape_html(&theme.name)
        );
        html.push_str("<head><meta charset=\"utf-8\"><title>KDV Export</title>");
        html.push_str("<style data-kdv-export-style>");
        HtmlExportStyle::append(&mut html, theme);
        html.push_str("</style></head>\n");
        html.push_str("<body><main data-kdv-export=\"foundation\">\n");
        for node in &graph.snapshot.document.nodes {
            if FootnoteHtmlWriter::is_definition(node) {
                continue;
            }
            append_node(&mut html, graph, theme, node);
        }
        FootnoteHtmlWriter::append_definitions(&mut html, &graph.snapshot.document.nodes);
        html.push_str("</main></body>\n</html>\n");
        html.into_bytes()
    }
}

pub(crate) fn append_node(
    html: &mut String,
    graph: &BuildGraph,
    theme: &KdvThemeSnapshot,
    node: &KmmNode,
) {
    match &node.kind {
        KmmNodeKind::Heading(heading) => {
            HeadingHtmlWriter::append(html, node, heading.level, &heading.text)
        }
        KmmNodeKind::Paragraph => append_paragraph(html, node),
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
        | KmmNodeKind::Emoji(_) => InlineHtmlWriter::append_node(html, node),
        KmmNodeKind::FootnoteDefinition(definition) => {
            InlineHtmlWriter::append_footnote_definition(
                html,
                node,
                &definition.label,
                &definition.text,
            )
        }
        KmmNodeKind::DollarMathBlock(math) => {
            InlineHtmlWriter::append_dollar_math_block(html, &math.expression)
        }
        KmmNodeKind::HtmlBlock(_) => append_html_block(html, graph, theme, &node.source.raw.text),
        KmmNodeKind::List(list) => ListHtmlWriter::append(
            html,
            graph,
            theme,
            list.ordered,
            &list.items,
            &node.source.raw.text,
        ),
        KmmNodeKind::CodeBlock(role) => append_code(html, graph, theme, node, role),
        KmmNodeKind::Table(table) => TableHtmlWriter::append(html, table, &node.source.raw.text),
        KmmNodeKind::BlockQuote => BlockHtmlWriter::append_blockquote(html, graph, theme, node),
        KmmNodeKind::Alert { label } => {
            BlockHtmlWriter::append_alert(html, graph, theme, node, label)
        }
        KmmNodeKind::DescriptionList { items } => {
            BlockHtmlWriter::append_description_list(html, items)
        }
        KmmNodeKind::ThematicBreak => html.push_str("<hr>\n"),
        KmmNodeKind::RawBlock { reason } => {
            BlockHtmlWriter::append_raw_block(html, reason, &node.source.raw.text)
        }
    }
}

fn append_paragraph(html: &mut String, node: &KmmNode) {
    html.push_str("<p>");
    if node.children.is_empty() {
        html.push_str(&render_text(&node.source.raw.text));
    } else {
        InlineHtmlWriter::append_children(html, node);
    }
    html.push_str("</p>\n");
}

fn append_html_block(html: &mut String, graph: &BuildGraph, theme: &KdvThemeSnapshot, text: &str) {
    if DetailsHtmlWriter::try_append(html, graph, theme, text) {
        return;
    }
    html.push_str(&HtmlFragmentNormalizer::normalize(text));
}

fn append_code(
    html: &mut String,
    graph: &BuildGraph,
    theme: &KdvThemeSnapshot,
    node: &KmmNode,
    role: &CodeBlockRole,
) {
    match role {
        CodeBlockRole::Plain { language } => {
            CodeHtmlWriter::append_plain(html, language, &node.source.raw.text)
        }
        CodeBlockRole::Math => append_math_code(html, &node.source.raw.text),
        CodeBlockRole::Diagram { kind } => append_diagram_code(html, graph, theme, node, kind),
    }
}

fn append_math_code(html: &mut String, text: &str) {
    MathHtmlWriter::append_block(html, "block", &fenced_body(text));
}

fn append_diagram_code(
    html: &mut String,
    graph: &BuildGraph,
    theme: &KdvThemeSnapshot,
    node: &KmmNode,
    kind: &DiagramKind,
) {
    let kind_label = diagram_kind_label(kind);
    if let Some(diagram) = rendered_diagram(graph, &node.id.0) {
        html.push_str(&format!(
            "<figure data-kdv-diagram=\"{kind_label}\" data-kdv-diagram-theme=\"{}\">{}</figure>\n",
            theme.diagram_theme_label(),
            diagram.svg
        ));
        return;
    }
    html.push_str(&format!(
        "<figure data-kdv-diagram=\"{kind_label}\" data-kdv-export-readiness=\"{}\"><pre><code>{}</code></pre></figure>\n",
        diagram_readiness_label(kind),
        escape_html(&fenced_body(&node.source.raw.text))
    ));
}

fn rendered_diagram<'a>(graph: &'a BuildGraph, node_id: &str) -> Option<&'a RenderedDiagram> {
    graph
        .rendered_diagrams
        .iter()
        .find(|diagram| diagram.node_id == node_id)
}

fn diagram_readiness_label(kind: &DiagramKind) -> &'static str {
    match kind {
        DiagramKind::Mermaid | DiagramKind::DrawIo | DiagramKind::PlantUml => "requires-kdr-render",
    }
}

#[cfg(test)]
#[path = "export_html_payload_test_modules.rs"]
mod test_modules;
