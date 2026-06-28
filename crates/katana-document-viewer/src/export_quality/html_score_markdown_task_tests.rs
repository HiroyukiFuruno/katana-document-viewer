use super::HtmlQualityScore;

#[test]
fn markdown_task_source_rejects_generic_task_state_placeholder() {
    let score = HtmlQualityScore::score(
        br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<ul><li data-kdv-task-state="done">done</li></ul>
</main>
"#,
        "- [x] done\n- [/] doing\n- [-] hold\n",
    );

    assert_contains(&score.fatal_failures(), "Html: html evaluates task state");
}

#[test]
fn markdown_task_source_rejects_blocked_state_as_progress() {
    let score = HtmlQualityScore::score(
        br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<ul>
<li data-kdv-task-item="true"><input data-kdv-task-marker="[-]" data-kdv-task-state="in-progress"><span data-kdv-task-visual="in-progress-dash"></span>hold</li>
</ul>
</main>
"#,
        "- [-] hold\n",
    );

    assert_contains(&score.fatal_failures(), "Html: html evaluates task state");
}

#[test]
fn markdown_task_source_accepts_all_katana_task_states() {
    let score = HtmlQualityScore::score(
        br#"
<main data-kdv-export>
<style data-kdv-export-style></style>
<ul>
<li data-kdv-task-item="true"><input data-kdv-task-marker="[ ]" data-kdv-task-state="todo"><span data-kdv-task-visual="todo"></span>todo</li>
<li data-kdv-task-item="true"><input data-kdv-task-marker="[x]" data-kdv-task-state="done" checked><span data-kdv-task-visual="done-check"></span>done</li>
<li data-kdv-task-item="true"><input data-kdv-task-marker="[X]" data-kdv-task-state="done" checked><span data-kdv-task-visual="done-check"></span>upper done</li>
<li data-kdv-task-item="true"><input data-kdv-task-marker="[/]" data-kdv-task-state="in-progress" aria-checked="mixed"><span data-kdv-task-visual="in-progress-slash"></span>doing</li>
<li data-kdv-task-item="true"><input data-kdv-task-marker="[-]" data-kdv-task-state="blocked" aria-checked="mixed"><span data-kdv-task-visual="blocked-dash"></span>hold</li>
</ul>
</main>
"#,
        "- [ ] todo\n- [x] done\n- [X] upper done\n- [/] doing\n- [-] hold\n",
    );

    assert!(score.fatal_failures().is_empty(), "{score:#?}");
}

fn assert_contains(failures: &[String], expected: &str) {
    assert!(
        failures.iter().any(|failure| failure == expected),
        "{failures:#?}"
    );
}
