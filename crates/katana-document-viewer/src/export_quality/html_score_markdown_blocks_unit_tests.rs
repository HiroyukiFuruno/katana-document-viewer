use super::{
    html_score_markdown_blocks_helpers::{is_external_block_fence, starts_with_ordered_list},
    html_score_markdown_heading::source_is_setext_marker_line,
    html_score_markdown_visible_source::VisibleMarkdownSource,
};

#[test]
fn visible_markdown_source_ignores_fenced_blocks_for_code_block_detection() {
    let source = VisibleMarkdownSource::from("```rust\nlet x = 1;\n```\ntext");

    assert!(source.has_code_block());
    assert!(!source.has_list());
}

#[test]
fn visible_markdown_source_detects_ordered_list_markers() {
    let source = VisibleMarkdownSource::from("1) first\n2) second");

    assert!(source.has_list());
    assert_eq!(2, source.list_item_count());
    assert!(starts_with_ordered_list("1) first"));
}

#[test]
fn visible_markdown_source_counts_atx_and_setext_headings() {
    let source = VisibleMarkdownSource::from("# First\n\nSecond\n------\n\n### Third");

    assert_eq!(3, source.heading_count());
}

#[test]
fn visible_markdown_source_detects_nested_list_depth() {
    let source = VisibleMarkdownSource::from("- parent\n  - child");

    assert!(source.has_nested_list());
}

#[test]
fn visible_markdown_source_detects_syntax_code_fence() {
    let source = VisibleMarkdownSource::from("```rust\nlet x = 1;\n```");

    assert!(source.has_syntax_code_block());
    assert_eq!(1, source.code_block_count());
}

#[test]
fn visible_markdown_source_counts_tables() {
    let source = VisibleMarkdownSource::from("| A |\n| - |\n| 1 |\n\n| B |\n| - |\n| 2 |");

    assert!(source.has_table());
    assert_eq!(2, source.table_count());
}

#[test]
fn visible_markdown_source_skips_blockquotes_inside_alert_blocks() {
    let source = VisibleMarkdownSource::from("> [!NOTE]\n> alert\n\n> note");

    assert!(source.has_blockquote());
}

#[test]
fn visible_markdown_source_detects_rules_that_are_not_setext_markers() {
    let source = VisibleMarkdownSource::from("\n---\n");
    let lines = [String::new(), "---".to_string()];

    assert!(!source_is_setext_marker_line(&lines, 1));
    assert!(source.has_rule());
}

#[test]
fn visible_markdown_source_detects_footnotes() {
    let source = VisibleMarkdownSource::from("[^1]: note");

    assert!(source.has_footnote());
}

#[test]
fn visible_markdown_source_detects_external_block_fence_languages_case_insensitive() {
    assert!(is_external_block_fence("```MERMAID"));
    assert!(is_external_block_fence("```DRAWIO"));
    assert!(is_external_block_fence("```LaTeX"));
    assert!(!is_external_block_fence("```rust"));
}
