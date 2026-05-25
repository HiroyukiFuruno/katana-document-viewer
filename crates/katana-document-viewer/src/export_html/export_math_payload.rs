use crate::export_html_ops::ExportHtmlOps;
use crate::render_runtime::{
    KRR_RENDER_RUNTIME_ID, KrrMathMode, KrrRenderPayload, KrrRenderRuntimeAdapter,
};
use crate::theme::KdvThemeSnapshot;

pub(crate) struct MathHtmlWriter;

impl MathHtmlWriter {
    pub(crate) fn append_inline(html: &mut String, expression: &str, theme: &KdvThemeSnapshot) {
        Self::append_rendered(html, "span", "inline", expression, true, theme);
    }

    pub(crate) fn append_block(
        html: &mut String,
        role: &str,
        expression: &str,
        theme: &KdvThemeSnapshot,
    ) {
        Self::append_rendered(html, "div", role, expression, false, theme);
        html.push('\n');
    }

    fn append_rendered(
        html: &mut String,
        tag: &str,
        role: &str,
        expression: &str,
        inline: bool,
        theme: &KdvThemeSnapshot,
    ) {
        let trimmed = expression.trim();
        let math_mode = if inline {
            KrrMathMode::Inline
        } else {
            KrrMathMode::Display
        };
        let output = KrrRenderRuntimeAdapter::render_math_tex_with_theme(
            trimmed,
            math_mode,
            Some(theme.krr_math_theme()),
        );
        match output.payload {
            KrrRenderPayload::Svg(math_svg) => {
                Self::append_svg_payload(html, tag, role, &math_svg);
            }
            KrrRenderPayload::Raw(ref raw) => {
                Self::append_raw_payload(html, tag, role, raw, &output.diagnostic_message());
            }
        }
    }

    fn append_svg_payload(html: &mut String, tag: &str, role: &str, math_svg: &str) {
        html.push_str(&format!(
            "<{tag} data-kdv-math=\"{role}\" data-kdv-render-runtime=\"{KRR_RENDER_RUNTIME_ID}\">{math_svg}</{tag}>"
        ));
    }

    fn append_raw_payload(html: &mut String, tag: &str, role: &str, raw: &str, diagnostic: &str) {
        html.push_str(&format!(
            "<{tag} data-kdv-math=\"{role}\" data-kdv-render-runtime=\"{KRR_RENDER_RUNTIME_ID}\" data-kdv-render-error=\"{}\">{}</{tag}>",
            ExportHtmlOps::escape_html(diagnostic),
            ExportHtmlOps::escape_html(raw)
        ));
    }
}
