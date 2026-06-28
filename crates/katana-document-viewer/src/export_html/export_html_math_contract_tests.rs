use super::contract_test_support::HtmlContractTestSupport;

#[test]
fn red_detects_math_html_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(math_contract_markdown())?;
    assert_math_contract_contains(&html);
    assert_math_contract_hides_raw_markdown(&html);
    Ok(())
}

fn math_contract_markdown() -> &'static str {
    r##"```math
f(x) = x^2
```

質量: $E=mc^2$

$$ \sum_{k=1}^{n} k = \frac{n(n+1)}{2} $$
"##
}

fn assert_math_contract_contains(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "fenced math block",
                r#"<div data-kdv-math="block" data-kdv-render-runtime="katana-render-runtime">"#,
            ),
            (
                "compact inline math",
                r#"<span data-kdv-math="inline" data-kdv-render-runtime="katana-render-runtime">"#,
            ),
            (
                "one line dollar math",
                r#"<div data-kdv-math="dollar-block" data-kdv-render-runtime="katana-render-runtime">"#,
            ),
            ("svg output", "<svg"),
        ],
    );
}

fn assert_math_contract_hides_raw_markdown(html: &str) {
    HtmlContractTestSupport::assert_not_contains_any(
        html,
        &[
            ("math fence marker", "```math"),
            (
                "direct math renderer leak",
                "data-kdv-math-renderer=\"mathjax-svg\"",
            ),
        ],
    );
}

#[test]
fn red_detects_katana_specific_html_contract_gaps() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(katana_specific_markdown())?;
    assert_katana_specific_contract(&html);
    Ok(())
}

fn katana_specific_markdown() -> &'static str {
    r##"- [-] 完了扱い
- [/] 進行中

柔軟な数式: $ E = mc^2 $ と $$ a+b=c $$
"##
}

fn assert_katana_specific_contract(html: &str) {
    HtmlContractTestSupport::assert_contains_all(
        html,
        &[
            (
                "task done marker",
                r#"<input type="checkbox" disabled data-kdv-task-marker="[-]" data-kdv-task-state="blocked" aria-checked="mixed">"#,
            ),
            (
                "task in progress marker",
                r#"<input type="checkbox" disabled data-kdv-task-marker="[/]" data-kdv-task-state="in-progress" aria-checked="mixed">"#,
            ),
            (
                "spaced inline math",
                r#"<span data-kdv-math="inline" data-kdv-render-runtime="katana-render-runtime">"#,
            ),
            (
                "inline dollar block",
                r#"<span data-kdv-math="inline" data-kdv-render-runtime="katana-render-runtime">"#,
            ),
            ("svg output", "<svg"),
        ],
    );
}

#[test]
fn red_detects_math_svg_color_in_html_contract() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html(
        r##"本文: $E=mc^2$ の式と

```math
x^2 + y^2 = r^2
```

$$ \frac{1}{2} = 0.5 $$

$A+B=C$
"##,
    )?;

    let inline_svg = extract_math_svg(&html, "span", "inline").ok_or_else(|| {
        std::io::Error::other(format!("inline math should contain an svg element: {html}"))
    })?;
    assert_math_svg_uses_color(&inline_svg, "#242424");
    let block_svg = extract_math_svg(&html, "div", "block").ok_or_else(|| {
        std::io::Error::other(format!("math block should contain an svg element: {html}"))
    })?;
    assert_math_svg_uses_color(&block_svg, "#242424");
    let one_line_svg = extract_math_svg(&html, "div", "dollar-block").ok_or_else(|| {
        std::io::Error::other(format!(
            "one line dollar math should contain an svg element: {html}"
        ))
    })?;
    assert_math_svg_uses_color(&one_line_svg, "#242424");
    Ok(())
}

#[test]
fn app_supplied_complete_theme_reaches_krr_math_svg() -> Result<(), Box<dyn std::error::Error>> {
    let mut theme = crate::KdvThemeSnapshot::katana_light();
    theme.name = "app-supplied-light".to_string();
    theme.text = "#123456".to_string();

    let html = HtmlContractTestSupport::export_html_with_theme("本文: $E=mc^2$\n", theme)?;
    let inline_svg = extract_math_svg(&html, "span", "inline")
        .ok_or("inline math should contain an svg element")?;

    assert_math_svg_uses_color(&inline_svg, "#123456");
    assert!(!inline_svg.to_ascii_lowercase().contains("#242424"));
    Ok(())
}

#[test]
fn inline_math_svg_must_render_full_expression() -> Result<(), Box<dyn std::error::Error>> {
    let html = HtmlContractTestSupport::export_html("質量とエネルギーの等価原理: $E = mc^2$\n")?;
    let inline_svg = extract_math_svg(&html, "span", "inline")
        .ok_or("inline math should contain an svg element")?;

    assert!(
        inline_svg.contains(r#"data-latex="E = mc^2""#),
        "inline math root metadata must keep the full expression: {inline_svg}"
    );
    assert_eq!(
        extract_data_latex(&inline_svg).as_deref(),
        Some("E = mc^2"),
        "inline math svg should carry the full raw expression as data-latex: {inline_svg}"
    );
    assert!(
        inline_svg.contains(r#"data-latex="=""#)
            && inline_svg.contains(r#"data-latex="m""#)
            && inline_svg.contains(r#"data-latex="c""#),
        "inline math must render the full expression, not only the first token: {inline_svg}"
    );
    Ok(())
}

fn extract_math_svg(html: &str, tag: &str, role: &str) -> Option<String> {
    let marker = format!(r#"<{tag} data-kdv-math="{role}""#);
    let marker_index = html.find(&marker)?;
    let remainder = &html[marker_index..];
    let svg_start = remainder.find("<svg")?;
    let svg_end = remainder[svg_start..].find("</svg>")?;
    Some(remainder[svg_start..svg_start + svg_end + "</svg>".len()].to_string())
}

fn extract_data_latex(svg: &str) -> Option<String> {
    let marker = "data-latex=\"";
    let start = svg.find(marker)?;
    let value_start = start + marker.len();
    let value_end = svg[value_start..].find('\"')? + value_start;
    Some(svg[value_start..value_end].to_string())
}

fn assert_math_svg_uses_color(svg: &str, expected: &str) {
    let normalized = svg.to_ascii_lowercase().replace(' ', "");
    let expected = expected.to_ascii_lowercase();
    assert!(
        normalized.contains(&format!("color:{expected}")) || normalized.contains(&expected),
        "math svg should use theme text color {expected}: {svg}"
    );
    assert!(
        !normalized.contains("#e0e0e0"),
        "math svg should not fall back to KRR dark default: {svg}"
    );
}
