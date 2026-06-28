use super::{HtmlImageRef, KucNodeFactory, svg_payload};
use katana_document_viewer::{ViewerHtmlRole, ViewerNode, ViewerNodeKind, ViewerRect};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::UiNodeKind;

const VALID_SVG_IMAGE: &str = r#"<p align="center"><img src="data:image/svg+xml,%3Csvg%3E%3C%2Fsvg%3E" width="128" alt="icon"></p>"#;
const BROKEN_KATANA_SVG_IMAGE: &str = r#"<p align="center"><img src="data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22> width=%22128%22 height=%22128%22%3E%3Crect width=%22128%22 height=%22128%22 fill=%22%23ddd%22/%3E%3C/svg%3E" width="128" alt="icon"></p>"#;
const MAX_MEDIA_WIDTH: u32 = 240;
const HTML_IMAGE_NODE_WIDTH: f32 = 240.0;
const HTML_IMAGE_NODE_HEIGHT: f32 = 32.0;

#[test]
fn parses_valid_svg_data_uri_image() -> Result<(), Box<dyn std::error::Error>> {
    let image = HtmlImageRef::parse(VALID_SVG_IMAGE).ok_or("image should parse")?;

    assert_eq!(Some(128), image.width);
    assert_eq!("<svg></svg>", svg_payload(&image.src).ok_or("svg payload")?);
    Ok(())
}

#[test]
fn broken_katana_svg_data_uri_renders_image_surface_for_export_surface_parity() {
    let factory = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH);
    let node = html_node(BROKEN_KATANA_SVG_IMAGE);

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiNodeKind::ImageSurface, ui_node.kind());
}

#[test]
fn valid_svg_data_uri_still_renders_image_surface() {
    let factory = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH);
    let node = html_node(VALID_SVG_IMAGE);

    let ui_node = factory.viewer_node(&node);

    assert_eq!(UiNodeKind::ImageSurface, ui_node.kind());
}

fn html_node(raw: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("html-image-node".to_string()),
        kind: ViewerNodeKind::Html {
            role: ViewerHtmlRole::Centered,
        },
        source: source(raw),
        text: raw.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: HTML_IMAGE_NODE_WIDTH,
            height: HTML_IMAGE_NODE_HEIGHT,
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
