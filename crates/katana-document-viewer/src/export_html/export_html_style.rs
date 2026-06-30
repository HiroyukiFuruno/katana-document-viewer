use crate::theme::KdvThemeSnapshot;

pub(crate) struct HtmlExportStyle;

impl HtmlExportStyle {
    pub(crate) fn append(html: &mut String, theme: &KdvThemeSnapshot) {
        Self::append_variables(html, theme);
        html.push_str(STATIC_STYLE);
    }

    fn append_variables(html: &mut String, theme: &KdvThemeSnapshot) {
        html.push_str(":root{");
        Self::append_var(html, "text", &theme.text);
        Self::append_var(html, "background", &theme.background);
        Self::append_var(html, "table-border", theme.export_table_border());
        Self::append_var(html, "table-header", theme.export_table_header_background());
        Self::append_var(html, "table-even", theme.export_table_even_row_background());
        Self::append_var(html, "quote-border", &theme.quote_border);
        Self::append_var(html, "quote-text", &theme.quote_text);
        Self::append_var(html, "alert-bg", &theme.alert_background);
        Self::append_var(html, "code-bg", &theme.code_background);
        Self::append_var(html, "code-border", &theme.code_border);
        Self::append_var(html, "task-active-bg", &theme.task_active_background);
        Self::append_var(html, "task-empty-bg", &theme.task_empty_background);
        Self::append_var(html, "task-done", &theme.task_done_accent);
        Self::append_var(html, "task-progress", &theme.task_in_progress_accent);
        Self::append_var(html, "footnote-border", &theme.footnote_border);
        Self::append_var(html, "footnote-text", &theme.footnote_text);
        Self::append_var(html, "alert-note", &theme.alert_note);
        Self::append_var(html, "alert-tip", &theme.alert_tip);
        Self::append_var(html, "alert-important", &theme.alert_important);
        Self::append_var(html, "alert-warning", &theme.alert_warning);
        Self::append_var(html, "alert-caution", &theme.alert_caution);
        Self::append_var(html, "diagram-background", &theme.diagram_background);
        html.push('}');
    }

    fn append_var(html: &mut String, name: &str, value: &str) {
        html.push_str(&format!("--kdv-{name}:{value};"));
    }
}

