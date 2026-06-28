use super::KucNodeFactory;
use katana_document_viewer::{ViewerNode, ViewerNodeKind};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiBorder, UiNode, UiTextWrapMode, UiTone};

const ALERT_ROLE: &str = "alert";
const ALERT_BORDER_WIDTH_PX: u16 = 4;

impl KucNodeFactory<'_> {
    pub(super) fn alert_node(&self, node: &ViewerNode) -> UiNode {
        let kind = alert_kind(node);
        let body = alert_body(&node.text, kind, alert_label(node));
        let text = if body.is_empty() {
            kind.title().to_string()
        } else {
            format!("{}\n{body}", kind.title())
        };
        let rendered: UiNode = Text::new(text)
            .font_role("body")
            .text_role(ALERT_ROLE)
            .wrap(UiTextWrapMode::Wrap)
            .selectable(self.interaction.selection_enabled)
            .into();
        rendered.severity(kind.severity()).border(UiBorder::solid(
            ALERT_BORDER_WIDTH_PX,
            0,
            kind.color_token(),
        ))
    }
}

fn alert_label(node: &ViewerNode) -> &str {
    match &node.kind {
        ViewerNodeKind::Alert { label } => label,
        _ => "NOTE",
    }
}

fn alert_kind(node: &ViewerNode) -> AlertKind {
    AlertKind::from_label(alert_label(node))
}

fn alert_body(text: &str, kind: AlertKind, raw_label: &str) -> String {
    let trimmed = text.trim();
    for prefix in [raw_label, kind.title()] {
        let label_prefix = format!("{prefix}:");
        if let Some(body) = strip_case_insensitive_prefix(trimmed, &label_prefix) {
            return body.trim().to_string();
        }
    }
    trimmed.to_string()
}

fn strip_case_insensitive_prefix<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    let head = text.get(..prefix.len())?;
    if head.eq_ignore_ascii_case(prefix) {
        return text.get(prefix.len()..);
    }
    None
}

#[derive(Clone, Copy)]
enum AlertKind {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl AlertKind {
    fn from_label(label: &str) -> Self {
        match label.trim().to_ascii_lowercase().as_str() {
            "tip" => Self::Tip,
            "important" => Self::Important,
            "warning" => Self::Warning,
            "caution" => Self::Caution,
            _ => Self::Note,
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Note => "Note",
            Self::Tip => "Tip",
            Self::Important => "Important",
            Self::Warning => "Warning",
            Self::Caution => "Caution",
        }
    }

    fn severity(self) -> UiTone {
        match self {
            Self::Tip => UiTone::Success,
            Self::Warning => UiTone::Warning,
            Self::Caution => UiTone::Danger,
            Self::Important | Self::Note => UiTone::Accent,
        }
    }

    fn color_token(self) -> &'static str {
        match self {
            Self::Note => "alert-note",
            Self::Tip => "alert-tip",
            Self::Important => "alert-important",
            Self::Warning => "alert-warning",
            Self::Caution => "alert-caution",
        }
    }
}
