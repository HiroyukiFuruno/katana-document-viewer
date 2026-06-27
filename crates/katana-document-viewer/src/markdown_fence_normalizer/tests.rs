use super::MarkdownFenceNormalizer;
use super::fence::FenceNormalizer;

#[test]
fn normalizes_tilde_fences_to_backtick_fences() {
    let normalized = MarkdownFenceNormalizer::normalize(
        "~~~mermaid\ngraph TD\n~~~\n\n  ~~~plantuml\r\n@enduml\r\n  ~~~\r\n",
    );

    assert_eq!(
        normalized,
        "```mermaid\ngraph TD\n```\n\n```plantuml\r\n@enduml\r\n```\r\n"
    );
}

#[test]
fn keeps_sources_without_tilde_fences_unchanged() {
    let source = "```rust\nfn main() {}\n```\n";

    assert_eq!(MarkdownFenceNormalizer::normalize(source), source);
}

#[test]
fn normalizes_single_tilde_fence_without_line_ending() {
    assert_eq!(MarkdownFenceNormalizer::normalize("~~~"), "```");
}

#[test]
fn wraps_raw_drawio_and_plantuml_blocks() {
    let normalized = MarkdownFenceNormalizer::normalize(
        "<mxGraphModel></mxGraphModel>\n@startuml\nA -> B\n@enduml",
    );

    assert!(normalized.contains("```drawio\n<mxGraphModel></mxGraphModel>\n```"));
    assert!(normalized.contains("```plantuml\n@startuml\nA -> B\n@enduml\n```"));
}

#[test]
fn wraps_raw_mxfile_drawio_blocks() {
    let normalized =
        MarkdownFenceNormalizer::normalize("<mxfile><diagram id=\"1\"></diagram></mxfile>\n");

    assert!(normalized.contains("```drawio\n<mxfile><diagram id=\"1\"></diagram></mxfile>\n```"));
}

#[test]
fn keeps_empty_mermaid_fence_as_code() {
    let normalized = MarkdownFenceNormalizer::normalize("```mermaid\n  \n```\n");

    assert_eq!(normalized, "```text\n  \n```\n");
}

#[test]
fn fence_block_requires_at_least_three_backticks() {
    assert!(FenceNormalizer::fence_block("``\nno fence").is_none());
}

#[test]
fn find_line_marker_finds_non_prefixed_marker() {
    assert_eq!(
        FenceNormalizer::find_line_marker("no\n```rust\n", "```"),
        Some(3)
    );
}

#[test]
fn line_bounds_with_newline_uses_line_end_and_next_start() {
    assert_eq!(FenceNormalizer::line_bounds("a\nb\nc", 2), (3, 4));
}

#[test]
fn keep_source_when_mermaid_fence_has_body() {
    let normalized = MarkdownFenceNormalizer::normalize("```mermaid\nbody\n```\n");

    assert_eq!(normalized, "```mermaid\nbody\n```\n");
}

#[test]
fn fence_block_without_closing_marker_returns_none() {
    assert!(FenceNormalizer::fence_block("```rust\nunclosed\n").is_none());
}

#[test]
fn line_bounds_without_newline_returns_source_length() {
    assert_eq!(FenceNormalizer::line_bounds("abc", 0), (3, 3));
}

#[test]
fn line_ending_len_detects_crlf() {
    assert_eq!(FenceNormalizer::line_ending_len("\r\nrest"), 2);
}

#[test]
fn line_ending_len_detects_lf_and_absent_line_ending() {
    assert_eq!(FenceNormalizer::line_ending_len("\nrest"), 1);
    assert_eq!(FenceNormalizer::line_ending_len("rest"), 0);
}

#[test]
fn flattens_indented_diagram_fence() {
    let normalized = MarkdownFenceNormalizer::normalize("  ```mermaid\n  graph TD\n  ```");

    assert_eq!(normalized, "```mermaid\ngraph TD\n```");
}
