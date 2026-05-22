use super::contract_test_support::{assert_contains_all, assert_not_contains_any, export_html};

#[test]
fn red_detects_math_html_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r##"```math
f(x) = x^2
```

質量: $E=mc^2$

$$ \sum_{k=1}^{n} k = \frac{n(n+1)}{2} $$
"##,
    )?;

    assert_contains_all(
        &html,
        &[
            (
                "fenced math block",
                r#"<div data-kdv-math="block" data-kdv-render-runtime="katana-render-runtime-stub">"#,
            ),
            (
                "compact inline math",
                r#"<span data-kdv-math="inline" data-kdv-render-runtime="katana-render-runtime-stub">"#,
            ),
            (
                "one line dollar math",
                r#"<div data-kdv-math="dollar-block" data-kdv-render-runtime="katana-render-runtime-stub">"#,
            ),
            ("svg output", "<svg"),
        ],
    );
    assert_not_contains_any(
        &html,
        &[
            ("math fence marker", "```math"),
            (
                "direct math renderer leak",
                "data-kdv-math-renderer=\"mathjax-svg\"",
            ),
        ],
    );
    Ok(())
}

#[test]
fn red_detects_katana_specific_html_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r##"- [-] 完了扱い
- [/] 進行中

柔軟な数式: $ E = mc^2 $ と $$ a+b=c $$
"##,
    )?;

    assert_contains_all(
        &html,
        &[
            (
                "task done marker",
                r#"<input type="checkbox" disabled data-kdv-task-marker="[-]" data-kdv-task-state="in-progress" aria-checked="mixed">"#,
            ),
            (
                "task in progress marker",
                r#"<input type="checkbox" disabled data-kdv-task-marker="[/]" data-kdv-task-state="in-progress" aria-checked="mixed">"#,
            ),
            (
                "spaced inline math",
                r#"<span data-kdv-math="inline" data-kdv-render-runtime="katana-render-runtime-stub">"#,
            ),
            (
                "inline dollar block",
                r#"<span data-kdv-math="inline" data-kdv-render-runtime="katana-render-runtime-stub">"#,
            ),
            ("svg output", "<svg"),
        ],
    );
    Ok(())
}
