use crate::export_surface_math::SurfaceMathText;
use crate::export_surface_text::{decode_basic_entities, html_fragment_text};
use image::Rgba;
use katana_markdown_model::{KmmNode, KmmNodeKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SurfaceTextSpan {
    pub(crate) text: String,
    pub(crate) style: SurfaceTextStyle,
    pub(crate) link_target: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SurfaceTextStyle {
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    pub(crate) monospace: bool,
    pub(crate) underline: bool,
    pub(crate) strikethrough: bool,
    pub(crate) highlight: bool,
    pub(crate) inline_code: bool,
    pub(crate) color: Option<Rgba<u8>>,
}

impl SurfaceTextSpan {
    pub(crate) fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: SurfaceTextStyle::default(),
            link_target: None,
        }
    }

    pub(crate) fn styled(text: impl Into<String>, style: SurfaceTextStyle) -> Self {
        Self {
            text: text.into(),
            style,
            link_target: None,
        }
    }

    pub(crate) fn linked(
        text: impl Into<String>,
        target: impl Into<String>,
        style: SurfaceTextStyle,
    ) -> Self {
        Self {
            text: text.into(),
            style,
            link_target: Some(target.into()),
        }
    }
}

pub(crate) struct SurfaceInlineSpans;

impl SurfaceInlineSpans {
    pub(crate) fn from_node(node: &KmmNode) -> Vec<SurfaceTextSpan> {
        let mut spans = Vec::new();
        Self::append_node(&mut spans, node, SurfaceTextStyle::default());
        spans
    }

    pub(crate) fn from_nodes(nodes: &[KmmNode]) -> Vec<SurfaceTextSpan> {
        let mut spans = Vec::new();
        for node in nodes {
            Self::append_node(&mut spans, node, SurfaceTextStyle::default());
        }
        spans
    }

    fn append_node(spans: &mut Vec<SurfaceTextSpan>, node: &KmmNode, style: SurfaceTextStyle) {
        match &node.kind {
            KmmNodeKind::Text(text) => Self::push(spans, decode_basic_entities(&text.text), style),
            KmmNodeKind::Strong(span) => Self::append_span(spans, node, &span.text, style.bold()),
            KmmNodeKind::Emphasis(span) => {
                Self::append_span(spans, node, &span.text, style.italic())
            }
            KmmNodeKind::Strikethrough(span) => {
                Self::append_span(spans, node, &span.text, style.strikethrough())
            }
            KmmNodeKind::InlineCode(code) => Self::push(spans, &code.code, style.inline_code()),
            KmmNodeKind::InlineHtml(html) => Self::push(
                spans,
                html_fragment_text(&html.html),
                html_style(&html.html, style),
            ),
            KmmNodeKind::Link(link) => {
                Self::push_link(spans, &link.label, link.destination.clone(), style.link())
            }
            KmmNodeKind::Image(image) => Self::push(spans, &image.alt, style),
            KmmNodeKind::FootnoteReference(reference) => Self::push_link(
                spans,
                format!("[{}]", reference.label),
                format!("#fn-{}", reference.label),
                style.link(),
            ),
            KmmNodeKind::InlineMath(math) => {
                Self::push(spans, SurfaceMathText::render(&math.expression), style)
            }
            KmmNodeKind::Emoji(emoji) => Self::push(spans, &emoji.value, style),
            _ if node.children.is_empty() => {
                Self::push(spans, decode_basic_entities(&node.source.raw.text), style)
            }
            _ => {
                for child in &node.children {
                    Self::append_node(spans, child, style);
                }
            }
        }
    }

    fn append_span(
        spans: &mut Vec<SurfaceTextSpan>,
        node: &KmmNode,
        text: &str,
        style: SurfaceTextStyle,
    ) {
        if node.children.is_empty() {
            Self::push(spans, text, style);
            return;
        }
        for child in &node.children {
            Self::append_node(spans, child, style);
        }
    }

    fn push(spans: &mut Vec<SurfaceTextSpan>, text: impl Into<String>, style: SurfaceTextStyle) {
        let text = text.into();
        if !text.is_empty() {
            spans.push(SurfaceTextSpan::styled(text, style));
        }
    }

    fn push_link(
        spans: &mut Vec<SurfaceTextSpan>,
        text: impl Into<String>,
        target: impl Into<String>,
        style: SurfaceTextStyle,
    ) {
        let text = text.into();
        if !text.is_empty() {
            spans.push(SurfaceTextSpan::linked(text, target, style));
        }
    }
}

impl SurfaceTextStyle {
    pub(crate) fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub(crate) fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub(crate) fn monospace(mut self) -> Self {
        self.monospace = true;
        self
    }

    pub(crate) fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub(crate) fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub(crate) fn highlight(mut self) -> Self {
        self.highlight = true;
        self
    }

    pub(crate) fn inline_code(mut self) -> Self {
        self.monospace = true;
        self.inline_code = true;
        self
    }

    pub(crate) fn link(self) -> Self {
        self.underline().with_color(Rgba([9, 105, 218, 255]))
    }

    pub(crate) fn with_color(mut self, color: Rgba<u8>) -> Self {
        self.color = Some(color);
        self
    }
}

fn html_style(html: &str, style: SurfaceTextStyle) -> SurfaceTextStyle {
    let lower = html.to_ascii_lowercase();
    if lower.contains("<code") {
        return style.inline_code();
    }
    if lower.contains("<strong") || lower.contains("<b") {
        return style.bold();
    }
    if lower.contains("<em") || lower.contains("<i") {
        return style.italic();
    }
    if lower.contains("<u") {
        return style.underline();
    }
    if lower.contains("<mark") {
        return style.highlight();
    }
    if lower.contains("<s") || lower.contains("<del") {
        return style.strikethrough();
    }
    style
}
