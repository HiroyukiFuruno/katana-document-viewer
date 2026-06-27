use super::score_pdf_test_support::{
    draw_single_dot, draw_text_like_marks, pdf_rgb_len, pdf_with_rgb,
};
use super::*;

#[test]
fn blank_pdf_image_stream_fails_visual_content_check() -> std::io::Result<()> {
    let rgb = vec![255; pdf_rgb_len()];
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, "");

    assert_failed_check(&score, "pdf image stream is not visually blank");
    assert!(score.score < 95, "{score:#?}");
    assert!(!score.is_pass(), "{score:#?}");
    Ok(())
}

#[test]
fn text_like_pdf_image_stream_passes_visual_content_check() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, "");

    assert_passed_check(&score, "pdf image stream is not visually blank");
    assert!(score.fatal_failures().is_empty());
    Ok(())
}

#[test]
fn single_dot_pdf_image_stream_fails_visual_content_check() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_single_dot(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, "");

    assert_failed_check(&score, "pdf image stream is not visually blank");
    assert!(score.score < 95, "{score:#?}");
    assert!(!score.is_pass(), "{score:#?}");
    Ok(())
}

#[test]
fn reference_markdown_link_requires_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score =
        ExportQualityGate::score_pdf(&pdf, "[Reference][id]\n\n[id]: https://example.com\n");

    assert_failed_check(&score, "pdf keeps link annotations");
    Ok(())
}

#[test]
fn reference_markdown_link_accepts_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let mut pdf = pdf_with_rgb(&rgb)?;
    pdf.extend_from_slice(b"\n<< /Subtype /Link >>\n");

    let score =
        ExportQualityGate::score_pdf(&pdf, "[Reference][id]\n\n[id]: https://example.com\n");

    assert_passed_check(&score, "pdf keeps link annotations");
    assert!(score.fatal_failures().is_empty());
    Ok(())
}

#[test]
fn shortcut_reference_markdown_link_requires_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score =
        ExportQualityGate::score_pdf(&pdf, "[Shortcut]\n\n[Shortcut]: https://example.com\n");

    assert_failed_check(&score, "pdf keeps link annotations");
    Ok(())
}

#[test]
fn autolink_requires_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, "<https://example.com>\n");

    assert_failed_check(&score, "pdf keeps link annotations");
    Ok(())
}

#[test]
fn email_autolink_requires_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, "<user@example.com>\n");

    assert_failed_check(&score, "pdf keeps link annotations");
    Ok(())
}

#[test]
fn uppercase_html_link_requires_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;

    let score = ExportQualityGate::score_pdf(&pdf, r#"<A HREF="https://example.com">Link</A>"#);

    assert_failed_check(&score, "pdf keeps link annotations");
    Ok(())
}

#[test]
fn html_link_with_attributes_requires_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;
    let source = r#"<a class="button" href="https://example.com">Link</a>"#;

    let score = ExportQualityGate::score_pdf(&pdf, source);

    assert_failed_check(&score, "pdf keeps link annotations");
    Ok(())
}

#[test]
fn html_link_with_newline_attribute_requires_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;
    let source = "<a\nhref=\"https://example.com\">Link</a>";

    let score = ExportQualityGate::score_pdf(&pdf, source);

    assert_failed_check(&score, "pdf keeps link annotations");
    Ok(())
}

#[test]
fn html_data_href_without_href_does_not_require_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;
    let source = r#"<a data-href="https://example.com">Not a link</a>"#;

    let score = ExportQualityGate::score_pdf(&pdf, source);

    assert_passed_check(&score, "pdf keeps link annotations");
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
    Ok(())
}

#[test]
fn fenced_markdown_link_does_not_require_pdf_annotation() -> std::io::Result<()> {
    let mut rgb = vec![255; pdf_rgb_len()];
    draw_text_like_marks(&mut rgb);
    let pdf = pdf_with_rgb(&rgb)?;
    let source = "```text\n[link](https://example.com)\n```\n";

    let score = ExportQualityGate::score_pdf(&pdf, source);

    assert_passed_check(&score, "pdf keeps link annotations");
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
    Ok(())
}

fn assert_failed_check(score: &ExportFormatQualityScore, name: &str) {
    assert!(
        score
            .checks
            .iter()
            .any(|check| check.name == name && !check.passed)
    );
}

fn assert_passed_check(score: &ExportFormatQualityScore, name: &str) {
    assert!(
        score
            .checks
            .iter()
            .any(|check| check.name == name && check.passed)
    );
}
