use super::*;
use crate::export_surface::{SurfaceImageBlock, SurfaceTableBlock};
use image::Rgba;
use katana_markdown_model::{
    ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TableAlignment, TableCell,
    TableNode, TableRow,
};

const IMAGE_WIDTH: u32 = 16;
const IMAGE_HEIGHT: u32 = 10;
const BLUE_CHANNEL: u8 = 3;
const OPAQUE_CHANNEL: u8 = 255;
const WHITE_PIXEL: Rgba<u8> = Rgba([
    OPAQUE_CHANNEL,
    OPAQUE_CHANNEL,
    OPAQUE_CHANNEL,
    OPAQUE_CHANNEL,
]);

fn row(cells: &[&str]) -> TableRow {
    TableRow {
        cells: cells
            .iter()
            .map(|text| TableCell {
                text: (*text).to_string(),
                source: SourceSpan {
                    byte_range: ByteRange { start: 0, end: 0 },
                    line_column_range: LineColumnRange {
                        start: LineColumn { line: 0, column: 0 },
                        end: LineColumn { line: 0, column: 0 },
                    },
                    raw: RawSnippet {
                        text: (*text).to_string(),
                    },
                },
            })
            .collect(),
    }
}

fn table_block() -> SurfaceBlock {
    let table = SurfaceTableBlock::new(&TableNode {
        alignments: vec![TableAlignment::Left],
        rows: vec![row(&["head"]), row(&["value"])],
    });
    SurfaceBlock::Table(table)
}

fn image_block() -> Option<SurfaceBlock> {
    let path = std::env::temp_dir().join("kdv-paint-block-image.png");
    let saved = RgbaImage::from_pixel(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        Rgba([1, 2, BLUE_CHANNEL, OPAQUE_CHANNEL]),
    )
    .save(&path);
    assert!(saved.is_ok());
    SurfaceImageBlock::from_path(&path, None, "alt".to_string()).map(SurfaceBlock::Image)
}

fn paint_test_block(image: &mut RgbaImage, block: &SurfaceBlock, y: u32) {
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut painter = crate::export_surface_font::SurfaceTextPainter::from_system_fonts();
    SurfacePainter::paint_block(image, block, y, &mut painter, &palette);
}

fn has_painted_pixel(image: &RgbaImage) -> bool {
    image.pixels().any(|pixel| *pixel != WHITE_PIXEL)
}

#[test]
fn paint_block_dispatches_table_and_image_blocks() {
    let mut image = RgbaImage::from_pixel(SURFACE_WIDTH, 240, WHITE_PIXEL);

    paint_test_block(&mut image, &table_block(), 0);
    assert!(image_block().is_some());
    if let Some(block) = image_block() {
        paint_test_block(&mut image, &block, 120);
    }

    assert!(has_painted_pixel(&image));
}

#[test]
fn paint_stacks_surface_blocks_without_implicit_gap() {
    let theme = KdvThemeSnapshot::katana_light();
    let background = SurfaceHelpers::parse_color(&theme.background);
    let mut image = RgbaImage::from_pixel(SURFACE_WIDTH, 220, background);
    let blocks = vec![SurfaceBlock::Rule, SurfaceBlock::Rule];

    SurfacePainter::paint(&mut image, &blocks, &theme);

    let rule_height = SurfaceBlock::Rule.height();
    let second_rule_y = PAGE_PADDING + rule_height + rule_height / 2;
    assert_ne!(background, *image.get_pixel(PAGE_PADDING, second_rule_y));
}
