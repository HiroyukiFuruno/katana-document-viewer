use crate::export_inline_payload::InlineHtmlWriter;
use crate::theme::KdvThemeSnapshot;

pub(crate) struct NestedBlockquoteHtml;

impl NestedBlockquoteHtml {
    pub(crate) fn is_nested(text: &str) -> bool {
        text.lines()
            .filter_map(Self::line_parts)
            .any(|line| line.depth > 1)
    }

    pub(crate) fn append(html: &mut String, theme: &KdvThemeSnapshot, text: &str) {
        let mut current_depth = 0;
        for line in text.lines().filter_map(Self::line_parts) {
            while current_depth > line.depth {
                html.push_str("</blockquote>\n");
                current_depth -= 1;
            }
            while current_depth < line.depth {
                current_depth += 1;
                html.push_str(&format!(
                    "<blockquote data-kdv-blockquote=\"quote\" data-kdv-quote-depth=\"{current_depth}\">"
                ));
            }
            if !line.text.trim().is_empty() {
                html.push_str("<p>");
                InlineHtmlWriter::append_fragment(html, line.text, theme);
                html.push_str("</p>\n");
            }
        }
        while current_depth > 0 {
            html.push_str("</blockquote>\n");
            current_depth -= 1;
        }
    }

    fn line_parts(line: &str) -> Option<BlockquoteLine<'_>> {
        let mut rest = line.trim_start();
        let mut depth = 0;
        while let Some(stripped) = rest.strip_prefix('>') {
            depth += 1;
            rest = stripped.trim_start();
        }
        (depth > 0).then_some(BlockquoteLine { depth, text: rest })
    }
}

struct BlockquoteLine<'a> {
    depth: usize,
    text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::KdvThemeSnapshot;

    #[test]
    fn is_nested_detects_depth() {
        assert!(NestedBlockquoteHtml::is_nested("> one\n> > two"));
        assert!(!NestedBlockquoteHtml::is_nested("> one"));
    }

    #[test]
    fn append_blocks_open_and_close_depths() {
        let mut html = String::new();
        NestedBlockquoteHtml::append(
            &mut html,
            &KdvThemeSnapshot::katana_light(),
            "> one\n> > two",
        );
        assert!(html.contains("data-kdv-quote-depth=\"1\""));
        assert!(html.contains("data-kdv-quote-depth=\"2\""));
    }

    #[test]
    fn append_ignores_non_blockquote_lines() {
        let mut html = String::new();
        NestedBlockquoteHtml::append(&mut html, &KdvThemeSnapshot::katana_light(), "plain text");
        assert_eq!(html, "");
    }

    #[test]
    fn append_closes_quote_blocks_when_depth_decreases() {
        let mut html = String::new();
        NestedBlockquoteHtml::append(
            &mut html,
            &KdvThemeSnapshot::katana_light(),
            "> > one\n> two",
        );

        assert!(html.contains("</blockquote>"));
    }

    #[test]
    fn line_parts_ignores_trimming() {
        assert!(NestedBlockquoteHtml::line_parts(" > x").is_some());
    }
}
