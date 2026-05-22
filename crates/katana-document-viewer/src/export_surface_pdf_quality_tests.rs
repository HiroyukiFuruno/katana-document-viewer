use super::test_support::{
    assert_contains_all, assert_not_contains_any, graph_from_markdown,
    graph_with_rendered_diagram_svg, surface_debug, surface_page_texts, surface_text,
};
use crate::ExportFormat;
use crate::KdvThemeSnapshot;
use crate::export_payload::ExportPayloadFactory;
use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface_line::SurfaceLine;
use katana_markdown_model::TableAlignment;

#[test]
fn pdf_surface_keeps_markdown_inline_semantics() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown("inline.md", inline_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "太字:[\"bold\"]",
            "斜体:[\"italic\"]",
            "取り消し線:[\"strikethrough\"]",
            "下線:[\"underline\"]",
            "code:[\"monospace\", \"inline-code\"]",
            "ハイライト:[\"highlight\"]",
            "太字と:[\"bold\"]",
            "イタリック:[\"bold\", \"italic\"]",
        ],
    );
    assert_not_contains_any(&debug, &["**太字**", "*斜体*", "`code`"]);
    Ok(())
}

#[test]
fn pdf_surface_expands_details_body() -> Result<(), Box<dyn std::error::Error>> {
    let text = surface_text(&graph_from_markdown("details.md", details_markdown())?);

    assert_contains_all(&text, &["詳細を見る", "刀", "孫六兼元", "菊一文字則宗"]);
    assert_not_contains_any(&text, &["<details>", "<summary>", "</details>"]);
    Ok(())
}

#[test]
fn pdf_surface_uses_ast_meaning_instead_of_raw_export_text()
-> Result<(), Box<dyn std::error::Error>> {
    let text = surface_text(&graph_from_markdown("ast.md", ast_markdown())?);

    assert_contains_all(
        &text,
        &[
            "KatanA 描画",
            "English | 日本語",
            "1. 最初のステップ:",
            "2. 次のステップ:",
            "3. 確認:",
            "☑ 完了",
            "☐ 未完了",
            "⊟ 保留",
            "◩ 進行中",
            "Note 本文",
            "GitHub Note",
            "詳細を見る",
            "刀",
            "HTML エンティティ: & < > \"",
            "[1]",
            "1. 脚注本文。 ↩",
        ],
    );
    assert_not_contains_any(
        &text,
        &[
            "KDV export:",
            "---------",
            "[!NOTE]",
            "<details>",
            "<summary>",
            "&amp;",
            "&lt;",
            "&gt;",
            "&quot;",
            "| English",
        ],
    );
    Ok(())
}

#[test]
fn pdf_surface_keeps_pipe_sentence_as_paragraph() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown(
        "pipe-sentence.md",
        pipe_sentence_markdown(),
    )?);

    assert_contains_all(
        &debug,
        &["↑ 「English | 日本語」が中央揃えの同一行に表示されること。"],
    );
    assert_not_contains_any(&debug, &["table:", "↑ 「English  日本語」"]);
    Ok(())
}

#[test]
fn pdf_surface_keeps_list_indentation_without_quote_bars() -> Result<(), Box<dyn std::error::Error>>
{
    let debug = surface_debug(&graph_from_markdown("list.md", list_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "項目 1:[\"indent=0\", \"list-marker=bullet\", \"marker-column=36\"]",
            "ネストされた項目 2-1:[\"indent=1\", \"list-marker=bullet\", \"marker-column=36\"]",
            "さらにネスト 2-2-1:[\"indent=2\", \"list-marker=bullet\", \"marker-column=36\"]",
            "ネストされた番号 2-1:[\"indent=1\", \"list-marker=ordered\", \"marker-column=36\"]",
        ],
    );
    assert_not_contains_any(&debug, &["ネストされた項目 2-1:[\"quote=1\"]"]);
    Ok(())
}

