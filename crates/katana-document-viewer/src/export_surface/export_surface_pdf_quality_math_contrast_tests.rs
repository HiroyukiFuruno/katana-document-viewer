use crate::export_surface::test_modules::test_support::SurfaceTestSupport;
use crate::export_surface::{SurfaceBlock, SurfaceBlockFactory};
use crate::export_surface_helpers::SurfaceHelpers;

type ErrorBox = Box<dyn std::error::Error>;

const ALPHA_CHANNEL_INDEX: usize = 3;
const MIN_OPAQUE_SAMPLE_ALPHA: u8 = 64;
const MAX_COLOR_CHANNEL_FLOAT: f32 = 255.0;
const LUMINANCE_RED_WEIGHT: f32 = 0.299;
const LUMINANCE_GREEN_WEIGHT: f32 = 0.587;
const LUMINANCE_BLUE_WEIGHT: f32 = 0.114;

#[test]
fn pdf_surface_math_svg_is_dark_enough_for_katana_light_theme() -> Result<(), ErrorBox> {
    MathDarknessContractCase::assert_math_svg_contract()
}

struct MathDarknessContractCase;

impl MathDarknessContractCase {
    fn assert_math_svg_contract() -> Result<(), ErrorBox> {
        let graph = Self::build_math_fixture_graph()?;
        let theme = crate::KdvThemeSnapshot::katana_light();
        let blocks = SurfaceBlockFactory::create(&graph, &theme);
        let expected_color = SurfaceHelpers::parse_color(&theme.text);
        let (math_blocks, inline_math_spans) = Self::count_math_targets(&blocks, expected_color)?;

        Self::assert_sufficient_contract(math_blocks, inline_math_spans)
    }

    fn build_math_fixture_graph() -> Result<crate::BuildGraph, ErrorBox> {
        SurfaceTestSupport::graph_from_markdown(
            "math-contrast.md",
            concat!(
                "本文: $E=mc^2$ があります。\n\n",
                "$$ \\frac{1}{2} = 0.5 $$\n\n",
                "```math\nx^2 + y^2 = r^2\n```",
            )
            .to_string(),
        )
    }

    fn count_math_targets(
        blocks: &[SurfaceBlock],
        expected_color: image::Rgba<u8>,
    ) -> Result<(usize, usize), ErrorBox> {
        let mut math_blocks = 0usize;
        let mut inline_math_spans = 0usize;
        for block in blocks {
            match block {
                SurfaceBlock::Math(math) => {
                    math_blocks += Self::count_math_block(math, expected_color)?
                }
                SurfaceBlock::Line(line) => {
                    inline_math_spans += Self::count_inline_math_spans(line, expected_color)
                }
                _ => {}
            }
        }
        Ok((math_blocks, inline_math_spans))
    }

    fn count_math_block(
        math: &crate::export_surface::SurfaceMathBlock,
        expected_color: image::Rgba<u8>,
    ) -> Result<usize, ErrorBox> {
        let Some(svg_raster) = math.image.as_ref() else {
            return Err("math block must be rasterized to svg before PDF rendering".into());
        };
        MathRasterDarknessStats::assert_image_is_sufficiently_dark(
            &svg_raster.image,
            expected_color,
        );
        Ok(1)
    }

    fn count_inline_math_spans(
        line: &crate::export_surface_line::SurfaceLine,
        expected_color: image::Rgba<u8>,
    ) -> usize {
        line.spans
            .iter()
            .filter(|span| Self::inline_math_is_dark(span, expected_color))
            .count()
    }

    fn inline_math_is_dark(
        span: &crate::export_surface_span::SurfaceTextSpan,
        expected_color: image::Rgba<u8>,
    ) -> bool {
        let Some(inline_image) = &span.inline_image else {
            return false;
        };
        MathRasterDarknessStats::assert_image_is_sufficiently_dark(
            inline_image.image(),
            expected_color,
        );
        true
    }

    fn assert_sufficient_contract(
        math_blocks: usize,
        inline_math_spans: usize,
    ) -> Result<(), ErrorBox> {
        if math_blocks < 2 {
            return Err("block math must be rasterized from SVG".into());
        }
        if inline_math_spans < 1 {
            return Err(
                "inline math must be rasterized from SVG instead of approximate text".into(),
            );
        }
        Ok(())
    }
}

struct MathRasterDarknessStats {
    mean_luminance: f32,
    max_distance: f32,
}

#[derive(Default)]
struct MathRasterDarknessAccumulator {
    sample_count: usize,
    luminance_sum: f32,
    max_distance: f32,
}

impl MathRasterDarknessStats {
    fn assert_image_is_sufficiently_dark(image: &image::RgbaImage, expected: image::Rgba<u8>) {
        Self::from(image, expected).assert_satisfy_contract();
    }

    fn from(image: &image::RgbaImage, expected: image::Rgba<u8>) -> Self {
        let accumulator = MathRasterDarknessAccumulator::from_image(image, expected);
        accumulator.into_stats()
    }

    fn assert_satisfy_contract(&self) {
        assert!(
            self.mean_luminance <= 140.0,
            "math rasterized pixels are too bright (mean={})",
            self.mean_luminance
        );
        assert!(
            self.max_distance <= 150.0,
            "math rasterized pixels differ too much from theme text color (max_distance={})",
            self.max_distance
        );
    }

    fn color_distance(left: &image::Rgba<u8>, right: &image::Rgba<u8>) -> f32 {
        let red_delta = left[0] as f32 - right[0] as f32;
        let green_delta = left[1] as f32 - right[1] as f32;
        let blue_delta = left[2] as f32 - right[2] as f32;
        (red_delta * red_delta + green_delta * green_delta + blue_delta * blue_delta).sqrt()
    }
}

impl MathRasterDarknessAccumulator {
    fn from_image(image: &image::RgbaImage, expected: image::Rgba<u8>) -> Self {
        let mut accumulator = Self::default();
        for pixel in image.pixels() {
            accumulator.collect_pixel(pixel, &expected);
        }
        accumulator
    }

    fn collect_pixel(&mut self, pixel: &image::Rgba<u8>, expected: &image::Rgba<u8>) {
        if pixel[ALPHA_CHANNEL_INDEX] < MIN_OPAQUE_SAMPLE_ALPHA {
            return;
        }
        self.luminance_sum += Self::pixel_luminance(pixel);
        self.sample_count += 1;
        self.max_distance = self
            .max_distance
            .max(MathRasterDarknessStats::color_distance(pixel, expected));
    }

    fn pixel_luminance(pixel: &image::Rgba<u8>) -> f32 {
        let alpha = pixel[ALPHA_CHANNEL_INDEX] as f32 / MAX_COLOR_CHANNEL_FLOAT;
        (LUMINANCE_RED_WEIGHT * pixel[0] as f32
            + LUMINANCE_GREEN_WEIGHT * pixel[1] as f32
            + LUMINANCE_BLUE_WEIGHT * pixel[2] as f32)
            * alpha
    }

    fn into_stats(self) -> MathRasterDarknessStats {
        assert!(
            self.sample_count > 0,
            "math raster image should contain sample pixels"
        );
        let mean_luminance = self.luminance_sum / self.sample_count.max(1) as f32;
        MathRasterDarknessStats {
            mean_luminance,
            max_distance: self.max_distance,
        }
    }
}
