pub(crate) struct ExportPayloadContractTestMarkDowns;

impl ExportPayloadContractTestMarkDowns {
    pub(crate) fn contract_markdown() -> String {
        [
            "# 契約",
            "",
            "**太字** *斜体* ~~取り消し~~ `code` [リンク](https://example.com)",
            "",
            "> [!WARNING]",
            "> **危険** な操作です。",
            "",
            "- [/] 進行中",
            "- [-] 保留",
            "",
            "inline math: $a^2 + b^2 = c^2$",
            "",
            "```mermaid",
            "graph TD",
            "  A --> B",
            "```",
        ]
        .join("\n")
    }

    pub(crate) fn diagram_markdown() -> String {
        ["# 図形", "", "```mermaid", "graph TD", "  A --> B", "```"].join("\n")
    }

    pub(crate) fn interaction_exception_markdown() -> String {
        [
            "# export互換",
            "",
            "[通常のリンク](https://example.com)",
            "",
            "<details><summary>詳細を見る</summary><div>",
            "",
            "- 刀",
            "  - 孫六兼元",
            "  - 菊一文字則宗",
            "",
            "</div></details>",
        ]
        .join("\n")
    }

    pub(crate) fn multi_footnote_markdown() -> String {
        [
            "# footnotes",
            "",
            "これは脚注付きの本文です[^1]。これは脚注付きの本文です[^2]。",
            "",
            "[^1]: 最初の脚注本文。",
            "",
            "[^2]: 二番目の脚注本文。",
        ]
        .join("\n")
    }

    pub(crate) fn tall_markdown() -> String {
        let mut lines = vec!["# 長い文書".to_string(), String::new()];
        for index in 1..=TALL_DOC_LINE_COUNT {
            lines.push(format!(
                "段落 {index}: PDFは巨大な1ページではなく複数ページに分割する。"
            ));
            lines.push(String::new());
        }
        lines.join("\n")
    }
}

const TALL_DOC_LINE_COUNT: usize = 120;
