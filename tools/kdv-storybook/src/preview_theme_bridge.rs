use katana_document_viewer::{KdvThemeMode, KdvThemeSnapshot};
use katana_ui_core::theme::{ColorToken, Rgba, ThemeId, ThemeSnapshot};

pub(crate) struct KucThemeBridge;

impl KucThemeBridge {
    pub(crate) fn from_kdv(theme: &KdvThemeSnapshot) -> Result<ThemeSnapshot, String> {
        let mut snapshot = match theme.mode {
            KdvThemeMode::Light => ThemeSnapshot::light(),
            KdvThemeMode::Dark => ThemeSnapshot::dark(),
        };
        snapshot.id = ThemeId::new(theme.name.clone());
        for token in Self::color_tokens(theme) {
            Self::set_color(&mut snapshot, token.0, token.1)?;
        }
        Ok(snapshot)
    }

    pub(crate) fn from_kdv_export_surface(
        theme: &KdvThemeSnapshot,
    ) -> Result<ThemeSnapshot, String> {
        let mut snapshot = Self::from_kdv(theme)?;
        Self::set_color(&mut snapshot, "preview-text", &theme.text)?;
        Self::set_color(&mut snapshot, "inline-code-background", "#eff2f6")?;
        Ok(snapshot)
    }

    fn color_tokens(theme: &KdvThemeSnapshot) -> Vec<(&'static str, &str)> {
        vec![
            ("background", &theme.background),
            ("surface", &theme.background),
            ("panel", &theme.background),
            ("code-background", &theme.code_background),
            (
                "inline-code-background",
                Self::inline_code_background(theme),
            ),
            ("text", &theme.text),
            ("preview-text", &theme.text),
            ("link", Self::hyperlink_color(theme)),
            ("muted", &theme.quote_text),
            ("border", &theme.table_border),
            ("selection", &theme.task_active_background),
            ("table-row-background", &theme.background),
            ("table-header-background", &theme.table_header_background),
            (
                "table-even-row-background",
                &theme.table_even_row_background,
            ),
            ("alert-background", &theme.alert_background),
            ("alert-note", &theme.alert_note),
            ("alert-tip", &theme.alert_tip),
            ("alert-important", &theme.alert_important),
            ("alert-warning", &theme.alert_warning),
            ("alert-caution", &theme.alert_caution),
            ("quote-background", Self::quote_background(theme)),
            ("footnote-background", Self::footnote_background(theme)),
        ]
    }

    fn quote_background(theme: &KdvThemeSnapshot) -> &str {
        &theme.alert_background
    }

    fn footnote_background(theme: &KdvThemeSnapshot) -> &str {
        &theme.alert_background
    }

    fn inline_code_background(theme: &KdvThemeSnapshot) -> &str {
        match theme.mode {
            KdvThemeMode::Light => "#f6f8fa",
            KdvThemeMode::Dark => &theme.code_background,
        }
    }

    fn hyperlink_color(theme: &KdvThemeSnapshot) -> &'static str {
        match theme.mode {
            KdvThemeMode::Light => "#009bff",
            KdvThemeMode::Dark => "#5aaaff",
        }
    }

    fn set_color(snapshot: &mut ThemeSnapshot, name: &str, value: &str) -> Result<(), String> {
        let rgba = Self::parse_hex_color(name, value)?;
        if let Some(token) = snapshot.colors.iter_mut().find(|token| token.name == name) {
            token.rgba = rgba;
            return Ok(());
        }
        snapshot.colors.push(ColorToken {
            name: name.to_string(),
            rgba,
        });
        Ok(())
    }

    fn parse_hex_color(name: &str, value: &str) -> Result<Rgba, String> {
        let Some(hex) = value.strip_prefix('#') else {
            return Err(format!("theme color {name} must be #rrggbb: {value}"));
        };
        if hex.len() != 6 {
            return Err(format!("theme color {name} must be 6 hex digits: {value}"));
        }
        let red = Self::hex_pair(name, value, &hex[0..2])?;
        let green = Self::hex_pair(name, value, &hex[2..4])?;
        let blue = Self::hex_pair(name, value, &hex[4..6])?;
        Ok([red, green, blue, 255])
    }

    fn hex_pair(name: &str, value: &str, pair: &str) -> Result<u8, String> {
        u8::from_str_radix(pair, 16)
            .map_err(|error| format!("theme color {name} is invalid {value}: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use super::KucThemeBridge;
    use katana_document_viewer::KdvThemeSnapshot;

    #[test]
    fn bridge_passes_kdv_table_theme_tokens_to_kuc() -> Result<(), Box<dyn std::error::Error>> {
        let snapshot = KucThemeBridge::from_kdv(&KdvThemeSnapshot::katana_light())?;

        assert_eq!(
            Some([243, 243, 243, 255]),
            snapshot.color("table-header-background")
        );
        assert_eq!(
            Some([255, 255, 255, 255]),
            snapshot.color("table-even-row-background")
        );
        assert_eq!(Some([220, 220, 220, 255]), snapshot.color("border"));
        assert_eq!(Some([0, 155, 255, 255]), snapshot.color("link"));
        assert_eq!(Some([36, 36, 36, 255]), snapshot.color("preview-text"));
        assert_eq!(
            Some([246, 248, 250, 255]),
            snapshot.color("inline-code-background")
        );
        assert_eq!(Some([0, 120, 212, 255]), snapshot.color("alert-note"));
        assert_eq!(Some([64, 160, 43, 255]), snapshot.color("alert-tip"));
        assert_eq!(Some([0, 120, 212, 255]), snapshot.color("alert-important"));
        assert_eq!(Some([223, 142, 29, 255]), snapshot.color("alert-warning"));
        assert_eq!(Some([210, 15, 57, 255]), snapshot.color("alert-caution"));
        assert_eq!(
            Some([243, 243, 243, 255]),
            snapshot.color("quote-background")
        );
        assert_eq!(
            Some([243, 243, 243, 255]),
            snapshot.color("footnote-background")
        );
        Ok(())
    }

    #[test]
    fn bridge_passes_kdv_dark_document_role_backgrounds_to_kuc()
    -> Result<(), Box<dyn std::error::Error>> {
        let snapshot = KucThemeBridge::from_kdv(&KdvThemeSnapshot::katana_dark())?;

        assert_eq!(Some([90, 170, 255, 255]), snapshot.color("link"));
        assert_eq!(Some([212, 212, 212, 255]), snapshot.color("preview-text"));
        assert_eq!(
            Some([40, 40, 40, 255]),
            snapshot.color("inline-code-background")
        );
        assert_eq!(Some([40, 40, 40, 255]), snapshot.color("quote-background"));
        assert_eq!(
            Some([40, 40, 40, 255]),
            snapshot.color("footnote-background")
        );
        Ok(())
    }

    #[test]
    fn export_surface_bridge_uses_kdv_inline_code_background()
    -> Result<(), Box<dyn std::error::Error>> {
        let snapshot = KucThemeBridge::from_kdv_export_surface(&KdvThemeSnapshot::katana_light())?;

        assert_eq!(Some([36, 36, 36, 255]), snapshot.color("preview-text"));
        assert_eq!(
            Some([239, 242, 246, 255]),
            snapshot.color("inline-code-background")
        );
        Ok(())
    }
}