#[test]
fn pdf_surface_tasks_use_html_like_checkbox_contract() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown("tasks.md", task_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "完了タスク:[\"indent=0\", \"task-marker=done\", \"marker-column=36\"]",
            "未完了タスク:[\"indent=0\", \"task-marker=empty\", \"marker-column=36\"]",
            "保留タスク:[\"indent=0\", \"task-marker=blocked\", \"marker-column=36\"]",
            "進行中タスク:[\"indent=0\", \"task-marker=in-progress\", \"marker-column=36\"]",
        ],
    );
    Ok(())
}

#[test]
fn pdf_surface_marks_links_with_visual_link_style() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown("link.md", link_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "通常のリンク:[\"underline\", \"color\"]",
            "https://github.com:[\"underline\", \"color\"]",
            "[1]:[\"underline\", \"color\"]",
        ],
    );
    Ok(())
}

#[test]
fn pdf_surface_evaluates_shields_badges_as_badge_text() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown("badges.md", badges_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "License=MIT | CI=passing | platform=macOS",
            "[\"centered\"]",
        ],
    );
    assert_not_contains_any(
        &debug,
        &["img.shields.io", "<img", "License: MIT CI Platform"],
    );
    Ok(())
}

#[test]
fn pdf_surface_creates_link_annotations_for_badges() -> Result<(), Box<dyn std::error::Error>> {
    let graph = graph_from_markdown("badges.md", badges_markdown())?;
    let surface = DocumentSurfaceFactory::create(&graph, &KdvThemeSnapshot::katana_light());

    assert_eq!(
        surface
            .link_annotations
            .iter()
            .filter(|annotation| annotation.target == "#")
            .count(),
        3,
        "all badge images wrapped by links must keep clickable PDF areas"
    );
    Ok(())
}

#[test]
fn pdf_surface_renders_gfm_alerts_as_alert_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown("alerts.md", alerts_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "alert:NOTE:ⓘ Note",
            "alert:TIP:💡 Tip",
            "alert:IMPORTANT:▣ Important",
            "alert:WARNING:△ Warning",
            "alert:CAUTION:! Caution",
        ],
    );
    assert_not_contains_any(&debug, &["line:ⓘ Note", "[!NOTE]"]);
    Ok(())
}

#[test]
fn pdf_surface_keeps_structured_blocks_inside_regular_blockquote()
-> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown(
        "blockquote-children.md",
        structured_blockquote_markdown(),
    )?);

    assert_contains_all(
        &debug,
        &[
            "太字の引用:[\"bold\"]",
            "リスト項目 1:[\"indent=0\", \"list-marker=bullet\", \"marker-column=36\"]",
            "let:[\"monospace\", \"color\"]",
        ],
    );
    assert_not_contains_any(
        &debug,
        &["太字の引用 - リスト項目 1 - リスト項目 2 rust let"],
    );
    Ok(())
}

#[test]
fn pdf_surface_gives_headings_and_body_vertical_margins() {
    let heading = SurfaceLine::heading(2, "見出し".to_string());
    let body = SurfaceLine::body("本文".to_string());

    assert!(
        heading.line_height() >= 78,
        "heading lines must include vertical margin"
    );
    assert!(
        body.line_height() >= 44,
        "body lines must include readable vertical margin"
    );
}

#[test]
fn pdf_surface_math_keeps_fraction_denominator_group() {
    let text = crate::export_surface_math::SurfaceMathText::render(
        r"f(x) = \int_{0}^{x} \frac{t^2}{1 + t^4} \, dt",
    );

    assert!(text.contains("(t²)⁄(1 + t⁴)"), "{text}");
    assert!(!text.contains("t²⁄1 + t⁴"), "{text}");
}

#[test]
fn pdf_surface_caps_diagram_width() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_with_rendered_diagram_svg(
        diagram_markdown(),
        wide_svg(),
    )?);

    assert!(debug.contains("diagram:860x430"), "{debug}");
    Ok(())
}

#[test]
fn pdf_surface_does_not_upscale_small_diagram_svg() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_with_rendered_diagram_svg(
        diagram_markdown(),
        small_svg(),
    )?);

    assert!(
        debug.contains("diagram:320x160"),
        "small diagrams must keep their natural size instead of expanding to page width: {debug}"
    );
    Ok(())
}

