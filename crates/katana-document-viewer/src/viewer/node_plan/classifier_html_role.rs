use super::super::types::{ViewerHtmlAlignment, ViewerHtmlRole};
use katana_markdown_model::HtmlBlockRole;

pub(super) struct ViewerHtmlRoleClassifier;

impl ViewerHtmlRoleClassifier {
    pub(super) fn from_role(role: &HtmlBlockRole) -> ViewerHtmlRole {
        match role {
            HtmlBlockRole::Generic => ViewerHtmlRole::Generic,
            HtmlBlockRole::Centered => ViewerHtmlRole::Centered,
            HtmlBlockRole::BadgeRow => ViewerHtmlRole::BadgeRow,
        }
    }

    #[inline(never)]
    pub(super) fn from_source(role: &HtmlBlockRole, raw: &str) -> ViewerHtmlRole {
        let lower = normalized(raw);
        if contains_shields_badge(&lower) {
            return ViewerHtmlRole::BadgeRow;
        }
        if let Some(role) = heading_role(&lower) {
            return role;
        }
        if contains_center_alignment(&lower) {
            return ViewerHtmlRole::Centered;
        }
        if contains_right_alignment(&lower) {
            return ViewerHtmlRole::Right;
        }
        if contains_left_alignment(&lower) {
            return ViewerHtmlRole::Left;
        }
        Self::from_role(role)
    }

    pub(super) fn from_paragraph_source(raw: &str) -> Option<ViewerHtmlRole> {
        let trimmed = raw.trim_start();
        if !starts_with_html_block_tag(trimmed) {
            return None;
        }
        Some(Self::from_source(&HtmlBlockRole::Generic, trimmed))
    }
}

fn starts_with_html_block_tag(raw: &str) -> bool {
    let lower = raw.to_ascii_lowercase();
    [
        "<h1", "<h2", "<h3", "<h4", "<h5", "<h6", "<p", "<img", "<div", "<section", "<article",
    ]
    .iter()
    .any(|tag| lower.starts_with(tag))
}

fn contains_center_alignment(value: &str) -> bool {
    contains_any(
        value,
        &["text-align:center", "align=\"center\"", "align=center"],
    )
}

fn contains_right_alignment(value: &str) -> bool {
    contains_any(
        value,
        &["text-align:right", "align=\"right\"", "align=right"],
    )
}

fn contains_left_alignment(value: &str) -> bool {
    contains_any(value, &["text-align:left", "align=\"left\"", "align=left"])
}

fn heading_role(value: &str) -> Option<ViewerHtmlRole> {
    let level = heading_level(value)?;
    Some(ViewerHtmlRole::Heading {
        level,
        alignment: html_alignment(value),
    })
}

fn heading_level(value: &str) -> Option<u8> {
    for level in 1..=6 {
        if value.starts_with(&format!("<h{level}")) {
            return Some(level);
        }
    }
    None
}

fn html_alignment(value: &str) -> ViewerHtmlAlignment {
    if contains_center_alignment(value) {
        return ViewerHtmlAlignment::Center;
    }
    if contains_right_alignment(value) {
        return ViewerHtmlAlignment::Right;
    }
    ViewerHtmlAlignment::Left
}

fn contains_shields_badge(value: &str) -> bool {
    value.contains("img.shields.io/badge/")
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    for needle in needles {
        if value.contains(needle) {
            return true;
        }
    }
    false
}

fn normalized(raw: &str) -> String {
    raw.to_ascii_lowercase()
        .split_whitespace()
        .collect::<String>()
        .replace('\'', "\"")
}
