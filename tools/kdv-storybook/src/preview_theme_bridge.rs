use katana_document_viewer::{KdvThemeMode, KdvThemeSnapshot, ViewerTypographyConfig};
use katana_ui_core::raster_host::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography};
use katana_ui_core::text_raster::PlatformTextRasterConfig;
use katana_ui_core::theme::{ColorToken, Rgba, ThemeId, ThemeSnapshot};
use katana_ui_core_storybook::UiTreeSurfaceHost;
use std::path::PathBuf;

const KATANA_HEADING_1_SIZE_RATIO: f32 = 1.5;
const KATANA_HEADING_2_PROGRESS: f32 = 0.835;
const KATANA_HEADING_3_PROGRESS: f32 = 0.668;
const KATANA_HEADING_4_PROGRESS: f32 = 0.501;
const KATANA_HEADING_5_PROGRESS: f32 = 0.334;
const KATANA_HEADING_6_PROGRESS: f32 = 0.167;

pub(crate) struct KucThemeBridge;

impl KucThemeBridge {
    pub(crate) fn document_typography(
        typography: ViewerTypographyConfig,
    ) -> UiTreeDocumentTypography {
        let font_size = f32::from(typography.preview_font_size);
        let heading_1_font_size = font_size * KATANA_HEADING_1_SIZE_RATIO;
        let heading_2_font_size =
            font_size + (heading_1_font_size - font_size) * KATANA_HEADING_2_PROGRESS;
        let heading_3_font_size =
            font_size + (heading_1_font_size - font_size) * KATANA_HEADING_3_PROGRESS;
        let heading_4_font_size =
            font_size + (heading_1_font_size - font_size) * KATANA_HEADING_4_PROGRESS;
        let heading_5_font_size =
            font_size + (heading_1_font_size - font_size) * KATANA_HEADING_5_PROGRESS;
        let heading_6_font_size =
            font_size + (heading_1_font_size - font_size) * KATANA_HEADING_6_PROGRESS;
        let scale = font_size / 14.0;
        UiTreeDocumentTypography::new()
            .with_body_baseline(UiTreeTextRoleBaselineTypography::new(
                font_size,
                21.0 * scale,
                12.5 * scale,
            ))
            .with_heading_1_baseline(UiTreeTextRoleBaselineTypography::new(
                heading_1_font_size,
                31.5 * scale,
                18.5 * scale,
            ))
            .with_heading_2_baseline(UiTreeTextRoleBaselineTypography::new(
                heading_2_font_size,
                30.0 * scale,
                17.5 * scale,
            ))
            .with_heading_3_baseline(UiTreeTextRoleBaselineTypography::new(
                heading_3_font_size,
                28.0 * scale,
                16.5 * scale,
            ))
            .with_heading_4_baseline(UiTreeTextRoleBaselineTypography::new(
                heading_4_font_size,
                26.5 * scale,
                15.5 * scale,
            ))
            .with_heading_5_baseline(UiTreeTextRoleBaselineTypography::new(
                heading_5_font_size,
                24.5 * scale,
                14.5 * scale,
            ))
            .with_heading_6_baseline(UiTreeTextRoleBaselineTypography::new(
                heading_6_font_size,
                23.0 * scale,
                13.5 * scale,
            ))
    }

    pub(crate) fn document_host(
        theme: ThemeSnapshot,
        typography: ViewerTypographyConfig,
    ) -> UiTreeSurfaceHost {
        Self::document_host_for_surface(theme, typography, false)
    }

    pub(crate) fn document_host_for_surface(
        theme: ThemeSnapshot,
        typography: ViewerTypographyConfig,
        export_surface: bool,
    ) -> UiTreeSurfaceHost {
        // export role は KUC 自身が KDV export と同じ compact metrics を持つ。
        // interactive 用 baseline override を重ねると独立 export reference とずれるため分離する。
        let document_typography = if export_surface {
            UiTreeDocumentTypography::new()
        } else {
            Self::document_typography(typography)
        };
        UiTreeSurfaceHost::with_text_raster_config_and_document_typography(
            theme,
            Self::katana_text_raster_config(),
            katana_ui_core::text_raster::PlatformTextFaceSelection::FirstCandidate,
            document_typography,
        )
    }

    fn katana_text_raster_config() -> PlatformTextRasterConfig {
        // KatanAの本文候補順を投影する。KUC既定のOS別候補だけではWindowsがSegoe UIを
        // 選び、KatanAのYu Gothic/Meiryo基準と字幅・折返し位置がずれる。
        PlatformTextRasterConfig {
            proportional_candidates: Self::katana_proportional_font_candidates(),
            monospace_candidates: Self::katana_monospace_font_candidates(),
            ..PlatformTextRasterConfig::default()
        }
    }

