use crate::diagnostics::{KdvLintError, Violation};
use std::path::{Path, PathBuf};

const CHANGES_PATH: &str = "openspec/changes";
const ARCHIVE_DIR_NAME: &str = "archive";

const FAKE_VENDOR_MESSAGE: &str =
    "adapter plan or egui route switching must not be documented as 4 real vendor runtime display.";
const MISSING_EVIDENCE_MESSAGE: &str = "high-risk completed OpenSpec item must include `証跡:` with command, test, file, screenshot, or issue URL evidence.";
const CHANGE_DOCUMENT_FILENAMES: [&str; 3] = ["proposal.md", "design.md", "handoff.md"];
const FAKE_RUNTIME_SOURCE_TERMS: &str =
    "adapter plan|Adapter plan|egui route selector|route selector|ルートセレクタ";
const FOUR_RUNTIME_TERMS: &str = "4実runtime|4つの実runtime|4 runtime|4経路|4 vendor|4 route";
const RUNTIME_DISPLAY_TERMS: &str = "表示|開|Storybook";
const NEGATIVE_RUNTIME_TERMS: &str =
    "不可|ではない|誤り|扱ってはならない|扱わない|扱いにしない|完了にしない";
const COMPLETION_CLAIM_TERMS: &str = "DoD|Storybook|score|自動テスト|gate|検証|互換|4実runtime";
const EVIDENCE_TERMS: &str = "`rtk |`just |`cargo |command:|コマンド:|test:|テスト:|file:|ファイル:|screenshot:|スクリーンショット:";

pub struct OpenSpecIntegrityRule;

impl OpenSpecIntegrityRule {
    pub fn check(root: &Path) -> Result<Vec<Violation>, KdvLintError> {
        let mut violations = Vec::new();
        for change_path in active_change_paths(root)? {
            Self::check_change(&change_path, &mut violations)?;
        }
        Ok(violations)
    }

    fn check_change(
        change_path: &Path,
        violations: &mut Vec<Violation>,
    ) -> Result<(), KdvLintError> {
        let tasks_path = change_path.join("tasks.md");
        if tasks_path.is_file() {
            Self::check_tasks(&tasks_path, violations)?;
        }
        for path in change_document_paths(change_path)? {
            Self::check_text_file(&path, violations)?;
        }
        Ok(())
    }

    fn check_tasks(path: &Path, violations: &mut Vec<Violation>) -> Result<(), KdvLintError> {
        let source = read_text(path)?;
        for (line_number, line) in source.lines().enumerate() {
            if is_completed_task_line(line) && contains_fake_vendor_runtime_claim(line) {
                violations.push(violation(
                    path,
                    line_number,
                    "openspec-fake-vendor-dod",
                    FAKE_VENDOR_MESSAGE,
                ));
            }
            if is_completed_task_line(line) && requires_evidence(line) && !has_evidence(line) {
                violations.push(violation(
                    path,
                    line_number,
                    "openspec-missing-evidence",
                    MISSING_EVIDENCE_MESSAGE,
                ));
            }
        }
        Ok(())
    }

    fn check_text_file(path: &Path, violations: &mut Vec<Violation>) -> Result<(), KdvLintError> {
        let source = read_text(path)?;
        for (line_number, line) in source.lines().enumerate() {
            if contains_fake_vendor_runtime_claim(line) {
                violations.push(violation(
                    path,
                    line_number,
                    "openspec-fake-vendor-wording",
                    FAKE_VENDOR_MESSAGE,
                ));
            }
        }
        Ok(())
    }
}

fn active_change_paths(root: &Path) -> Result<Vec<PathBuf>, KdvLintError> {
    let changes_path = root.join(CHANGES_PATH);
    if !changes_path.exists() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    for entry in read_directory_entries(&changes_path)? {
        let path = entry.path();
        if path.is_dir() && entry.file_name() != ARCHIVE_DIR_NAME {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn change_document_paths(change_path: &Path) -> Result<Vec<PathBuf>, KdvLintError> {
    let mut paths = Vec::new();
    for filename in CHANGE_DOCUMENT_FILENAMES {
        let path = change_path.join(filename);
        if path.is_file() {
            paths.push(path);
        }
    }
    collect_markdown_files(&change_path.join("specs"), &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_markdown_files(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), KdvLintError> {
    if !directory.exists() {
        return Ok(());
    }
    for entry in read_directory_entries(directory)? {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, paths)?;
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn is_completed_task_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("- [x]") || trimmed.starts_with("- [/]")
}

fn contains_fake_vendor_runtime_claim(line: &str) -> bool {
    contains_term(line, FAKE_RUNTIME_SOURCE_TERMS)
        && contains_term(line, FOUR_RUNTIME_TERMS)
        && contains_term(line, RUNTIME_DISPLAY_TERMS)
        && !contains_term(line, NEGATIVE_RUNTIME_TERMS)
}

fn contains_term(line: &str, terms: &str) -> bool {
    terms.split('|').any(|term| line.contains(term))
}

fn requires_evidence(line: &str) -> bool {
    contains_term(line, COMPLETION_CLAIM_TERMS)
}

fn has_evidence(line: &str) -> bool {
    match line.split_once("証跡:") {
        Some((_, evidence)) => contains_evidence_reference(evidence),
        None => false,
    }
}

fn contains_evidence_reference(evidence: &str) -> bool {
    contains_term(evidence, EVIDENCE_TERMS)
        || (evidence.contains("https://github.com/") && evidence.contains("/issues/"))
}

fn read_text(path: &Path) -> Result<String, KdvLintError> {
    std::fs::read_to_string(path).map_err(|source| KdvLintError::Read {
        path: path.to_path_buf(),
        source,
    })
}

fn read_directory_entries(path: &Path) -> Result<Vec<std::fs::DirEntry>, KdvLintError> {
    let entries = std::fs::read_dir(path).map_err(|source| KdvLintError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    entries
        .map(|entry| {
            entry.map_err(|source| KdvLintError::Read {
                path: path.to_path_buf(),
                source,
            })
        })
        .collect()
}

fn violation(
    path: &Path,
    line_number: usize,
    rule: &'static str,
    message: &'static str,
) -> Violation {
    Violation::new(PathBuf::from(path), line_number + 1, 1, rule, message)
}

#[cfg(test)]
#[path = "openspec_integrity_tests.rs"]
mod tests;
