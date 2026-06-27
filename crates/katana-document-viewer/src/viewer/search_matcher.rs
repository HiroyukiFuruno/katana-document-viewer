use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerSearchTextMatch {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

pub struct ViewerSearchTextMatcher;

impl ViewerSearchTextMatcher {
    pub fn find(query: &str, text: &str) -> Vec<ViewerSearchTextMatch> {
        if query.is_empty() {
            return Vec::new();
        }
        let query_chars = Self::folded_chars(query);
        let text_chars = text.char_indices().collect::<Vec<_>>();
        let mut matches = Vec::new();
        let mut index = 0;
        while index < text_chars.len() {
            if Self::matches_at(&text_chars, index, &query_chars) {
                let start = text_chars[index].0;
                let end = Self::end_byte(&text_chars, index + query_chars.len(), text.len());
                matches.push(ViewerSearchTextMatch {
                    start,
                    end,
                    text: text[start..end].to_string(),
                });
                index += query_chars.len();
                continue;
            }
            index += 1;
        }
        matches
    }

    fn matches_at(text_chars: &[(usize, char)], index: usize, query_chars: &[String]) -> bool {
        if index + query_chars.len() > text_chars.len() {
            return false;
        }
        text_chars[index..index + query_chars.len()]
            .iter()
            .zip(query_chars)
            .all(|((_, value), query)| Self::folded_char(*value) == *query)
    }

    fn folded_chars(value: &str) -> Vec<String> {
        value.chars().map(Self::folded_char).collect()
    }

    fn folded_char(value: char) -> String {
        value.to_lowercase().collect()
    }

    fn end_byte(text_chars: &[(usize, char)], next_index: usize, text_len: usize) -> usize {
        text_chars
            .get(next_index)
            .map(|(byte_index, _)| *byte_index)
            .unwrap_or(text_len)
    }
}

#[cfg(test)]
mod tests {
    use super::ViewerSearchTextMatcher;

    #[test]
    fn finds_ascii_matches_case_insensitively() {
        let matches = ViewerSearchTextMatcher::find("hello", "Hello HELLO hello");

        assert_eq!(3, matches.len());
        assert_eq!("Hello", matches[0].text);
        assert_eq!(0, matches[0].start);
        assert_eq!(5, matches[0].end);
    }

    #[test]
    fn keeps_multibyte_safe_byte_offsets() {
        let matches = ViewerSearchTextMatcher::find("search", "あいうSearch");

        assert_eq!(1, matches.len());
        assert_eq!("Search", matches[0].text);
        assert_eq!("あいう".len(), matches[0].start);
        assert_eq!("あいうSearch".len(), matches[0].end);
    }

    #[test]
    fn returns_empty_for_query_longer_than_text() {
        assert!(ViewerSearchTextMatcher::find("longquery", "tiny").is_empty());
    }

    #[test]
    fn matches_unicode_case_folding_for_length_variants() {
        let matches = ViewerSearchTextMatcher::find("ẞ", "ß");

        assert_eq!(1, matches.len());
        assert_eq!("ß", matches[0].text);
        assert_eq!(0, matches[0].start);
        assert_eq!("ß".len(), matches[0].end);
    }
}
