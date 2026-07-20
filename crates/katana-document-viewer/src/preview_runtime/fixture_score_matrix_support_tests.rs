use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_NONCE: AtomicUsize = AtomicUsize::new(1);

fn preview_outputs(content: String, document_id: String) -> Vec<PreviewOutput> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content,
            document_id: Some(document_id),
        },
        &config(),
        CONTENT_HEIGHT,
    )
    .into_iter()
    .collect()
}

#[test]
fn fixture_score_case_builds_preview_output() {
    let fixture = FixtureScoreCase {
        name: "preview-output-contract",
        document_id: "assets/fixtures/preview-output-contract.md",
        content: "# Preview output contract",
    };
    let outputs = fixture.preview_output().into_iter().collect::<Vec<_>>();
    assert_eq!(1, outputs.len());
    assert_eq!("preview-output-contract", fixture.name);
    assert_eq!(
        fixture.document_id,
        outputs[0].input.snapshot.source_path.to_string_lossy()
    );
}

#[test]
fn fixture_score_case_propagates_preview_errors() {
    let fixture = FixtureScoreCase {
        name: "empty-preview-error",
        document_id: "assets/fixtures/empty-preview-error.md",
        content: "",
    };
    assert!(fixture.preview_output().is_err());
}

#[test]
fn fixture_export_cache_propagates_build_errors() {
    let key = ExportCacheKey {
        document_id: "error-document".to_string(),
        revision: "error-revision".to_string(),
        kind: "Markdown".to_string(),
        source_path: "error.md".to_string(),
    };
    let result = cache_built_export(
        key,
        Err(ForgeError::Export("fixture build failed".to_string())),
    );
    assert!(matches!(
        result,
        Err(ForgeError::Export(message)) if message == "fixture build failed"
    ));
}

#[test]
fn fixture_export_bytes_propagate_payload_errors() {
    let result =
        export_bytes_from_payloads(Err(ForgeError::Export("payload build failed".to_string())));
    assert!(matches!(
        result,
        Err(ForgeError::Export(message)) if message == "payload build failed"
    ));
}

#[test]
fn raster_payload_join_preserves_format_error_precedence() {
    let ok = || Ok(vec![1]);
    let cases = [
        (
            Err(ForgeError::Export("pdf".to_string())),
            ok(),
            ok(),
            "pdf",
        ),
        (
            ok(),
            Err(ForgeError::Export("png".to_string())),
            ok(),
            "png",
        ),
        (
            ok(),
            ok(),
            Err(ForgeError::Export("jpeg".to_string())),
            "jpeg",
        ),
    ];
    for (pdf, png, jpeg, expected) in cases {
        let result = join_raster_payloads(pdf, png, jpeg);
        assert!(matches!(
            result,
            Err(ForgeError::Export(message)) if message == expected
        ));
    }
}

#[test]
fn fixture_export_payloads_propagate_raster_errors() {
    let result = fixture_export_payloads(
        b"<html></html>".to_vec(),
        Err(ForgeError::Export("raster failed".to_string())),
    );
    assert!(matches!(
        result,
        Err(ForgeError::Export(message)) if message == "raster failed"
    ));
}

#[test]
fn fixture_export_bytes_from_output_reuses_cache() {
    let nonce = TEST_NONCE.fetch_add(1, Ordering::Relaxed);
    let outputs = preview_outputs(
        format!("# score matrix\nnonce: {nonce}\n"),
        format!("assets/fixtures/score-matrix-export-cache-{nonce}.md"),
    );
    assert_eq!(1, outputs.len());
    let first = ExportBytes::from_output(&outputs[0])
        .into_iter()
        .collect::<Vec<_>>();
    let second = ExportBytes::from_output(&outputs[0])
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!((1, 1), (first.len(), second.len()));
    assert_eq!(first[0].html, second[0].html);
    assert_eq!(first[0].pdf, second[0].pdf);
    assert_eq!(first[0].png, second[0].png);
    assert_eq!(first[0].jpeg, second[0].jpeg);
}

#[test]
fn score_report_has_scores_for_all_formats() {
    let nonce = TEST_NONCE.fetch_add(1, Ordering::Relaxed);
    let outputs = preview_outputs(
        format!("# fixture score report\nnonce: {nonce}"),
        format!("assets/fixtures/score-matrix-export-report-{nonce}.md"),
    );
    assert_eq!(1, outputs.len());
    let bytes = ExportBytes::from_output(&outputs[0])
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(1, bytes.len());
    let report = bytes[0].score_report(&outputs[0].input.snapshot.revision.0);
    assert_eq!(4, report.format_scores.len());
    assert!(
        report
            .format_scores
            .iter()
            .all(|format_score| !format_score.checks.is_empty())
    );
}
