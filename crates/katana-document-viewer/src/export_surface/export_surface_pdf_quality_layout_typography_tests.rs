use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

const FILLER_LINE_COUNT_BEFORE_TABLE: u32 = 31;

#[test]
fn pdf_surface_gives_headings_and_body_vertical_margins() {
    let heading = crate::export_surface_line::SurfaceLine::heading(2, "見出し".to_string());
    let body = crate::export_surface_line::SurfaceLine::body("本文".to_string());

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
fn pdf_surface_does_not_leave_section_heading_orphan_before_subheading_table()
-> Result<(), Box<dyn std::error::Error>> {
    let graph = SurfaceTestSupport::graph_from_markdown(
        "heading-table.md",
        heading_before_table_markdown(),
    )?;
    let pages = SurfaceTestSupport::surface_page_texts(&graph);
    let Some(page) = pages.iter().find(|page| page.contains("5. テーブル")) else {
        return Err(format!("section heading page is missing: {pages:#?}").into());
    };

    assert!(
        page.contains("5.1 基本テーブル") && page.contains("PreviewPane"),
        "section heading must move with its following subheading and table: {pages:#?}"
    );
    Ok(())
}

fn heading_before_table_markdown() -> String {
    let mut lines = vec!["# paged".to_string(), String::new()];
    for index in 1..=FILLER_LINE_COUNT_BEFORE_TABLE {
        lines.push(format!("filler line {index}"));
        lines.push(String::new());
    }
    lines.extend([
        "## 5. テーブル".to_string(),
        String::new(),
        "### 5.1 基本テーブル".to_string(),
        String::new(),
        "| コンポーネント | 役割 |".to_string(),
        "|---|---|".to_string(),
        "| PreviewPane | セクション管理 |".to_string(),
        "| show_content | UI描画 |".to_string(),
    ]);
    lines.join("\n")
}
