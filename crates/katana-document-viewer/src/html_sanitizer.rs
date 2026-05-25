const BROKEN_SVG_XMLNS: &str = "%22<http://www.w3.org/2000/svg%22>";
const ENCODED_SVG_XMLNS: &str = "%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20";

pub(crate) struct HtmlFragmentNormalizer;

impl HtmlFragmentNormalizer {
    pub(crate) fn normalize(fragment: &str) -> String {
        fragment.replace(BROKEN_SVG_XMLNS, ENCODED_SVG_XMLNS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_katana_fixture_svg_data_uri() {
        let fragment = r#"<img src="data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22> width=%22128%22%3E" alt="icon">"#;

        let normalized = HtmlFragmentNormalizer::normalize(fragment);

        assert!(!normalized.contains("<http://www.w3.org/2000/svg"));
        assert!(normalized.contains("xmlns=%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20"));
    }
}
