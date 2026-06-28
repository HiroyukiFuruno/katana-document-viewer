pub(super) struct DirectVisualSource;

impl DirectVisualSource {
    pub(super) fn is_direct_image(source: &str) -> bool {
        if Self::is_raw_svg(source) {
            return true;
        }
        Self::extension(source).is_some_and(|extension| {
            matches!(
                extension.as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg"
            )
        })
    }

    pub(super) fn is_drawio(source: &str) -> bool {
        let lower = source.trim_start().to_ascii_lowercase();
        lower.contains("<mxfile")
            || lower.contains("<mxgraphmodel")
            || Self::extension(source)
                .is_some_and(|extension| matches!(extension.as_str(), "drawio" | "drowio"))
    }

    pub(super) fn is_raw_svg(source: &str) -> bool {
        source.trim_start().to_ascii_lowercase().starts_with("<svg")
    }

    pub(super) fn is_svg_file(source: &str) -> bool {
        Self::extension(source).is_some_and(|extension| extension == "svg")
    }

    pub(super) fn source_uri(source: &str) -> Option<String> {
        let path = Self::direct_file_path(source)?;
        if source.trim().starts_with("file://") {
            return Some(source.trim().to_string());
        }
        Some(format!("file://{path}"))
    }

    pub(super) fn is_mermaid(source: &str) -> bool {
        if Self::extension(source)
            .is_some_and(|extension| matches!(extension.as_str(), "mmd" | "mermaid"))
        {
            return true;
        }
        let Some(first_line) = source.lines().map(str::trim).find(|line| !line.is_empty()) else {
            return false;
        };
        Self::line_starts_mermaid(&first_line.to_ascii_lowercase())
    }

    pub(super) fn is_plantuml(source: &str) -> bool {
        if Self::extension(source)
            .is_some_and(|extension| matches!(extension.as_str(), "puml" | "plantuml"))
        {
            return true;
        }
        source
            .trim_start()
            .to_ascii_lowercase()
            .starts_with("@startuml")
    }

    fn line_starts_mermaid(line: &str) -> bool {
        Self::line_starts_mermaid_flow_diagram(line)
            || line.starts_with("sequencediagram")
            || line.starts_with("classdiagram")
            || line.starts_with("statediagram")
            || line.starts_with("erdiagram")
            || line.starts_with("gantt")
            || line.starts_with("pie")
            || line.starts_with("journey")
            || line.starts_with("gitgraph")
            || line.starts_with("mindmap")
            || line.starts_with("timeline")
            || line.starts_with("quadrantchart")
            || line.starts_with("requirementdiagram")
            || line.starts_with("c4context")
    }

    pub(super) fn line_starts_mermaid_flow_diagram(line: &str) -> bool {
        let mut parts = line.split_whitespace();
        let Some(keyword) = parts.next() else {
            return false;
        };
        if !matches!(keyword, "graph" | "flowchart") {
            return false;
        }
        parts.next().is_some_and(Self::is_mermaid_direction)
    }

    fn is_mermaid_direction(value: &str) -> bool {
        let normalized = value.trim_matches(|character: char| !character.is_ascii_alphabetic());
        matches!(normalized, "td" | "tb" | "bt" | "rl" | "lr")
    }

    fn extension(source: &str) -> Option<String> {
        let path = Self::direct_file_path(source)?;
        let without_fragment = path.split(['?', '#']).next().unwrap_or(path);
        without_fragment
            .rsplit_once('.')
            .map(|(_, extension)| extension.to_ascii_lowercase())
    }

    fn direct_file_path(source: &str) -> Option<&str> {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            return None;
        }
        if let Some(path) = trimmed.strip_prefix("file://") {
            return if path.is_empty() { None } else { Some(path) };
        }
        if trimmed.contains(['\n', '\r']) {
            return None;
        }
        if trimmed.chars().any(char::is_whitespace) {
            return Self::looks_like_spaced_file_path(trimmed).then_some(trimmed);
        }
        Some(trimmed)
    }

    fn looks_like_spaced_file_path(path: &str) -> bool {
        path.starts_with('/')
            || path.starts_with("./")
            || path.starts_with("../")
            || path.starts_with("~/")
            || Self::looks_like_windows_file_path(path)
    }

    fn looks_like_windows_file_path(path: &str) -> bool {
        let Some((drive, rest)) = path.split_once(':') else {
            return false;
        };
        drive.len() == 1
            && drive
                .chars()
                .all(|character| character.is_ascii_alphabetic())
            && rest.starts_with(['\\', '/'])
    }
}
