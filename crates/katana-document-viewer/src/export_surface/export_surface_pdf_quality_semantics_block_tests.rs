use crate::export_surface::SurfaceBlock;
use crate::export_surface::SurfaceBlockFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_wraps_long_rich_list_items_before_next_item()
-> Result<(), Box<dyn std::error::Error>> {
    PdfSurfaceLongListItemCase::assert_wraps()
}

#[test]
fn pdf_surface_expands_details_body() -> Result<(), Box<dyn std::error::Error>> {
    let text = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        "details.md",
        details_markdown(),
    )?);

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

fn pipe_sentence_markdown() -> String {
    [
        r#"<p align="center"><a href="sample.md">English</a> | 日本語</p>"#,
        "",
        "↑ 「English | 日本語」が中央揃えの同一行に表示されること。",
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
