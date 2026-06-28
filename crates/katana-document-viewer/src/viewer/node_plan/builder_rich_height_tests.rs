use super::test_support::{input_with_nodes, node};
use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{ViewerCodeBlockMetrics, ViewerTypographyConfig};
use katana_markdown_model::{CodeBlockRole, InlineCodeNode, KmmNodeKind};
const BODY_LINE_HEIGHT: f32 = 46.0;
const NARROW_INLINE_CODE_LINE_COUNT: f32 = 5.0;

#[test]
fn planner_uses_rich_span_width_for_long_inline_code_height() {
    let code = "very long inline code with wrapping points ".repeat(5);
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Paragraph,
        &format!("`{code}`"),
        vec![node(
            KmmNodeKind::InlineCode(InlineCodeNode { code }),
            "`inline code`",
            Vec::new(),
        )],
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(ViewerNodeKind::Paragraph, plan.nodes[0].kind);
    assert!(plan.nodes[0].spans[0].style.inline_code);
    assert_eq!(
        BODY_LINE_HEIGHT * NARROW_INLINE_CODE_LINE_COUNT,
        plan.nodes[0].rect.height
    );
}

#[test]
fn planner_uses_export_surface_height_for_multiline_fenced_code() {
    let input = multiline_fenced_code_input();

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        ViewerNodeKind::Code {
            language: Some("rust".to_string())
        },
        plan.nodes[0].kind
    );
    assert_eq!(
        "fn main() {\n    println!(\"Hello\");\n}",
        plan.nodes[0].text
    );
    assert_eq!(multiline_fenced_code_height(), plan.nodes[0].rect.height);
}

fn multiline_fenced_code_input() -> crate::ViewerInput {
    input_with_nodes(vec![node(
        KmmNodeKind::CodeBlock(CodeBlockRole::Plain {
            language: Some("rust".to_string()),
        }),
        "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```",
        Vec::new(),
    )])
}

fn multiline_fenced_code_height() -> f32 {
    ViewerCodeBlockMetrics::block_height_from_line_count(
        3,
        ViewerTypographyConfig {
            preview_font_size: 24,
        },
    )
}

#[test]
fn planner_keeps_export_surface_alert_vertical_padding() {
    let input = input_with_nodes(vec![node(
        KmmNodeKind::Alert {
            label: "TIP".to_string(),
        },
        "> [!TIP]\n> body",
        Vec::new(),
    )]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(
        ViewerNodeKind::Alert {
            label: "TIP".to_string()
        },
        plan.nodes[0].kind
    );
    assert_eq!("TIP: body", plan.nodes[0].text);
    assert_eq!(124.0, plan.nodes[0].rect.height);
}
