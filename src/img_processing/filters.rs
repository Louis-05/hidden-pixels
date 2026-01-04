use crate::bool_image::BoolImage;
use rand::prelude::*;

pub fn filter_full_rng(width: u32, height: u32, probability: f64) -> BoolImage {
    let mut image = BoolImage::new(width, height);
    let mut rng = rand::rng();

    for w in 0..width {
        for h in 0..height {
            let is_black = rng.random_bool(probability);
            image.set(w, h, is_black).unwrap();
        }
    }

    image
}

pub fn filter_grid_rng(width: u32, height: u32, probability: f64) -> BoolImage {
    let mut image = BoolImage::new(width, height);
    let mut rng = rand::rng();

    let mut rng_vec: Vec<bool> = Vec::new();
    let total = ((width + 1) / 2) * ((height + 1) / 2);
    for _ in 0..total {
        rng_vec.push(rng.random_bool(probability));
    }

    for w in 0..width {
        for h in 0..height {
            let rng_bool = rng_vec.get((w / 2 + ((w / 2) * (h / 2))) as usize).unwrap();
            image.set(w, h, rng_bool ^ ((w + h) % 2 == 0)).unwrap();
        }
    }

    image
}
