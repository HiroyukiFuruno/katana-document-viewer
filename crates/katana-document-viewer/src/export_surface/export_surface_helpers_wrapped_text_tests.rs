use super::*;

#[test]
fn empty_text_still_produces_one_empty_chunk() {
    let wrapped = WrappedText::new("", 5);
    let chunks: Vec<_> = wrapped.collect();
    assert_eq!(chunks, vec![String::new()]);
}

#[test]
fn wraps_text_by_character_count() {
    let wrapped = WrappedText::new("abcdef", 2);
    let chunks: Vec<_> = wrapped.collect();
    assert_eq!(chunks, vec!["ab", "cd", "ef"]);
}
