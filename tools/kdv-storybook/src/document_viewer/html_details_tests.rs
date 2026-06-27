use super::DetailsParts;

#[test]
fn parses_summary_and_body_without_wrapper_tags() -> Result<(), String> {
    let details = "<details><summary>Title</summary><div>Body<br>Line</div></details>";

    let parts = DetailsParts::parse(details).ok_or_else(|| "details should parse".to_string())?;

    assert_eq!("Title", parts.summary);
    assert_eq!("Body\nLine", parts.body);
    assert!(!parts.open);
    Ok(())
}

#[test]
fn parses_open_attribute_and_summary_attributes() -> Result<(), String> {
    let details = "<details open><summary class=\"x\">Title</summary><p>Body</p></details>";

    let parts = DetailsParts::parse(details).ok_or_else(|| "details should parse".to_string())?;

    assert_eq!("Title", parts.summary);
    assert_eq!("Body", parts.body);
    assert!(parts.open);
    Ok(())
}

#[test]
fn parses_open_attribute_with_explicit_value() -> Result<(), String> {
    let details = "<details open=\"true\"><summary>Title</summary><p>Body</p></details>";

    let parts = DetailsParts::parse(details).ok_or_else(|| "details should parse".to_string())?;

    assert!(parts.open);
    assert_eq!("Title", parts.summary);
    assert_eq!("Body", parts.body);
    Ok(())
}

#[test]
fn parse_returns_none_when_not_details() {
    assert!(DetailsParts::parse("<summary>Title</summary>").is_none());
}

#[test]
fn plain_text_fallback_splits_summary_and_body() -> Result<(), String> {
    let parts = DetailsParts::from_plain_text("Details title\n\nDetails body")
        .ok_or_else(|| "plain details should parse".to_string())?;

    assert_eq!("Details title", parts.summary);
    assert_eq!("Details body", parts.body);
    assert!(parts.open);
    Ok(())
}
