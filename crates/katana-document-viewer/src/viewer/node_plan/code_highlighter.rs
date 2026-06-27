use super::types::{VIEWER_TEXT_COLOR_CHANNELS, ViewerTextSpan, ViewerTextStyle};
use crate::KdvThemeSnapshot;
use crate::export_surface_code::SurfaceCodeHighlighter;
use crate::export_surface_span::SurfaceTextSpan;

const NO_COLOR: [u8; VIEWER_TEXT_COLOR_CHANNELS] = [u8::MIN; VIEWER_TEXT_COLOR_CHANNELS];

pub struct ViewerCodeHighlighter;

impl ViewerCodeHighlighter {
    #[must_use]
    pub fn highlight(language: Option<&str>, body: &str) -> Vec<ViewerTextSpan> {
        let highlighted = SurfaceCodeHighlighter::highlight(language, body);
        Self::highlighted_spans(highlighted)
    }

    #[must_use]
    pub fn highlight_with_theme(
        language: Option<&str>,
        body: &str,
        theme: &KdvThemeSnapshot,
    ) -> Vec<ViewerTextSpan> {
        let highlighted = SurfaceCodeHighlighter::highlight_with_theme(language, body, theme);
        Self::highlighted_spans(highlighted)
    }

    fn highlighted_spans(highlighted: Vec<Vec<SurfaceTextSpan>>) -> Vec<ViewerTextSpan> {
        let mut spans = Vec::new();
        for (index, line) in highlighted.iter().enumerate() {
            if index > 0 {
                spans.push(ViewerTextSpan::styled(
                    "\n",
                    ViewerTextStyle::default().monospace(),
                ));
            }
            spans.extend(line.iter().map(Self::code_span));
        }
        spans
    }

    fn code_span(span: &SurfaceTextSpan) -> ViewerTextSpan {
        ViewerTextSpan::styled(
            span.text.clone(),
            ViewerTextStyle {
                bold: span.style.bold,
                italic: span.style.italic,
                monospace: true,
                underline: span.style.underline,
                strikethrough: span.style.strikethrough,
                highlight: span.style.highlight,
                current_highlight: false,
                inline_code: false,
                inline_math: false,
                emoji: false,
                color_rgba: span_color(span),
            },
        )
    }
}

fn span_color(span: &SurfaceTextSpan) -> [u8; VIEWER_TEXT_COLOR_CHANNELS] {
    let Some(color) = span.style.color else {
        return NO_COLOR;
    };
    color.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{KdvThemeMode, KdvThemeSnapshot};

    #[test]
    fn dark_viewer_code_highlight_uses_katana_dark_syntax_theme() {
        let spans = ViewerCodeHighlighter::highlight_with_theme(
            Some("rust"),
            "enum RenderedSection {\n    Markdown(String),\n}\n",
            &dark_code_theme(),
        );
        let text_color = spans
            .iter()
            .find(|span| span.text.contains("RenderedSection"))
            .map(|span| span.style.color_rgba);
        assert!(
            text_color.is_some(),
            "identifier span should be highlighted"
        );
        let text_color = text_color.unwrap_or(NO_COLOR);
        let luminance =
            (u16::from(text_color[0]) + u16::from(text_color[1]) + u16::from(text_color[2])) / 3;

        assert!(
            luminance >= 150,
            "dark code text must stay readable on KatanA dark code background: rgba={text_color:?}"
        );
    }

    fn dark_code_theme() -> KdvThemeSnapshot {
        KdvThemeSnapshot {
            mode: KdvThemeMode::Dark,
            syntax_theme_dark: "base16-ocean.dark".to_string(),
            ..KdvThemeSnapshot::default()
        }
    }
}
