use crate::export_html_ops::ExportHtmlOps;
use crate::export_html_payload::HtmlExportPayloadFactory;
use crate::export_semantics::EvaluatedMarkdownFragment;
use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;

pub(crate) struct DetailsHtmlWriter;

impl DetailsHtmlWriter {
    pub(crate) fn try_append(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        fragment: &str,
    ) -> bool {
        let Some(parts) = DetailsParts::parse(fragment) else {
            return false;
        };
        html.push_str("<details data-kdv-accordion=\"true\" open><summary>");
        html.push_str(&ExportHtmlOps::escape_html(parts.summary.trim()));
        html.push_str("</summary><div data-kdv-accordion-body>\n");
        Self::append_markdown_body(html, graph, theme, parts.body);
        html.push_str("</div></details>");
        true
    }

    fn append_markdown_body(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        body: &str,
    ) {
        let fragment = EvaluatedMarkdownFragment::evaluate("details-body.md", body.trim());
        if !fragment.has_nodes() {
            html.push_str(&ExportHtmlOps::escape_html(body));
            return;
        }
        for node in fragment.nodes() {
            HtmlExportPayloadFactory::append_node(html, graph, theme, node);
        }
    }
}

struct DetailsParts<'a> {
    summary: &'a str,
    body: &'a str,
}

impl<'a> DetailsParts<'a> {
    fn parse(fragment: &'a str) -> Option<Self> {
        let trimmed = fragment.trim();
        if !trimmed.starts_with("<details") {
            return None;
        }
        let summary_start = trimmed.find("<summary>")? + "<summary>".len();
        let summary_end = trimmed.find("</summary>")?;
        let body_start = summary_end + "</summary>".len();
        let body_end = trimmed.rfind("</details>")?;
        let body = Self::strip_div(&trimmed[body_start..body_end]);
        Some(Self {
            summary: &trimmed[summary_start..summary_end],
            body,
        })
    }

    fn strip_div(value: &'a str) -> &'a str {
        let trimmed = value.trim();
        if let Some(body) = trimmed.strip_prefix("<div>") {
            return body.strip_suffix("</div>").unwrap_or(body);
        }
        trimmed
    }
}
