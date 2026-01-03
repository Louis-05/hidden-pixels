use crate::consts::BLACK_COLOR;
use crate::consts::{ TRANSPARENT_COLOR, TRANSPARENT_THRESHOLD};
use image::Pixel;
use image::{GrayAlphaImage, RgbaImage};

pub fn apply_contrast(input: &RgbaImage) -> GrayAlphaImage {
    let mut output = GrayAlphaImage::new(input.width(), input.height());
    for w in 0..input.width() {
        for h in 0..input.height() {
            let in_pixel = input.get_pixel(w, h);
            let lumaa_pixel = in_pixel.to_luma_alpha();

            if lumaa_pixel.alpha() <= TRANSPARENT_THRESHOLD {
                output.put_pixel(w, h, TRANSPARENT_COLOR);
            } else {
                output.put_pixel(w, h, BLACK_COLOR)
            }
        }
    }

    output
}
