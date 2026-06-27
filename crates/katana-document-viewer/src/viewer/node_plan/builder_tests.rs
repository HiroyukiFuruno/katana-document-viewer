use super::test_support::{
    assert_node_span_texts, assert_node_texts, input_with_nodes, long_list, node, node_at_line,
    text_node, text_node_at_line,
};
use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{KdvThemeMode, KdvThemeSnapshot, ViewerHtmlRole};
use katana_markdown_model::{CodeBlockRole, HtmlBlockRole, ImageNode, KmmNodeKind, TextSpan};

#[test]
fn planner_recurses_into_unclassified_parent_nodes() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Text(TextSpan {
            text: "wrapper".to_string(),
        }),
        "wrapper",
        vec![node(
            KmmNodeKind::Paragraph,
            "child paragraph",
            vec![node(
                KmmNodeKind::Text(TextSpan {
                    text: "child paragraph".to_string(),
                }),
                "child paragraph",
                Vec::new(),
            )],
        )],
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(1, plan.nodes.len());
    assert_eq!(ViewerNodeKind::Paragraph, plan.nodes[0].kind);
    assert_eq!("child paragraph", plan.nodes[0].text);
}

#[test]
fn planner_collapses_soft_paragraph_line_breaks_before_width_wrapping() {
    let input = input_with_nodes(vec![
        node_at_line(
            KmmNodeKind::Paragraph,
            "first line",
            vec![text_node_at_line("first line", 1)],
            1,
        ),
        node_at_line(
            KmmNodeKind::Paragraph,
            "second line",
            vec![text_node_at_line("second line", 2)],
            2,
        ),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(1, plan.nodes.len());
    assert_eq!("first line second line", plan.nodes[0].text);
    assert_eq!(
        "first line second line",
        plan.nodes[0]
            .spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>()
    );
}

#[test]
fn planner_collapses_soft_line_breaks_inside_paragraph_spans() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Paragraph,
        "↑ \"English | 日本語\" should appear on the same line, centered.",
        vec![
            text_node("↑ \"English | 日本語\" should appear on the same line, centered"),
            text_node("\n"),
            text_node("."),
        ],
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);
    let span_text = plan.nodes[0]
        .spans
        .iter()
        .map(|span| span.text.as_str())
        .collect::<String>();

    assert_eq!(
        "↑ \"English | 日本語\" should appear on the same line, centered.",
        span_text
    );
    assert!(!span_text.contains('\n'));
}

#[test]
fn planner_preserves_physical_paragraph_rows_for_export_surface_parity() {
    let input = input_with_nodes(vec![
        node_at_line(
            KmmNodeKind::Paragraph,
            "first line",
            vec![text_node_at_line("first line", 1)],
            1,
        ),
        node_at_line(
            KmmNodeKind::Paragraph,
            "second line",
            vec![text_node_at_line("second line", 2)],
            2,
        ),
    ]);

    let plan = ViewerNodePlanner::create_export_surface(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_node_texts(&plan, &["first line", "second line"]);
    assert_node_span_texts(&plan, &["first line", "second line"]);
}

#[test]
fn planner_passes_dark_theme_to_code_highlighter() {
    let mut input = input_with_nodes(vec![node(
        KmmNodeKind::CodeBlock(CodeBlockRole::Plain {
            language: Some("rust".to_string()),
        }),
        "```rust\nenum RenderedSection {\n    Markdown(String),\n}\n```",
        Vec::new(),
    )]);
    input.theme = dark_code_theme();

    let plan = ViewerNodePlanner::create(&input, 0.0);
    let text_color = plan.nodes[0]
        .spans
        .iter()
        .find(|span| span.text.contains("RenderedSection"))
        .map(|span| span.style.color_rgba);
    assert!(
        text_color.is_some(),
        "identifier span should be highlighted"
    );
    let text_color = text_color.unwrap_or([0, 0, 0, 0]);
    let luminance =
        (u16::from(text_color[0]) + u16::from(text_color[1]) + u16::from(text_color[2])) / 3;

    assert!(
        luminance >= 150,
        "planner must not use a light syntax theme on dark code background: rgba={text_color:?}"
    );
}

fn dark_code_theme() -> KdvThemeSnapshot {
    KdvThemeSnapshot {
        mode: KdvThemeMode::Dark,
        syntax_theme_dark: "base16-ocean.dark".to_string(),
        ..KdvThemeSnapshot::default()
    }
}

#[test]
fn planner_uses_multiline_list_height_for_scroll_extent() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::List(long_list()),
        "list",
        Vec::new(),
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert!(plan.nodes[0].rect.height > 200.0);
    assert_eq!(plan.nodes[0].rect.height, plan.content_height);
}

#[test]
fn planner_treats_aligned_html_text_as_text_height_not_media_height() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        r#"<p align="center">Center</p>"#,
        Vec::new(),
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Centered
        },
        plan.nodes[0].kind
    );
    assert_eq!(46.0, plan.nodes[0].rect.height);
}

#[test]
fn planner_loads_standalone_image_paragraph_as_media_node() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Paragraph,
        "![screen](screen.webp)",
        vec![node(
            KmmNodeKind::Image(ImageNode {
                alt: "screen".to_string(),
                src: "screen.webp".to_string(),
                title: None,
            }),
            "![screen](screen.webp)",
            Vec::new(),
        )],
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(1, plan.nodes.len());
    assert_eq!(ViewerNodeKind::Image, plan.nodes[0].kind);
    assert_eq!(1, plan.asset_requests.len());
    assert_eq!(
        plan.nodes[0].artifact_id,
        Some(plan.asset_requests[0].artifact_id.clone())
    );
}
