use anyhow::{Result, anyhow};
use image::GrayAlphaImage;

use crate::{
    consts::{BLACK_COLOR, TRANSPARENT_COLOR},
    utils::same_size::is_same_size,
};

pub fn apply_filter(input: &GrayAlphaImage, filter: &GrayAlphaImage) -> Result<GrayAlphaImage> {
    if !is_same_size(input, filter) {
        return Err(anyhow!("input and filter dimensions does not match"));
    }

    let mut output = GrayAlphaImage::new(input.width(), input.height());

    for w in 0..input.width() {
        for h in 0..input.height() {
            let input_pixel = input.get_pixel(w, h);
            let filter_pixel = filter.get_pixel(w, h);

            let out_pixel = match (input_pixel, filter_pixel) {
                (&BLACK_COLOR, &TRANSPARENT_COLOR) => BLACK_COLOR,
                (&TRANSPARENT_COLOR, &TRANSPARENT_COLOR) => TRANSPARENT_COLOR,
                (&BLACK_COLOR, &BLACK_COLOR) => TRANSPARENT_COLOR,
                (&TRANSPARENT_COLOR, &BLACK_COLOR) => BLACK_COLOR,
                _ => return Err(anyhow!("Invalid pixel value at x:{} , y:{}", w, h)),
            };

            output.put_pixel(w, h, out_pixel);
        }
    }

    Ok(output)
}
