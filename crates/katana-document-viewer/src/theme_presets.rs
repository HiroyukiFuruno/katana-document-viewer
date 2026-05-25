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
    "#24292f",
    "#d0d7de",
    "#eaf5ff",
    "#f7fbff",
    "#d0d7de",
    "#57606a",
    "#f6f8fa",
    "#f6f8fa",
    "#d0d7de",
    "#add6ff",
    "#f3f3f3",
    "#0078d4",
    "#0078d4",
    "#d0d7de",
    "#57606a",
    "#0969da",
    "#1a7f37",
    "#8250df",
    "#d1242f",
    "#bf8700",
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
    "#0d1117",
    "#f0f6fc",
    "#30363d",
    "#161b22",
    "#111820",
    "#484f58",
    "#8b949e",
    "#161b22",
    "#161b22",
    "#30363d",
    "#264f78",
    "#252526",
    "#569cd6",
    "#569cd6",
    "#30363d",
    "#8b949e",
    "#58a6ff",
    "#3fb950",
    "#a371f7",
    "#f85149",
    "#d29922",
    "transparent",
    "#f0f6fc",
    "#1f2937",
    "#8b949e",
    "#8b949e",
    "dark",
    "base16-ocean.dark",
    "InspiredGitHub",
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
