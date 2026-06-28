pub(super) struct RawSvgHtmlQuality;

impl RawSvgHtmlQuality {
    pub(super) fn has_media(html: &str, source: &str) -> bool {
        if !html.contains("<svg") {
            return false;
        }
        let Some(signature) = Self::signature_token(source) else {
            return Self::has_opening_tag_signature(html, source);
        };
        html.to_ascii_lowercase().contains(&signature)
    }

    fn signature_token(source: &str) -> Option<String> {
        Self::tokens(source)
            .into_iter()
            .find(|token| Self::is_signature_token(token))
    }

    fn tokens(source: &str) -> Vec<String> {
        source
            .split(|character: char| {
                !(character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
            })
            .filter(|token| !token.is_empty())
            .map(str::to_ascii_lowercase)
            .collect()
    }

    fn is_signature_token(token: &str) -> bool {
        token.len() >= 4
            && !matches!(
                token,
                "2000"
                    | "height"
                    | "http"
                    | "https"
                    | "org"
                    | "svg"
                    | "version"
                    | "viewbox"
                    | "width"
                    | "www"
                    | "xmlns"
            )
    }

    fn has_opening_tag_signature(html: &str, source: &str) -> bool {
        let Some(signature) = Self::opening_tag_signature(source) else {
            return false;
        };
        Self::compact(html).contains(&signature)
    }

    fn opening_tag_signature(source: &str) -> Option<String> {
        let lower = source.to_ascii_lowercase();
        let start = lower.find("<svg")?;
        let tail = &lower[start..];
        let end = tail.find('>')?;
        Some(Self::compact(&tail[..=end]))
    }

    fn compact(value: &str) -> String {
        value
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_svg_media_uses_opening_tag_signature_when_signature_token_is_missing() {
        let source = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>"#;
        let html = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>"#;

        assert!(RawSvgHtmlQuality::has_media(html, source));
    }

    #[test]
    fn raw_svg_media_returns_false_without_opening_signature() {
        assert!(!RawSvgHtmlQuality::has_media("<svg></svg>", "<svg"));
    }
}
