use katana_markdown_model::DiagramKind;
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

    pub fn katana_export_reference() -> Self {
        let mut theme = Self::katana_light();
        theme.name = "katana-export-reference".to_string();
        theme.text = "#333333".to_string();
        theme.code_background = "#f6f8fa".to_string();
        theme.code_border = "#d0d7de".to_string();
        theme
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
            drawio_label_color: self.krr_drawio_label_color(),
            mermaid_theme: self.mermaid_theme.clone(),
            plantuml_class_bg: self.krr_plantuml_class_background(),
            plantuml_note_bg: self.krr_plantuml_note_background(),
            plantuml_note_text: self.diagram_text.clone(),
            syntax_theme_dark: self.syntax_theme_dark.clone(),
            syntax_theme_light: self.syntax_theme_light.clone(),
            preview_text: self.diagram_text.clone(),
        }
    }

    pub(crate) fn krr_theme_for_diagram(&self, kind: &DiagramKind) -> RenderThemeSnapshot {
        let mut snapshot = self.krr_theme();
        if matches!(kind, DiagramKind::Mermaid | DiagramKind::DrawIo) {
            snapshot.text = self.text.clone();
            snapshot.preview_text = self.text.clone();
        }
        snapshot
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

    fn krr_drawio_label_color(&self) -> String {
        match self.mode {
            KdvThemeMode::Light => "#333333".to_string(),
            KdvThemeMode::Dark => "#1A1A1A".to_string(),
        }
    }

    fn krr_plantuml_class_background(&self) -> String {
        match self.mode {
            KdvThemeMode::Light => "#FEFECE".to_string(),
            KdvThemeMode::Dark => "#2D2D2D".to_string(),
        }
    }

    fn krr_plantuml_note_background(&self) -> String {
        match self.mode {
            KdvThemeMode::Light => "#FBFB77".to_string(),
            KdvThemeMode::Dark => "#3A3A3A".to_string(),
        }
    }
}

impl Default for KdvThemeSnapshot {
    fn default() -> Self {
        Self::katana_light()
    }
}

#[cfg(test)]
mod tests {
    use super::KdvThemeSnapshot;

    #[test]
    fn katana_export_reference_keeps_document_light_and_blocks_dark() {
        let theme = KdvThemeSnapshot::katana_export_reference();

        assert_eq!(theme.background, "#ffffff");
        assert_eq!(theme.text, "#333333");
        assert_eq!(theme.code_background, "#f6f8fa");
        assert_eq!(theme.code_border, "#d0d7de");
        assert_eq!(theme.table_header_background, "#f3f3f3");
        assert_eq!(theme.table_even_row_background, "#ffffff");
        assert_eq!(theme.diagram_fill, "#fff2cc");
        assert_eq!(theme.diagram_stroke, "#d6b656");
        assert_eq!(theme.diagram_arrow, "#555555");
        assert_eq!(theme.mermaid_theme, "default");
    }

    #[test]
    fn katana_dark_diagram_tokens_match_katana_reference_preset() {
        let theme = KdvThemeSnapshot::katana_dark();

        assert_eq!(theme.diagram_background, "transparent");
        assert_eq!(theme.diagram_text, "#E0E0E0");
        assert_eq!(theme.diagram_fill, "#2d2d2d");
        assert_eq!(theme.diagram_stroke, "#888888");
        assert_eq!(theme.diagram_arrow, "#aaaaaa");
        assert_eq!(theme.mermaid_theme, "dark");
    }

    #[test]
    fn katana_krr_theme_matches_katana_diagram_backend_preset_specific_tokens() {
        let dark = KdvThemeSnapshot::katana_dark().krr_theme();
        assert_eq!(dark.drawio_label_color, "#1A1A1A");
        assert_eq!(dark.plantuml_class_bg, "#2D2D2D");
        assert_eq!(dark.plantuml_note_bg, "#3A3A3A");
        assert_eq!(dark.plantuml_note_text, "#E0E0E0");
        assert_eq!(dark.preview_text, "#E0E0E0");

        let light = KdvThemeSnapshot::katana_light().krr_theme();
        assert_eq!(light.drawio_label_color, "#333333");
        assert_eq!(light.plantuml_class_bg, "#FEFECE");
        assert_eq!(light.plantuml_note_bg, "#FBFB77");
        assert_eq!(light.plantuml_note_text, "#333333");
        assert_eq!(light.preview_text, "#333333");
    }

    #[test]
    fn katana_dark_mermaid_theme_uses_app_preview_text_like_katana_viewer() {
        let dark = KdvThemeSnapshot::katana_dark()
            .krr_theme_for_diagram(&katana_markdown_model::DiagramKind::Mermaid);

        assert_eq!(dark.text, "#d4d4d4");
        assert_eq!(dark.preview_text, "#d4d4d4");
        assert_eq!(dark.plantuml_note_text, "#E0E0E0");
    }
}