#[test]
fn pdf_surface_does_not_leave_heading_orphan_before_diagram()
-> Result<(), Box<dyn std::error::Error>> {
    let pages = surface_page_texts(&graph_with_rendered_diagram_svg(
        heading_orphan_markdown(),
        small_svg(),
    )?);
    let Some(page) = pages
        .iter()
        .find(|page| page.contains("Heading before diagram"))
    else {
        return Err(format!("heading page is missing: {pages:#?}").into());
    };

    assert!(
        page.contains("Rendered diagram"),
        "heading and following diagram must stay on the same page: {pages:#?}"
    );
    Ok(())
}

#[test]
fn pdf_surface_code_uses_syntax_colored_spans() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown("code.md", code_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "fn:[\"monospace\", \"color\"]",
            "main:[\"monospace\", \"color\"]",
        ],
    );
    Ok(())
}

#[test]
fn pdf_surface_code_block_strips_trailing_line_endings() -> Result<(), Box<dyn std::error::Error>> {
    let text = surface_text(&graph_from_markdown(
        "code-string.md",
        code_string_markdown(),
    )?);

    assert_contains_all(&text, &[r#"let code = "引用ブロックの直後";"#]);
    assert_not_contains_any(&text, &["let code =\n\""]);
    Ok(())
}

#[test]
fn pdf_surface_code_block_after_blockquote_keeps_string_literal()
-> Result<(), Box<dyn std::error::Error>> {
    let text = surface_text(&graph_from_markdown(
        "quote-code.md",
        quote_then_code_markdown(),
    )?);

    assert_contains_all(&text, &[r#"let code = "引用ブロックの直後";"#]);
    assert_not_contains_any(&text, &["let code =\n\""]);
    Ok(())
}

#[test]
fn pdf_surface_table_is_a_table_block() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown("table.md", table_markdown())?);

    assert_contains_all(
        &debug,
        &[
            "table:3x2",
            "コンポーネント  役割",
            "PreviewPane  セクション管理",
        ],
    );
    assert_not_contains_any(&debug, &["| コンポーネント |", "|---|"]);
    Ok(())
}

#[test]
fn pdf_surface_table_right_alignment_keeps_cell_padding() {
    let cell_x = 120;
    let cell_width = 360;
    let text = "テキスト";
    let text_x = super::super::table_cell_text_x(text, TableAlignment::Right, cell_x, cell_width);
    let text_right = text_x + super::super::estimated_cell_text_width(text);

    assert!(
        text_right <= cell_x + cell_width - 16,
        "right-aligned table text must not run into the cell border"
    );
}

#[test]
fn pdf_surface_table_uses_html_like_padding_and_theme_colors()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = KdvThemeSnapshot::katana_light();
    assert_eq!(theme.table_header_background, "#eaf5ff");
    assert_eq!(theme.table_even_row_background, "#f7fbff");

    let (row_height, text_y) =
        surface_table_metrics_from_markdown(table_markdown()).ok_or("table fixture must parse")?;

    assert!(row_height >= 66, "table rows need vertical breathing room");
    assert!(
        text_y >= super::super::TABLE_CELL_PADDING,
        "table text must be vertically centered with padding"
    );
    Ok(())
}

#[test]
fn pdf_surface_footnote_definitions_have_backlinks() -> Result<(), Box<dyn std::error::Error>> {
    let text = surface_text(&graph_from_markdown("footnote.md", footnote_markdown())?);

    assert_contains_all(
        &text,
        &[
            "これは脚注付きのテキストです[1]。",
            "1. 最初の脚注の内容。 ↩",
        ],
    );
    assert_not_contains_any(&text, &["[1] ↩ 最初の脚注の内容。"]);
    Ok(())
}

#[test]
fn pdf_payload_uses_document_internal_links_for_footnotes() -> Result<(), Box<dyn std::error::Error>>
{
    let graph = graph_from_markdown("footnote.md", footnote_markdown())?;
    let pdf =
        ExportPayloadFactory::create(&graph, ExportFormat::Pdf, &KdvThemeSnapshot::katana_light())?;
    let text = String::from_utf8_lossy(&pdf);

    assert!(text.contains("/S /GoTo"), "{text}");
    assert!(!text.contains("/URI (#fn-1)"), "{text}");
    assert!(!text.contains("/URI (#fnref-1)"), "{text}");
    Ok(())
}

#[test]
fn pdf_surface_empty_code_block_keeps_visible_code_area() -> Result<(), Box<dyn std::error::Error>>
{
    let box_height = surface_code_box_height_from_markdown(empty_code_markdown())
        .ok_or("code fixture must parse")?;

    assert!(
        box_height >= 56,
        "empty code block must not collapse into a horizontal line"
    );
    Ok(())
}

#[test]
fn pdf_surface_empty_code_block_paints_visible_code_area() -> Result<(), Box<dyn std::error::Error>>
{
    let graph = graph_from_markdown("empty-code.md", empty_code_markdown())?;
    let theme = KdvThemeSnapshot::katana_light();
    let surface = DocumentSurfaceFactory::create(&graph, &theme);
    let code_background = crate::export_surface_helpers::parse_color(&theme.code_background);
    let painted_rows = count_rows_with_code_background(&surface.image, code_background);

    assert!(
        painted_rows >= 48,
        "empty code block must paint a visible rectangular area, not a thin line: {painted_rows}"
    );
    Ok(())
}

#[test]
fn pdf_surface_places_footnotes_after_following_body() -> Result<(), Box<dyn std::error::Error>> {
    let text = surface_text(&graph_from_markdown(
        "footnote-bottom.md",
        footnote_with_following_body_markdown(),
    )?);
    let body_position = text
        .find("脚注定義の後に続く本文。")
        .ok_or("following body is missing")?;
    let footnote_position = text
        .find("1. 脚注本文。 ↩")
        .ok_or("footnote definition is missing")?;

    assert!(
        footnote_position > body_position,
        "footnotes must be collected at the bottom instead of interrupting body flow: {text}"
    );
    Ok(())
}

#[test]
fn pdf_surface_renders_math_from_shared_svg() -> Result<(), Box<dyn std::error::Error>> {
    let debug = surface_debug(&graph_from_markdown(
        "math.md",
        [
            "# math",
            "",
            "```math",
            r"f(x) = \int_{0}^{x} \frac{t^2}{1 + t^4} \, dt",
            "```",
            "",
            r"$$ \sum_{k=1}^{n} k = \frac{n(n+1)}{2} $$",
        ]
        .join("\n"),
    )?);

    assert_contains_all(&debug, &["math-svg:"]);
    assert_not_contains_any(&debug, &[r"\frac", "lower=", "upper=", "math:f(x) = ∫"]);
    Ok(())
}

#[test]
fn pdf_surface_keeps_sample_math_fraction_on_same_page() -> Result<(), Box<dyn std::error::Error>> {
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/rendering/sample.ja.md");
    let markdown = std::fs::read_to_string(&fixture)?;
    let pages = surface_page_texts(&graph_from_markdown(
        &fixture.display().to_string(),
        markdown,
    )?);
    let page = pages
        .iter()
        .find(|page| page.contains("math-svg:"))
        .ok_or("integral page is missing")?;

    assert!(
        !page.contains(r"\frac"),
        "display math fraction must not be split across pages: {pages:#?}"
    );
    Ok(())
}

#[test]
fn pdf_surface_keeps_emoji_sequence_on_one_line() -> Result<(), Box<dyn std::error::Error>> {
    let text = surface_text(&graph_from_markdown("emoji.md", emoji_markdown())?);

    assert_contains_all(&text, &["絵文字: 🦀 ⚡ 📝 🔧 ✅ ❌ ⚠️ 💡 ⭐"]);
    Ok(())
}

#[test]
fn pdf_surface_renders_code_highlight_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let graph = graph_from_markdown("code.md", code_markdown())?;
    let surface = DocumentSurfaceFactory::create(&graph, &KdvThemeSnapshot::katana_light());

    assert!(
        contains_non_black_code_pixel(&surface.image),
        "syntax color pixel missing"
    );
    Ok(())
}

