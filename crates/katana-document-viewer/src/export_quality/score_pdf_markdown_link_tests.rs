use super::score_pdf_test_support::{draw_text_like_marks, pdf_rgb_len, pdf_with_rgb};
use super::*;

#[test]
fn unresolved_reference_link_does_not_require_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, "[link][missing]\n");

    assert_passed_check(&score, "pdf keeps link annotations");
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
    Ok(())
}

#[test]
fn unresolved_collapsed_reference_link_does_not_require_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, "[link][]\n");

    assert_passed_check(&score, "pdf keeps link annotations");
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
    Ok(())
}

fn assert_passed_check(score: &ExportFormatQualityScore, name: &str) {
    assert!(
        score
            .checks
            .iter()
            .any(|check| check.name == name && check.passed)
    );
}
