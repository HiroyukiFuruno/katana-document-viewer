use katana_diagram_renderer::{RenderThemeMode, RenderThemeSnapshot};
use serde::{Deserialize, Serialize};

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
        Self {
            name: "katana-light".to_string(),
            mode: KdvThemeMode::Light,
            background: "#ffffff".to_string(),
            text: "#24292f".to_string(),
            table_border: "#d0d7de".to_string(),
            table_header_background: "#eaf5ff".to_string(),
            table_even_row_background: "#f7fbff".to_string(),
            quote_border: "#d0d7de".to_string(),
            quote_text: "#57606a".to_string(),
            alert_background: "#f6f8fa".to_string(),
            code_background: "#f6f8fa".to_string(),
            code_border: "#d0d7de".to_string(),
            task_active_background: "#add6ff".to_string(),
            task_empty_background: "#f3f3f3".to_string(),
            task_done_accent: "#0078d4".to_string(),
            task_in_progress_accent: "#0078d4".to_string(),
            footnote_border: "#d0d7de".to_string(),
            footnote_text: "#57606a".to_string(),
            alert_note: "#0969da".to_string(),
            alert_tip: "#1a7f37".to_string(),
            alert_important: "#8250df".to_string(),
            alert_warning: "#d1242f".to_string(),
            alert_caution: "#bf8700".to_string(),
            diagram_background: "transparent".to_string(),
            diagram_text: "#333333".to_string(),
            diagram_fill: "#fff2cc".to_string(),
            diagram_stroke: "#d6b656".to_string(),
            diagram_arrow: "#555555".to_string(),
            mermaid_theme: "default".to_string(),
            syntax_theme_dark: "base16-ocean.dark".to_string(),
            syntax_theme_light: "InspiredGitHub".to_string(),
        }
    }

    pub fn katana_dark() -> Self {
        Self {
            name: "katana-dark".to_string(),
            mode: KdvThemeMode::Dark,
            background: "#0d1117".to_string(),
            text: "#f0f6fc".to_string(),
            table_border: "#30363d".to_string(),
            table_header_background: "#161b22".to_string(),
            table_even_row_background: "#111820".to_string(),
            quote_border: "#484f58".to_string(),
            quote_text: "#8b949e".to_string(),
            alert_background: "#161b22".to_string(),
            code_background: "#161b22".to_string(),
            code_border: "#30363d".to_string(),
            task_active_background: "#264f78".to_string(),
            task_empty_background: "#252526".to_string(),
            task_done_accent: "#569cd6".to_string(),
            task_in_progress_accent: "#569cd6".to_string(),
            footnote_border: "#30363d".to_string(),
            footnote_text: "#8b949e".to_string(),
            alert_note: "#58a6ff".to_string(),
            alert_tip: "#3fb950".to_string(),
            alert_important: "#a371f7".to_string(),
            alert_warning: "#f85149".to_string(),
            alert_caution: "#d29922".to_string(),
            diagram_background: "transparent".to_string(),
            diagram_text: "#f0f6fc".to_string(),
            diagram_fill: "#1f2937".to_string(),
            diagram_stroke: "#8b949e".to_string(),
            diagram_arrow: "#8b949e".to_string(),
            mermaid_theme: "dark".to_string(),
            syntax_theme_dark: "base16-ocean.dark".to_string(),
            syntax_theme_light: "InspiredGitHub".to_string(),
        }
    }

    pub(crate) fn diagram_theme_label(&self) -> &'static str {
        match self.mode {
            KdvThemeMode::Light => "light",
            KdvThemeMode::Dark => "dark",
        }
    }

    pub(crate) fn kdr_theme(&self) -> RenderThemeSnapshot {
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
}
