pub(super) struct HtmlMarkdownTask;

impl HtmlMarkdownTask {
    pub(super) fn evaluates(html: &str, source: &str) -> bool {
        Self::required_states(source)
            .iter()
            .all(|state| state.is_rendered_in(html))
    }

    pub(super) fn requires_task(source: &str) -> bool {
        !Self::required_states(source).is_empty()
    }

    fn required_states(source: &str) -> Vec<TaskStateExpectation> {
        TaskStateExpectation::all()
            .iter()
            .copied()
            .filter(|state| source_contains_task_marker(source, state.marker))
            .collect()
    }
}

#[derive(Clone, Copy)]
struct TaskStateExpectation {
    marker: &'static str,
    state: &'static str,
    visual: &'static str,
}

impl TaskStateExpectation {
    fn all() -> &'static [Self] {
        &TASK_STATE_EXPECTATIONS
    }

    const fn new(marker: &'static str, state: &'static str, visual: &'static str) -> Self {
        Self {
            marker,
            state,
            visual,
        }
    }

    fn is_rendered_in(&self, html: &str) -> bool {
        html.contains(&format!(r#"data-kdv-task-marker="{}""#, self.marker))
            && html.contains(&format!(r#"data-kdv-task-state="{}""#, self.state))
            && html.contains(&format!(r#"data-kdv-task-visual="{}""#, self.visual))
    }
}

const TASK_STATE_EXPECTATIONS: [TaskStateExpectation; 5] = [
    TaskStateExpectation::new("[ ]", "todo", "todo"),
    TaskStateExpectation::new("[x]", "done", "done-check"),
    TaskStateExpectation::new("[X]", "done", "done-check"),
    TaskStateExpectation::new("[/]", "in-progress", "in-progress-slash"),
    TaskStateExpectation::new("[-]", "blocked", "blocked-dash"),
];

fn source_contains_task_marker(source: &str, marker: &str) -> bool {
    source.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with(&format!("- {marker}"))
            || trimmed.starts_with(&format!("* {marker}"))
            || trimmed.starts_with(&format!("+ {marker}"))
    })
}

#[cfg(test)]
#[path = "html_score_markdown_task_private_tests.rs"]
mod tests;
