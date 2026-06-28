pub(crate) struct EmojiTextSegments;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EmojiTextSegment<'a> {
    pub(crate) text: &'a str,
    pub(crate) emoji: bool,
}

impl EmojiTextSegments {
    pub(crate) fn split(text: &str) -> Vec<EmojiTextSegment<'_>> {
        if text.is_empty() {
            return Vec::new();
        }
        let mut segments = Vec::new();
        let mut start = 0usize;
        let mut current = None;
        for (index, character) in text.char_indices() {
            let emoji = is_emoji_part(character);
            if current.is_none() {
                current = Some(emoji);
                continue;
            }
            if current == Some(emoji) {
                continue;
            }
            segments.push(EmojiTextSegment {
                text: &text[start..index],
                emoji: current.unwrap_or(false),
            });
            start = index;
            current = Some(emoji);
        }
        segments.push(EmojiTextSegment {
            text: &text[start..],
            emoji: current.unwrap_or(false),
        });
        segments
    }
}

fn is_emoji_part(character: char) -> bool {
    is_emoji_code(character as u32) || is_emoji_range(character as u32)
}

fn is_emoji_code(code: u32) -> bool {
    matches!(
        code,
        0x00A9
            | 0x00AE
            | 0x200D
            | 0x203C
            | 0x2049
            | 0x20E3
            | 0x2122
            | 0x2139
            | 0x23CF
            | 0x24C2
            | 0x25B6
            | 0x25C0
            | 0x3030
            | 0x303D
            | 0x3297
            | 0x3299
            | 0xFE0F
    )
}

fn is_emoji_range(code: u32) -> bool {
    matches!(
        code,
        0x2194..=0x21AA
            | 0x231A..=0x2328
            | 0x23E9..=0x23FA
            | 0x25AA..=0x25AB
            | 0x25FB..=0x25FE
            | 0x2600..=0x27BF
            | 0x2934..=0x2935
            | 0x2B05..=0x2B55
            | 0x1F000..=0x1FAFF
    )
}

#[cfg(test)]
mod tests {
    use super::EmojiTextSegments;

    #[test]
    fn split_marks_raw_emoji_runs_without_marking_surrounding_text() {
        let segments = EmojiTextSegments::split("Emoji: 🦀 text ⚠️");

        assert_eq!(
            segments
                .iter()
                .map(|segment| (segment.text, segment.emoji))
                .collect::<Vec<_>>(),
            vec![
                ("Emoji: ", false),
                ("🦀", true),
                (" text ", false),
                ("⚠️", true)
            ]
        );
    }
}
