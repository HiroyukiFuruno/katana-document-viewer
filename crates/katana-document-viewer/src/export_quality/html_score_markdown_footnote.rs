pub(super) fn footnote_reference_raw_leaked(html: &str, source: &str) -> bool {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with("[^"))
        .flat_map(footnote_reference_tokens)
        .any(|token| html.contains(&token))
}

fn footnote_reference_tokens(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("[^") {
        let candidate = &rest[start..];
        let Some(end) = candidate.find(']') else {
            break;
        };
        tokens.push(candidate[..=end].to_string());
        rest = &candidate[end + 1..];
    }
    tokens
}
