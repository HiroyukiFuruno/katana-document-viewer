use std::path::PathBuf;

const DEFAULT_WIDTH: usize = 1280;
const DEFAULT_HEIGHT: usize = 900;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorybookArgs {
    pub fixture_root: PathBuf,
    pub width: usize,
    pub height: usize,
    pub frames: usize,
    pub interactive: bool,
    pub clipboard_smoke: bool,
    pub clipboard_keyboard_smoke: bool,
    pub clipboard_drag_smoke: bool,
    pub selection_screenshot_smoke: bool,
    pub window_selection_screenshot_smoke: bool,
    pub window_hover_screenshot_smoke: bool,
    pub window_footnote_screenshot_smoke: bool,
    pub window_table_screenshot_smoke: bool,
    pub window_code_copy_screenshot_smoke: bool,
    pub slideshow_screenshot_smoke: bool,
    pub window_slideshow_screenshot_smoke: bool,
    pub window_sidebar_screenshot_smoke: bool,
    pub window_diagram_screenshot_smoke: bool,
    pub print_live_dark_toggle_point: bool,
    pub live_acceptance_artifact: bool,
    pub window_smoke_fixture: String,
    pub diagram_smoke_fixture: String,
    pub screenshot_output: PathBuf,
    pub light_screenshot_output: PathBuf,
}

impl StorybookArgs {
    pub fn parse<I>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
    {
        let mut parsed = Self::default();
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            parsed.apply_arg(arg, &mut iter)?;
        }
        Ok(parsed)
    }

    fn apply_arg<I>(&mut self, arg: String, iter: &mut I) -> Result<(), String>
    where
        I: Iterator<Item = String>,
    {
        match arg.as_str() {
            "--interactive" => {
                self.interactive = true;
                self.frames = 0;
            }
            "--clipboard-smoke" => self.clipboard_smoke = true,
            "--clipboard-keyboard-smoke" => self.clipboard_keyboard_smoke = true,
            "--clipboard-drag-smoke" => self.clipboard_drag_smoke = true,
            "--selection-screenshot-smoke" => self.selection_screenshot_smoke = true,
            "--window-selection-screenshot-smoke" => self.window_selection_screenshot_smoke = true,
            "--window-hover-screenshot-smoke" => self.window_hover_screenshot_smoke = true,
            "--window-footnote-screenshot-smoke" => self.window_footnote_screenshot_smoke = true,
            "--window-table-screenshot-smoke" => self.window_table_screenshot_smoke = true,
            "--window-code-copy-screenshot-smoke" => self.window_code_copy_screenshot_smoke = true,
            "--slideshow-screenshot-smoke" => self.slideshow_screenshot_smoke = true,
            "--window-slideshow-screenshot-smoke" => self.window_slideshow_screenshot_smoke = true,
            "--window-sidebar-screenshot-smoke" => self.window_sidebar_screenshot_smoke = true,
            "--window-diagram-screenshot-smoke" => self.window_diagram_screenshot_smoke = true,
            "--print-live-dark-toggle-point" => self.print_live_dark_toggle_point = true,
            "--live-acceptance-artifact" => self.live_acceptance_artifact = true,
            "--window-smoke-fixture" => {
                self.window_smoke_fixture = next_value(iter, "--window-smoke-fixture")?
            }
            "--diagram-smoke-fixture" => {
                self.diagram_smoke_fixture = next_value(iter, "--diagram-smoke-fixture")?
            }
            "--screenshot-output" => {
                self.screenshot_output = PathBuf::from(next_value(iter, "--screenshot-output")?)
            }
            "--light-screenshot-output" => {
                self.light_screenshot_output =
                    PathBuf::from(next_value(iter, "--light-screenshot-output")?)
            }
            "--smoke" => self.interactive = false,
            "--frames" => self.frames = parse_usize(next_value(iter, "--frames")?, "--frames")?,
            "--width" => self.width = parse_usize(next_value(iter, "--width")?, "--width")?,
            "--height" => self.height = parse_usize(next_value(iter, "--height")?, "--height")?,
            "--fixture-root" => {
                self.fixture_root = PathBuf::from(next_value(iter, "--fixture-root")?)
            }
            value => return Err(format!("unknown storybook argument: {value}")),
        }
        Ok(())
    }
}

impl Default for StorybookArgs {
    fn default() -> Self {
        Self {
            fixture_root: PathBuf::from("assets/fixtures"),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            frames: 1,
            interactive: false,
            clipboard_smoke: false,
            clipboard_keyboard_smoke: false,
            clipboard_drag_smoke: false,
            selection_screenshot_smoke: false,
            window_selection_screenshot_smoke: false,
            window_hover_screenshot_smoke: false,
            window_footnote_screenshot_smoke: false,
            window_table_screenshot_smoke: false,
            window_code_copy_screenshot_smoke: false,
            slideshow_screenshot_smoke: false,
            window_slideshow_screenshot_smoke: false,
            window_sidebar_screenshot_smoke: false,
            window_diagram_screenshot_smoke: false,
            print_live_dark_toggle_point: false,
            live_acceptance_artifact: false,
            window_smoke_fixture: "katana/sample.md".to_string(),
            diagram_smoke_fixture: "katana/sample_diagrams.md".to_string(),
            screenshot_output: PathBuf::from("target/kdv-storybook-selection-smoke.png"),
            light_screenshot_output: PathBuf::from("target/kdv-storybook-live-light-toggle.png"),
        }
    }
}

