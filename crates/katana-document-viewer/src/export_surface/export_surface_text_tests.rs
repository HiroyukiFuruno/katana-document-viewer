use katana_markdown_model::{
    ByteRange, EmojiNode, FootnoteReferenceNode, HeadingNode, InlineCodeNode, InlineHtmlNode,
    InlineSpan, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, LinkNode, RawSnippet,
    SourceSpan, TextSpan,
};

use super::SurfaceTextParser;

const EMPTY_ID: &str = "text-node";

fn span(text: &str) -> SourceSpan {
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

fn node(kind: KmmNodeKind, children: Vec<KmmNode>, source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind,
        source: span(source_text),
        children,
    }
}

#[test]
fn inline_text_decodes_text_node() {
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::Text(TextSpan {
                text: "A &amp; B".to_string()
            }),
            Vec::new(),
            "A &amp; B",
        )),
        "A & B"
    );
}

#[test]
fn inline_text_decodes_span_nodes() {
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::Strong(InlineSpan {
                text: "strong".to_string(),
            }),
            Vec::new(),
            "",
        )),
        "strong"
    );
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::Strikethrough(InlineSpan {
                text: "strike".to_string(),
            }),
            Vec::new(),
            "",
        )),
        "strike"
    );
}

#[test]
fn inline_text_handles_inline_code() {
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::InlineCode(InlineCodeNode {
                code: "code".to_string(),
            }),
            Vec::new(),
            "",
        )),
        "code"
    );
}

#[test]
fn inline_text_handles_inline_html() {
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::InlineHtml(InlineHtmlNode {
                html: "<span title=\"x\">html</span>".to_string(),
            }),
            Vec::new(),
            "",
        )),
        "html"
    );
}

#[test]
fn inline_text_handles_links() {
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::Link(LinkNode {
                label: "link text".to_string(),
                destination: "http://example.com".to_string(),
                title: None,
                autolink: false,
            }),
            Vec::new(),
            "",
        )),
        "link text"
    );
}

#[test]
fn inline_text_handles_emoji_and_footnotes() {
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::Emoji(EmojiNode {
                value: "😀".to_string(),
                shortcode: None,
            }),
            Vec::new(),
            "",
        )),
        "😀"
    );
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::FootnoteReference(FootnoteReferenceNode {
                label: "1".to_string(),
            }),
            Vec::new(),
            "",
        )),
        "[1]"
    );
}

#[test]
fn inline_text_falls_back_to_raw_or_children() {
    assert_eq!(
        SurfaceTextParser::inline_text(&node(
            KmmNodeKind::Heading(HeadingNode {
                level: 1,
                text: "Heading".to_string(),
            }),
            Vec::new(),
            "Raw body",
        )),
        "Raw body"
    );
    let parent = node(
        KmmNodeKind::BlockQuote,
        vec![node(
            KmmNodeKind::Text(TextSpan {
                text: "child".to_string(),
            }),
            Vec::new(),
            "child",
        )],
        "parent",
    );
    assert_eq!(SurfaceTextParser::inline_text(&parent), "child");
}

#[test]
fn inline_markdown_text_removes_links_and_formatting() {
    let text = "**bold** [link](url) `code` [another](path)";
    let parsed = SurfaceTextParser::inline_markdown_text(text);

    assert_eq!(parsed, "bold link code another");
}