    fn katana_proportional_font_candidates() -> Vec<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            Self::font_paths(&[
                "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
                "/System/Library/Fonts/Hiragino Sans GB.ttc",
                "/System/Library/Fonts/AquaKana.ttc",
            ])
        }
        #[cfg(target_os = "windows")]
        {
            Self::font_paths(&[
                "C:/Windows/Fonts/YuGothR.ttc",
                "C:/Windows/Fonts/yugothic.ttf",
                "C:/Windows/Fonts/meiryo.ttc",
                "C:/Windows/Fonts/segoeui.ttf",
            ])
        }
        #[cfg(target_os = "linux")]
        {
            Self::font_paths(&[
                "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ])
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        Vec::new()
    }

    fn katana_monospace_font_candidates() -> Vec<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            Self::font_paths(&[
                "/System/Library/Fonts/Menlo.ttc",
                "/System/Library/Fonts/SFMono-Regular.otf",
                "/System/Library/Fonts/Monaco.ttf",
            ])
        }
        #[cfg(target_os = "windows")]
        {
            Self::font_paths(&["C:/Windows/Fonts/consola.ttf", "C:/Windows/Fonts/cour.ttf"])
        }
        #[cfg(target_os = "linux")]
        {
            Self::font_paths(&[
                "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
                "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf",
                "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            ])
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        Vec::new()
    }

    fn font_paths(paths: &[&str]) -> Vec<PathBuf> {
        paths.iter().map(PathBuf::from).collect()
    }

    pub(crate) fn from_kdv(theme: &KdvThemeSnapshot) -> Result<ThemeSnapshot, String> {
        let mut snapshot = match theme.mode {
            KdvThemeMode::Light => ThemeSnapshot::light(),
            KdvThemeMode::Dark => ThemeSnapshot::dark(),
        };
        snapshot.id = ThemeId::new(theme.name.clone());
        for token in Self::color_tokens(theme) {
            Self::set_color(&mut snapshot, token.0, token.1)?;
        }
        let highlight_background = Self::highlight_background(theme)?;
        Self::set_color(
            &mut snapshot,
            "text-highlight-background",
            &highlight_background,
        )?;
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
            ("document-rule-border", &theme.table_border),
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

    fn highlight_background(theme: &KdvThemeSnapshot) -> Result<String, String> {
        const MARK_YELLOW: [u8; 3] = [255, 255, 0];
        const MARK_ALPHA: u16 = 60;
        const OPAQUE: u16 = 255;
        let background = Self::parse_hex_color("background", &theme.background)?;
        let blend = |foreground: u8, background: u8| {
            ((u16::from(foreground) * MARK_ALPHA + u16::from(background) * (OPAQUE - MARK_ALPHA))
                / OPAQUE) as u8
        };
        Ok(format!(
            "#{:02x}{:02x}{:02x}",
            blend(MARK_YELLOW[0], background[0]),
            blend(MARK_YELLOW[1], background[1]),
            blend(MARK_YELLOW[2], background[2]),
        ))
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
        assert_eq!(
            Some([220, 220, 220, 255]),
            snapshot.color("document-rule-border")
        );
        assert_eq!(
            Some([255, 255, 195, 255]),
            snapshot.color("text-highlight-background")
        );
        assert_eq!(Some([0, 155, 255, 255]), snapshot.color("link"));
        assert_eq!(Some([36, 36, 36, 255]), snapshot.color("preview-text"));
        assert_eq!(
            Some([246, 248, 250, 255]),
            snapshot.color("inline-code-background")
        );
        assert_eq!(Some([0, 120, 212, 255]), snapshot.color("alert-note"));
        assert_eq!(Some([64, 160, 43, 255]), snapshot.color("alert-tip"));
        assert_eq!(Some([130, 80, 223, 255]), snapshot.color("alert-important"));
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
    fn bridge_blends_mark_background_over_custom_kdv_background()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut theme = KdvThemeSnapshot::katana_light();
        theme.background = "#112233".to_string();

        let snapshot = KucThemeBridge::from_kdv(&theme)?;

        assert_eq!(
            Some([73, 86, 39, 255]),
            snapshot.color("text-highlight-background")
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

    #[test]
    fn bridge_uses_katana_platform_font_priority() {
        let config = KucThemeBridge::katana_text_raster_config();

        #[cfg(target_os = "macos")]
        assert_eq!(
            Some("/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc"),
            config
                .proportional_candidates
                .first()
                .and_then(|path| path.to_str())
        );
        #[cfg(target_os = "windows")]
        assert_eq!(
            Some("C:/Windows/Fonts/YuGothR.ttc"),
            config
                .proportional_candidates
                .first()
                .and_then(|path| path.to_str())
        );
        #[cfg(target_os = "linux")]
        assert_eq!(
            Some("/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc"),
            config
                .proportional_candidates
                .first()
                .and_then(|path| path.to_str())
        );
    }
}
