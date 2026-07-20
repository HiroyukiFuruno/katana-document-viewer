use super::ViewerSpanWrapper;

#[test]
fn plain_surface_text_preserves_explicit_line_breaks() {
    let spans = ViewerSpanWrapper::wrap_plain_surface_text("one\ntwo".to_string());

    assert_eq!(
        vec!["one", "\n", "two"],
        spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<Vec<_>>()
    );
}
