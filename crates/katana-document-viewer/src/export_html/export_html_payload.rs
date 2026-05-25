use crate::export_assets::ExportAssetResolver;
use crate::export_code_payload::CodeHtmlWriter;
use crate::export_details_payload::DetailsHtmlWriter;
use crate::export_footnote_payload::FootnoteHtmlWriter;
use crate::export_heading_payload::HeadingHtmlWriter;
use crate::export_html_ops::ExportHtmlOps;
use crate::export_html_style::HtmlExportStyle;
use crate::export_inline_payload::InlineHtmlWriter;
use crate::export_math_payload::MathHtmlWriter;
use crate::forge::{BuildGraph, RenderedDiagram};
use crate::html_sanitizer::HtmlFragmentNormalizer;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};

#[path = "export_html_payload_nodes.rs"]
mod export_html_payload_nodes;
use export_html_payload_nodes::RemainingHtmlNodeWriter;

pub(crate) struct HtmlExportPayloadFactory;

impl HtmlExportPayloadFactory {
    pub(crate) fn create(graph: &BuildGraph, theme: &KdvThemeSnapshot) -> Vec<u8> {
        let mut html = format!(
            "<!doctype html>\n<html lang=\"ja\" data-kdv-theme=\"{}\">\n",
            ExportHtmlOps::escape_html(&theme.name)
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
            HtmlExportPayloadFactory::append_node(&mut html, graph, theme, node);
        }
        FootnoteHtmlWriter::append_definitions(&mut html, &graph.snapshot.document.nodes, theme);
        html.push_str("</main></body>\n</html>\n");
        ExportAssetResolver::rewrite_html_image_sources(&html, &graph.snapshot.source_uri)
            .into_bytes()
    }

    pub(crate) fn append_node(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
    ) {
        match &node.kind {
            KmmNodeKind::Heading(heading) => {
                HeadingHtmlWriter::append(html, node, heading.level, &heading.text, theme)
            }
            KmmNodeKind::Paragraph => append_paragraph(html, node, theme),
            KmmNodeKind::FootnoteDefinition(definition) => {
                append_footnote_definition(html, node, &definition.label, &definition.text, theme)
            }
            KmmNodeKind::DollarMathBlock(math) => {
                InlineHtmlWriter::append_dollar_math_block(html, &math.expression, theme)
            }
            KmmNodeKind::HtmlBlock(_) => {
                append_html_block(html, graph, theme, &node.source.raw.text)
            }
            KmmNodeKind::CodeBlock(role) => append_code(html, graph, theme, node, role),
            _ => {
                RemainingHtmlNodeWriter::append(html, graph, theme, node);
            }
        }
    }
}

fn append_paragraph(html: &mut String, node: &KmmNode, theme: &KdvThemeSnapshot) {
    html.push_str("<p>");
    if node.children.is_empty() {
        InlineHtmlWriter::append_text(html, &node.source.raw.text, theme);
    } else {
        InlineHtmlWriter::append_children(html, node, theme);
    }
    html.push_str("</p>\n");
}

fn append_footnote_definition(
    html: &mut String,
    node: &KmmNode,
    label: &str,
    text: &str,
    theme: &KdvThemeSnapshot,
) {
    InlineHtmlWriter::append_footnote_definition(html, node, label, text, theme);
}

fn append_math_code(html: &mut String, text: &str, theme: &KdvThemeSnapshot) {
    MathHtmlWriter::append_block(html, "block", &ExportHtmlOps::fenced_body(text), theme);
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
        CodeBlockRole::Math => append_math_code(html, &node.source.raw.text, theme),
        CodeBlockRole::Diagram { kind } => append_diagram_code(html, graph, theme, node, kind),
    }
}

fn append_diagram_code(
    html: &mut String,
    graph: &BuildGraph,
    theme: &KdvThemeSnapshot,
    node: &KmmNode,
    kind: &DiagramKind,
) {
    let kind_label = ExportHtmlOps::diagram_kind_label(kind);
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
        ExportHtmlOps::escape_html(&ExportHtmlOps::fenced_body(&node.source.raw.text))
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
        DiagramKind::Mermaid | DiagramKind::DrawIo | DiagramKind::PlantUml => "requires-krr-render",
    }
}

#[cfg(test)]
#[path = "export_html_payload_test_modules.rs"]
mod test_modules;
