use super::*;

#[test]
fn font_size_matches_heading_levels() {
    let heading_1 = SurfaceLine::heading(1, "h1".to_string()).font_size();
    let heading_2 = SurfaceLine::heading(2, "h2".to_string()).font_size();
    let heading_3 = SurfaceLine::heading(3, "h3".to_string()).font_size();

    assert_eq!(heading_1, 40.0);
    assert_eq!(heading_2, 34.0);
    assert_eq!(heading_3, 28.0);
}