fn next_value<I>(iter: &mut I, name: &str) -> Result<String, String>
where
    I: Iterator<Item = String>,
{
    iter.next()
        .ok_or_else(|| format!("missing value for storybook argument: {name}"))
}

fn parse_usize(value: String, name: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|error| format!("invalid {name} value `{value}`: {error}"))
}

#[cfg(test)]
mod tests {
    use super::StorybookArgs;

    #[test]
    fn interactive_mode_runs_until_window_close() -> Result<(), String> {
        let args = StorybookArgs::parse(["--interactive".to_string()])?;

        assert!(args.interactive);
        assert_eq!(0, args.frames);
        Ok(())
    }

    #[test]
    fn clipboard_smoke_uses_headless_clipboard_check() -> Result<(), String> {
        let args = StorybookArgs::parse(["--clipboard-smoke".to_string()])?;

        assert!(args.clipboard_smoke);
        assert!(!args.interactive);
        Ok(())
    }

    #[test]
    fn clipboard_keyboard_smoke_uses_headless_keyboard_copy_check() -> Result<(), String> {
        let args = StorybookArgs::parse(["--clipboard-keyboard-smoke".to_string()])?;

        assert!(args.clipboard_keyboard_smoke);
        assert!(!args.interactive);
        Ok(())
    }

    #[test]
    fn clipboard_drag_smoke_uses_mouse_selection_copy_check() -> Result<(), String> {
        let args = StorybookArgs::parse(["--clipboard-drag-smoke".to_string()])?;

        assert!(args.clipboard_drag_smoke);
        assert!(!args.interactive);
        Ok(())
    }

    #[test]
    fn selection_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--selection-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-selection.png".to_string(),
        ])?;

        assert!(args.selection_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-selection.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_selection_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-selection-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-selection.png".to_string(),
        ])?;

        assert!(args.window_selection_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-selection.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_hover_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-hover-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-hover.png".to_string(),
        ])?;

        assert!(args.window_hover_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-hover.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_smoke_fixture_can_target_direct_html_margin_review() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-hover-screenshot-smoke".to_string(),
            "--window-smoke-fixture".to_string(),
            "direct/html-alignment.html".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-html-margin.png".to_string(),
        ])?;

        assert!(args.window_hover_screenshot_smoke);
        assert_eq!("direct/html-alignment.html", args.window_smoke_fixture);
        assert_eq!(
            std::path::Path::new("target/example-window-html-margin.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_footnote_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-footnote-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-footnote.png".to_string(),
        ])?;

        assert!(args.window_footnote_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-footnote.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_table_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-table-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-table.png".to_string(),
        ])?;

        assert!(args.window_table_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-table.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_code_copy_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-code-copy-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-code-copy.png".to_string(),
        ])?;

        assert!(args.window_code_copy_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-code-copy.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn slideshow_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--slideshow-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-slideshow.png".to_string(),
        ])?;

        assert!(args.slideshow_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-slideshow.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_slideshow_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-slideshow-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-slideshow.png".to_string(),
        ])?;

        assert!(args.window_slideshow_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-slideshow.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_diagram_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-diagram-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-diagram.png".to_string(),
        ])?;

        assert!(args.window_diagram_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-diagram.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_sidebar_screenshot_smoke_writes_requested_output_path() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-sidebar-screenshot-smoke".to_string(),
            "--screenshot-output".to_string(),
            "target/example-window-sidebar.png".to_string(),
        ])?;

        assert!(args.window_sidebar_screenshot_smoke);
        assert_eq!(
            std::path::Path::new("target/example-window-sidebar.png"),
            args.screenshot_output
        );
        Ok(())
    }

    #[test]
    fn live_dark_toggle_point_can_be_printed_for_acceptance_click() -> Result<(), String> {
        let args = StorybookArgs::parse(["--print-live-dark-toggle-point".to_string()])?;

        assert!(args.print_live_dark_toggle_point);
        assert!(!args.interactive);
        Ok(())
    }

    #[test]
    fn live_acceptance_artifact_writes_dark_and_light_outputs() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--live-acceptance-artifact".to_string(),
            "--screenshot-output".to_string(),
            "target/acceptance/dark.png".to_string(),
            "--light-screenshot-output".to_string(),
            "target/acceptance/light.png".to_string(),
        ])?;

        assert!(args.live_acceptance_artifact);
        assert!(!args.interactive);
        assert_eq!(
            std::path::Path::new("target/acceptance/dark.png"),
            args.screenshot_output
        );
        assert_eq!(
            std::path::Path::new("target/acceptance/light.png"),
            args.light_screenshot_output
        );
        Ok(())
    }

    #[test]
    fn window_size_arguments_override_default_window_size() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--width".to_string(),
            "2048".to_string(),
            "--height".to_string(),
            "1496".to_string(),
        ])?;

        assert_eq!(2048, args.width);
        assert_eq!(1496, args.height);
        Ok(())
    }

    #[test]
    fn diagram_smoke_fixture_can_target_standalone_drawio() -> Result<(), String> {
        let args = StorybookArgs::parse([
            "--window-diagram-screenshot-smoke".to_string(),
            "--diagram-smoke-fixture".to_string(),
            "katana/drawio/basic/03-basic-flow.drawio".to_string(),
        ])?;

        assert!(args.window_diagram_screenshot_smoke);
        assert_eq!(
            "katana/drawio/basic/03-basic-flow.drawio",
            args.diagram_smoke_fixture
        );
        Ok(())
    }
}
