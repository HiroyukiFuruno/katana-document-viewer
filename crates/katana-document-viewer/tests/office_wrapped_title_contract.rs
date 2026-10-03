use process_control::{ChildExt, Control};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use zip::write::SimpleFileOptions;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
const TITLE: &str = "A centered title retains its declared size without implicit autofit";

fn title_pptx(autofit: bool, anchor: &str) -> TestResult<Vec<u8>> {
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.pptx");
    let mut archive = zip::ZipArchive::new(Cursor::new(std::fs::read(input)?))?;
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let fitting = if autofit { "<a:normAutofit/>" } else { "" };
    let slide = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>
<p:sp><p:nvSpPr><p:cNvPr id="2" name="Synthetic wrapped title"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr>
<p:spPr><a:xfrm><a:off x="914400" y="914400"/><a:ext cx="5080000" cy="152400"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/></p:spPr>
<p:txBody><a:bodyPr wrap="square" anchor="{anchor}" lIns="0" rIns="0" tIns="0" bIns="0">{fitting}</a:bodyPr><a:lstStyle/>
<a:p><a:pPr/><a:r><a:rPr sz="2600"><a:solidFill><a:srgbClr val="000000"/></a:solidFill><a:latin typeface="Arial"/></a:rPr><a:t>{TITLE}</a:t></a:r><a:endParaRPr sz="2600"/></a:p>
</p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sld>"#
    );
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let name = entry.name().to_owned();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
            bytes = slide.as_bytes().to_vec();
        }
        output.start_file(name, SimpleFileOptions::default())?;
        output.write_all(&bytes)?;
    }
    Ok(output.finish()?.into_inner())
}

fn converted_pdf(autofit: bool, anchor: &str) -> TestResult<Vec<u8>> {
    let directory = tempfile::tempdir()?;
    std::fs::write(
        directory.path().join("input.office"),
        title_pptx(autofit, anchor)?,
    )?;
    let worker = std::env::var_os("KDV_WRAPPED_TITLE_WORKER")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_kdv-office-worker")));
    let mut child = Command::new(worker)
        .arg(directory.path())
        .args(["pptx", "2147483648", "46", "134217728"])
        .env_clear()
        .stdin(Stdio::null())
        .spawn()?;
    let status = child
        .controlled()
        .time_limit(Duration::from_secs(45))
        .terminate_for_timeout()
        .strict_errors()
        .wait()?
        .ok_or("wrapped-title worker exceeded its existing deadline")?;
    assert_eq!(
        status.code(),
        Some(0),
        "actual worker must convert the fixture"
    );
    let response: serde_json::Value =
        serde_json::from_slice(&std::fs::read(directory.path().join("response.json"))?)?;
    assert_eq!(response["status"], "completed");
    Ok(std::fs::read(directory.path().join("output.pdf"))?)
}

#[derive(Debug)]
struct GlyphObservation {
    character: String,
    height_pt: f64,
    baseline_y_pt: f64,
}

#[derive(Default)]
struct GlyphProbe(Vec<GlyphObservation>);

impl pdf_extract::OutputDev for GlyphProbe {
    fn begin_page(
        &mut self,
        _: u32,
        _: &pdf_extract::MediaBox,
        _: Option<(f64, f64, f64, f64)>,
    ) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
    fn end_page(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
    fn begin_word(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
    fn end_word(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
    fn end_line(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }

    fn output_character(
        &mut self,
        transform: &pdf_extract::Transform,
        _: f64,
        _: f64,
        font_size: f64,
        character: &str,
    ) -> Result<(), pdf_extract::OutputError> {
        // 横のadvance-grid補正と区別し、CTMを含む実フォント高さで暗黙の縮小を検出する。
        self.0.push(GlyphObservation {
            character: character.to_owned(),
            height_pt: font_size * transform.m21.hypot(transform.m22),
            baseline_y_pt: transform.m32,
        });
        Ok(())
    }
}

fn observations(bytes: Vec<u8>) -> TestResult<Vec<GlyphObservation>> {
    let pdf = pdf_extract::Document::load_mem(&bytes)?;
    let mut probe = GlyphProbe::default();
    pdf_extract::output_doc_page(&pdf, &mut probe, 1)?;
    let actual: String = probe
        .0
        .iter()
        .map(|glyph| glyph.character.as_str())
        .collect();
    let normalize = |text: &str| {
        text.chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>()
    };
    assert_eq!(
        normalize(&actual),
        normalize(TITLE),
        "complete title must survive the worker"
    );
    assert!(
        probe
            .0
            .iter()
            .all(|glyph| glyph.height_pt.is_finite() && glyph.baseline_y_pt.is_finite())
    );
    Ok(probe.0)
}

#[test]
fn wrapped_title_without_autofit_keeps_26pt_in_actual_worker_pdf() -> TestResult {
    let text = observations(converted_pdf(false, "ctr")?)?;
    let mut baselines = text
        .iter()
        .map(|glyph| glyph.baseline_y_pt)
        .collect::<Vec<_>>();
    baselines.sort_by(f64::total_cmp);
    baselines.dedup_by(|left, right| (*left - *right).abs() < 1e-6);
    assert!(
        baselines.len() >= 2,
        "natural-size title must wrap into multiple physical lines"
    );
    assert!(
        text.iter()
            .all(|glyph| (glyph.height_pt - 26.0).abs() < 1e-6),
        "no-autofit text must not inherit an enclosing shrink: {text:?}"
    );
    Ok(())
}

#[test]
fn wrapped_title_with_explicit_autofit_still_shrinks_in_actual_worker_pdf() -> TestResult {
    let text = observations(converted_pdf(true, "ctr")?)?;
    assert!(
        text.iter()
            .all(|glyph| glyph.height_pt > 0.0 && glyph.height_pt < 26.0),
        "explicit autofit must remain effective: {text:?}"
    );
    Ok(())
}

#[test]
fn centered_overflow_moves_above_top_anchor_in_actual_worker_pdf() -> TestResult {
    let centered = observations(converted_pdf(false, "ctr")?)?;
    let top = observations(converted_pdf(false, "t")?)?;
    assert_eq!(centered.len(), top.len());
    // PDFのyは下端原点。負の余白を0へ丸めると中央と上端が同じ位置になってしまう。
    assert!(
        centered
            .iter()
            .zip(&top)
            .all(|(center, top)| center.character == top.character
                && center.baseline_y_pt > top.baseline_y_pt),
        "centered overflow must retain its negative block offset"
    );
    Ok(())
}
