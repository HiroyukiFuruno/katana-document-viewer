use super::fence::FenceNormalizer;

pub(super) struct RawDiagramNormalizer;

impl RawDiagramNormalizer {
    pub(super) fn wrap(source: &str) -> String {
        let mut normalized = String::with_capacity(source.len());
        let mut offset = 0;
        while let Some((pos, marker)) = Self::find_next_marker(&source[offset..]) {
            let absolute = offset + pos;
            normalized.push_str(&source[offset..absolute]);
            let remaining = &source[absolute..];
            if marker == "```" {
                let Some(block) = FenceNormalizer::fence_block(remaining) else {
                    normalized.push_str(remaining);
                    return normalized;
                };
                normalized.push_str(&remaining[..block.close_end]);
                offset = absolute + block.close_end;
                continue;
            }
            offset = Self::push_raw_diagram(&mut normalized, source, absolute, marker);
        }
        normalized.push_str(&source[offset..]);
        normalized
    }

    fn find_next_marker(source: &str) -> Option<(usize, &'static str)> {
        ["```", "<mxGraphModel", "<mxfile", "@startuml"]
            .iter()
            .filter_map(|marker| {
                FenceNormalizer::find_line_marker(source, marker).map(|pos| (pos, *marker))
            })
            .min_by_key(|(pos, _)| *pos)
    }

    fn push_raw_diagram(
        normalized: &mut String,
        source: &str,
        absolute: usize,
        marker: &str,
    ) -> usize {
        let remaining = &source[absolute..];
        let Some((language, end_tag)) = Self::raw_diagram_spec(marker) else {
            normalized.push_str(marker);
            return absolute + marker.len();
        };
        let Some(end_pos) = remaining.find(end_tag) else {
            normalized.push_str(marker);
            return absolute + marker.len();
        };
        let source_end = end_pos + end_tag.len();
        normalized.push_str("```");
        normalized.push_str(language);
        normalized.push('\n');
        normalized.push_str(&remaining[..source_end]);
        normalized.push_str("\n```\n");
        absolute + source_end + FenceNormalizer::line_ending_len(&remaining[source_end..])
    }

    fn raw_diagram_spec(marker: &str) -> Option<(&'static str, &'static str)> {
        match marker {
            "<mxfile" => Some(("drawio", "</mxfile>")),
            "<mxGraphModel" => Some(("drawio", "</mxGraphModel>")),
            "@startuml" => Some(("plantuml", "@enduml")),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RawDiagramNormalizer;

    #[test]
    fn unknown_marker_is_written_as_text() {
        let mut normalized = String::new();
        let next = RawDiagramNormalizer::push_raw_diagram(
            &mut normalized,
            "<svg>content</svg>",
            0,
            "<svg",
        );
        assert_eq!(next, "<svg".len());
        assert_eq!(normalized, "<svg");
    }

    #[test]
    fn truncated_marker_without_end_tag_keeps_source_and_consumes_marker() {
        let mut normalized = String::new();
        let next = RawDiagramNormalizer::push_raw_diagram(
            &mut normalized,
            "<mxfile><diagram/></missing",
            0,
            "<mxfile",
        );
        assert_eq!(next, "<mxfile".len());
        assert_eq!(normalized, "<mxfile");
    }
}
