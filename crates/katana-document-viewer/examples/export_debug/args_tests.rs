use super::*;
use std::error::Error;

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
fn complete_theme_json_is_read() -> Result<(), Box<dyn Error>> {
    let theme_json = ThemeJsonFixture::complete_json()?;
    let args = parse_args(&["--theme", &theme_json, "input.md", "out"])?;

    assert_eq!(args.theme.name, "cli-json");
    assert_eq!(args.theme.background, "#010203");
    assert_eq!(args.input_path, PathBuf::from("input.md"));
    Ok(())
}

#[test]
fn partial_theme_json_is_rejected() -> Result<(), Box<dyn Error>> {
    let error = parse_args(&["--theme", r#"{"name":"partial"}"#, "input.md", "out"])
        .err()
        .ok_or("expected partial theme JSON to fail")?;

    assert!(error.to_string().contains("missing field"));
    Ok(())
}

#[test]
fn theme_json_cannot_be_combined_with_light_or_dark() -> Result<(), Box<dyn Error>> {
    let theme_json = ThemeJsonFixture::complete_json()?;
    let light_error = parse_args(&["--light", "--theme", &theme_json, "input.md", "out"])
        .err()
        .ok_or("expected combined light theme options to fail")?;
    let dark_error = parse_args(&["--dark", "--theme", &theme_json, "input.md", "out"])
        .err()
        .ok_or("expected combined dark theme options to fail")?;

    assert_eq!(light_error.to_string(), "theme option cannot be combined");
    assert_eq!(dark_error.to_string(), "theme option cannot be combined");
    Ok(())
}

#[test]
fn thema_spelling_is_unknown_option() -> Result<(), Box<dyn Error>> {
    let error = parse_args(&["--thema", "{}", "input.md", "out"])
        .err()
        .ok_or("--thema must not be accepted")?;

    assert_eq!(error.to_string(), "unknown option: --thema");
    Ok(())
}

struct ThemeJsonFixture;

impl ThemeJsonFixture {
    fn complete_json() -> Result<String, Box<dyn Error>> {
        let mut theme = KdvThemeSnapshot::katana_dark();
        theme.name = "cli-json".to_string();
        theme.background = "#010203".to_string();
        Ok(serde_json::to_string(&theme)?)
    }
}

fn parse_args(values: &[&str]) -> Result<CommandArgs, Box<dyn Error>> {
    CommandArgsParser::parse_from(values.iter().map(|value| value.to_string()))
}
