const LIGHT_DARK_FUNCTION: &str = "light-dark(";
const SVG_TAG_PREFIX_LEN: usize = 4;

pub(super) fn preprocess_for_rasterizer(svg_text: &str, root_font_size: Option<f32>) -> String {
    let with_xml_entities = svg_text.replace("&nbsp;", "&#160;");
    let without_foreign_objects = strip_foreign_objects(&with_xml_entities);
    let with_font_context = root_font_size
        .and_then(|font_size| apply_root_font_size_css_unit(&without_foreign_objects, font_size))
        .unwrap_or(without_foreign_objects);
    resolve_light_dark_functions(&with_font_context)
}

fn apply_root_font_size_css_unit(svg_text: &str, root_font_size: f32) -> Option<String> {
    let root_start = svg_text.to_ascii_lowercase().find("<svg")?;
    let root_end = locate_root_svg_tag_end(&svg_text[root_start..])? + root_start;
    let root_tag = &svg_text[root_start..=root_end];
    let Some((style_start, style_end)) = style_attr_range(root_tag) else {
        return Some(add_root_style_attribute(svg_text, root_font_size, root_end));
    };
    let style = &root_tag[style_start..style_end];
    let style_key = "font-size";
    if style.to_ascii_lowercase().contains(style_key) {
        return Some(svg_text.to_string());
    }
    let style_suffix = if style.ends_with(';') || style.is_empty() {
        String::new()
    } else {
        "; ".to_string()
    };
    let style_attr = format!("{style}{style_suffix}font-size:{}px;", root_font_size);
    let mut updated_tag = String::with_capacity(root_tag.len() + style_attr.len());
    updated_tag.push_str(&root_tag[..style_start]);
    updated_tag.push_str(&style_attr);
    updated_tag.push_str(&root_tag[style_end..]);
    let mut output = String::with_capacity(svg_text.len() + style_attr.len());
    output.push_str(&svg_text[..root_start]);
    output.push_str(&updated_tag);
    output.push_str(&svg_text[root_end + 1..]);
    Some(output)
}

fn locate_root_svg_tag_end(svg_fragment: &str) -> Option<usize> {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    for (index, byte) in svg_fragment
        .as_bytes()
        .iter()
        .enumerate()
        .skip(SVG_TAG_PREFIX_LEN)
    {
        match byte {
            b'\'' if !in_double_quote => in_single_quote = !in_single_quote,
            b'"' if !in_single_quote => in_double_quote = !in_double_quote,
            b'>' if !in_single_quote && !in_double_quote => return Some(index),
            _ => {}
        }
    }
    None
}

fn style_attr_range(svg_tag: &str) -> Option<(usize, usize)> {
    let lower = svg_tag.to_ascii_lowercase();
    let style_key = "style=";
    let start = lower.find(style_key)?;
    let value_start = start + style_key.len();
    let quote = *svg_tag.as_bytes().get(value_start)?;
    let quote_char = match quote {
        b'"' | b'\'' => quote,
        _ => return None,
    };
    let style_end = svg_tag
        .as_bytes()
        .get(value_start + 1..)?
        .iter()
        .position(|byte| *byte == quote_char)?;
    Some((value_start + 1, value_start + 1 + style_end))
}

fn add_root_style_attribute(svg_text: &str, root_font_size: f32, root_end: usize) -> String {
    let style_attr = format!(" style=\"font-size:{}px;\"", root_font_size);
    let mut output = String::with_capacity(svg_text.len() + style_attr.len());
    output.push_str(&svg_text[..root_end]);
    output.push_str(&style_attr);
    output.push_str(&svg_text[root_end..]);
    output
}

fn strip_foreign_objects(svg_text: &str) -> String {
    let mut output = String::with_capacity(svg_text.len());
    let mut remaining = svg_text;
    while let Some(start) = remaining.to_ascii_lowercase().find("<foreignobject") {
        output.push_str(&remaining[..start]);
        let after_open = &remaining[start..];
        let lower_after_open = after_open.to_ascii_lowercase();
        if let Some(self_close) = lower_after_open.find("/>") {
            remaining = &after_open[self_close + "/>".len()..];
            continue;
        }
        let Some(close) = lower_after_open.find("</foreignobject>") else {
            output.push_str(after_open);
            return output;
        };
        remaining = &after_open[close + "</foreignobject>".len()..];
    }
    output.push_str(remaining);
    output
}

fn resolve_light_dark_functions(svg_text: &str) -> String {
    let mut result = String::with_capacity(svg_text.len());
    let mut remaining = svg_text;
    while let Some(start) = find_light_dark_function(remaining) {
        let content_start = start + LIGHT_DARK_FUNCTION.len();
        result.push_str(&remaining[..start]);
        let Some((content_end, light_color)) =
            parse_light_dark_function(&remaining[content_start..])
        else {
            result.push_str(&remaining[start..content_start]);
            remaining = &remaining[content_start..];
            continue;
        };
        result.push_str(light_color.trim());
        remaining = &remaining[content_start + content_end + 1..];
    }
    result.push_str(remaining);
    result
}

fn find_light_dark_function(text: &str) -> Option<usize> {
    text.to_ascii_lowercase().find(LIGHT_DARK_FUNCTION)
}

fn parse_light_dark_function(content: &str) -> Option<(usize, &str)> {
    let mut depth = 0usize;
    let mut comma = None;
    for (index, character) in content.char_indices() {
        match character {
            '(' => depth += 1,
            ')' if depth == 0 => return comma.map(|comma_index| (index, &content[..comma_index])),
            ')' => depth -= 1,
            ',' if depth == 0 && comma.is_none() => comma = Some(index),
            _ => {}
        }
    }
    None
}

#[cfg(test)]
#[path = "export_surface_svg_preprocess_tests.rs"]
mod tests;
