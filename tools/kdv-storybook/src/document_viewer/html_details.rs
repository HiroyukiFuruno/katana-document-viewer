pub(crate) struct DetailsParts {
    pub(crate) summary: String,
    pub(crate) body: String,
    pub(crate) open: bool,
}

impl DetailsParts {
    pub(crate) fn parse(raw: &str) -> Option<Self> {
        let trimmed = raw.trim();
        let lower = trimmed.to_ascii_lowercase();
        if !lower.starts_with("<details") {
            return None;
        }
        let details_tag_end = lower.find('>')?;
        let summary_tag_start = lower.find("<summary")?;
        let summary_tag_end = lower[summary_tag_start..].find('>')? + summary_tag_start;
        let summary_start = summary_tag_end + 1;
        let summary_end = lower[summary_start..].find("</summary>")? + summary_start;
        let body_start = summary_end + "</summary>".len();
        let body_end = lower.rfind("</details>")?;
        Some(Self {
            summary: normalize_details_text(&trimmed[summary_start..summary_end]),
            body: normalize_details_text(&trimmed[body_start..body_end]),
            open: details_tag_has_open(&lower[..=details_tag_end]),
        })
    }

    pub(crate) fn from_plain_text(text: &str) -> Option<Self> {
        let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
        let summary = lines.next()?.to_string();
        let body = lines.collect::<Vec<_>>().join("\n");
        if body.is_empty() {
            return None;
        }
        Some(Self {
            summary,
            body,
            open: true,
        })
    }
}

fn details_tag_has_open(tag: &str) -> bool {
    tag.trim_matches(['<', '>'])
        .split_whitespace()
        .skip(1)
        .any(|attribute| attribute == "open" || attribute.starts_with("open="))
}

fn normalize_details_text(raw: &str) -> String {
    raw.replace("<div>", "")
        .replace("</div>", "")
        .replace("<p>", "")
        .replace("</p>", "")
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .trim()
        .to_string()
}
