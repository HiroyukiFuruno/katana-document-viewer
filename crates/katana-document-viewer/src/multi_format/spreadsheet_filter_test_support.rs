use std::io::{Cursor, Read, Write};
use zip::write::SimpleFileOptions;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(super) fn filter_catalog_package(
    workbook: Option<&[u8]>,
    relationships: Option<&[u8]>,
    worksheet: Option<&[u8]>,
) -> TestResult<Vec<u8>> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in [
        ("xl/workbook.xml", workbook),
        ("xl/_rels/workbook.xml.rels", relationships),
    ] {
        if let Some(bytes) = bytes {
            writer.start_file(name, SimpleFileOptions::default())?;
            writer.write_all(bytes)?;
        }
    }
    if let Some(worksheet) = worksheet {
        writer.start_file("xl/worksheets/sheet1.xml", SimpleFileOptions::default())?;
        writer.write_all(worksheet)?;
    }
    Ok(writer.finish()?.into_inner())
}

pub(super) fn representative_with_auto_filter() -> TestResult<Vec<u8>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    inject_auto_filter(&std::fs::read(fixture)?)
}

pub(super) fn representative_with_worksheet_xml(worksheet: &[u8]) -> TestResult<Vec<u8>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    rewrite_worksheet(&std::fs::read(fixture)?, |_| Ok(worksheet.to_vec()))
}

pub(super) fn representative_with_merge_cell_references(
    references: &[&str],
) -> TestResult<Vec<u8>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    let replacement = format!(
        "<mergeCells count=\"{}\">{}</mergeCells>",
        references.len(),
        references
            .iter()
            .map(|reference| format!("<mergeCell ref=\"{reference}\"/>"))
            .collect::<String>()
    );
    rewrite_worksheet(&std::fs::read(fixture)?, |content| {
        let xml = String::from_utf8(content)?;
        let source = r#"<mergeCells count="1"><mergeCell ref="A1:F1"/></mergeCells>"#;
        if !xml.contains(source) {
            return Err("representative merge-cell references are missing".into());
        }
        Ok(xml.replacen(source, &replacement, 1).into_bytes())
    })
}

pub(super) fn representative_with_nested_merge_cells() -> TestResult<Vec<u8>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    rewrite_worksheet(&std::fs::read(fixture)?, |content| {
        let xml = String::from_utf8(content)?;
        let source = r#"<mergeCells count="1"><mergeCell ref="A1:F1"/></mergeCells>"#;
        let replacement = r#"<extLst><ext><mergeCells><mergeCell ref="invalid"/></mergeCells></ext></extLst><mergeCells count="1"><mergeCell ref="A1:F1"/></mergeCells>"#;
        if !xml.contains(source) {
            return Err("representative merge-cell references are missing".into());
        }
        Ok(xml.replacen(source, replacement, 1).into_bytes())
    })
}

pub(super) fn representative_with_auto_filter_and_blank() -> TestResult<Vec<u8>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    inject_auto_filter_and_blank(&std::fs::read(fixture)?)
}

pub(super) fn representative_with_persisted_auto_filter_and_saved_filter_hidden_rows()
-> TestResult<Vec<u8>> {
    representative_with_persisted_hidden_rows(&[5, 6, 7])
}

pub(super) fn representative_with_persisted_auto_filter_and_authored_hidden() -> TestResult<Vec<u8>>
{
    representative_with_persisted_hidden_rows(&[4])
}

fn representative_with_persisted_hidden_rows(rows: &[usize]) -> TestResult<Vec<u8>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    rewrite_worksheet(&std::fs::read(fixture)?, |content| {
        let content = worksheet_with_filter(content)?;
        let mut xml = String::from_utf8(content)?;
        for row in rows {
            let source = format!(r#"<row r="{row}""#);
            let replacement = format!(r#"<row r="{row}" hidden="1""#);
            if !xml.contains(&source) {
                return Err(format!("worksheet row {row} is missing").into());
            }
            xml = xml.replacen(&source, &replacement, 1);
        }
        Ok(xml.into_bytes())
    })
}

fn inject_auto_filter(bytes: &[u8]) -> TestResult<Vec<u8>> {
    rewrite_worksheet(bytes, worksheet_with_filter)
}

fn inject_auto_filter_and_blank(bytes: &[u8]) -> TestResult<Vec<u8>> {
    rewrite_worksheet(bytes, |content| {
        let content = worksheet_with_filter(content)?;
        let xml = String::from_utf8(content)?;
        let blank_cell = r#"<c r="A7" s="3" t="inlineStr"><is><t>West</t></is></c>"#;
        if !xml.contains(blank_cell) {
            return Err("worksheet blank-cell source is missing".into());
        }
        Ok(xml
            .replacen(blank_cell, r#"<c r="A7" s="3"/>"#, 1)
            .into_bytes())
    })
}

fn rewrite_worksheet(
    bytes: &[u8],
    rewrite: impl FnOnce(Vec<u8>) -> TestResult<Vec<u8>>,
) -> TestResult<Vec<u8>> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let mut rewrite = Some(rewrite);
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let name = entry.name()?.into_owned();
        let options = SimpleFileOptions::default().compression_method(entry.compression());
        if entry.is_dir() {
            output.add_directory(name, options)?;
            continue;
        }
        let mut content = Vec::new();
        entry.read_to_end(&mut content)?;
        if name == "xl/worksheets/sheet1.xml" {
            let rewrite = rewrite
                .take()
                .ok_or("worksheet rewrite ran more than once")?;
            content = rewrite(content)?;
        }
        output.start_file(name, options)?;
        output.write_all(&content)?;
    }
    Ok(output.finish()?.into_inner())
}

fn worksheet_with_filter(content: Vec<u8>) -> TestResult<Vec<u8>> {
    let xml = String::from_utf8(content)?;
    let filter = r#"<autoFilter ref="A3:F7"><filterColumn colId="0"><filters><filter val="North"/></filters></filterColumn></autoFilter>"#;
    if !xml.contains("</worksheet>") {
        return Err("worksheet closing tag is missing".into());
    }
    Ok(xml
        .replacen("</worksheet>", &format!("{filter}</worksheet>"), 1)
        .into_bytes())
}
