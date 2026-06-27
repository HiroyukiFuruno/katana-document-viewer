use super::HtmlQualityScore;

#[test]
fn markdown_source_requires_alert_html_for_gfm_alert_markers() {
    for marker in [
        "[!TIP]",
        "[!IMPORTANT]",
        "[!CAUTION]",
        "[!NOTE]",
        "[!WARNING]",
    ] {
        let score = HtmlQualityScore::score(
            br#"<main data-kdv-export><style data-kdv-export-style></style><p>body</p></main>"#,
            marker,
        );

        assert_contains(&score.fatal_failures(), "Html: html evaluates gfm alert");

        let with_alert = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<div data-github-alert="tip">body</div>
</main>
"#;
        let score_with_alert = HtmlQualityScore::score(with_alert, marker);
        assert!(
            score_with_alert.fatal_failures().is_empty(),
            "{marker}: {score_with_alert:#?}"
        );
    }
}

#[test]
fn markdown_source_ignores_alert_markers_inside_fenced_code_blocks() {
    let source = "```\n[!TIP]\n[!IMPORTANT]\n[!CAUTION]\n```";
    let html = br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<pre data-kdv-code-role="plain"><code>[!TIP]
[!IMPORTANT]
[!CAUTION]</code></pre>
</main>
"#;

    let score = HtmlQualityScore::score(html, source);
    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

fn assert_contains(failures: &[String], expected: &str) {
    assert!(
        failures.iter().any(|failure| failure == expected),
        "{failures:#?}"
    );
}
