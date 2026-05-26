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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::KdvThemeSnapshot;
    use crate::{
        BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, SourceKind,
        SourceRevision,
    };
    use katana_markdown_model::{KatanaMarkdownModel, KmmDocument, MarkdownInput, TextFingerprint};

    #[test]
    fn try_append_rejects_missing_details_wrapper() {
        let mut html = String::new();
        let graph = graph();
        let theme = KdvThemeSnapshot::katana_light();
        let ok = DetailsHtmlWriter::try_append(&mut html, &graph, &theme, "not a details block");
        assert!(!ok);
        assert_eq!(html, "");
    }

    #[test]
    fn try_append_parses_summary_and_body() {
        let mut html = String::new();
        let graph = graph();
        let theme = KdvThemeSnapshot::katana_light();
        let details = "<details><summary>title</summary><div>body</div></details>";
        let ok = DetailsHtmlWriter::try_append(&mut html, &graph, &theme, details);
        assert!(ok);
        assert!(html.contains("<details data-kdv-accordion=\"true\" open>"));
        assert!(html.contains("<summary>title</summary>"));
    }

    #[test]
    fn parse_supports_divipped_body() {
        let mut html = String::new();
        let graph = graph();
        let theme = KdvThemeSnapshot::katana_light();
        let details = "<details><summary>title</summary><div><p>body</p></div></details>";
        let ok = DetailsHtmlWriter::try_append(&mut html, &graph, &theme, details);
        assert!(ok);
        assert!(html.contains("body"));
    }

    #[test]
    fn append_markdown_body_uses_escaped_raw_text_without_nodes() {
        let mut html = String::new();
        let graph = graph();
        let theme = KdvThemeSnapshot::katana_light();
        let fragment = "";
        DetailsHtmlWriter::append_markdown_body(&mut html, &graph, &theme, fragment);

        assert_eq!(html, "");
    }

    #[test]
    fn parse_handles_fragment_without_div() {
        let parsed = DetailsParts::parse("<details><summary>title</summary>body</details>");
        assert!(parsed.is_some());
        assert_eq!(parsed.map(|parts| parts.body).unwrap_or(""), "body");
    }

    fn graph() -> BuildGraph {
        let source = DocumentSource {
            uri: crate::SourceUri("file:///test.md".to_string()),
            kind: SourceKind::Markdown,
            revision: SourceRevision("r".to_string()),
            content: "x".to_string(),
        };
        let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "test.md",
            source.content.clone(),
        ));
        assert!(document.is_ok());
        let document = document.unwrap_or(KmmDocument {
            path: std::path::PathBuf::from("test.md"),
            fingerprint: TextFingerprint {
                algorithm: "manual".to_string(),
                value: "fallback".to_string(),
            },
            nodes: Vec::new(),
        });
        let snapshot = DocumentSnapshotFactory::from_kmm(source.clone(), document);
        BuildGraph::from_request(&BuildRequest {
            snapshot,
            profile: BuildProfile::markdown_export(),
            theme: KdvThemeSnapshot::katana_light(),
        })
    }
}
