const DEFAULT_FOREIGN_OBJECT_FONT_SIZE: f32 = 14.0;
const FOREIGN_OBJECT_BASELINE_OFFSET_RATIO: f32 = 0.35;

pub(super) fn to_svg_text(fragment: &str) -> Option<String> {
    let element = xmltree::Element::parse(fragment.as_bytes()).ok()?;
    ForeignObjectText::from_element(&element).map(|text| text.to_svg())
}

struct ForeignObjectText {
    label: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    font_size: f32,
    fill: String,
    text_anchor: &'static str,
}

impl ForeignObjectText {
    fn from_element(element: &xmltree::Element) -> Option<Self> {
        let label = normalized_text_content(element);
        if label.is_empty() {
            return None;
        }
        Some(Self {
            label,
            x: parse_number_attr(element, "x").unwrap_or(0.0),
            y: parse_number_attr(element, "y").unwrap_or(0.0),
            width: parse_number_attr(element, "width").unwrap_or(0.0),
            height: parse_number_attr(element, "height").unwrap_or(0.0),
            font_size: foreign_object_font_size(element),
            fill: foreign_object_fill(element),
            text_anchor: resolve_text_anchor(element),
        })
    }

    fn text_x(&self) -> f32 {
        match self.text_anchor {
            "middle" => self.x + (self.width / 2.0),
            "end" => self.x + self.width,
            _ => self.x,
        }
    }

    fn text_y(&self) -> f32 {
        if self.height > 0.0 {
            return self.y
                + (self.height / 2.0)
                + (self.font_size * FOREIGN_OBJECT_BASELINE_OFFSET_RATIO);
        }
        self.y + self.font_size
    }

    fn to_svg(&self) -> String {
        format!(
            r#"<text x="{x}" y="{y}" fill="{fill}" font-size="{font_size}" text-anchor="{anchor}">{label}</text>"#,
            x = format_svg_number(self.text_x()),
            y = format_svg_number(self.text_y()),
            fill = escape_xml_attr(&self.fill),
            font_size = format_svg_number(self.font_size),
            anchor = self.text_anchor,
            label = escape_xml_text(&self.label),
        )
    }
}

fn foreign_object_font_size(element: &xmltree::Element) -> f32 {
    find_style_property(element, "font-size")
        .as_deref()
        .and_then(parse_leading_number)
        .unwrap_or(DEFAULT_FOREIGN_OBJECT_FONT_SIZE)
}

fn foreign_object_fill(element: &xmltree::Element) -> String {
    find_style_property(element, "color")
        .or_else(|| find_style_property(element, "fill"))
        .or_else(|| find_attr_recursive(element, "fill").map(ToString::to_string))
        .unwrap_or_else(|| "currentColor".to_string())
}

fn normalized_text_content(element: &xmltree::Element) -> String {
    let mut text = String::new();
    collect_text_content(element, &mut text);
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn collect_text_content(element: &xmltree::Element, output: &mut String) {
    for child in &element.children {
        match child {
            xmltree::XMLNode::Text(text) | xmltree::XMLNode::CData(text) => {
                output.push(' ');
                output.push_str(text);
            }
            xmltree::XMLNode::Element(child_element) => {
                if child_element.name.eq_ignore_ascii_case("br") {
                    output.push(' ');
                }
                collect_text_content(child_element, output);
            }
            _ => {}
        }
    }
}

fn parse_number_attr(element: &xmltree::Element, name: &str) -> Option<f32> {
    element
        .attributes
        .get(name)
        .and_then(|value| parse_leading_number(value))
}

fn find_style_property(element: &xmltree::Element, property: &str) -> Option<String> {
    element
        .attributes
        .get("style")
        .and_then(|style| style_property(style, property))
        .or_else(|| {
            element.children.iter().find_map(|child| match child {
                xmltree::XMLNode::Element(child_element) => {
                    find_style_property(child_element, property)
                }
                _ => None,
            })
        })
}

fn style_property(style: &str, property: &str) -> Option<String> {
    style.split(';').find_map(|declaration| {
        let (name, value) = declaration.split_once(':')?;
        name.trim()
            .eq_ignore_ascii_case(property)
            .then(|| value.trim().to_string())
    })
}

fn find_attr_recursive<'a>(element: &'a xmltree::Element, name: &str) -> Option<&'a str> {
    element
        .attributes
        .get(name)
        .map(String::as_str)
        .or_else(|| {
            element.children.iter().find_map(|child| match child {
                xmltree::XMLNode::Element(child_element) => {
                    find_attr_recursive(child_element, name)
                }
                _ => None,
            })
        })
}

fn resolve_text_anchor(element: &xmltree::Element) -> &'static str {
    find_style_property(element, "text-align")
        .as_deref()
        .map(str::trim)
        .map(|align| {
            if align.eq_ignore_ascii_case("right") {
                "end"
            } else if align.eq_ignore_ascii_case("left") {
                "start"
            } else {
                "middle"
            }
        })
        .unwrap_or("middle")
}

fn parse_leading_number(value: &str) -> Option<f32> {
    let number_end = value
        .char_indices()
        .find_map(|(index, character)| {
            (!matches!(character, '0'..='9' | '.' | '-' | '+')).then_some(index)
        })
        .unwrap_or(value.len());
    value[..number_end].trim().parse::<f32>().ok()
}

fn format_svg_number(value: f32) -> String {
    if value.fract().abs() < f32::EPSILON {
        return format!("{value:.0}");
    }
    format!("{value:.3}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

fn escape_xml_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_xml_attr(text: &str) -> String {
    escape_xml_text(text).replace('"', "&quot;")
}

#[cfg(test)]
#[path = "export_surface_svg_preprocess_foreign_object_label_tests.rs"]
mod tests;
