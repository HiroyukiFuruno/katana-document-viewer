pub(super) fn details_markdown() -> String {
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

pub(super) fn pipe_sentence_markdown() -> String {
    [
        r#"<p align="center"><a href="sample.md">English</a> | 日本語</p>"#,
        "",
        "↑ 「English | 日本語」が中央揃えの同一行に表示されること。",
    ]
    .join("\n")
}

pub(super) fn list_markdown() -> String {
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

pub(super) fn task_markdown() -> String {
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
