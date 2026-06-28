use crate::PreviewOutputFactory;
use crate::{MarkdownPreview, MarkdownSource, PreviewConfig, PreviewError, PreviewOutput};
use surface_exporter::PreviewSurfaceExporter;

const DEFAULT_LINE_HEIGHT: f32 = 20.0;

pub struct PreviewRenderEngine;

impl MarkdownPreview for PreviewRenderEngine {
    fn render(
        &self,
        source: &MarkdownSource,
        config: &PreviewConfig,
    ) -> Result<PreviewOutput, PreviewError> {
        let mut output = self.render_viewer_output(source, config)?;
        self.attach_surface(&mut output, config);
        Ok(output)
    }
}

impl PreviewRenderEngine {
    pub fn render_viewer_output(
        &self,
        source: &MarkdownSource,
        config: &PreviewConfig,
    ) -> Result<PreviewOutput, PreviewError> {
        PreviewOutputFactory::from_source(
            source,
            config,
            Self::estimated_content_height(source, config),
        )
    }

    pub fn attach_surface(&self, output: &mut PreviewOutput, config: &PreviewConfig) {
        PreviewSurfaceExporter::attach_surface(output, config);
    }

    fn estimated_content_height(source: &MarkdownSource, config: &PreviewConfig) -> f32 {
        let line_height = config.line_height.unwrap_or(DEFAULT_LINE_HEIGHT);
        let line_count = source.content.lines().count().max(1) as f32;
        let bottom_spacer_height = config.viewport.height.max(0.0);
        line_count * line_height + bottom_spacer_height
    }
}

#[path = "engine_surface_exporter.rs"]
mod surface_exporter;

#[cfg(test)]
#[path = "engine_test_modules.rs"]
mod test_modules;
