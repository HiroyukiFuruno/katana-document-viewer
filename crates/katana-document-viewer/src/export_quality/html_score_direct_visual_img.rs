pub(super) struct HtmlImgSourceQuality;

impl HtmlImgSourceQuality {
    pub(super) fn has_uri(html: &str, uri: &str) -> bool {
        let lower = html.to_ascii_lowercase();
        let mut cursor = 0;
        while let Some(offset) = lower[cursor..].find("<img") {
            let start = cursor + offset;
            let Some(end_offset) = lower[start..].find('>') else {
                return false;
            };
            let end = start + end_offset + 1;
            if Self::tag_has_uri(&html[start..end], uri) {
                return true;
            }
            cursor = end;
        }
        false
    }

    fn tag_has_uri(tag: &str, uri: &str) -> bool {
        Self::attribute_matches(tag, "src", uri) || Self::srcset_matches(tag, uri)
    }

    fn srcset_matches(tag: &str, uri: &str) -> bool {
        let Some(value) = HtmlAttributeScanner::new(tag).value("srcset") else {
            return false;
        };
        value.split(',').any(|candidate| {
            let candidate = candidate.trim();
            candidate == uri
                || candidate
                    .strip_prefix(uri)
                    .is_some_and(|tail| tail.chars().next().is_some_and(char::is_whitespace))
        })
    }

    fn attribute_matches(tag: &str, name: &str, expected: &str) -> bool {
        HtmlAttributeScanner::new(tag)
            .value(name)
            .is_some_and(|value| value == expected)
    }
}

struct HtmlAttributeScanner<'a> {
    tag: &'a str,
}

impl<'a> HtmlAttributeScanner<'a> {
    fn new(tag: &'a str) -> Self {
        Self { tag }
    }

    fn value(&self, expected: &str) -> Option<&'a str> {
        let bytes = self.tag.as_bytes();
        let mut index = 0;
        while index < bytes.len() {
            let parsed = self.next_attribute(bytes, index)?;
            if parsed.name.eq_ignore_ascii_case(expected) {
                return Some(parsed.value);
            }
            index = parsed.end;
        }
        None
    }

    fn next_attribute(&self, bytes: &[u8], index: usize) -> Option<HtmlAttribute<'a>> {
        let name_start = Self::skip_to_name(bytes, index);
        let name_end = Self::skip_name(bytes, name_start);
        if name_start == name_end {
            return None;
        }
        let equals = Self::skip_spaces(bytes, name_end);
        if equals >= bytes.len() || bytes[equals] != b'=' {
            return self.next_attribute(bytes, name_end);
        }
        let value_start = Self::skip_spaces(bytes, equals + 1);
        let value = self.attribute_value(bytes, value_start);
        Some(HtmlAttribute {
            name: &self.tag[name_start..name_end],
            value: value.value,
            end: value.end,
        })
    }

    fn attribute_value(&self, bytes: &[u8], index: usize) -> HtmlAttributeValue<'a> {
        if index >= bytes.len() {
            return HtmlAttributeValue::new("", bytes.len());
        }
        if bytes[index] == b'"' || bytes[index] == b'\'' {
            return self.quoted_attribute_value(bytes, index);
        }
        self.unquoted_attribute_value(bytes, index)
    }

    fn quoted_attribute_value(&self, bytes: &[u8], index: usize) -> HtmlAttributeValue<'a> {
        let quote = bytes[index];
        let start = index + 1;
        let end = bytes[start..]
            .iter()
            .position(|byte| *byte == quote)
            .map(|offset| start + offset)
            .unwrap_or(bytes.len());
        HtmlAttributeValue::new(
            &self.tag[start..end],
            end.saturating_add(1).min(bytes.len()),
        )
    }

    fn unquoted_attribute_value(&self, bytes: &[u8], index: usize) -> HtmlAttributeValue<'a> {
        let end = bytes[index..]
            .iter()
            .position(|byte| byte.is_ascii_whitespace() || *byte == b'>')
            .map(|offset| index + offset)
            .unwrap_or(bytes.len());
        HtmlAttributeValue::new(&self.tag[index..end], end)
    }

    fn skip_to_name(bytes: &[u8], mut index: usize) -> usize {
        while index < bytes.len() && !Self::is_name_byte(bytes[index]) {
            index += 1;
        }
        index
    }

    fn skip_name(bytes: &[u8], mut index: usize) -> usize {
        while index < bytes.len() && Self::is_name_byte(bytes[index]) {
            index += 1;
        }
        index
    }

    fn skip_spaces(bytes: &[u8], mut index: usize) -> usize {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        index
    }

    fn is_name_byte(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':')
    }
}

struct HtmlAttribute<'a> {
    name: &'a str,
    value: &'a str,
    end: usize,
}

struct HtmlAttributeValue<'a> {
    value: &'a str,
    end: usize,
}

impl<'a> HtmlAttributeValue<'a> {
    fn new(value: &'a str, end: usize) -> Self {
        Self { value, end }
    }
}

#[cfg(test)]
#[path = "html_score_direct_visual_img_tests.rs"]
mod tests;
