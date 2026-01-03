use crate::{
    consts::{BLACK_COLOR, TRANSPARENT_COLOR},
    utils::same_size::is_same_size,
};
use anyhow::Result;
use anyhow::anyhow;
use image::{GrayAlphaImage, Rgba, RgbaImage};

pub fn gen_demo(
    input: &GrayAlphaImage,
    filter: &GrayAlphaImage,
    front_color: Rgba<u8>,
    back_color: Rgba<u8>,
) -> Result<RgbaImage> {
    if !is_same_size(input, filter) {
        return Err(anyhow!("input and filter dimensions does not match"));
    }
    let mut output = RgbaImage::new(input.width(), input.height());
    for w in 0..input.width() {
        for h in 0..input.height() {
            let out_pixel = match (*filter.get_pixel(w, h), *input.get_pixel(w, h)) {
                (BLACK_COLOR, _) => front_color,
                (TRANSPARENT_COLOR, TRANSPARENT_COLOR) => back_color,
                (TRANSPARENT_COLOR, BLACK_COLOR) => front_color,
                _ => return Err(anyhow!("Invalid pixel value at x:{} , y:{}", w, h)),
            };
            output.put_pixel(w, h, out_pixel);
        }
    }
    Ok(output)
}
