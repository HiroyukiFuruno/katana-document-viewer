use super::SurfaceCodeHighlighter;

#[test]
fn highlights_plain_body_lines_when_language_is_absent() {
    let lines = SurfaceCodeHighlighter::highlight(None, "line one\nline two\n");

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0][0].text, "line one");
    assert!(lines[0][0].style.monospace);
    assert!(!lines[0][0].style.bold);
    assert_eq!(lines[1][0].text, "line two");
}