fn contains_non_black_code_pixel(image: &image::RgbaImage) -> bool {
    image
        .pixels()
        .any(|pixel| pixel[3] == 255 && pixel[0] > 120 && pixel[1] < 80 && pixel[2] > 80)
}

fn count_rows_with_code_background(image: &image::RgbaImage, color: image::Rgba<u8>) -> usize {
    (0..image.height())
        .filter(|y| {
            image
                .pixels()
                .skip((*y * image.width()) as usize)
                .take(image.width() as usize)
                .filter(|pixel| **pixel == color)
                .count()
                > image.width() as usize / 2
        })
        .count()
}

fn inline_markdown() -> String {
    [
        "# inline",
        "",
        "**太字** *斜体* ~~取り消し線~~ <u>下線</u> `code` <mark>ハイライト</mark> **太字と*イタリック*の混在**",
    ]
    .join("\n")
}

fn list_markdown() -> String {
    [
        "# list",
        "",
        "- 項目 1",
        "- 項目 2",
        "  - ネストされた項目 2-1",
        "  - ネストされた項目 2-2",
        "    - さらにネスト 2-2-1",
        "",
        "1. 最初の項目",
        "2. 次の項目",
        "   1. ネストされた番号 2-1",
    ]
    .join("\n")
}

fn task_markdown() -> String {
    [
        "# tasks",
        "",
        "- [x] 完了タスク",
        "- [ ] 未完了タスク",
        "- [-] 保留タスク",
        "- [/] 進行中タスク",
    ]
    .join("\n")
}

