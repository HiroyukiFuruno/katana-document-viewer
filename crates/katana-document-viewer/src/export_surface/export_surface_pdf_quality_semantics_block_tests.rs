use crate::export_surface::SurfaceBlock;
use crate::export_surface::SurfaceBlockFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[path = "export_surface_pdf_quality_semantics_block_fixtures.rs"]
mod fixtures;
use fixtures::{details_markdown, list_markdown, pipe_sentence_markdown, task_markdown};

#[test]
fn pdf_surface_wraps_long_rich_list_items_before_next_item()
-> Result<(), Box<dyn std::error::Error>> {
    PdfSurfaceLongListItemCase::assert_wraps()
}

#[test]
fn pdf_surface_wraps_japanese_without_spaces_before_next_heading()
-> Result<(), Box<dyn std::error::Error>> {
    PdfSurfaceJapaneseParagraphCase::assert_wraps()
}

#[test]
fn pdf_surface_expands_details_body() -> Result<(), Box<dyn std::error::Error>> {
    let graph = SurfaceTestSupport::graph_from_markdown("details.md", details_markdown())?;
    let text = SurfaceTestSupport::surface_text(&graph);

    SurfaceTestSupport::assert_contains_all(
        &text,
        &["詳細を見る", "刀", "孫六兼元", "菊一文字則宗"],
    );
    SurfaceTestSupport::assert_not_contains_any(&text, &["<details>", "<summary>", "</details>"]);
    Ok(())
}

#[test]
fn pdf_surface_keeps_pipe_sentence_as_paragraph() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "pipe-sentence.md",
        pipe_sentence_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &["↑ 「English | 日本語」が中央揃えの同一行に表示されること。"],
    );
    SurfaceTestSupport::assert_not_contains_any(&debug, &["table:", "↑ 「English  日本語"]);
    Ok(())
}

#[test]
fn pdf_surface_keeps_list_indentation_without_quote_bars() -> Result<(), Box<dyn std::error::Error>>
{
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "list.md",
        list_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "項目 1:[\"indent=0\", \"list-marker=bullet\", \"marker-column=36\", \"marker-paint=material-dot\"]",
            "ネストされた項目 2-1:[\"indent=1\", \"list-marker=bullet\", \"marker-column=36\", \"marker-paint=material-circle\"]",
            "さらにネスト 2-2-1:[\"indent=2\", \"list-marker=bullet\", \"marker-column=36\", \"marker-paint=material-square\"]",
            "ネストされた番号 2-1:[\"indent=1\", \"list-marker=ordered\", \"marker-column=36\", \"marker-paint=text\"]",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(&debug, &["ネストされた項目 2-1:[\"quote=1\"]"]);
    Ok(())
}

#[test]
fn pdf_surface_tasks_use_html_like_checkbox_contract() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "tasks.md",
        task_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "完了タスク:[\"indent=0\", \"task-marker=done\", \"marker-column=36\", \"marker-paint=material-checkbox\"]",
            "未完了タスク:[\"indent=0\", \"task-marker=empty\", \"marker-column=36\", \"marker-paint=material-checkbox\"]",
            "保留タスク:[\"indent=0\", \"task-marker=blocked\", \"marker-column=36\", \"marker-paint=material-checkbox\"]",
            "進行中タスク:[\"indent=0\", \"task-marker=in-progress\", \"marker-column=36\", \"marker-paint=material-checkbox\"]",
        ],
    );
    Ok(())
}

struct PdfSurfaceLongListItemCase;

impl PdfSurfaceLongListItemCase {
    fn assert_wraps() -> Result<(), Box<dyn std::error::Error>> {
        let graph = SurfaceTestSupport::graph_from_markdown(
            "list-wrap.md",
            [
                "# wrapped list",
                "",
                "- macOS / Windows / Linux: You can update the **release asset** from the latest tag, choose the installer that matches your platform, then run the script and verify the checksum before starting the application for the next release cycle.",
                "",
                "- macOS Homebrew installs: brew install katana",
            ]
            .join("\n"),
        )?;

        let blocks = SurfaceBlockFactory::create(&graph, &graph.theme);
        let first_list_line = Self::first_line_index(&blocks, "macOS / Windows / Linux")
            .ok_or_else(|| std::io::Error::other("long rich list item should be emitted"))?;
        let next_item_line = Self::first_line_index(&blocks, "macOS Homebrew installs")
            .ok_or_else(|| std::io::Error::other("next list item should be emitted"))?;
        assert!(
            next_item_line > first_list_line + 1,
            "rich list item should be wrapped into multiple surface lines"
        );
        Ok(())
    }

    fn first_line_index(blocks: &[SurfaceBlock], marker: &str) -> Option<usize> {
        blocks
            .iter()
            .enumerate()
            .find_map(|(index, block)| match block {
                SurfaceBlock::Line(line) if line.text.contains(marker) => Some(index),
                _ => None,
            })
    }
}

struct PdfSurfaceJapaneseParagraphCase;

impl PdfSurfaceJapaneseParagraphCase {
    fn assert_wraps() -> Result<(), Box<dyn std::error::Error>> {
        let long_paragraph =
            "これはPDF出力の日本語折り返しを確認するための合成テキストです。".repeat(12);
        let graph = SurfaceTestSupport::graph_from_markdown(
            "japanese-wrap.md",
            [
                "# PDF overlap repro".to_string(),
                String::new(),
                "## 職務要約".to_string(),
                String::new(),
                long_paragraph,
                String::new(),
                "## 次の見出し".to_string(),
                String::new(),
                "ここは前の段落より下に独立して表示される必要があります。".to_string(),
            ]
            .join("\n"),
        )?;

        let blocks = SurfaceBlockFactory::create(&graph, &graph.theme);
        let paragraph_line = Self::first_line_index(&blocks, "これはPDF出力")
            .ok_or_else(|| std::io::Error::other("Japanese paragraph should be emitted"))?;
        let next_heading_line = Self::first_line_index(&blocks, "次の見出し")
            .ok_or_else(|| std::io::Error::other("next heading should be emitted"))?;
        assert!(
            next_heading_line > paragraph_line + 1,
            "Japanese paragraph must reserve wrapped surface lines before next heading"
        );
        Ok(())
    }

    fn first_line_index(blocks: &[SurfaceBlock], marker: &str) -> Option<usize> {
        blocks
            .iter()
            .enumerate()
            .find_map(|(index, block)| match block {
                SurfaceBlock::Line(line) if line.text.contains(marker) => Some(index),
                _ => None,
            })
    }
}
