#[path = "markdown_fence_normalizer/fence.rs"]
mod fence;
#[path = "markdown_fence_normalizer/raw_diagram.rs"]
mod raw_diagram;

use fence::FenceNormalizer;
use raw_diagram::RawDiagramNormalizer;

pub struct MarkdownFenceNormalizer;

impl MarkdownFenceNormalizer {
    pub fn normalize(source: &str) -> String {
        let flattened = FenceNormalizer::flatten_indented_fences(source);
        let backtick_fences = FenceNormalizer::normalize_tilde_fences(&flattened);
        let preserved = FenceNormalizer::preserve_empty_mermaid_fences(&backtick_fences);
        RawDiagramNormalizer::wrap(&preserved)
    }
}

#[cfg(test)]
#[path = "markdown_fence_normalizer/tests.rs"]
mod tests;