fn details_markdown() -> String {
    [
        "<details><summary>詳細を見る</summary>",
        "",
        "- 刀",
        "  - 孫六兼元",
        "  - 菊一文字則宗",
        "",
        "</details>",
    ]
    .join("\n")
}

fn diagram_markdown() -> String {
    [
        "# diagram",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
    .join("\n")
}

fn wide_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="2000" height="1000">"#,
        r##"<rect width="2000" height="1000" fill="#ff0000"/>"##,
        "</svg>",
    ]
    .join("")
}

fn code_markdown() -> String {
    [
        "# code",
        "",
        "```rust",
        "fn main() {",
        "  println!(\"hi\");",
        "}",
        "```",
    ]
    .join("\n")
}

fn empty_code_markdown() -> String {
    ["# empty code", "", "```", "```"].join("\n")
}

fn code_string_markdown() -> String {
    [
        "# code",
        "",
        "```rust",
        r#"let code = "引用ブロックの直後";"#,
        "```",
    ]
    .join("\n")
}

fn quote_then_code_markdown() -> String {
    [
        "> 引用ブロック",
        "",
        "```rust",
        r#"let code = "引用ブロックの直後";"#,
        "```",
    ]
    .join("\n")
}

fn table_markdown() -> String {
    [
        "| コンポーネント | 役割 |",
        "|---|---|",
        "| `PreviewPane` | **セクション管理** |",
        "| `show_content` | UI描画 |",
    ]
    .join("\n")
}

fn surface_table_metrics_from_markdown(markdown: String) -> Option<(u32, u32)> {
    let graph = graph_from_markdown("table.md", markdown).ok()?;
    let blocks = super::super::SurfaceBlockFactory::create(&graph);
    blocks.iter().find_map(|block| match block {
        super::super::SurfaceBlock::Table(table) => {
            let column_width = super::super::SURFACE_CONTENT_WIDTH / table.column_count() as u32;
            let row_height = table.row_height(0, column_width);
            Some((row_height, super::super::table_cell_text_y(row_height, 1)))
        }
        _ => None,
    })
}

fn surface_code_box_height_from_markdown(markdown: String) -> Option<u32> {
    let graph = graph_from_markdown("code.md", markdown).ok()?;
    let blocks = super::super::SurfaceBlockFactory::create(&graph);
    blocks.iter().find_map(|block| match block {
        super::super::SurfaceBlock::Code(code) => Some(code.box_height()),
        _ => None,
    })
}

