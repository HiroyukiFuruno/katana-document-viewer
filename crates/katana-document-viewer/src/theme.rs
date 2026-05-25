use katana_render_runtime::{RenderThemeMode, RenderThemeSnapshot};
use serde::{Deserialize, Serialize};

#[path = "theme_presets.rs"]
mod theme_presets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KdvThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KdvThemeSnapshot {
    pub name: String,
    pub mode: KdvThemeMode,
    pub background: String,
    pub text: String,
    pub table_border: String,
    pub table_header_background: String,
    pub table_even_row_background: String,
    pub quote_border: String,
    pub quote_text: String,
    pub alert_background: String,
    pub code_background: String,
    pub code_border: String,
    pub task_active_background: String,
    pub task_empty_background: String,
    pub task_done_accent: String,
    pub task_in_progress_accent: String,
    pub footnote_border: String,
    pub footnote_text: String,
    pub alert_note: String,
    pub alert_tip: String,
    pub alert_important: String,
    pub alert_warning: String,
    pub alert_caution: String,
    pub diagram_background: String,
    pub diagram_text: String,
    pub diagram_fill: String,
    pub diagram_stroke: String,
    pub diagram_arrow: String,
    pub mermaid_theme: String,
    pub syntax_theme_dark: String,
    pub syntax_theme_light: String,
}

impl KdvThemeSnapshot {
    pub fn katana_light() -> Self {
        theme_presets::katana_light()
    }

    pub fn katana_dark() -> Self {
        theme_presets::katana_dark()
    }

    pub(crate) fn diagram_theme_label(&self) -> &'static str {
        match self.mode {
            KdvThemeMode::Light => "light",
            KdvThemeMode::Dark => "dark",
        }
    }

    pub(crate) fn krr_theme(&self) -> RenderThemeSnapshot {
        RenderThemeSnapshot {
            mode: match self.mode {
                KdvThemeMode::Light => RenderThemeMode::Light,
                KdvThemeMode::Dark => RenderThemeMode::Dark,
            },
            background: self.diagram_background.clone(),
            text: self.diagram_text.clone(),
            fill: self.diagram_fill.clone(),
            stroke: self.diagram_stroke.clone(),
            arrow: self.diagram_arrow.clone(),
            drawio_label_color: self.diagram_text.clone(),
            mermaid_theme: self.mermaid_theme.clone(),
            plantuml_class_bg: self.diagram_fill.clone(),
            plantuml_note_bg: self.alert_background.clone(),
            plantuml_note_text: self.diagram_text.clone(),
            syntax_theme_dark: self.syntax_theme_dark.clone(),
            syntax_theme_light: self.syntax_theme_light.clone(),
            preview_text: self.diagram_text.clone(),
        }
    }

    pub(crate) fn krr_math_theme(&self) -> RenderThemeSnapshot {
        let text = self.text.clone();
        let mut snapshot = self.krr_theme();
        snapshot.text = text.clone();
        snapshot.fill = text.clone();
        snapshot.stroke = text.clone();
        snapshot.drawio_label_color = text.clone();
        snapshot.plantuml_note_text = text.clone();
        snapshot.preview_text = text;
        snapshot
    }
}
