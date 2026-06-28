use super::{KucNodeFactory, QuoteLineParser};
use katana_document_viewer::{ViewerNode, ViewerNodeKind, ViewerRect};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind};

const MAX_MEDIA_WIDTH: u32 = 120;
const OUTER_QUOTE_MARGIN_LEFT_PX: u16 = 32;
const INNER_QUOTE_MARGIN_LEFT_PX: u16 = 64;
const LIST_QUOTE_PADDING_LEFT_PX: u16 = 28;
const QUOTED_CODE_PADDING_LEFT_PX: u16 = 24;
const QUOTED_CODE_PADDING_TOP_PX: u16 = 8;
const OUTER_QUOTE_HEIGHT_PX: u16 = 23;
const QUOTED_CODE_HEIGHT_PX: u16 = 43;
const VIEWER_NODE_WIDTH: f32 = 120.0;
const VIEWER_NODE_HEIGHT: f32 = 180.0;
const RGBA_ALPHA_INDEX: usize = 3;
const BLOCKQUOTE_CHILD_COUNT: usize = 4;
const OUTER_QUOTE_CHILD_INDEX: usize = 0;
const INNER_QUOTE_CHILD_INDEX: usize = 1;
const LIST_QUOTE_CHILD_INDEX: usize = 2;
const QUOTED_CODE_CHILD_INDEX: usize = 3;

#[test]
fn blockquote_parser_keeps_nested_depth_and_decorated_lines() {
    let lines = QuoteLineParser::parse(
        "> Outer quote\n> > Inner quote\n> **Bold quote**\n>\n> - List item 1\n> - List item 2\n>\n> ```rust\n> let quoted_code = true;\n> ```",
    );

    assert_eq!(1, lines[0].depth);
    assert_eq!("Outer quote", lines[0].text);
    assert_eq!(2, lines[1].depth);
    assert_eq!("Inner quote", lines[1].text);
    assert_eq!("Bold quote", lines[2].text);
    assert_eq!("List item 1", lines[3].text);
    assert!(lines[3].bullet);
    assert_eq!("List item 2", lines[4].text);
    assert!(lines[4].bullet);
    assert_eq!("let quoted_code = true;", lines[5].text);
    assert!(lines[5].code);
    assert_eq!(Some("rust"), lines[5].language.as_deref());
}

#[test]
fn blockquote_parser_collapses_legacy_note_to_single_quote_line() {
    let lines = QuoteLineParser::parse(
        "> **Note**\n> GitHub では note 系ブロックを blockquote として表現する。",
    );

    assert_eq!(1, lines.len());
    assert_eq!(
        "Note GitHub では note 系ブロックを blockquote として表現する。",
        lines[0].text
    );
}

#[test]
fn blockquote_node_uses_kuc_lines_with_common_depth_props() {
    let factory = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH);
    let ui_node = factory.viewer_node(&viewer_node(
        "> Outer quote\n> > Inner quote\n> - List item\n> ```rust\n> let quoted_code = true;\n> ```",
    ));

    assert_eq!(UiNodeKind::Column, ui_node.kind());
    assert_eq!(BLOCKQUOTE_CHILD_COUNT, ui_node.children().len());
    assert_blockquote_labels(ui_node.children());
    assert_quoted_code_node(&ui_node.children()[QUOTED_CODE_CHILD_INDEX]);
    assert_blockquote_styles(ui_node.children());
    assert_blockquote_heights(ui_node.children());
}

fn assert_blockquote_labels(children: &[UiNode]) {
    assert_eq!(
        "Outer quote",
        children[OUTER_QUOTE_CHILD_INDEX].props().label
    );
    assert_eq!(
        "Inner quote",
        children[INNER_QUOTE_CHILD_INDEX].props().label
    );
    assert_eq!("List item", children[LIST_QUOTE_CHILD_INDEX].props().label);
    assert_eq!(
        "let quoted_code = true;",
        children[QUOTED_CODE_CHILD_INDEX].props().label
    );
}

fn assert_quoted_code_node(node: &UiNode) {
    assert_eq!("code", node.props().text.role);
    assert!(
        node.props()
            .text
            .spans
            .iter()
            .any(|span| span.style.color_rgba[RGBA_ALPHA_INDEX] > 0),
        "quoted code must carry syntax-colored spans"
    );
}

fn assert_blockquote_styles(children: &[UiNode]) {
    assert_margin_left_px(
        &children[OUTER_QUOTE_CHILD_INDEX],
        OUTER_QUOTE_MARGIN_LEFT_PX,
    );
    assert_margin_left_px(
        &children[INNER_QUOTE_CHILD_INDEX],
        INNER_QUOTE_MARGIN_LEFT_PX,
    );
    assert_padding_left_px(
        &children[LIST_QUOTE_CHILD_INDEX],
        LIST_QUOTE_PADDING_LEFT_PX,
    );
    assert_quote_background_theme_slot(children);
    assert_no_quote_style_classes(children);
    assert!(!has_style_class(
        &children[QUOTED_CODE_CHILD_INDEX],
        "kdv-document-code"
    ));
    assert!(
        children[QUOTED_CODE_CHILD_INDEX]
            .props()
            .common
            .border
            .visible
    );
    assert_eq!(
        UiDimension::Px(QUOTED_CODE_PADDING_LEFT_PX),
        children[QUOTED_CODE_CHILD_INDEX].props().common.padding.left
    );
    assert_eq!(
        UiDimension::Px(QUOTED_CODE_PADDING_TOP_PX),
        children[QUOTED_CODE_CHILD_INDEX].props().common.padding.top
    );
    assert_margin_left_px(
        &children[QUOTED_CODE_CHILD_INDEX],
        OUTER_QUOTE_MARGIN_LEFT_PX,
    );
}

fn assert_quote_background_theme_slot(children: &[UiNode]) {
    for node in children {
        assert_eq!("quote-background", node.props().common.theme_slot);
    }
}

fn assert_blockquote_heights(children: &[UiNode]) {
    assert_eq!(
        UiDimension::Px(OUTER_QUOTE_HEIGHT_PX),
        children[OUTER_QUOTE_CHILD_INDEX].props().common.height
    );
    assert_eq!(
        UiDimension::Px(QUOTED_CODE_HEIGHT_PX),
        children[QUOTED_CODE_CHILD_INDEX].props().common.height
    );
}

fn viewer_node(raw: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("quote".to_string()),
        kind: ViewerNodeKind::BlockQuote,
        source: source(raw),
        text: String::new(),
        spans: Vec::new(),
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: VIEWER_NODE_WIDTH,
            height: VIEWER_NODE_HEIGHT,
        },
        artifact_id: None,
    }
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}

fn assert_margin_left_px(node: &UiNode, expected: u16) {
    assert_eq!(UiDimension::Px(expected), node.props().common.margin.left);
}

fn assert_padding_left_px(node: &UiNode, expected: u16) {
    assert_eq!(UiDimension::Px(expected), node.props().common.padding.left);
}

fn assert_no_quote_style_classes(nodes: &[UiNode]) {
    for node in nodes {
        assert!(
            !node
                .props()
                .style_classes
                .iter()
                .any(|class| class.starts_with("kdv-document-quote")),
            "{:#?}",
            node.props().style_classes
        );
    }
}

fn has_style_class(node: &katana_ui_core::render_model::UiNode, expected: &str) -> bool {
    node.props()
        .style_classes
        .iter()
        .any(|class| class == expected)
}
