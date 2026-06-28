use super::super::super::types::{ViewerTextSpan, ViewerTextStyle};
use super::super::ViewerNodeClassifier;
use crate::export_surface_math::SurfaceMathText;
use katana_markdown_model::KmmNodeKind;

impl ViewerNodeClassifier {
    pub(in crate::viewer::node_plan) fn styled_inline_atom_spans(
        kind: &KmmNodeKind,
        style: ViewerTextStyle,
    ) -> Option<Vec<ViewerTextSpan>> {
        match kind {
            KmmNodeKind::Strong(span) => Some(Self::styled_span(
                Self::inline_marker_text(&span.text),
                style.bold(),
            )),
            KmmNodeKind::Emphasis(span) => Some(Self::styled_span(
                Self::inline_marker_text(&span.text),
                style.italic(),
            )),
            KmmNodeKind::Strikethrough(span) => Some(Self::styled_span(
                Self::inline_marker_text(&span.text),
                style.strikethrough(),
            )),
            KmmNodeKind::InlineCode(code) => {
                Some(Self::styled_span(code.code.clone(), style.inline_code()))
            }
            KmmNodeKind::InlineMath(math) => Some(Self::styled_span(
                SurfaceMathText::render(&math.expression),
                style.inline_math(),
            )),
            KmmNodeKind::Emoji(emoji) => {
                Some(Self::styled_span(emoji.value.clone(), style.emoji()))
            }
            _ => None,
        }
    }
}
