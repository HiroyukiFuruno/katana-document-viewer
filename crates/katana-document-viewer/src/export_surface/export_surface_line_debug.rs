#[cfg(test)]
use super::{SurfaceLine, SurfaceLineMarker, SurfaceTaskMarker};
#[cfg(test)]
use crate::export_surface_span::SurfaceTextSpan;

#[cfg(test)]
const CENTERED_STYLE_TAG: &str = "line:[\"centered\"]";
#[cfg(test)]
const MARKER_COLUMN: &str = "marker-column=36";
#[cfg(test)]
const INDENT_PREFIX: &str = "indent=";
#[cfg(test)]
const BOLD_TAG: &str = "bold";
#[cfg(test)]
const ITALIC_TAG: &str = "italic";
#[cfg(test)]
const MONOSPACE_TAG: &str = "monospace";
#[cfg(test)]
const UNDERLINE_TAG: &str = "underline";
#[cfg(test)]
const STRIKE_TAG: &str = "strikethrough";
#[cfg(test)]
const HIGHLIGHT_TAG: &str = "highlight";
#[cfg(test)]
const INLINE_CODE_TAG: &str = "inline-code";
#[cfg(test)]
const COLOR_TAG: &str = "color";
#[cfg(test)]
const LIST_MARKER_BULLET: &str = "list-marker=bullet";
#[cfg(test)]
const LIST_MARKER_ORDERED: &str = "list-marker=ordered";
#[cfg(test)]
const TASK_MARKER_DONE: &str = "task-marker=done";
#[cfg(test)]
const TASK_MARKER_EMPTY: &str = "task-marker=empty";
#[cfg(test)]
const TASK_MARKER_BLOCKED: &str = "task-marker=blocked";
#[cfg(test)]
const TASK_MARKER_IN_PROGRESS: &str = "task-marker=in-progress";
#[cfg(test)]
const MARKER_PAINT_TEXT: &str = "marker-paint=text";
#[cfg(test)]
const MARKER_PAINT_MATERIAL_DOT: &str = "marker-paint=material-dot";
#[cfg(test)]
const MARKER_PAINT_MATERIAL_CIRCLE: &str = "marker-paint=material-circle";
#[cfg(test)]
const MARKER_PAINT_MATERIAL_SQUARE: &str = "marker-paint=material-square";
#[cfg(test)]
const MARKER_PAINT_MATERIAL_CHECKBOX: &str = "marker-paint=material-checkbox";

#[cfg(test)]
pub(super) fn debug_style_tags(line: &SurfaceLine) -> Vec<String> {
    let marker = line.list_marker();
    let mut tags = line
        .spans
        .iter()
        .skip(usize::from(marker.is_some()))
        .map(|span| styled_tags(span, marker.as_ref(), line.indent_depth))
        .collect::<Vec<_>>();

    if line.is_centered() {
        tags.push(CENTERED_STYLE_TAG.to_string());
    }
    tags
}

#[cfg(test)]
fn styled_tags(
    span: &SurfaceTextSpan,
    marker: Option<&SurfaceLineMarker>,
    indent_depth: u32,
) -> String {
    let mut tags: Vec<String> = Vec::new();
    append_span_style_tags(&mut tags, span);
    append_context_tags(&mut tags, marker, indent_depth);
    format!("{}:{tags:?}", span.text)
}

#[cfg(test)]
fn append_span_style_tags(tags: &mut Vec<String>, span: &SurfaceTextSpan) {
    if span.style.bold {
        tags.push(BOLD_TAG.to_string());
    }
    if span.style.italic {
        tags.push(ITALIC_TAG.to_string());
    }
    if span.style.monospace {
        tags.push(MONOSPACE_TAG.to_string());
    }
    if span.style.underline {
        tags.push(UNDERLINE_TAG.to_string());
    }
    if span.style.strikethrough {
        tags.push(STRIKE_TAG.to_string());
    }
    if span.style.highlight {
        tags.push(HIGHLIGHT_TAG.to_string());
    }
    if span.style.inline_code {
        tags.push(INLINE_CODE_TAG.to_string());
    }
    if span.style.color.is_some() {
        tags.push(COLOR_TAG.to_string());
    }
}

#[cfg(test)]
fn append_context_tags(
    tags: &mut Vec<String>,
    marker: Option<&SurfaceLineMarker>,
    indent_depth: u32,
) {
    if indent_depth > 0 || marker.is_some() {
        tags.push(format!("{INDENT_PREFIX}{indent_depth}"));
    }
    if let Some(marker) = marker {
        append_marker_debug_tags(tags, marker, indent_depth);
    }
}

#[cfg(test)]
fn append_marker_debug_tags(tags: &mut Vec<String>, marker: &SurfaceLineMarker, indent_depth: u32) {
    match marker {
        SurfaceLineMarker::Bullet => {
            tags.push(LIST_MARKER_BULLET.to_string());
            tags.push(MARKER_COLUMN.to_string());
            tags.push(bullet_marker_paint(indent_depth).to_string());
        }
        SurfaceLineMarker::Ordered(_) => {
            tags.push(LIST_MARKER_ORDERED.to_string());
            tags.push(MARKER_COLUMN.to_string());
            tags.push(MARKER_PAINT_TEXT.to_string());
        }
        SurfaceLineMarker::Task(task) => {
            tags.push(task.debug_name().to_string());
            tags.push(MARKER_COLUMN.to_string());
            tags.push(MARKER_PAINT_MATERIAL_CHECKBOX.to_string());
        }
    }
}

#[cfg(test)]
fn bullet_marker_paint(indent_depth: u32) -> &'static str {
    match indent_depth % 3 {
        0 => MARKER_PAINT_MATERIAL_DOT,
        1 => MARKER_PAINT_MATERIAL_CIRCLE,
        _ => MARKER_PAINT_MATERIAL_SQUARE,
    }
}

pub(super) fn ordered_marker(marker: &str) -> bool {
    marker.ends_with('.') && marker.trim_end_matches('.').parse::<usize>().is_ok()
}

#[cfg(test)]
impl SurfaceTaskMarker {
    #[cfg(test)]
    pub(super) fn debug_name(self) -> &'static str {
        match self {
            SurfaceTaskMarker::Done => TASK_MARKER_DONE,
            SurfaceTaskMarker::Empty => TASK_MARKER_EMPTY,
            SurfaceTaskMarker::Blocked => TASK_MARKER_BLOCKED,
            SurfaceTaskMarker::InProgress => TASK_MARKER_IN_PROGRESS,
        }
    }
}
