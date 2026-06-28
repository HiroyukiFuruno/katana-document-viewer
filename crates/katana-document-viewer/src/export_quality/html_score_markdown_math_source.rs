pub(super) struct HtmlMarkdownMathSource;

impl HtmlMarkdownMathSource {
    pub(super) fn contains_math(source: &str) -> bool {
        let mut inside_fence = false;
        for line in source.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                inside_fence = !inside_fence;
                continue;
            }
            if !inside_fence && Self::line_contains_math(line) {
                return true;
            }
        }
        false
    }

    fn line_contains_math(line: &str) -> bool {
        let bytes = line.as_bytes();
        let mut index = 0;
        let mut inside_code = false;
        while index < bytes.len() {
            match bytes[index] {
                b'`' if !Self::is_escaped(bytes, index) => {
                    inside_code = !inside_code;
                    index += 1;
                }
                b'$' if !inside_code && !Self::is_escaped(bytes, index) => {
                    if Self::math_delimiter_has_content(line, index) {
                        return true;
                    }
                    index += Self::delimiter_length(bytes, index);
                }
                _ => {
                    index += 1;
                }
            }
        }
        false
    }

    fn math_delimiter_has_content(line: &str, start: usize) -> bool {
        let bytes = line.as_bytes();
        let delimiter_length = Self::delimiter_length(bytes, start);
        if delimiter_length == 2 {
            return true;
        }

        let content_start = start + delimiter_length;
        let Some(end) = Self::closing_delimiter_start(bytes, content_start, delimiter_length)
        else {
            return false;
        };
        end > content_start
    }

    fn closing_delimiter_start(
        bytes: &[u8],
        content_start: usize,
        delimiter_length: usize,
    ) -> Option<usize> {
        let mut index = content_start;
        while index < bytes.len() {
            if bytes[index] == b'$'
                && !Self::is_escaped(bytes, index)
                && Self::delimiter_length(bytes, index) == delimiter_length
            {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn delimiter_length(bytes: &[u8], index: usize) -> usize {
        if bytes.get(index + 1) == Some(&b'$') {
            return 2;
        }
        1
    }

    fn is_escaped(bytes: &[u8], index: usize) -> bool {
        if index == 0 {
            return false;
        }
        let mut backslash_count = 0;
        let mut cursor = index - 1;
        loop {
            if bytes[cursor] != b'\\' {
                break;
            }
            backslash_count += 1;
            if cursor == 0 {
                break;
            }
            cursor -= 1;
        }
        backslash_count % 2 == 1
    }
}
