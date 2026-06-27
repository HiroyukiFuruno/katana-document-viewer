use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::node;
use crate::{
    DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, SourceKind, SourceRevision,
    SourceUri, ViewerInput, ViewerInteractionConfig, ViewerMode, ViewerNodePlan, ViewerNodePlanner,
    ViewerSearchState, ViewerTextSpan, ViewerTypographyConfig, ViewerViewport,
};
use katana_markdown_model::{
    FootnoteDefinitionNode, FootnoteReferenceNode, InlineCodeNode, InlineHtmlNode, InlineMathNode,
    InlineSpan, KatanaMarkdownModel, KmmNode, KmmNodeKind, LinkNode, MarkdownInput, TextSpan,
};

pub(super) enum SpanKind {
    Strong,
    Emphasis,
    Strikethrough,
}

pub(super) fn assert_inline_text(kind: KmmNodeKind, expected: &str) {
    let current = node(kind, expected, Vec::new());
    assert_eq!(
        expected,
        ViewerNodeClassifier::node_text(&current, &ViewerNodeKind::Paragraph)
    );
}

pub(super) fn styled_span_paragraph() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "inline",
        vec![
            node(span_kind("bold", SpanKind::Strong), "bold", Vec::new()),
            node(
                span_kind("italic", SpanKind::Emphasis),
                "italic",
                Vec::new(),
            ),
            node(link_kind("link"), "link", Vec::new()),
            node(
                inline_html_kind("<mark>hot</mark>"),
                "<mark>hot</mark>",
                Vec::new(),
            ),
            node(
                inline_html_kind("<a href=\"https://html.example\">html link</a>"),
                "<a href=\"https://html.example\">html link</a>",
                Vec::new(),
            ),
        ],
    )
}

pub(super) fn text_kind(value: &str) -> KmmNodeKind {
    KmmNodeKind::Text(TextSpan {
        text: value.to_string(),
    })
}

pub(super) fn span_kind(value: &str, kind: SpanKind) -> KmmNodeKind {
    let span = InlineSpan {
        text: value.to_string(),
    };
    match kind {
        SpanKind::Strong => KmmNodeKind::Strong(span),
        SpanKind::Emphasis => KmmNodeKind::Emphasis(span),
        SpanKind::Strikethrough => KmmNodeKind::Strikethrough(span),
    }
}

pub(super) fn inline_code_kind(value: &str) -> KmmNodeKind {
    KmmNodeKind::InlineCode(InlineCodeNode {
        code: value.to_string(),
    })
}

pub(super) fn inline_html_kind(value: &str) -> KmmNodeKind {
    KmmNodeKind::InlineHtml(InlineHtmlNode {
        html: value.to_string(),
    })
}

pub(super) fn link_kind(label: &str) -> KmmNodeKind {
    KmmNodeKind::Link(LinkNode {
        label: label.to_string(),
        destination: "https://example.com".to_string(),
        title: None,
        autolink: false,
    })
}

pub(super) fn footnote_reference_kind(label: &str) -> KmmNodeKind {
    KmmNodeKind::FootnoteReference(FootnoteReferenceNode {
        label: label.to_string(),
    })
}

pub(super) fn footnote_definition_kind(label: &str, text: &str) -> KmmNodeKind {
    KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
        label: label.to_string(),
        text: text.to_string(),
    })
}

pub(super) fn inline_math_kind(expression: &str) -> KmmNodeKind {
    KmmNodeKind::InlineMath(InlineMathNode {
        expression: expression.to_string(),
    })
}

pub(super) fn emoji_kind(value: &str, shortcode: &str) -> KmmNodeKind {
    KmmNodeKind::Emoji(katana_markdown_model::EmojiNode {
        value: value.to_string(),
        shortcode: Some(shortcode.to_string()),
    })
}

pub(super) fn plan_for(source: &str) -> Result<ViewerNodePlan, Box<dyn std::error::Error>> {
    let parsed = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "inline-spacing.md",
        source.to_string(),
    ))?;
    let input = ViewerInput {
        snapshot: DocumentSnapshotFactory::from_kmm(document_source(source, &parsed), parsed),
        artifacts: Vec::new(),
        theme: KdvThemeSnapshot::default(),
        mode: ViewerMode::Document,
        interaction: ViewerInteractionConfig::default(),
        typography: ViewerTypographyConfig::default(),
        viewport: ViewerViewport {
            width: 1280.0,
            height: 720.0,
        },
        search: ViewerSearchState::default(),
    };
    Ok(ViewerNodePlanner::create(&input, 0.0))
}

pub(super) fn span_text(spans: &[ViewerTextSpan]) -> String {
    spans.iter().map(|span| span.text.as_str()).collect()
}

fn document_source(source: &str, document: &katana_markdown_model::KmmDocument) -> DocumentSource {
    DocumentSource {
        uri: SourceUri("preview://inline-spacing.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision(document.fingerprint.value.clone()),
        content: source.to_string(),
    }
}
