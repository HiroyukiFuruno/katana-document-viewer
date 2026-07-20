use super::*;

fn must_apply_root_font_size(source: &str, font_size: f32) -> String {
    match apply_root_font_size_css_unit(source, font_size) {
        Some(processed) => processed,
        None => std::panic::resume_unwind(Box::new(format!(
            "svg root font-size preprocessing failed for source: {source}"
        ))),
    }
}

#[test]
fn strips_foreign_object_nodes_from_svg() {
    let before = "<svg><foreignObject><body>keep</body></foreignObject><text>ok</text></svg>";
    let after = preprocess_for_rasterizer(before, None);

    assert!(!after.contains("foreignObject"));
    assert!(after.contains(">keep<"));
    assert!(after.contains("<text>ok</text>"));
}

#[test]
fn preprocess_converts_foreign_object_label_to_svg_text() {
    let before = concat!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="60">"#,
        r#"<foreignObject x="10" y="20" width="80" height="24">"#,
        r#"<div xmlns="http://www.w3.org/1999/xhtml" "#,
        r#"style="font-size: 12px; color: #E0E0E0;">Node &amp; Label</div>"#,
        r#"</foreignObject></svg>"#
    );

    let after = preprocess_for_rasterizer(before, None);

    assert!(!after.contains("<foreignObject"));
    assert!(after.contains("<text"));
    assert!(after.contains(r##"fill="#E0E0E0""##));
    assert!(after.contains(">Node &amp; Label<"));
}

#[test]
fn preprocess_uses_existing_svg_text_in_switch_fallback() {
    let before = concat!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="60">"#,
        r#"<switch><foreignObject x="10" y="20" width="80" height="24">"#,
        r#"<div xmlns="http://www.w3.org/1999/xhtml">HTML Label</div>"#,
        r#"</foreignObject><text x="10" y="20">SVG Label</text></switch></svg>"#
    );

    let after = preprocess_for_rasterizer(before, None);

    assert!(!after.contains("<foreignObject"));
    assert!(!after.contains("HTML Label"));
    assert!(after.contains("SVG Label"));
    assert_eq!(after.matches("<text").count(), 1);
}

#[test]
fn preprocess_removes_plantuml_processing_instructions() {
    let raw = concat!(
        r#"<?plantuml 1.2026.2?>"#,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="20px" height="20px">"#,
        r##"<g><?plantuml-src abc?><rect width="20" height="20" fill="#2D2D2D"/>"##,
        r##"<text x="2" y="12" fill="#E0E0E0">PUML</text></g></svg>"##
    );

    let processed = preprocess_for_rasterizer(raw, None);

    assert!(!processed.contains("<?plantuml"));
    assert!(processed.contains("PUML"));
}

#[test]
fn apply_root_font_size_inserts_style_attribute() {
    let original = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"><text>ok</text></svg>";
    let processed = must_apply_root_font_size(original, 16.0);

    assert!(processed.contains("style=\"font-size:16px;\""));
}

#[test]
fn preprocess_handles_light_dark_function_rewrite() {
    let raw = "<svg><g fill='light-dark(red, blue)'></g></svg>";
    let processed = preprocess_for_rasterizer(raw, None);

    assert!(processed.contains("fill='red'"));
    assert!(!processed.contains("light-dark("));
}

#[test]
fn preprocess_keeps_light_dark_if_missing_closing_parenthesis() {
    let raw = "<svg><g fill='light-dark(red)'></g></svg>";
    let processed = preprocess_for_rasterizer(raw, None);

    assert!(processed.contains("light-dark("));
    assert!(processed.contains("fill='light-dark(red)'"));
}

#[test]
fn locate_root_style_handles_single_and_double_quotes() {
    let with_single = "<svg style='color:#000' width='4' height='4'></svg>";
    let with_double = "<svg style=\"font-size:12px\" width=\"4\" height=\"4\"></svg>";

    let processed_single = must_apply_root_font_size(with_single, 12.0);
    let processed_double = must_apply_root_font_size(with_double, 12.0);

    assert!(
        processed_single.contains("style='color:#000;font-size:12px;'")
            || processed_single.contains("style='font-size:12px; color:#000;'")
            || processed_single.contains("style='color:#000; font-size:12px;'")
    );
    assert!(processed_double.contains("font-size:12px"));
}

#[test]
fn strip_foreign_objects_preserves_self_closing_tags() {
    let before = "<svg><foreignObject/><text>ok</text></svg>";
    let after = preprocess_for_rasterizer(before, None);

    assert!(!after.contains("foreignObject"));
    assert!(after.contains("<text>ok</text>"));
}

#[test]
fn locate_root_style_does_not_duplicate_existing_font_size() {
    let source = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\" style=\"font-size:10px;\"><rect/></svg>";
    let processed = must_apply_root_font_size(source, 20.0);

    assert!(processed.contains("font-size:10px"));
    assert!(!processed.contains("font-size:20px"));
}

#[test]
fn parse_light_dark_function_requires_second_color() {
    assert!(parse_light_dark_function("red)").is_none());
    assert!(parse_light_dark_function("red,blue)").is_some());
}

#[test]
fn root_font_size_skips_missing_or_unclosed_svg_root() {
    assert!(apply_root_font_size_css_unit("<g></g>", 16.0).is_none());
    assert!(apply_root_font_size_css_unit("<svg width=\"10\"", 16.0).is_none());
}

#[test]
fn style_attribute_without_quotes_gets_new_root_style() {
    let raw = "<svg style=color:#000 width=\"10\"><rect/></svg>";
    let processed = must_apply_root_font_size(raw, 18.0);

    assert!(processed.contains("style=\"font-size:18px;\""));
}

#[test]
fn strip_foreign_objects_keeps_unclosed_node_for_visibility() {
    let raw = "<svg><foreignObject><body>visible fallback</body>";
    let processed = preprocess_for_rasterizer(raw, None);

    assert!(processed.contains("foreignObject"));
    assert!(processed.contains("visible fallback"));
}

#[test]
fn light_dark_parser_handles_nested_functions_and_unclosed_content() {
    let nested = resolve_light_dark_functions("<svg fill=\"light-dark(rgb(1,2,3), blue)\"></svg>");
    let unclosed = resolve_light_dark_functions("<svg fill=\"light-dark(red, blue\"></svg>");

    assert!(nested.contains("rgb(1,2,3)"));
    assert!(!nested.contains("blue"));
    assert!(unclosed.contains("light-dark("));
}

#[test]
fn strip_unclosed_plantuml_processing_instruction_keeps_original_payload() {
    let raw = "<?plantuml 1.2026.2 <svg><text>kept</text></svg>";
    let processed = preprocess_for_rasterizer(raw, None);

    assert!(processed.contains("<?plantuml 1.2026.2 <svg>"));
    assert!(processed.contains("<text>kept</text>"));
}

#[test]
fn find_light_dark_function_is_case_insensitive() {
    assert_eq!(
        find_light_dark_function("fill=\"Light-Dark(red, blue)\""),
        Some(6)
    );
}

#[test]
fn parse_light_dark_function_returns_none_for_unclosed_expression() {
    assert!(parse_light_dark_function("rgb(1,2,3)").is_none());
}
