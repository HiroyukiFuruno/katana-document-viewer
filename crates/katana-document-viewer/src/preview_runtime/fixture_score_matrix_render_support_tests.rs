use super::*;
use crate::{MarkdownSource, PreviewConfig, PreviewOutputFactory, ViewerViewport};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_NONCE: AtomicUsize = AtomicUsize::new(1);
const SCORE_SOURCE: &str = "# Rendered export fixtures\n\n```mermaid\ngraph TD\n  A --> B\n```";

fn preview_outputs(content: String, document_id: String, height: f32) -> Vec<PreviewOutput> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content,
            document_id: Some(document_id),
        },
        &PreviewConfig {
            viewport: ViewerViewport {
                width: 1024.0,
                height,
            },
            ..PreviewConfig::default()
        },
        320.0,
    )
    .into_iter()
    .collect()
}

#[test]
fn fixture_rendered_export_bytes_from_output_reuses_cache() {
    let nonce = TEST_NONCE.fetch_add(1, Ordering::Relaxed);
    let outputs = preview_outputs(
        format!("```mermaid\ngraph TD\n  A --> B\n```\nnonce: {nonce}\n"),
        format!("assets/fixtures/score-matrix-render-cache-{nonce}.md"),
        320.0,
    );
    assert_eq!(1, outputs.len());
    let first = RenderedExportBytes::from_output(&outputs[0])
        .into_iter()
        .collect::<Vec<_>>();
    let second = RenderedExportBytes::from_output(&outputs[0])
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!((1, 1), (first.len(), second.len()));
    assert_eq!(first[0].html, second[0].html);
    assert_eq!(first[0].pdf, second[0].pdf);
    assert_eq!(first[0].png, second[0].png);
    assert_eq!(first[0].jpeg, second[0].jpeg);
}

#[test]
fn fixture_svg_encodes_expected_runtime_and_kind() {
    let svg = fixture_svg("mermaid", "katana-light");
    assert!(svg.contains("data-kdv-render-runtime=\"katana-render-runtime\""));
    assert!(svg.contains(r#"data-kdv-rendered="mermaid""#));
}

#[test]
fn diagram_kind_label_supports_all_kinds() {
    assert_eq!("mermaid", diagram_kind_label(&DiagramKind::Mermaid));
    assert_eq!("drawio", diagram_kind_label(&DiagramKind::DrawIo));
    assert_eq!("plantuml", diagram_kind_label(&DiagramKind::PlantUml));
}

#[test]
fn score_report_has_format_scores_for_rendered_output() {
    let outputs = preview_outputs(
        format!("{SCORE_SOURCE}\n"),
        "assets/fixtures/score-matrix-render-report.md".to_string(),
        512.0,
    );
    assert_eq!(1, outputs.len());
    let rendered = RenderedExportBytes::from_output(&outputs[0])
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, rendered.len());
    let report = rendered[0].score_report(SCORE_SOURCE);
    assert_eq!(4, report.format_scores.len());
    assert!(
        report
            .format_scores
            .iter()
            .all(|format_score| !format_score.checks.is_empty())
    );
}
