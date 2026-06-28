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
    if before
        .rfind("</switch>")
        .is_some_and(|end| end > switch_start)
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
    switch_tail
        .find("<foreignObject")
        .is_none_or(|next_foreign_object| text_start < next_foreign_object)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_unclosed_foreign_object_for_visibility() {
        let raw = "<svg><foreignObject><body>visible fallback</body>";
        let processed = replace_with_text_fallbacks(raw);

        assert!(processed.contains("foreignObject"));
        assert!(processed.contains("visible fallback"));
    }
}
