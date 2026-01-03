use crate::consts::BLACK_COLOR;
use crate::consts::TRANSPARENT_COLOR;
use anyhow::Result;
use anyhow::anyhow;
use image::GrayAlphaImage;
use image::{Rgba, RgbaImage};

pub fn apply_colors(
    input: &GrayAlphaImage,
    black_color: Rgba<u8>,
    transparent_color: Rgba<u8>,
) -> Result<RgbaImage> {
    let mut output = RgbaImage::new(input.width(), input.height());
    for w in 0..input.width() {
        for h in 0..input.height() {
            let out_pixel = match *input.get_pixel(w, h) {
                BLACK_COLOR => black_color,
                TRANSPARENT_COLOR => transparent_color,
                _ => return Err(anyhow!("Invalid pixel value at x:{} , y:{}", w, h)),
            };
            output.put_pixel(w, h, out_pixel);
        }
    }

    Ok(output)
}
