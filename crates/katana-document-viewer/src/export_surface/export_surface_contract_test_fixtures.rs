use crate::export_surface_helpers::PAGE_PADDING;
use crate::export_surface_line::SurfaceLine;

const DIAGRAM_SAMPLE_TOP_OFFSET: u32 = 28;

pub(super) fn surface_markdown() -> String {
    [
        "# 装飾",
        "",
        "**太字** と [リンク](https://example.com) と ![アイコン](icon.png)",
        "",
        r#"<p align="center"><img alt="English" src="badge.svg"></p>"#,
        "",
        "| コンポーネント | 役割 |",
        "|---|---|",
        "| `PreviewPane` | **セクション管理** |",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
    .join("\n")
}

pub(super) fn code_markdown() -> String {
    ["# コード", "", "```rust", "fn main() {}", "```"].join("\n")
}

pub(super) fn math_markdown() -> String {
    [
        "# 数式",
        "",
        "```math",
        r"f(x) = \int_{0}^{x} \frac{t^2}{1 + t^4} \, dt",
        "```",
        "",
        "inline: $ E = mc^2 $",
        "",
        r"$$ \sum_{k=1}^{n} k = \frac{n(n+1)}{2} $$",
    ]
    .join("\n")
}

pub(super) fn first_code_line_y() -> u32 {
    PAGE_PADDING
        + SurfaceLine::body("source".to_string()).line_height()
        + SurfaceLine::heading(1, "コード".to_string()).line_height()
}

pub(super) fn first_diagram_sample_y() -> u32 {
    PAGE_PADDING
        + SurfaceLine::body("source".to_string()).line_height()
        + SurfaceLine::heading(1, "図形".to_string()).line_height()
        + DIAGRAM_SAMPLE_TOP_OFFSET
}

pub(super) fn diagram_markdown() -> String {
    ["# 図形", "", "```mermaid", "graph TD", "  A --> B", "```"].join("\n")
}

pub(super) fn red_rect_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="60">"#,
        r##"<rect width="120" height="60" fill="#ff0000"/>"##,
        "</svg>",
    ]
    .join("")
}

pub(super) fn styled_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="240" height="120">"#,
        r#"<style>#katana-mermaid-svg{font-family:trebuchet ms;}@keyframes edge-animation{from{stroke-dasharray:0;}}</style>"#,
        r#"<text x="16" y="48">Rendered diagram</text>"#,
        "</svg>",
    ]
    .join("")
}

pub(super) fn contract_markdown() -> String {
    contract_markdown_lines().join("\n")
}

fn contract_markdown_lines() -> Vec<&'static str> {
    vec![
        "# 契約",
        "",
        "**太字** *斜体* ~~取り消し~~ `code` [リンク](https://example.com) ![アイコン](icon.png)",
        "",
        "> [!WARNING]",
        "> **危険** な操作です。",
        "",
        "- [/] 進行中",
        "- [-] 保留",
        "",
        "<details><summary>詳細</summary>",
        "",
        "- 刀",
        "",
        "</details>",
        "",
        "inline math: $a^2 + b^2 = c^2$",
        "",
        "```math",
        "a^2 + b^2 = c^2",
        "```",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
}
