use crate::export_html_ops::escape_html;
use crate::render_runtime::{
    KRR_STUB_RUNTIME_ID, KrrMathMode, KrrRenderPayload, StubKrrRenderRuntime,
};

pub(crate) struct MathHtmlWriter;

impl MathHtmlWriter {
    pub(crate) fn append_inline(html: &mut String, expression: &str) {
        Self::append_rendered(html, "span", "inline", expression, true);
    }

    pub(crate) fn append_block(html: &mut String, role: &str, expression: &str) {
        Self::append_rendered(html, "div", role, expression, false);
        html.push('\n');
    }

    fn append_rendered(html: &mut String, tag: &str, role: &str, expression: &str, inline: bool) {
        let trimmed = expression.trim();
        let math_mode = if inline {
            KrrMathMode::Inline
        } else {
            KrrMathMode::Display
        };
        let output = StubKrrRenderRuntime::render_math_tex(trimmed, math_mode);
        match output.payload {
            KrrRenderPayload::Svg(math_svg) => {
                html.push_str(&format!(
                    "<{tag} data-kdv-math=\"{role}\" data-kdv-render-runtime=\"{KRR_STUB_RUNTIME_ID}\">{math_svg}</{tag}>"
                ));
            }
            KrrRenderPayload::Raw(ref raw) => {
                html.push_str(&format!(
                    "<{tag} data-kdv-math=\"{role}\" data-kdv-render-runtime=\"{KRR_STUB_RUNTIME_ID}\" data-kdv-render-error=\"{}\">{}</{tag}>",
                    escape_html(&output.diagnostic_message()),
                    escape_html(raw)
                ));
            }
        }
    }
}
