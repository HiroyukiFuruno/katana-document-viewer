use crate::export_html_ops::ExportHtmlOps;
use crate::export_html_payload::HtmlExportPayloadFactory;
use crate::export_inline_payload::InlineHtmlWriter;
use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::ListItemNode;

pub(crate) struct ListHtmlWriter;

impl ListHtmlWriter {
    pub(crate) fn append(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        ordered: bool,
        items: &[ListItemNode],
        fallback_text: &str,
    ) {
        let tag = if ordered { "ol" } else { "ul" };
        Self::append_list_open(html, tag, ordered, items);
        if items.is_empty() {
            html.push_str(&format!(
                "<li>{}</li>",
                ExportHtmlOps::render_text(fallback_text)
            ));
        } else {
            for item in items {
                Self::append_item(html, graph, theme, item);
            }
        }
        html.push_str(&format!("</{tag}>\n"));
    }

    fn append_list_open(html: &mut String, tag: &str, ordered: bool, items: &[ListItemNode]) {
        let start_attr = if ordered {
            Self::ordered_start_attr(items)
        } else {
            String::new()
        };
        html.push_str(&format!("<{tag}{start_attr}>"));
    }

    fn ordered_start_attr(items: &[ListItemNode]) -> String {
        items
            .first()
            .and_then(|item| item.ordered_number)
            .filter(|number| *number != 1)
            .map_or_else(String::new, |number| format!(" start=\"{number}\""))
    }

    fn append_item(
        html: &mut String,
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        item: &ListItemNode,
    ) {
        if let Some(marker) = &item.task_marker {
            html.push_str("<li data-kdv-task-item=\"true\">");
            Self::append_task_marker(html, marker);
        } else {
            html.push_str("<li>");
        }
        for node in &item.body {
            InlineHtmlWriter::append_node(html, node, theme);
        }
        for child in &item.children {
            HtmlExportPayloadFactory::append_node(html, graph, theme, child);
        }
        html.push_str("</li>");
    }

    fn append_task_marker(html: &mut String, marker: &str) {
        let state = Self::task_state(marker);
        let checked_attr = if state.checked { " checked" } else { "" };
        let mixed_attr = if state.mixed {
            " aria-checked=\"mixed\""
        } else {
            ""
        };
        html.push_str(&format!(
            "<input type=\"checkbox\" disabled data-kdv-task-marker=\"{}\" data-kdv-task-state=\"{}\"{mixed_attr}{checked_attr}>",
            ExportHtmlOps::escape_html(marker),
            state.label
        ));
        html.push_str(&format!(
            "<span data-kdv-task-visual=\"{}\" aria-hidden=\"true\"></span> ",
            state.visual_kind
        ));
    }

    fn task_state(marker: &str) -> TaskMarkerState {
        match marker {
            "[x]" => TaskMarkerState {
                label: "done",
                checked: true,
                mixed: false,
                visual_kind: "done-check",
            },
            "[-]" => TaskMarkerState {
                label: "in-progress",
                checked: false,
                mixed: true,
                visual_kind: "in-progress-dash",
            },
            "[/]" => TaskMarkerState {
                label: "in-progress",
                checked: false,
                mixed: true,
                visual_kind: "in-progress-slash",
            },
            _ => TaskMarkerState {
                label: "todo",
                checked: false,
                mixed: false,
                visual_kind: "todo",
            },
        }
    }
}

struct TaskMarkerState {
    label: &'static str,
    checked: bool,
    mixed: bool,
    visual_kind: &'static str,
}
