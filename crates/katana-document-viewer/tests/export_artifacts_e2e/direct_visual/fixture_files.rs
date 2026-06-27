use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder, Rgba, RgbaImage};
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn write_png(path: &Path) -> Result<(), Box<dyn Error>> {
    let image = fixture_image();
    let mut bytes = Vec::new();
    PngEncoder::new(&mut bytes).write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        ColorType::Rgba8.into(),
    )?;
    fs::write(path, bytes)?;
    Ok(())
}

pub fn write_jpeg(path: &Path) -> Result<(), Box<dyn Error>> {
    let image = fixture_image();
    let mut rgb = Vec::new();
    for pixel in image.pixels() {
        rgb.extend_from_slice(&[pixel[0], pixel[1], pixel[2]]);
    }
    let mut bytes = Vec::new();
    JpegEncoder::new_with_quality(&mut bytes, 90).encode(
        &rgb,
        image.width(),
        image.height(),
        ColorType::Rgb8.into(),
    )?;
    fs::write(path, bytes)?;
    Ok(())
}

pub fn write_svg(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::write(path, large_svg())?;
    Ok(())
}

pub fn mermaid_source() -> &'static str {
    "graph TD\n  A[Direct visual] --> B[Exported artifact]"
}

pub fn drawio_source() -> &'static str {
    "<mxfile><diagram><mxGraphModel><root /></mxGraphModel></diagram></mxfile>"
}

fn fixture_image() -> RgbaImage {
    let mut image = RgbaImage::from_pixel(800, 520, Rgba([245, 245, 245, 255]));
    for y in 80..440 {
        for x in 120..680 {
            image.put_pixel(x, y, Rgba([34, 94, 168, 255]));
        }
    }
    image
}

fn large_svg() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="520"><rect width="800" height="520" fill="#f5f5f5"/><rect x="120" y="80" width="560" height="360" fill="#225ea8"/></svg>"##
}
