use katana_document_viewer::{ExportFormat, KdvThemeSnapshot};
use std::env;
use std::error::Error;
use std::path::PathBuf;

pub(crate) const EXPORT_FORMATS: [ExportFormat; 4] = [
    ExportFormat::Html,
    ExportFormat::Pdf,
    ExportFormat::Png,
    ExportFormat::Jpeg,
];

#[derive(Debug)]
pub(crate) struct CommandArgs {
    pub(crate) input_path: PathBuf,
    pub(crate) output_dir: PathBuf,
    pub(crate) theme: KdvThemeSnapshot,
}

pub(crate) struct CommandArgsParser;

impl CommandArgsParser {
    pub(crate) fn parse() -> Result<CommandArgs, Box<dyn Error>> {
        Self::parse_from(env::args().skip(1))
    }

    pub(crate) fn parse_from<I>(args: I) -> Result<CommandArgs, Box<dyn Error>>
    where
        I: IntoIterator<Item = String>,
    {
        let mut args = args.into_iter();
        let mut theme = KdvThemeSnapshot::katana_light();
        let first = required_arg(&mut args, "missing input markdown path")?;
        let input_path = match first.as_str() {
            "--light" => required_arg(&mut args, "missing input markdown path")?,
            "--dark" => {
                theme = KdvThemeSnapshot::katana_dark();
                required_arg(&mut args, "missing input markdown path")?
            }
            "--theme" => return Err(invalid_input("--theme is tracked by a later change").into()),
            "--thema" => return Err(invalid_input("--thema is not supported").into()),
            value => value.to_string(),
        };
        let output_dir = required_arg(&mut args, "missing output directory")?;
        if args.next().is_some() {
            return Err(invalid_input("too many arguments").into());
        }
        Ok(CommandArgs {
            input_path: PathBuf::from(input_path),
            output_dir: PathBuf::from(output_dir),
            theme,
        })
    }
}

fn required_arg(
    args: &mut impl Iterator<Item = String>,
    message: &'static str,
) -> Result<String, std::io::Error> {
    args.next().ok_or_else(|| invalid_input(message))
}

pub(crate) fn invalid_input(message: &'static str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_is_katana_light() -> Result<(), Box<dyn Error>> {
        let args = parse_args(&["input.md", "out"])?;

        assert_eq!(args.theme.name, "katana-light");
        assert_eq!(args.input_path, PathBuf::from("input.md"));
        assert_eq!(args.output_dir, PathBuf::from("out"));
        Ok(())
    }

    #[test]
    fn light_theme_is_katana_light() -> Result<(), Box<dyn Error>> {
        let args = parse_args(&["--light", "input.md", "out"])?;

        assert_eq!(args.theme.name, "katana-light");
        assert_eq!(args.input_path, PathBuf::from("input.md"));
        Ok(())
    }

    #[test]
    fn dark_theme_is_katana_dark() -> Result<(), Box<dyn Error>> {
        let args = parse_args(&["--dark", "input.md", "out"])?;

        assert_eq!(args.theme.name, "katana-dark");
        assert_eq!(args.output_dir, PathBuf::from("out"));
        Ok(())
    }

    #[test]
    fn theme_json_entry_is_rejected_until_later_change() {
        let error = parse_args(&["--theme", "theme.json", "input.md", "out"])
            .expect_err("theme JSON entry must be rejected in this change");

        assert_eq!(error.to_string(), "--theme is tracked by a later change");
    }

    #[test]
    fn thema_spelling_is_rejected() {
        let error = parse_args(&["--thema", "theme.json", "input.md", "out"])
            .expect_err("--thema must not be accepted");

        assert_eq!(error.to_string(), "--thema is not supported");
    }

    fn parse_args(values: &[&str]) -> Result<CommandArgs, Box<dyn Error>> {
        CommandArgsParser::parse_from(values.iter().map(|value| value.to_string()))
    }
}
