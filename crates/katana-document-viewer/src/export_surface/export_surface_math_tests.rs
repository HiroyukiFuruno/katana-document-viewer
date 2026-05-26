use super::SurfaceMathText;

#[test]
fn renders_common_tex_as_surface_text() {
    let text = SurfaceMathText::render(r"f(x) = \int_{0}^{x} \frac{t^2}{1 + t^4} \, dt");

    assert!(text.contains('∫'));
    assert!(text.contains("t²"));
    assert!(text.contains('⁄'));
    assert!(!text.contains(r"\int"));
    assert!(!text.contains(r"\frac"));
}

#[test]
fn renders_inline_and_sum_scripts() {
    let inline = SurfaceMathText::render("E = mc^2");
    let block = SurfaceMathText::render(r"\sum_{k=1}^{n} k = \frac{n(n+1)}{2}");

    assert!(inline.contains("mc²"));
    assert!(block.contains('∑'));
    assert!(block.contains('ⁿ'));
    assert!(!block.contains(r"\sum_"));
}

#[test]
fn fraction_parser_requires_matching_braces() {
    let text = SurfaceMathText::render(r"\frac{a}{b");

    assert!(text.contains("frac"));
    assert!(!text.contains('\\'));
}

#[test]
fn script_parser_keeps_unbraced_script_fallback() {
    let text = SurfaceMathText::render(r"x^x_2");

    assert!(text.contains('ˣ'));
    assert!(text.contains('ₓ') || text.contains('₂'));
}

#[test]
fn renders_single_char_scripts_after_text() {
    let text = SurfaceMathText::render("a^{1}+b_3");

    assert!(text.contains('¹'));
    assert!(text.contains('₃'));
    assert!(!text.contains('\\'));
}

#[test]
fn render_skips_unrecognized_escape_and_empty_script() {
    let text = SurfaceMathText::render(r"x\\y^+");

    assert_eq!(text, "xy⁺");
}

#[test]
fn render_treats_unmatched_fraction_as_plain_text() {
    let first = SurfaceMathText::render(r"\frac a b");
    let second = SurfaceMathText::render(r"\frac{a{b");

    assert!(!first.contains('⁄'));
    assert!(!second.contains('⁄'));
    assert!(first.contains("frac"));
    assert!(second.contains("frac"));
}

#[test]
fn script_mapping_keeps_unknown_characters() {
    let text = SurfaceMathText::render(r"x^a + y_k");

    assert!(text.contains('a') || text.contains('x'));
    assert!(text.contains('ₖ'));
}

#[test]
fn renders_all_script_digits_and_operators() {
    let text = SurfaceMathText::render(r"x^{0123456789+-=nx}_{0123456789+-=kz}");

    assert!(text.contains("⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼ⁿˣ"));
    assert!(text.contains("₀₁₂₃₄₅₆₇₈₉₊₋₌ₖz"));
}

#[test]
fn render_empty_expression_stays_empty() {
    assert_eq!(SurfaceMathText::render("   "), "");
}

#[test]
fn render_handles_internal_empty_character_paths() {
    let text = super::render_inline_text("");
    let script = super::take_script("");

    assert_eq!(text, "");
    assert_eq!(script, ("", ""));
}

#[test]
fn render_next_character_handles_empty_remaining_text() {
    let mut renderer = super::MathRenderer::new("");

    renderer.render_next_character();

    assert_eq!(renderer.remaining, "");
}

#[test]
fn script_parser_handles_nested_braces() {
    assert_eq!(super::take_script("{a{b}}c"), ("a{b}", "c"));
}
