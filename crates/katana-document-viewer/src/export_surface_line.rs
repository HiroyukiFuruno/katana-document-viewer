use crate::export_surface_helpers::{LIST_INDENT, PAGE_PADDING, QUOTE_INDENT};
use crate::export_surface_span::SurfaceTextSpan;

pub(crate) const LIST_MARKER_COLUMN_WIDTH: u32 = 36;

pub(crate) struct SurfaceLine {
    pub(crate) text: String,
    pub(crate) spans: Vec<SurfaceTextSpan>,
    level: SurfaceLineLevel,
    quote_depth: u32,
    indent_depth: u32,
    alignment: SurfaceLineAlignment,
}

enum SurfaceLineLevel {
    Heading(u8),
    Body,
    Code,
}

#[derive(Clone, Copy)]
enum SurfaceLineAlignment {
    Left,
    Center,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceLineMarker {
    Bullet,
    Ordered(String),
    Task(SurfaceTaskMarker),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceTaskMarker {
    Done,
    Empty,
    Blocked,
    InProgress,
}

impl SurfaceLine {
    pub(crate) fn heading(level: u8, text: String) -> Self {
        Self {
            spans: vec![SurfaceTextSpan::plain(text.clone())],
            text,
            level: SurfaceLineLevel::Heading(level),
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    #[cfg(test)]
    pub(crate) fn body(text: String) -> Self {
        Self::body_with_quote(text, 0)
    }

    pub(crate) fn body_with_quote(text: String, quote_depth: u32) -> Self {
        Self {
            spans: vec![SurfaceTextSpan::plain(text.clone())],
            text,
            level: SurfaceLineLevel::Body,
            quote_depth,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn body_centered(text: String) -> Self {
        Self {
            spans: vec![SurfaceTextSpan::plain(text.clone())],
            text,
            level: SurfaceLineLevel::Body,
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Center,
        }
    }

    pub(crate) fn centered_spans(spans: Vec<SurfaceTextSpan>) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Body,
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Center,
        }
    }

    pub(crate) fn body_spans(spans: Vec<SurfaceTextSpan>, quote_depth: u32) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Body,
            quote_depth,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn body_spans_with_indent(
        spans: Vec<SurfaceTextSpan>,
        quote_depth: u32,
        indent_depth: u32,
    ) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Body,
            quote_depth,
            indent_depth,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn code_spans(spans: Vec<SurfaceTextSpan>) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Code,
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn x(&self) -> u32 {
        PAGE_PADDING + self.quote_depth * QUOTE_INDENT + self.indent_depth * LIST_INDENT
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self.level {
            SurfaceLineLevel::Heading(1) => 40.0,
            SurfaceLineLevel::Heading(2) => 34.0,
            SurfaceLineLevel::Heading(_) => 28.0,
            SurfaceLineLevel::Body => 24.0,
            SurfaceLineLevel::Code => 22.0,
        }
    }

    pub(crate) fn line_height(&self) -> u32 {
        match self.level {
            SurfaceLineLevel::Heading(1) => 92,
            SurfaceLineLevel::Heading(2) => 78,
            SurfaceLineLevel::Heading(_) => 66,
            SurfaceLineLevel::Body => 46,
            SurfaceLineLevel::Code => 34,
        }
    }

    pub(crate) fn text_y(&self, y: u32) -> u32 {
        y + self.top_margin()
    }

    fn top_margin(&self) -> u32 {
        match self.level {
            SurfaceLineLevel::Heading(1) => 16,
            SurfaceLineLevel::Heading(2) => 14,
            SurfaceLineLevel::Heading(_) => 12,
            SurfaceLineLevel::Body => 5,
            SurfaceLineLevel::Code => 0,
        }
    }

    pub(crate) fn quote_depth(&self) -> u32 {
        self.quote_depth
    }

    pub(crate) fn is_code(&self) -> bool {
        matches!(self.level, SurfaceLineLevel::Code)
    }

    pub(crate) fn is_heading(&self) -> bool {
        matches!(self.level, SurfaceLineLevel::Heading(_))
    }

    pub(crate) fn is_centered(&self) -> bool {
        matches!(self.alignment, SurfaceLineAlignment::Center)
    }

    pub(crate) fn list_marker(&self) -> Option<SurfaceLineMarker> {
        let marker = self.spans.first()?.text.trim();
        match marker {
            "•" => Some(SurfaceLineMarker::Bullet),
            "☑" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Done)),
            "☐" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Empty)),
            "⊟" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::Blocked)),
            "◩" => Some(SurfaceLineMarker::Task(SurfaceTaskMarker::InProgress)),
            _ if ordered_marker(marker) => Some(SurfaceLineMarker::Ordered(marker.to_string())),
            _ => None,
        }
    }

    pub(crate) fn content_spans(&self) -> &[SurfaceTextSpan] {
        if self.list_marker().is_some() {
            return &self.spans[1..];
        }
        &self.spans
    }

    #[cfg(test)]
    pub(crate) fn debug_style_tags(&self) -> Vec<String> {
        let marker = self.list_marker();
        let mut tags = self
            .spans
            .iter()
            .skip(usize::from(marker.is_some()))
            .map(|span| {
                let mut tags = Vec::new();
                if span.style.bold {
                    tags.push("bold".to_string());
                }
                if span.style.italic {
                    tags.push("italic".to_string());
                }
                if span.style.monospace {
                    tags.push("monospace".to_string());
                }
                if span.style.underline {
                    tags.push("underline".to_string());
                }
                if span.style.strikethrough {
                    tags.push("strikethrough".to_string());
                }
                if span.style.highlight {
                    tags.push("highlight".to_string());
                }
                if span.style.inline_code {
                    tags.push("inline-code".to_string());
                }
                if span.style.color.is_some() {
                    tags.push("color".to_string());
                }
                if let Some(indent) = self.debug_list_indent() {
                    tags.push(format!("indent={indent}"));
                }
                if let Some(marker) = &marker {
                    append_marker_debug_tags(&mut tags, marker);
                }
                format!("{}:{tags:?}", span.text)
            })
            .collect::<Vec<_>>();
        if self.is_centered() {
            tags.push("line:[\"centered\"]".to_string());
        }
        tags
    }

    #[cfg(test)]
    fn debug_list_indent(&self) -> Option<u32> {
        self.list_marker().map(|_| self.indent_depth)
    }
}

fn ordered_marker(marker: &str) -> bool {
    marker.ends_with('.') && marker.trim_end_matches('.').parse::<usize>().is_ok()
}

#[cfg(test)]
fn append_marker_debug_tags(tags: &mut Vec<String>, marker: &SurfaceLineMarker) {
    match marker {
        SurfaceLineMarker::Bullet => tags.push("list-marker=bullet".to_string()),
        SurfaceLineMarker::Ordered(_) => tags.push("list-marker=ordered".to_string()),
        SurfaceLineMarker::Task(task) => tags.push(format!("task-marker={}", task.debug_name())),
    }
    tags.push(format!("marker-column={LIST_MARKER_COLUMN_WIDTH}"));
}

#[cfg(test)]
impl SurfaceTaskMarker {
    fn debug_name(self) -> &'static str {
        match self {
            SurfaceTaskMarker::Done => "done",
            SurfaceTaskMarker::Empty => "empty",
            SurfaceTaskMarker::Blocked => "blocked",
            SurfaceTaskMarker::InProgress => "in-progress",
        }
    }
}
