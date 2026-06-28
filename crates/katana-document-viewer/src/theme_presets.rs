use super::{KdvThemeMode, KdvThemeSnapshot};

const COLOR_SLOT_COUNT: usize = 29;

macro_rules! kdv_theme_snapshot {
    (
        $name:expr, $mode:expr, $background:expr, $text:expr, $table_border:expr,
        $table_header_background:expr, $table_even_row_background:expr, $quote_border:expr,
        $quote_text:expr, $alert_background:expr, $code_background:expr, $code_border:expr,
        $task_active_background:expr, $task_empty_background:expr, $task_done_accent:expr,
        $task_in_progress_accent:expr, $footnote_border:expr, $footnote_text:expr, $alert_note:expr,
        $alert_tip:expr, $alert_important:expr, $alert_warning:expr, $alert_caution:expr,
        $diagram_background:expr, $diagram_text:expr, $diagram_fill:expr, $diagram_stroke:expr,
        $diagram_arrow:expr, $mermaid_theme:expr, $syntax_theme_dark:expr, $syntax_theme_light:expr
    ) => {
        KdvThemeSnapshot {
            name: $name.to_string(),
            mode: $mode,
            background: $background.to_string(),
            text: $text.to_string(),
            table_border: $table_border.to_string(),
            table_header_background: $table_header_background.to_string(),
            table_even_row_background: $table_even_row_background.to_string(),
            quote_border: $quote_border.to_string(),
            quote_text: $quote_text.to_string(),
            alert_background: $alert_background.to_string(),
            code_background: $code_background.to_string(),
            code_border: $code_border.to_string(),
            task_active_background: $task_active_background.to_string(),
            task_empty_background: $task_empty_background.to_string(),
            task_done_accent: $task_done_accent.to_string(),
            task_in_progress_accent: $task_in_progress_accent.to_string(),
            footnote_border: $footnote_border.to_string(),
            footnote_text: $footnote_text.to_string(),
            alert_note: $alert_note.to_string(),
            alert_tip: $alert_tip.to_string(),
            alert_important: $alert_important.to_string(),
            alert_warning: $alert_warning.to_string(),
            alert_caution: $alert_caution.to_string(),
            diagram_background: $diagram_background.to_string(),
            diagram_text: $diagram_text.to_string(),
            diagram_fill: $diagram_fill.to_string(),
            diagram_stroke: $diagram_stroke.to_string(),
            diagram_arrow: $diagram_arrow.to_string(),
            mermaid_theme: $mermaid_theme.to_string(),
            syntax_theme_dark: $syntax_theme_dark.to_string(),
            syntax_theme_light: $syntax_theme_light.to_string(),
        }
    };
}

macro_rules! kdv_theme_snapshot_from_colors {
    ($name:expr, $mode:expr, $colors:expr) => {
        kdv_theme_snapshot!(
            $name,
            $mode,
            $colors[0],
            $colors[1],
            $colors[2],
            $colors[3],
            $colors[4],
            $colors[5],
            $colors[6],
            $colors[7],
            $colors[8],
            $colors[9],
            $colors[10],
            $colors[11],
            $colors[12],
            $colors[13],
            $colors[14],
            $colors[15],
            $colors[16],
            $colors[17],
            $colors[18],
            $colors[19],
            $colors[20],
            $colors[21],
            $colors[22],
            $colors[23],
            $colors[24],
            $colors[25],
            $colors[26],
            $colors[27],
            $colors[28]
        )
    };
}

const KATANA_LIGHT_COLORS: [&str; COLOR_SLOT_COUNT] = [
    "#ffffff",
    "#242424",
    "#dcdcdc",
    "#f3f3f3",
    "#ffffff",
    "#dcdcdc",
    "#6a6a6a",
    "#f3f3f3",
    "#f3f3f3",
    "#dcdcdc",
    "#add6ff",
    "#f3f3f3",
    "#0078d4",
    "#0078d4",
    "#dcdcdc",
    "#6a6a6a",
    "#0078d4",
    "#40a02b",
    "#0078d4",
    "#df8e1d",
    "#d20f39",
    "transparent",
    "#333333",
    "#fff2cc",
    "#d6b656",
    "#555555",
    "default",
    "base16-ocean.dark",
    "InspiredGitHub",
];

const KATANA_DARK_COLORS: [&str; COLOR_SLOT_COUNT] = [
    "#1e1e1e",
    "#d4d4d4",
    "#3c3c3c",
    "#252526",
    "#1e1e1e",
    "#3c3c3c",
    "#8e8e8e",
    "#282828",
    "#282828",
    "#3c3c3c",
    "#264f78",
    "#252526",
    "#569cd6",
    "#569cd6",
    "#3c3c3c",
    "#8e8e8e",
    "#569cd6",
    "#c3e88d",
    "#569cd6",
    "#ffcb6b",
    "#f07178",
    "transparent",
    "#E0E0E0",
    "#2d2d2d",
    "#888888",
    "#aaaaaa",
    "dark",
    "base16-ocean.dark",
    "base16-ocean.light",
];

pub(super) fn katana_light() -> KdvThemeSnapshot {
    from_color_slots("katana-light", KdvThemeMode::Light, KATANA_LIGHT_COLORS)
}

pub(super) fn katana_dark() -> KdvThemeSnapshot {
    from_color_slots("katana-dark", KdvThemeMode::Dark, KATANA_DARK_COLORS)
}

fn from_color_slots(
    name: &str,
    mode: KdvThemeMode,
    colors: [&str; COLOR_SLOT_COUNT],
) -> KdvThemeSnapshot {
    kdv_theme_snapshot_from_colors!(name, mode, colors)
}
