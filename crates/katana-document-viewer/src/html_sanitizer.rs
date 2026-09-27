pub struct HtmlFragmentNormalizer;

impl HtmlFragmentNormalizer {
    pub fn normalize(fragment: &str) -> String {
        if Self::has_malformed_image_source_attribute(fragment) {
            return Self::normalize_svg_namespace(fragment).unwrap_or_else(|| fragment.to_string());
        }
        fragment.to_string()
    }

    pub fn has_malformed_image_source_attribute(fragment: &str) -> bool {
        Self::malformed_image_source_attribute_tail(fragment).is_some()
    }

    pub fn malformed_image_source_attribute_tail(fragment: &str) -> Option<&str> {
        let bytes = fragment.as_bytes();
        let mut tag_start = 0;
        while let Some(start) = fragment[tag_start..].find('<').map(|i| tag_start + i) {
            if fragment[start..].starts_with("<!--") {
                tag_start = fragment[start + 4..].find("-->").map(|c| start + c + 7)?;
                continue;
            }

            let name_start = Self::skip_ascii_whitespace(bytes, start + 1);
            let index = Self::consume_tag_name(bytes, name_start);
            if fragment[name_start..index].eq_ignore_ascii_case("script") {
                tag_start = index
                    + fragment[index..].to_ascii_lowercase().find("</script")?
                    + "</script".len();
                continue;
            }
            if !fragment[name_start..index].eq_ignore_ascii_case("img") {
                tag_start = Self::quoted_tag_end(bytes, index).map_or(start + 1, |end| end + 1);
                continue;
            }
            if let Some(tail) = Self::image_source_attribute_tail(fragment, index) {
                return Some(tail);
            }
            tag_start = Self::quoted_tag_end(bytes, index).map_or(start + 1, |end| end + 1);
        }
        None
    }

    fn image_source_attribute_tail(fragment: &str, mut index: usize) -> Option<&str> {
        let bytes = fragment.as_bytes();
        loop {
            index = Self::skip_ascii_whitespace(bytes, index);
            if index == bytes.len() || bytes[index] == b'>' {
                return None;
            }
            let attribute_start = index;
            index = Self::consume_attribute_name(bytes, index);
            if attribute_start == index {
                index += 1;
                continue;
            }
            let (next_index, malformed_tail) =
                Self::image_source_attribute_tail_state(fragment, bytes, attribute_start, index);
            if let Some(tail) = malformed_tail {
                return Some(tail);
            }
            index = next_index;
        }
    }

    fn image_source_attribute_tail_state<'a>(
        fragment: &'a str,
        bytes: &'a [u8],
        attribute_start: usize,
        mut index: usize,
    ) -> (usize, Option<&'a str>) {
        let attribute = &fragment[attribute_start..index];
        index = Self::skip_ascii_whitespace(bytes, index);
        if index == bytes.len() || bytes[index] != b'=' {
            return (index, None);
        }
        index = Self::skip_ascii_whitespace(bytes, index + 1);
        let Some((value_start, value_end)) = Self::quoted_attribute_value_range(bytes, index)
        else {
            return (index, None);
        };
        (
            value_end.saturating_add(1),
            attribute
                .eq_ignore_ascii_case("src")
                .then(|| Self::malformed_tail(fragment, bytes, value_start, value_end))
                .flatten(),
        )
    }

    fn consume_tag_name(bytes: &[u8], mut index: usize) -> usize {
        while index < bytes.len() && bytes[index].is_ascii_alphanumeric() {
            index += 1;
        }
        index
    }

    fn consume_attribute_name(bytes: &[u8], mut index: usize) -> usize {
        while index < bytes.len()
            && (bytes[index].is_ascii_alphanumeric() || matches!(bytes[index], b'-' | b'_' | b':'))
        {
            index += 1;
        }
        index
    }

    fn quoted_attribute_value_range(bytes: &[u8], index: usize) -> Option<(usize, usize)> {
        if index == bytes.len() || !matches!(bytes[index], b'\'' | b'"') {
            return None;
        }
        let quote = bytes[index];
        let value_start = index + 1;
        let value_end = bytes[value_start..]
            .iter()
            .position(|byte| *byte == quote)
            .map(|offset| value_start + offset)
            .unwrap_or(bytes.len());
        Some((value_start, value_end))
    }

    fn malformed_tail<'a>(
        fragment: &'a str,
        bytes: &'a [u8],
        value_start: usize,
        value_end: usize,
    ) -> Option<&'a str> {
        let relative_angle = bytes[value_start..value_end]
            .iter()
            .position(|byte| *byte == b'<')?;
        let malformed_start = value_start + relative_angle;
        let tag_end = bytes[malformed_start..]
            .iter()
            .position(|byte| *byte == b'>')?;
        Some(&fragment[malformed_start + tag_end + 1..])
    }

    fn skip_ascii_whitespace(bytes: &[u8], mut index: usize) -> usize {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        index
    }

    fn quoted_tag_end(bytes: &[u8], mut index: usize) -> Option<usize> {
        let mut quote = None;
        while index < bytes.len() {
            match (quote, bytes[index]) {
                (None, b'\'' | b'"') => quote = Some(bytes[index]),
                (Some(active), byte) if byte == active => quote = None,
                (None, b'>') => return Some(index),
                _ => {}
            }
            index += 1;
        }
        None
    }

    fn normalize_svg_namespace(fragment: &str) -> Option<String> {
        let data_start = fragment.find("data:image/svg+xml")?;
        let namespace_start = fragment[data_start..].find("xmlns=%22<")? + data_start;
        let value_start = namespace_start + "xmlns=%22<".len();
        let namespace_end = fragment[value_start..].find("%22>")? + value_start;
        let namespace = &fragment[value_start..namespace_end];
        if namespace.is_empty() || namespace.chars().any(char::is_whitespace) {
            return None;
        }
        let encoded_namespace = Self::percent_encode_uri_value(namespace);
        let suffix_start = namespace_end + "%22>".len();
        Some(format!(
            "{}xmlns=%22{encoded_namespace}%22%20{}",
            &fragment[..namespace_start],
            &fragment[suffix_start..]
        ))
    }

    fn percent_encode_uri_value(value: &str) -> String {
        let mut encoded = String::with_capacity(value.len());
        for byte in value.bytes() {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b'~') {
                encoded.push(byte as char);
            } else {
                encoded.push_str(&format!("%{byte:02X}"));
            }
        }
        encoded
    }
}

#[cfg(test)]
#[path = "html_sanitizer_tests.rs"]
mod tests;
