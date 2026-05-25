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
