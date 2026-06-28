pub(super) struct RenderedSvgHtmlQuality;

impl RenderedSvgHtmlQuality {
    pub(super) fn has_rendered_svg(html: &str) -> bool {
        let lower = html.to_ascii_lowercase();
        let mut cursor = 0;
        while let Some(offset) = lower[cursor..].find("<svg") {
            let start = cursor + offset;
            let Some(next_cursor) = Self::next_svg_cursor(html, &lower, start) else {
                return false;
            };
            if Self::svg_at_has_visual_body(html, &lower, start) {
                return true;
            }
            cursor = next_cursor;
        }
        false
    }

    fn svg_at_has_visual_body(html: &str, lower: &str, start: usize) -> bool {
        let Some(open_end_delta) = html[start..].find('>') else {
            return false;
        };
        let body_start = start + open_end_delta + 1;
        let Some(close_delta) = lower[body_start..].find("</svg>") else {
            return false;
        };
        let body = &lower[body_start..body_start + close_delta];
        Self::body_has_visual_element(body)
    }

    fn next_svg_cursor(html: &str, lower: &str, start: usize) -> Option<usize> {
        let open_end_delta = html[start..].find('>')?;
        let body_start = start + open_end_delta + 1;
        Some(match lower[body_start..].find("</svg>") {
            Some(close_delta) => body_start + close_delta + "</svg>".len(),
            None => body_start,
        })
    }

    fn body_has_visual_element(body: &str) -> bool {
        [
            "<path",
            "<rect",
            "<circle",
            "<ellipse",
            "<line",
            "<polyline",
            "<polygon",
            "<text",
            "<image",
            "<foreignobject",
        ]
        .iter()
        .any(|element| body.contains(element))
    }
}

#[cfg(test)]
#[path = "html_score_svg_media_tests.rs"]
mod tests;
