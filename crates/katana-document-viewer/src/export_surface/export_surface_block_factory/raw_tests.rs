use super::*;

#[test]
fn split_raw_into_wrapped_lines() {
    let mut blocks = Vec::new();
    SurfaceBlockFactory::append_raw(&mut blocks, "first line\nsecond line", 0, 0);

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "first line");
    assert_eq!(blocks[1].text_for_tests(), "second line");
}

#[test]
fn split_raw_for_nested_list_depth() {
    let mut blocks = Vec::new();
    SurfaceBlockFactory::append_raw(&mut blocks, "alpha\nbeta", 0, 2);

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "alpha");
    assert_eq!(blocks[1].text_for_tests(), "beta");
}
