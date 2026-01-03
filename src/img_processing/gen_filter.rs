use image::GrayAlphaImage;
use rand::prelude::*;

use crate::consts::{BLACK_COLOR, TRANSPARENT_COLOR};

pub fn gen_filter_full_rng(width: u32, height: u32, probability: f64) -> GrayAlphaImage {
    let mut image = GrayAlphaImage::new(width, height);
    let mut rng = rand::rng();

    for w in 0..width {
        for h in 0..height {
            let is_black = rng.random_bool(probability);

            let pixalval = match is_black {
                true => BLACK_COLOR,
                false => TRANSPARENT_COLOR,
            };

            image.put_pixel(w, h, pixalval);
        }
    }

    image
}
