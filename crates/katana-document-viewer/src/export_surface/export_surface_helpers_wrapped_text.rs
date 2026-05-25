use crate::export_surface_text::SurfaceTextParser;

use super::SurfaceHelpers;

pub(crate) struct WrappedText {
    chunks: Vec<String>,
    index: usize,
}

impl WrappedText {
    pub(crate) fn new(text: &str, max_chars: usize) -> Self {
        let characters: Vec<char> = text.chars().collect();
        let mut chunks = Vec::new();
        for chunk in characters.chunks(max_chars) {
            chunks.push(chunk.iter().collect());
        }
        if chunks.is_empty() {
            chunks.push(String::new());
        }
        Self { chunks, index: 0 }
    }
}

impl Iterator for WrappedText {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.chunks.get(self.index)?.clone();
        self.index += 1;
        Some(item)
    }
}

impl SurfaceHelpers {
    pub(crate) fn is_nested_blockquote(text: &str) -> bool {
        text.lines()
            .filter_map(blockquote_line_parts)
            .any(|line| line.depth > 1)
    }

    pub(crate) fn nested_blockquote_lines(text: &str, base_depth: u32) -> Vec<(String, u32)> {
        text.lines()
            .filter_map(blockquote_line_parts)
            .filter(|line| !line.text.trim().is_empty())
            .map(|line| {
                (
                    SurfaceTextParser::inline_markdown_text(line.text),
                    base_depth + line.depth,
                )
            })
            .collect()
    }
}

fn blockquote_line_parts(line: &str) -> Option<BlockquoteLine<'_>> {
    let mut rest = line.trim_start();
    let mut depth = 0;
    while let Some(stripped) = rest.strip_prefix('>') {
        depth += 1;
        rest = stripped.trim_start();
    }
    (depth > 0).then_some(BlockquoteLine { depth, text: rest })
}

struct BlockquoteLine<'a> {
    depth: u32,
    text: &'a str,
}