fn pipe_sentence_markdown() -> String {
    [
        r#"<p align="center"><a href="sample.md">English</a> | 日本語</p>"#,
        "",
        "↑ 「English | 日本語」が中央揃えの同一行に表示されること。",
    ]
    .join("\n")
}

fn link_markdown() -> String {
    [
        "# links",
        "",
        "[通常のリンク](https://github.com)",
        "",
        "自動リンク: <https://github.com>",
        "",
        "脚注です[^1]。",
        "",
        "[^1]: 脚注本文。",
    ]
    .join("\n")
}

fn badges_markdown() -> String {
    [
        r#"<p align="center">"#,
        r##"  <a href="#"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>"##,
        r##"  <a href="#"><img src="https://img.shields.io/badge/CI-passing-brightgreen.svg" alt="CI"></a>"##,
        r##"  <a href="#"><img src="https://img.shields.io/badge/platform-macOS-lightgrey" alt="Platform: macOS"></a>"##,
        r#"</p>"#,
    ]
    .join("\n")
}

fn alerts_markdown() -> String {
    [
        "> [!NOTE]",
        "> note body",
        "",
        "> [!TIP]",
        "> tip body",
        "",
        "> [!IMPORTANT]",
        "> important body",
        "",
        "> [!WARNING]",
        "> warning body",
        "",
        "> [!CAUTION]",
        "> caution body",
    ]
    .join("\n")
}

fn structured_blockquote_markdown() -> String {
    [
        "> **太字の引用**",
        ">",
        "> - リスト項目 1",
        "> - リスト項目 2",
        ">",
        "> ```rust",
        "> let quoted_code = true;",
        "> ```",
    ]
    .join("\n")
}

fn emoji_markdown() -> String {
    ["# emoji", "", "絵文字: 🦀 ⚡ 📝 🔧 ✅ ❌ ⚠️ 💡 ⭐"].join("\n")
}

fn footnote_markdown() -> String {
    [
        "# footnote",
        "",
        "これは脚注付きのテキストです[^1]。",
        "",
        "[^1]: 最初の脚注の内容。",
    ]
    .join("\n")
}

fn footnote_with_following_body_markdown() -> String {
    [
        "# footnote",
        "",
        "これは脚注付きのテキストです[^1]。",
        "",
        "[^1]: 脚注本文。",
        "",
        "脚注定義の後に続く本文。",
    ]
    .join("\n")
}

fn heading_orphan_markdown() -> String {
    let mut lines = vec!["# paged".to_string(), String::new()];
    for index in 1..=42 {
        lines.push(format!("filler line {index}"));
        lines.push(String::new());
    }
    lines.extend([
        "## Heading before diagram".to_string(),
        String::new(),
        "```mermaid".to_string(),
        "graph TD".to_string(),
        "  A --> B".to_string(),
        "```".to_string(),
    ]);
    lines.join("\n")
}

fn ast_markdown() -> String {
    [
        "# 🧪 KatanA 描画",
        "",
        "<p align=\"center\"><a href=\"readme.en.md\">English</a> | 日本語</p>",
        "",
        "---",
        "",
        "1. 最初のステップ:",
        "",
        "   ```bash",
        "   cargo build --release",
        "   ```",
        "",
        "2. 次のステップ:",
        "",
        "   ```bash",
        "   ./target/release/KatanA",
        "   ```",
        "",
        "3. 確認:",
        "   - サブ項目 A",
        "",
        "- [x] 完了",
        "- [ ] 未完了",
        "- [-] 保留",
        "- [/] 進行中",
        "",
        "> **Note**",
        "> 本文",
        "",
        "> [!NOTE]",
        "> GitHub Note",
        "",
        "<details><summary>詳細を見る</summary><div>",
        "",
        "- 刀",
        "  - 孫六兼元",
        "",
        "</div></details>",
        "",
        "- HTML エンティティ: &amp; &lt; &gt; &quot;",
        "",
        "脚注です[^1]。",
        "",
        "[^1]: 脚注本文。",
    ]
    .join("\n")
}

fn small_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="320" height="160">"#,
        r##"<rect width="320" height="160" fill="#ff0000"/>"##,
        "</svg>",
    ]
    .join("")
}
