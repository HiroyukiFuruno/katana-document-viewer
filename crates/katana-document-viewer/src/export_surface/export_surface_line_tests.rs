use super::scale_for;

#[test]
fn scale_for_uses_1_for_infinite_or_invalid_values() {
    assert_eq!(scale_for(0.0, 24.0), 1.0);
    assert_eq!(scale_for(-10.0, 24.0), 1.0);
    assert_eq!(scale_for(f32::NAN, 24.0), 1.0);
    assert_eq!(scale_for(f32::INFINITY, 24.0), 1.0);
}
