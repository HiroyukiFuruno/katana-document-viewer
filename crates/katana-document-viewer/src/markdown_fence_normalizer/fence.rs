pub(super) struct FenceBlock {
    pub(super) opening_line_len: usize,
    pub(super) content_end: usize,
    pub(super) close_end: usize,
}
const MIN_FENCE_MARKER_LEN: usize = 3;
pub(super) struct FenceNormalizer;

impl FenceNormalizer {
    pub(super) fn flatten_indented_fences(source: &str) -> String {
        let mut normalized = String::with_capacity(source.len());
        let mut in_indented_fence = false;
        let mut fence_indent = 0;
        for line in source.split_inclusive('\n') {
            let (body, ending) = Self::split_line_ending(line);
            if in_indented_fence {
                let stripped = Self::strip_leading_spaces(body, fence_indent);
                Self::push_indented_body(&mut normalized, stripped, ending, &mut in_indented_fence);
                continue;
            }
            Self::push_outer_line(
                &mut normalized,
                body,
                ending,
                &mut in_indented_fence,
                &mut fence_indent,
            );
        }
        normalized
    }
    pub(super) fn normalize_tilde_fences(source: &str) -> String {
        if !source.contains("~~~") {
            return source.to_string();
        }
        let mut normalized = String::with_capacity(source.len());
        for line in source.split_inclusive('\n') {
            normalized.push_str(&Self::normalize_tilde_line(line));
        }
        normalized
    }
    pub(super) fn preserve_empty_mermaid_fences(source: &str) -> String {
        let mut normalized = String::with_capacity(source.len());
        let mut offset = 0;
        while let Some(pos) = Self::find_line_marker(&source[offset..], "```") {
            let absolute = offset + pos;
            normalized.push_str(&source[offset..absolute]);
            let remaining = &source[absolute..];
            let Some(block) = Self::fence_block(remaining) else {
                normalized.push_str(remaining);
                return normalized;
            };
            Self::push_mermaid_or_original(&mut normalized, remaining, &block);
            offset = absolute + block.close_end;
        }
        normalized.push_str(&source[offset..]);
        normalized
    }

    pub(super) fn find_line_marker(source: &str, marker: &str) -> Option<usize> {
        if source.starts_with(marker) {
            return Some(0);
        }
        source.find(&format!("\n{marker}")).map(|pos| pos + 1)
    }
    pub(super) fn fence_block(source: &str) -> Option<FenceBlock> {
        let marker_len = Self::marker_run_len(source);
        if marker_len < MIN_FENCE_MARKER_LEN {
            return None;
        }
        let opening_line_len = source.find('\n')? + 1;
        let mut line_start = opening_line_len;
        while line_start < source.len() {
            let (body_end, next_start) = Self::line_bounds(source, line_start);
            let line = &source[line_start..body_end];
            if Self::is_closing_fence(line.trim_start(), marker_len) {
                return Some(FenceBlock {
                    opening_line_len,
                    content_end: line_start,
                    close_end: next_start,
                });
            }
            line_start = next_start;
        }
        None
    }

    pub(super) fn line_ending_len(source: &str) -> usize {
        if source.starts_with("\r\n") {
            return 2;
        }
        usize::from(source.starts_with('\n'))
    }

    fn push_indented_body(
        normalized: &mut String,
        stripped: &str,
        ending: &str,
        in_indented_fence: &mut bool,
    ) {
        if Self::starts_with_fence(stripped.trim_start()) {
            normalized.push_str(stripped.trim_start());
            *in_indented_fence = false;
        } else {
            normalized.push_str(stripped);
        }
        normalized.push_str(ending);
    }

    fn push_outer_line(
        normalized: &mut String,
        body: &str,
        ending: &str,
        in_indented_fence: &mut bool,
        fence_indent: &mut usize,
    ) {
        let trimmed = body.trim_start();
        let indent = body.len() - trimmed.len();
        if indent > 0 && Self::starts_with_fence(trimmed) {
            *in_indented_fence = true;
            *fence_indent = indent;
            normalized.push_str(trimmed);
        } else {
            normalized.push_str(body);
        }
        normalized.push_str(ending);
    }

    fn normalize_tilde_line(line: &str) -> String {
        let (body, ending) = Self::split_line_ending(line);
        let trimmed = body.trim_start();
        let leading_len = body.len() - trimmed.len();
        let Some(info) = trimmed.strip_prefix("~~~") else {
            return line.to_string();
        };
        let leading = &body[..leading_len];
        format!("{leading}```{info}{ending}")
    }

    fn push_mermaid_or_original(normalized: &mut String, remaining: &str, block: &FenceBlock) {
        let opening = &remaining[..block.opening_line_len];
        let body = &remaining[block.opening_line_len..block.content_end];
        if Self::is_mermaid_opening(opening) && body.trim().is_empty() {
            normalized.push_str(&Self::text_opening_line(opening));
            normalized.push_str(body);
            normalized.push_str(&remaining[block.content_end..block.close_end]);
        } else {
            normalized.push_str(&remaining[..block.close_end]);
        }
    }

    pub(super) fn line_bounds(source: &str, start: usize) -> (usize, usize) {
        let Some(relative_end) = source[start..].find('\n') else {
            return (source.len(), source.len());
        };
        let line_end = start + relative_end;
        (line_end, line_end + 1)
    }

    fn is_closing_fence(line: &str, opening_len: usize) -> bool {
        let marker_len = Self::marker_run_len(line);
        marker_len >= opening_len && line[marker_len..].trim().is_empty()
    }

    fn is_mermaid_opening(line: &str) -> bool {
        let marker_len = Self::marker_run_len(line);
        marker_len >= MIN_FENCE_MARKER_LEN
            && line[marker_len..].trim().eq_ignore_ascii_case("mermaid")
    }

    fn text_opening_line(line: &str) -> String {
        let (_, ending) = Self::split_line_ending(line);
        format!("```text{ending}")
    }

    fn marker_run_len(source: &str) -> usize {
        source.bytes().take_while(|byte| *byte == b'`').count()
    }

    fn starts_with_fence(source: &str) -> bool {
        if source.starts_with("```") {
            return true;
        }
        source.starts_with("~~~")
    }

    fn strip_leading_spaces(source: &str, max: usize) -> &str {
        let count = source.bytes().take_while(|byte| *byte == b' ').count();
        &source[count.min(max)..]
    }

    fn split_line_ending(line: &str) -> (&str, &str) {
        if let Some(body) = line.strip_suffix("\r\n") {
            return (body, "\r\n");
        }
        if let Some(body) = line.strip_suffix('\n') {
            return (body, "\n");
        }
        (line, "")
    }
}
