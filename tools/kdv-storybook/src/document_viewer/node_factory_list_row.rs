use super::super::task_state::KdvTaskState;

const TASK_MARKER_WIDTH_BYTES: usize = 3;
const SPACES_PER_INDENT_DEPTH: usize = 2;

pub(super) struct KdvListRow<'a> {
    pub(super) marker: KdvListMarker<'a>,
    pub(super) body: &'a str,
    pub(super) depth: usize,
}

impl<'a> KdvListRow<'a> {
    pub(super) fn parse(line: &'a str) -> Self {
        let depth = Self::indent_depth(line);
        let trimmed = line.trim_start();
        if let Some(mut row) = Self::task_row(trimmed) {
            row.depth = depth;
            return row;
        }
        let Some((marker, body)) = trimmed.split_once(char::is_whitespace) else {
            return Self {
                marker: KdvListMarker::Text("-"),
                body: trimmed,
                depth,
            };
        };
        Self::marked_or_nested_task(marker, body, depth)
    }

    fn marked_or_nested_task(marker: &'a str, body: &'a str, depth: usize) -> Self {
        if Self::is_list_marker(marker)
            && let Some(mut row) = Self::task_row(body.trim_start())
        {
            row.depth = depth;
            return row;
        }
        Self {
            marker: KdvListMarker::Text(marker),
            body: body.trim_start(),
            depth,
        }
    }

    fn task_row(trimmed: &'a str) -> Option<Self> {
        let source = trimmed.get(..TASK_MARKER_WIDTH_BYTES)?;
        let state = KdvTaskState::from_marker(source)?;
        let body = trimmed.get(TASK_MARKER_WIDTH_BYTES..)?;
        Some(Self {
            marker: KdvListMarker::Task(state),
            body: body.trim_start(),
            depth: 0,
        })
    }

    fn is_list_marker(marker: &str) -> bool {
        let number = marker.trim_end_matches(['.', ')']);
        matches!(marker, "-" | "*" | "+")
            || ((marker.ends_with('.') || marker.ends_with(')'))
                && !number.is_empty()
                && number.chars().all(|value| value.is_ascii_digit()))
    }

    fn indent_depth(line: &str) -> usize {
        let mut tabs = 0usize;
        let mut spaces = 0usize;
        for character in line.chars() {
            match character {
                '\t' => tabs += 1,
                ' ' => spaces += 1,
                _ => break,
            }
        }
        tabs + spaces.div_ceil(SPACES_PER_INDENT_DEPTH)
    }
}

pub(super) enum KdvListMarker<'a> {
    Text(&'a str),
    Task(KdvTaskState),
}
