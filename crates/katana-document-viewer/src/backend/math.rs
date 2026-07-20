use crate::KdvThemeSnapshot;
use crate::render_runtime::{KrrMathMode, KrrRenderRuntimeAdapter};

pub struct KrrMathRenderEngine;

impl KrrMathRenderEngine {
    pub fn render_display_svg(source: &str, theme: &KdvThemeSnapshot) -> Result<String, String> {
        let output = KrrRenderRuntimeAdapter::render_math_tex_with_theme(
            source,
            KrrMathMode::Display,
            Some(theme.krr_math_theme()),
        );
        match output.svg_payload() {
            Some(svg) => Ok(svg.to_string()),
            None => Err(output.diagnostic_message()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KrrMathRenderEngine;
    use crate::render_runtime::RenderRuntimeTestEnv;

    #[test]
    fn render_display_svg_returns_mathjax_svg() {
        RenderRuntimeTestEnv::with_mathjax_env(None, || {
            let result = KrrMathRenderEngine::render_display_svg(
                "E = mc^2",
                &crate::KdvThemeSnapshot::katana_light(),
            );

            assert!(matches!(result, Ok(svg) if svg.trim_start().starts_with("<svg")));
        })
    }

    #[test]
    fn render_display_svg_returns_error_for_empty_source() {
        let result =
            KrrMathRenderEngine::render_display_svg("", &crate::KdvThemeSnapshot::katana_light());

        assert!(matches!(result, Err(error) if !error.is_empty()));
    }
}
