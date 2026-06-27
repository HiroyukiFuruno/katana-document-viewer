use super::*;
use crate::{MarkdownPreview, PreviewConfig};

#[test]
fn sample_fixture_preserves_table_inline_code_spans() -> Result<(), Box<dyn std::error::Error>> {
    let source = MarkdownSource {
        content: include_str!("../../../../assets/fixtures/katana/sample.md").to_string(),
        document_id: Some("sample.md".to_string()),
    };

    let output = PreviewRenderEngine.render(&source, &PreviewConfig::default())?;
    let plan = crate::ViewerNodePlanner::create(&output.input, 0.0);

    assert!(has_inline_code_span(&plan, "PreviewPane"));
    assert!(has_inline_code_span(&plan, "show_content"));
    assert!(
        has_bold_text(&plan, "Bold and italic mixed"),
        "{:?}",
        bold_texts(&plan)
    );
    Ok(())
}

fn has_inline_code_span(plan: &crate::ViewerNodePlan, expected: &str) -> bool {
    plan.nodes.iter().any(|node| {
        node.spans
            .iter()
            .any(|span| span.text == expected && span.style.inline_code)
    })
}

fn has_bold_text(plan: &crate::ViewerNodePlan, expected: &str) -> bool {
    bold_texts(plan)
        .iter()
        .any(|text| normalize(text) == expected)
}

fn bold_texts(plan: &crate::ViewerNodePlan) -> Vec<String> {
    let mut runs = Vec::new();
    for node in &plan.nodes {
        let mut current = String::new();
        for span in &node.spans {
            if span.style.bold {
                current.push_str(&span.text);
                continue;
            }
            push_text_run(&mut runs, &mut current);
        }
        push_text_run(&mut runs, &mut current);
    }
    runs
}

fn push_text_run(runs: &mut Vec<String>, current: &mut String) {
    if current.is_empty() {
        return;
    }
    runs.push(std::mem::take(current));
}

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
