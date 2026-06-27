use super::*;
use crate::export_surface_span::SurfaceTextSpan;

#[test]
fn font_size_matches_heading_levels() {
    let heading_1 = SurfaceLine::heading(1, "h1".to_string()).font_size();
    let heading_2 = SurfaceLine::heading(2, "h2".to_string()).font_size();
    let heading_3 = SurfaceLine::heading(3, "h3".to_string()).font_size();

    assert_eq!(heading_1, 40.0);
    assert_eq!(heading_2, 34.0);
    assert_eq!(heading_3, 28.0);
}

#[test]
fn default_line_heights_match_katana_export_reference() {
    let heading_1 = SurfaceLine::heading(1, "h1".to_string());
    let heading_2 = SurfaceLine::heading(2, "h2".to_string());
    let heading_3 = SurfaceLine::heading(3, "h3".to_string());
    let body = SurfaceLine::body("body".to_string());
    let code = SurfaceLine::code_spans(vec![SurfaceTextSpan::plain("code")]);

    assert_eq!(92, heading_1.line_height());
    assert_eq!(78, heading_2.line_height());
    assert_eq!(66, heading_3.line_height());
    assert_eq!(46, body.line_height());
    assert_eq!(34, code.line_height());
}

#[test]
fn document_typography_scales_body_heading_and_code_lines() {
    let typography = SurfaceTypographyConfig::from_body_font_size(14.0);
    let mut body = SurfaceLine::body("body".to_string());
    let mut heading = SurfaceLine::heading(1, "heading".to_string());
    let mut code = SurfaceLine::code_spans(vec![SurfaceTextSpan::plain("code")]);

    body.apply_typography(typography);
    heading.apply_typography(typography);
    code.apply_typography(typography);

    assert_eq!(14.0, body.font_size());
    assert_eq!(23, body.line_height());
    assert!((23.333_332 - heading.font_size()).abs() < f32::EPSILON);
    assert_eq!(40, heading.line_height());
    assert_eq!(12.0, code.font_size());
    assert_eq!(19, code.line_height());
}
