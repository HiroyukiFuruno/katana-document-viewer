use crate::export_quality::types::{ExportQualityCheck, check};

pub(super) struct HtmlMarkdownAlignment;

impl HtmlMarkdownAlignment {
    pub(super) fn checks(html: &str, source: &str) -> Vec<ExportQualityCheck> {
        vec![check(
            "html preserves html alignment",
            Self::preserves_alignment(html, source),
            true,
            0,
        )]
    }

    fn preserves_alignment(html: &str, source: &str) -> bool {
        let source_compact = Self::compact(source);
        let html_without_style = Self::without_style_blocks(html);
        let html_compact = Self::compact(&html_without_style);
        ["left", "center", "right"].iter().all(|alignment| {
            !Self::contains_alignment(&source_compact, alignment)
                || Self::contains_alignment(&html_compact, alignment)
        })
    }

    fn contains_alignment(compact_html: &str, alignment: &str) -> bool {
        compact_html.contains(&format!("align=\"{alignment}\""))
            || compact_html.contains(&format!("align='{alignment}'"))
            || compact_html.contains(&format!("align={alignment}"))
            || compact_html.contains(&format!("text-align:{alignment}"))
            || (alignment == "center" && compact_html.contains("<center"))
    }

    fn without_style_blocks(html: &str) -> String {
        let lower = html.to_ascii_lowercase();
        let mut output = String::new();
        let mut cursor = 0;
        while let Some(offset) = lower[cursor..].find("<style") {
            let start = cursor + offset;
            output.push_str(&html[cursor..start]);
            let Some(close_offset) = lower[start..].find("</style>") else {
                return output;
            };
            cursor = start + close_offset + "</style>".len();
        }
        output.push_str(&html[cursor..]);
        output
    }

    fn compact(value: &str) -> String {
        value
            .chars()
            .filter(|character| !character.is_whitespace())
            .flat_map(char::to_lowercase)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_style_blocks_removes_style_sections() {
        let input = "<p>left</p><style>.a{}</style><p>right</p>";

        assert_eq!(
            HtmlMarkdownAlignment::without_style_blocks(input),
            "<p>left</p><p>right</p>"
        );
    }

    #[test]
    fn without_style_blocks_ignores_unclosed_style_sections() {
        let input = "<p>left</p><style>.a";
        assert_eq!(
            HtmlMarkdownAlignment::without_style_blocks(input),
            "<p>left</p>"
        );
    }

    #[test]
    fn preserves_alignment_detects_center_tag_and_attributes() {
        let source = "<align='center'>";
        assert!(HtmlMarkdownAlignment::contains_alignment(source, "center"));

        let html = "<p style=\"text-align:right\">x</p>";
        assert!(HtmlMarkdownAlignment::contains_alignment(html, "right"));
    }

    #[test]
    fn checks_reports_alignment_only_when_required() {
        let checks = HtmlMarkdownAlignment::checks(
            "<p align=\"center\">x</p><p>y</p>",
            "<p align=\"center\">x</p><p>y</p>",
        );
        assert!(checks.first().is_some_and(|check| {
            check.passed && check.name == "html preserves html alignment"
        }));
    }
}
