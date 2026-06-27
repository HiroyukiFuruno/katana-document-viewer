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
        output
            .svg_payload()
            .map(str::to_string)
            .ok_or_else(|| output.diagnostic_message())
    }
}

#[cfg(test)]
mod tests {
    use super::KrrMathRenderEngine;

    #[test]
    fn render_display_svg_returns_mathjax_svg() -> Result<(), String> {
        let svg = KrrMathRenderEngine::render_display_svg(
            "E = mc^2",
            &crate::KdvThemeSnapshot::katana_light(),
        )?;

        assert!(svg.trim_start().starts_with("<svg"));
        Ok(())
    }
}
