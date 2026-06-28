use crate::export_surface_font::SurfaceTextPainter;
use crate::export_surface_line::BODY_FONT_SIZE;
use crate::export_surface_span::SurfaceTextSpan;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, RawSnippet,
    SourceSpan, TextSpan,
};

use super::*;

const WIDE_WRAP_WIDTH: u32 = 300;

#[test]
fn append_wrapped_chunks_for_list_and_root_depths() {
    let mut blocks = Vec::new();
    SurfaceBlockFactory::append_wrapped(&mut blocks, "root text".to_string(), 0, 0);
    SurfaceBlockFactory::append_wrapped(&mut blocks, "list text".to_string(), 0, 2);

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "root text");
    assert_eq!(blocks[1].text_for_tests(), "list text");
}

#[test]
fn append_rich_line_keeps_empty_span_result_empty() {
    let mut blocks = Vec::new();
    let node = KmmNode {
        id: KmmNodeId("empty".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span(""),
        children: Vec::new(),
    };

    SurfaceBlockFactory::append_rich_line(
        &mut blocks,
        &node,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );
    assert!(blocks.is_empty());
}

#[test]
fn append_rich_line_renders_text_line() {
    let mut blocks = Vec::new();
    let node = KmmNode {
        id: KmmNodeId("rich".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("rich text"),
        children: vec![KmmNode {
            id: KmmNodeId("child".to_string()),
            kind: KmmNodeKind::Text(TextSpan {
                text: "rich text".to_string(),
            }),
            source: source_span("rich text"),
            children: Vec::new(),
        }],
    };

    SurfaceBlockFactory::append_rich_line(
        &mut blocks,
        &node,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "rich text");
}

#[test]
fn line_wrapper_handles_whitespace_and_wraps() {
    let segments = vec![
        SurfaceTextSpan::plain("   "),
        SurfaceTextSpan::plain("first"),
        SurfaceTextSpan::plain(" "),
        SurfaceTextSpan::plain("second"),
    ];
    let wrapped = SurfaceInlineLineWrapper::wrap(segments, WIDE_WRAP_WIDTH);
    assert_eq!(wrapped.len(), 1);
    assert_eq!(wrapped[0][0].text, "first");

    let long = SurfaceTextSpan::plain("tok ".repeat(200));
    let wrapped_many = SurfaceInlineLineWrapper::wrap(vec![long], 20);
    assert!(wrapped_many.len() > 1);
}

#[test]
fn line_wrapper_wraps_japanese_without_spaces_before_next_block() {
    let text = "これはPDF出力の日本語折り返しを確認するための合成テキストです。".repeat(12);
    let wrapped = SurfaceInlineLineWrapper::wrap(vec![SurfaceTextSpan::plain(text)], 240);

    assert!(
        wrapped.len() > 1,
        "Japanese text without spaces must reserve multiple surface lines"
    );
    assert!(
        wrapped.iter().all(|line| !line.is_empty()),
        "each wrapped surface line should keep visible text"
    );
}

#[test]
fn line_wrapper_keeps_japanese_lines_within_measured_width() {
    let text = "これはPDF出力の日本語折り返しを確認するための合成テキストです。".repeat(4);
    let wrapped =
        SurfaceInlineLineWrapper::wrap(vec![SurfaceTextSpan::plain(text)], WIDE_WRAP_WIDTH);

    let measured_widths = SurfaceTextPainter::with_system_fonts(|painter| {
        wrapped
            .iter()
            .map(|line| painter.measure_spans_width(line, BODY_FONT_SIZE, 2000.0))
            .collect::<Vec<_>>()
    });

    assert!(
        measured_widths
            .iter()
            .all(|width| *width <= WIDE_WRAP_WIDTH),
        "wrapped Japanese lines must fit measured width: {measured_widths:?}"
    );
}

#[test]
fn line_wrapper_can_wrap_empty_input() {
    let wrapped = SurfaceInlineLineWrapper::wrap(Vec::new(), 80);
    assert_eq!(wrapped.len(), 1);
    assert_eq!(wrapped[0][0].text, "");
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
