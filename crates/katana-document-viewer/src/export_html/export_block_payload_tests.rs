use super::*;
use crate::SourceRevision;
use crate::theme::KdvThemeSnapshot;
use crate::{BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, SourceKind};
use katana_markdown_model::InlineSpan;
use katana_markdown_model::{
    ByteRange, KatanaMarkdownModel, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    MarkdownInput, RawSnippet, SourceSpan, TextSpan,
};

#[test]
fn append_blockquote_uses_nested_output() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = node(KmmNodeKind::BlockQuote, Vec::new(), "> one\n> > two");
    BlockHtmlWriter::append_blockquote(&mut html, &graph, &theme, &node);

    assert!(html.contains("data-kdv-quote-depth=\"2\""));
}

#[test]
fn append_blockquote_uses_raw_children_when_present() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let child = KmmNode {
        id: KmmNodeId("quote-child".to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: "child".to_string(),
        }),
        source: source_span("child"),
        children: Vec::new(),
    };
    let node = KmmNode {
        id: KmmNodeId("quote-parent".to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("> child"),
        children: vec![child],
    };
    BlockHtmlWriter::append_blockquote(&mut html, &graph, &theme, &node);

    assert!(html.contains("<blockquote data-kdv-blockquote=\"quote\">"));
    assert!(html.contains("child"));
}

#[test]
fn append_alert_uses_legacy_note_when_not_gfm() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = node(
        KmmNodeKind::Alert {
            label: "NOTE".to_string(),
        },
        Vec::new(),
        "> note\nbody",
    );
    BlockHtmlWriter::append_alert(&mut html, &graph, &theme, &node, "NOTE");
    assert!(html.contains("<blockquote data-kdv-blockquote=\"quote\">"));
}

#[test]
fn append_alert_renders_gfm_alert_markup() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = node(
        KmmNodeKind::Alert {
            label: "TIP".to_string(),
        },
        Vec::new(),
        "> [!TIP]\n> tip body",
    );
    BlockHtmlWriter::append_alert(&mut html, &graph, &theme, &node, "TIP");
    assert!(html.contains("<aside data-github-alert=\"TIP\" data-kdv-blockquote=\"alert\">"));
}

#[test]
fn append_blockquote_renders_raw_text_when_children_missing() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = node(KmmNodeKind::BlockQuote, Vec::new(), "> missing child");
    BlockHtmlWriter::append_blockquote(&mut html, &graph, &theme, &node);

    assert!(html.contains("&gt; missing child"));
}

#[test]
fn append_alert_renders_default_icon_for_unknown_label() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = node(
        KmmNodeKind::Alert {
            label: "CUSTOM".to_string(),
        },
        Vec::new(),
        "> [!CUSTOM]\n> body",
    );
    BlockHtmlWriter::append_alert(&mut html, &graph, &theme, &node, "CUSTOM");

    assert!(html.contains("data-kdv-alert-icon-svg=\"NOTE\""));
}

#[test]
fn alert_title_and_icon_have_all_labels() {
    let _ = BlockHtmlWriter::alert_title("NOTE");
    let _ = BlockHtmlWriter::alert_title("TIP");
    let _ = BlockHtmlWriter::alert_title("IMPORTANT");
    let _ = BlockHtmlWriter::alert_title("WARNING");
    let _ = BlockHtmlWriter::alert_title("CAUTION");
    let _ = BlockHtmlWriter::alert_title("OTHER");
    assert_eq!(BlockHtmlWriter::alert_title("OTHER"), "Note");
    assert!(BlockHtmlWriter::is_gfm_alert("> [!IMPORTANT]"));
    assert!(!BlockHtmlWriter::is_gfm_alert("> note"));
    assert!(BlockHtmlWriter::is_gfm_alert("> [!TIP]"));
    let title = KmmNode {
        id: KmmNodeId("title".to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: "title".to_string(),
        }),
        source: source_span("title"),
        children: vec![node(
            KmmNodeKind::Strong(InlineSpan {
                text: "Note".to_string(),
            }),
            Vec::new(),
            "Note",
        )],
    };
    assert!(BlockHtmlWriter::is_alert_title(&title, "NOTE"));
}

fn graph() -> BuildGraph {
    let source = DocumentSource {
        uri: crate::SourceUri("file:///test.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("r".to_string()),
        content: "x".to_string(),
    };
    let snapshot = DocumentSnapshotFactory::from_kmm(
        source.clone(),
        match KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "test.md",
            source.content.clone(),
        )) {
            Ok(model) => model,
            Err(error) => {
                std::panic::resume_unwind(Box::new(format!("parse test markdown: {error}")))
            }
        },
    );
    BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    })
}

fn node(kind: KmmNodeKind, children: Vec<KmmNode>, raw: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId("node".to_string()),
        kind,
        source: source_span(raw),
        children,
    }
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
