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
        RawCommandArgsParser::new(args.into_iter()).parse()
    }
}

struct RawCommandArgsParser<I> {
    args: I,
    parsed: ParsedCommandArgs,
    parses_options: bool,
}

impl<I> RawCommandArgsParser<I>
where
    I: Iterator<Item = String>,
{
    fn new(args: I) -> Self {
        Self {
            args,
            parsed: ParsedCommandArgs::new(),
            parses_options: true,
        }
    }

    fn parse(mut self) -> Result<CommandArgs, Box<dyn Error>> {
        while let Some(arg) = self.args.next() {
            self.accept(arg)?;
        }
        self.parsed.into_command_args()
    }

    fn accept(&mut self, arg: String) -> Result<(), Box<dyn Error>> {
        if !self.parses_options {
            self.parsed.push_positional(arg);
            return Ok(());
        }
        match arg.as_str() {
            "--" => {
                self.parses_options = false;
                Ok(())
            }
            "--light" => self.parsed.set_theme(KdvThemeSnapshot::katana_light()),
            "--dark" => self.parsed.set_theme(KdvThemeSnapshot::katana_dark()),
            "--theme" => {
                let json = self.required_arg("missing theme JSON")?;
                self.parsed.set_theme(serde_json::from_str(&json)?)
            }
            "--thema" => Err(unknown_option("--thema").into()),
            _ => {
                self.parses_options = false;
                self.parsed.push_positional(arg);
                Ok(())
            }
        }
    }

    fn required_arg(&mut self, message: &'static str) -> Result<String, Box<dyn Error>> {
        self.args
            .next()
            .ok_or_else(|| invalid_input(message).into())
    }
}

struct ParsedCommandArgs {
    theme: KdvThemeSnapshot,
    theme_is_set: bool,
    positional_args: Vec<String>,
}

impl ParsedCommandArgs {
    fn new() -> Self {
        Self {
            theme: KdvThemeSnapshot::katana_light(),
            theme_is_set: false,
            positional_args: Vec::new(),
        }
    }

    fn set_theme(&mut self, theme: KdvThemeSnapshot) -> Result<(), Box<dyn Error>> {
        if self.theme_is_set {
            return Err(invalid_input("theme option cannot be combined").into());
        }
        self.theme_is_set = true;
        self.theme = theme;
        Ok(())
    }

    fn push_positional(&mut self, value: String) {
        self.positional_args.push(value);
    }

    fn into_command_args(self) -> Result<CommandArgs, Box<dyn Error>> {
        if self.positional_args.len() != 2 {
            return Err(positional_error(self.positional_args.len()).into());
        }
        Ok(CommandArgs {
            input_path: PathBuf::from(self.positional_args[0].clone()),
            output_dir: PathBuf::from(self.positional_args[1].clone()),
            theme: self.theme,
        })
    }
}

fn positional_error(count: usize) -> std::io::Error {
    match count {
        0 => invalid_input("missing input markdown path"),
        1 => invalid_input("missing output directory"),
        _ => invalid_input("too many arguments"),
    }
}

fn unknown_option(value: &str) -> std::io::Error {
    invalid_input(format!("unknown option: {value}"))
}

fn invalid_input(message: impl Into<String>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, message.into())
}

#[cfg(test)]
#[path = "args_tests.rs"]
mod tests;
