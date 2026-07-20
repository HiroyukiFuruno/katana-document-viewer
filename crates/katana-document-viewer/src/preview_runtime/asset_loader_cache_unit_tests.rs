#[cfg(test)]
use super::*;
#[cfg(test)]
use katana_markdown_model::DiagramKind;
#[cfg(test)]
use std::path::Path;

#[test]
fn source_key_normalizes_line_endings() {
    let crlf = PreviewDiagramAssetCache::source_key(&DiagramKind::Mermaid, "graph\r\nA --> B\rC");
    let normalized =
        PreviewDiagramAssetCache::source_key(&DiagramKind::Mermaid, "graph\nA --> B\nC");

    assert_eq!(crlf, normalized);
}

#[test]
fn hash_stays_stable_for_same_input() {
    let first = PreviewDiagramAssetCache::hash("diagram source");
    let second = PreviewDiagramAssetCache::hash("diagram source");
    let different = PreviewDiagramAssetCache::hash("other source");

    assert_eq!(first, second);
    assert_ne!(first, different);
}

#[test]
fn is_svg_distinguishes_svg_payload() {
    assert!(PreviewDiagramAssetCache::is_svg(
        b"<svg xmlns=\"http://www.w3.org/2000/svg\"><text>ok</text></svg>"
    ));
    assert!(!PreviewDiagramAssetCache::is_svg(
        b"<svg xmlns=\"http://www.w3.org/2000/svg\"><text>missing close</text>"
    ));
    assert!(!PreviewDiagramAssetCache::is_svg(b"\xFF\xFE\xFD"));
}

#[test]
fn temp_path_includes_original_file_name_suffix() {
    let path = Path::new("/tmp/kdv/diagram.svg");
    let temp = PreviewDiagramAssetCache::temp_path(path);

    assert!(
        temp.file_name()
            .and_then(|value| value.to_str())
            .is_some_and(|name| name.starts_with("diagram.svg.tmp."))
    );
    assert_eq!(path.parent(), temp.parent());
}
