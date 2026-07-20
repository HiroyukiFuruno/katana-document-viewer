#[path = "export_surface_svg_preprocess_foreign_object_label.rs"]
mod label;

pub(super) fn replace_with_text_fallbacks(svg_text: &str) -> String {
    let mut output = String::with_capacity(svg_text.len());
    let mut cursor = 0usize;
    let lower = svg_text.to_ascii_lowercase();
    while let Some(relative_start) = lower[cursor..].find("<foreignobject") {
        let start = cursor + relative_start;
        output.push_str(&svg_text[cursor..start]);
        let Some(end) = foreign_object_end(&lower, start) else {
            output.push_str(&svg_text[start..]);
            return output;
        };
        let fragment = &svg_text[start..end];
        if !has_svg_text_fallback_after(svg_text, start, end)
            && let Some(text) = label::to_svg_text(fragment)
        {
            output.push_str(&text);
        }
        cursor = end;
    }
    output.push_str(&svg_text[cursor..]);
    output
}

fn foreign_object_end(lower_svg: &str, start: usize) -> Option<usize> {
    let after_open = &lower_svg[start..];
    let tag_end = after_open.find('>')?;
    if after_open[..=tag_end].trim_end().ends_with("/>") {
        return Some(start + tag_end + 1);
    }
    let close = after_open.find("</foreignobject>")?;
    Some(start + close + "</foreignobject>".len())
}

fn has_svg_text_fallback_after(
    svg_text: &str,
    foreign_object_start: usize,
    foreign_object_end: usize,
) -> bool {
    let before = &svg_text[..foreign_object_start];
    let Some(switch_start) = before.rfind("<switch") else {
        return false;
    };
    if let Some(end) = before.rfind("</switch>")
        && end > switch_start
    {
        return false;
    }
    let after = &svg_text[foreign_object_end..];
    let Some(switch_end) = after.find("</switch>") else {
        return false;
    };
    let switch_tail = &after[..switch_end];
    let Some(text_start) = switch_tail.find("<text") else {
        return false;
    };
    match switch_tail.find("<foreignObject") {
        Some(next_foreign_object) => text_start < next_foreign_object,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_foreign_object_bounds(raw: &str) -> (usize, usize) {
        let lower = raw.to_ascii_lowercase();
        let starts = lower
            .match_indices("<foreignobject")
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let ends = lower
            .match_indices("</foreignobject>")
            .map(|(index, _)| index + "</foreignobject>".len())
            .collect::<Vec<_>>();
        assert!(!starts.is_empty());
        assert!(!ends.is_empty());
        (starts[0], ends[0])
    }

    #[test]
    fn keeps_unclosed_foreign_object_for_visibility() {
        let raw = "<svg><foreignObject><body>visible fallback</body>";
        let processed = replace_with_text_fallbacks(raw);

        assert!(processed.contains("foreignObject"));
        assert!(processed.contains("visible fallback"));
    }

    #[test]
    fn replaces_convertible_foreign_object_without_switch_fallback() {
        let raw = "<svg><foreignObject x=\"1\" y=\"1\" width=\"1\" height=\"1\"><div>Hello</div></foreignObject><rect /></svg>";
        let processed = replace_with_text_fallbacks(raw);

        assert!(!processed.contains("foreignObject"));
        assert!(processed.contains(">Hello<"));
        assert!(processed.contains("<rect />"));
    }

    #[test]
    fn keeps_foreign_object_when_switch_has_svg_fallback() {
        let raw = "<svg><switch><foreignObject><div>ignore</div></foreignObject><text>Fallback</text></switch><rect/></svg>";
        let (start, end) = first_foreign_object_bounds(raw);
        assert!(has_svg_text_fallback_after(raw, start, end));
    }

    #[test]
    fn detects_svg_text_fallback_after_foreign_object() {
        let raw = "<svg><foreignObject><div>hello</div></foreignObject><text>Fallback</text></svg>";
        let (start, foreign_end) = first_foreign_object_bounds(raw);
        assert!(!has_svg_text_fallback_after(raw, start, foreign_end));
    }

    #[test]
    fn detects_text_before_a_later_foreign_object_in_switch() {
        let raw = "<svg><switch><foreignObject><div>first</div></foreignObject><text>Fallback</text><foreignObject><div>second</div></foreignObject></switch></svg>";
        let (start, foreign_end) = first_foreign_object_bounds(raw);

        assert!(has_svg_text_fallback_after(raw, start, foreign_end));
    }

    #[test]
    fn does_not_report_fallback_without_switch_context() {
        let raw = "<svg><foreignObject><div>hello</div></foreignObject></svg>";
        let (start, foreign_end) = first_foreign_object_bounds(raw);
        assert!(!has_svg_text_fallback_after(raw, start, foreign_end));
    }

    #[test]
    fn ignores_fallback_check_when_switch_already_closed() {
        let raw = "<svg><switch><text>fallback</text></switch><foreignObject><div>ignore</div></foreignObject></svg>";
        let (start, end) = first_foreign_object_bounds(raw);
        assert!(!has_svg_text_fallback_after(raw, start, end));
    }

    #[test]
    fn reports_no_fallback_when_switch_end_is_missing() {
        let raw = "<svg><switch><foreignObject><div>broken</div></foreignObject></svg>";
        let (start, end) = first_foreign_object_bounds(raw);
        assert!(!has_svg_text_fallback_after(raw, start, end));
    }

    #[test]
    fn reports_no_fallback_when_switch_lacks_text_before_foreignobject() {
        let raw =
            "<svg><switch><foreignObject><div>no text</div></foreignObject><rect/></switch></svg>";
        let (start, end) = first_foreign_object_bounds(raw);
        assert!(!has_svg_text_fallback_after(raw, start, end));
    }

    #[test]
    fn foreign_object_end_returns_none_for_malformed_tag() {
        let lower = "<svg><foreignObject><div>broken";
        assert_eq!(foreign_object_end(lower, 0), None);
    }
}