const STATIC_STYLE: &str = concat!(
    "body{background:var(--kdv-background);color:var(--kdv-text);font-family:-apple-system,BlinkMacSystemFont,\"Segoe UI\",\"Apple Color Emoji\",\"Segoe UI Emoji\",\"Segoe UI Symbol\",\"Noto Color Emoji\",sans-serif;}",
    "main{max-width:980px;margin:0 auto;padding:32px;}",
    "em{font-style:italic;}",
    ":not(pre)>code{font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:.92em;background:var(--kdv-code-bg);border:1px solid var(--kdv-code-border);border-radius:4px;padding:.12em .32em;white-space:break-spaces;}",
    "[data-kdv-table=\"katana\"]{width:100%;border-collapse:collapse;table-layout:fixed;margin:1rem 0;}",
    "[data-kdv-table=\"katana\"] th,[data-kdv-table=\"katana\"] td{border:1px solid var(--kdv-table-border);padding:6px 10px;vertical-align:top;}",
    "[data-kdv-table=\"katana\"] th{background:var(--kdv-table-header);font-weight:600;}",
    "[data-kdv-table=\"katana\"] tbody tr:nth-child(even) td{background:var(--kdv-table-even);}",
    "col[data-kdv-column-size=\"short\"]{width:12em;}",
    "col[data-kdv-column-size=\"wide\"]{width:auto;}",
    "th[data-kdv-column-size=\"short\"],td[data-kdv-column-size=\"short\"]{width:12em;max-width:12em;white-space:normal;overflow-wrap:anywhere;}",
    "th[data-kdv-column-size=\"wide\"],td[data-kdv-column-size=\"wide\"]{width:auto;white-space:normal;overflow-wrap:anywhere;}",
    "th[data-align=\"left\"],td[data-align=\"left\"]{text-align:left;}",
    "th[data-align=\"center\"],td[data-align=\"center\"]{text-align:center;}",
    "th[data-align=\"right\"],td[data-align=\"right\"]{text-align:right;}",
    "pre[data-kdv-code-role=\"plain\"]{background:var(--kdv-code-bg);border:1px solid var(--kdv-code-border);border-radius:6px;padding:1rem;overflow:auto;margin:1rem 0 1.25rem;}",
    "pre[data-kdv-code-role=\"plain\"] code{font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:.92em;color:var(--kdv-text);}",
    "hr{border:0;border-top:1px solid var(--kdv-table-border);margin:2rem 0;}",
    "blockquote{border-left:4px solid var(--kdv-quote-border);margin:1rem 0;padding:.25rem 0 .25rem 1rem;color:var(--kdv-quote-text);}",
    "blockquote p:first-child{margin-top:0;}blockquote p:last-child{margin-bottom:0;}",
    "blockquote blockquote{margin:.75rem 0;}",
    "blockquote[data-kdv-quote-depth] blockquote[data-kdv-quote-depth]{margin-top:.75rem;margin-bottom:0;}",
    "[data-kdv-blockquote=\"alert\"]{border-left:4px solid var(--kdv-quote-border);margin:1rem 0;padding:.75rem 0 .75rem 1rem;}",
    "[data-kdv-alert-title]{display:flex;align-items:center;gap:.4rem;font-weight:600;margin-top:0;}",
    "[data-kdv-alert-title=\"NOTE\"]{color:var(--kdv-alert-note);}",
    "[data-kdv-alert-title=\"TIP\"]{color:var(--kdv-alert-tip);}",
    "[data-kdv-alert-title=\"IMPORTANT\"]{color:var(--kdv-alert-important);}",
    "[data-kdv-alert-title=\"WARNING\"]{color:var(--kdv-alert-warning);}",
    "[data-kdv-alert-title=\"CAUTION\"]{color:var(--kdv-alert-caution);}",
    "span[data-kdv-alert-icon]{display:inline-flex;align-items:center;justify-content:center;width:1.15em;height:1.15em;line-height:1;}",
    "svg[data-kdv-alert-icon-svg]{width:1.05em;height:1.05em;stroke:currentColor;fill:none;stroke-width:1.8;stroke-linecap:round;stroke-linejoin:round;}",
    "details[data-kdv-accordion]{margin:1rem 0;}",
    "details[data-kdv-accordion]>summary{cursor:pointer;font-weight:600;}",
    "details[data-kdv-accordion]>[data-kdv-accordion-body]{margin-top:.75rem;}",
    "[data-kdv-render-runtime]{overflow-x:auto;}",
    "[data-kdv-render-runtime] svg{max-width:100%;height:auto;vertical-align:-.2em;}",
    "div[data-kdv-render-runtime]{margin:1rem 0;text-align:center;}",
    "figure[data-kdv-diagram]{background:var(--kdv-diagram-background);}",
    "[data-github-alert=\"NOTE\"]{border-left-color:var(--kdv-alert-note);}",
    "[data-github-alert=\"TIP\"]{border-left-color:var(--kdv-alert-tip);}",
    "[data-github-alert=\"IMPORTANT\"]{border-left-color:var(--kdv-alert-important);}",
    "[data-github-alert=\"WARNING\"]{border-left-color:var(--kdv-alert-warning);}",
    "[data-github-alert=\"CAUTION\"]{border-left-color:var(--kdv-alert-caution);}",
    "li[data-kdv-task-item=\"true\"]{list-style:none;}",
    "li[data-kdv-task-item=\"true\"]>input{position:absolute;opacity:0;width:1px;height:1px;margin:0;}",
    "span[data-kdv-task-visual]{display:inline-flex;align-items:center;justify-content:center;width:.85em;height:.85em;margin-right:.55em;border-radius:2px;vertical-align:-.05em;}",
    "span[data-kdv-task-visual=\"todo\"]{background:var(--kdv-task-empty-bg);}",
    "span[data-kdv-task-visual=\"done-check\"]{background:var(--kdv-task-active-bg);color:var(--kdv-task-done);}",
    "span[data-kdv-task-visual=\"blocked-dash\"],span[data-kdv-task-visual=\"in-progress-slash\"]{background:var(--kdv-task-active-bg);color:var(--kdv-task-progress);}",
    "span[data-kdv-task-visual=\"done-check\"]::before{content:\"\";width:.32em;height:.58em;border:solid currentColor;border-width:0 .14em .14em 0;transform:rotate(45deg);}",
    "span[data-kdv-task-visual=\"blocked-dash\"]::before{content:\"\";width:.62em;border-top:.14em solid currentColor;}",
    "span[data-kdv-task-visual=\"in-progress-slash\"]::before{content:\"\";width:.72em;border-top:.14em solid currentColor;transform:rotate(-45deg);}",
    "section[data-kdv-footnotes]{border-top:1px solid var(--kdv-footnote-border);margin-top:2rem;padding-top:1rem;font-size:.92em;color:var(--kdv-footnote-text);}",
    "li[data-kdv-footnote-definition]:target{background:var(--kdv-table-even);scroll-margin-top:1rem;}"
);
