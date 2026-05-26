use katana_markdown_model::{
    ByteRange, ImageNode, InlineMathNode, InlineSpan, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan,
};

use super::SurfaceTextParser;

fn node(kind: KmmNodeKind) -> KmmNode {
    KmmNode {
        id: KmmNodeId("text-node".to_string()),
        kind,
        source: SourceSpan {
            byte_range: ByteRange { start: 0, end: 1 },
            line_column_range: LineColumnRange {
                start: LineColumn { line: 1, column: 1 },
                end: LineColumn { line: 1, column: 2 },
            },
            raw: RawSnippet {
                text: "x".to_string(),
            },
        },
        children: Vec::new(),
    }
}

#[test]
fn inline_text_handles_emphasis_image_and_inline_math_variants() {
    let emphasis = node(KmmNodeKind::Emphasis(InlineSpan {
        text: "italic".to_string(),
    }));
    let image = node(KmmNodeKind::Image(ImageNode {
        alt: "alt text".to_string(),
        src: "image.png".to_string(),
        title: None,
    }));
    let math = node(KmmNodeKind::InlineMath(InlineMathNode {
        expression: "E = mc^2".to_string(),
    }));

    assert_eq!(SurfaceTextParser::inline_text(&emphasis), "italic");
    assert_eq!(SurfaceTextParser::inline_text(&image), "alt text");
    assert_eq!(SurfaceTextParser::inline_text(&math), "E = mc²");
}
