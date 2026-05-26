use super::shields_badge;

fn require_badge(
    src: &str,
    link: Option<String>,
) -> Result<super::SurfaceBadge, Box<dyn std::error::Error>> {
    shields_badge(src, link)
        .ok_or_else(|| std::io::Error::other("shields badge should parse").into())
}

#[test]
fn shields_badge_parses_known_color_and_link() -> Result<(), Box<dyn std::error::Error>> {
    let badge = require_badge(
        "https://img.shields.io/badge/Type-Value-blue?labelColor=green",
        Some("/to/target".to_string()),
    )?;

    assert_eq!(badge.label, "Type");
    assert_eq!(badge.message, "Value");
    assert_eq!(badge.link_target, Some("/to/target".to_string()));
    Ok(())
}

#[test]
fn shields_badge_defaults_unknown_color_to_default_style() -> Result<(), Box<dyn std::error::Error>>
{
    let badge = require_badge("https://img.shields.io/badge/left-right-unknown", None)?;

    assert_eq!(badge.label, "left");
    assert_eq!(badge.message, "right");
    Ok(())
}

#[test]
fn shields_badge_decodes_percent_encoding_and_keeps_invalid_sequences()
-> Result<(), Box<dyn std::error::Error>> {
    let badge = require_badge(
        "https://img.shields.io/badge/left%20text-right%20text-%25bad",
        None,
    )?;

    assert_eq!(badge.label, "left text");
    assert_eq!(badge.message, "right text");
    assert_eq!(badge.text(), "left text=right text");
    Ok(())
}

#[test]
fn shields_badge_rejects_invalid_src_without_badge_segment() {
    assert!(shields_badge("https://example.com/not-a-badge", None).is_none());
}

#[test]
fn shields_badge_uses_default_color_for_plain_text_color_name()
-> Result<(), Box<dyn std::error::Error>> {
    let plain = require_badge("https://img.shields.io/badge/one-two-plain", None)?;
    let fallback = require_badge("https://img.shields.io/badge/one-two-lightgrey", None)?;

    assert_eq!(plain.color, fallback.color);
    Ok(())
}

#[test]
fn shields_badge_decodes_uppercase_hex_and_invalid_utf8_falls_back()
-> Result<(), Box<dyn std::error::Error>> {
    let uppercase = require_badge("https://img.shields.io/badge/LEFT%20A-RIGHT%2FZ-red", None)?;
    let invalid_utf8 = require_badge("https://img.shields.io/badge/%FF-right-green", None)?;

    assert_eq!(uppercase.label, "LEFT A");
    assert_eq!(uppercase.message, "RIGHT/Z");
    assert_eq!(invalid_utf8.label, "%FF");
    Ok(())
}

#[test]
fn shields_badge_decodes_lowercase_hex_and_keeps_bad_hex_sequences() {
    let lowercase = shields_badge("https://img.shields.io/badge/left-right%2fside-blue", None)
        .map(|badge| badge.message);
    let invalid =
        shields_badge("https://img.shields.io/badge/%G0-right-blue", None).map(|badge| badge.label);

    assert_eq!(lowercase, Some("right/side".to_string()));
    assert_eq!(invalid, Some("%G0".to_string()));
}
