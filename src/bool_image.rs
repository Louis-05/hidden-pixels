use anyhow::{Result, anyhow};
use image::{GrayAlphaImage, LumaA};

use crate::consts::TRANSPARENT_THRESHOLD;

#[derive(Debug, Clone)]
pub struct BoolImage {
    pub width: u32,
    pub height: u32,
    pub vec: Vec<bool>,
}

impl BoolImage {
    pub fn new(width: u32, height: u32) -> Self {
        let total_pixels = width * height;
        let mut vec: Vec<bool> = Vec::with_capacity(total_pixels as usize);
        for _ in 0..total_pixels {
            vec.push(false);
        }

        Self { width, height, vec }
    }

    fn from_vec(width: u32, height: u32, vec: Vec<bool>) -> Self {
        let total_pixels = width * height;
        if total_pixels != vec.len() as u32 {
            panic!("wrong size {}, {} , {}",vec.len(),width,height)
        }
        Self { width, height, vec }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<bool> {
        if x >= self.width || y >= self.height {
            return None;
        }

        return self.vec.get((x + y * self.width) as usize).copied();
    }

    pub fn set(&mut self, x: u32, y: u32, new_val: bool) -> Result<()> {
        if x >= self.width || y >= self.height {
            return Err(anyhow!("out of bound"));
        }
        let val = self.vec.get_mut((x + y * self.width) as usize).unwrap();
        *val = new_val;
        Ok(())
    }

    pub fn xor(&self, other: &Self) -> Result<Self> {
        if !self.is_same_size(other) {
            return Err(anyhow!("Size does not match"));
        }
        let out_vec: Vec<bool> = self
            .vec
            .iter()
            .zip(other.vec.iter())
            .map(|(a, b)| a ^ b)
            .collect();
        Ok(Self::from_vec(self.width, self.height, out_vec))
    }

    pub fn and(&self, other: &Self) -> Result<Self> {
        if !self.is_same_size(other) {
            return Err(anyhow!("Size does not match"));
        }
        let out_vec: Vec<bool> = self
            .vec
            .iter()
            .zip(other.vec.iter())
            .map(|(a, b)| a & b)
            .collect();
        Ok(Self::from_vec(self.width, self.height, out_vec))
    }

    pub fn or(&self, other: &Self) -> Result<Self> {
        if !self.is_same_size(other) {
            return Err(anyhow!("Size does not match"));
        }
        let out_vec: Vec<bool> = self
            .vec
            .iter()
            .zip(other.vec.iter())
            .map(|(a, b)| a | b)
            .collect();
        Ok(Self::from_vec(self.width, self.height, out_vec))
    }

    pub fn inverse(&self) -> Self {
        let mut out = self.clone();
        out.vec.iter_mut().for_each(|b| *b = !*b);
        out
    }

    pub fn is_same_size(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }

    pub fn from_lumaa_image(image: &GrayAlphaImage) -> Self {
        let mut out_image = BoolImage::new(image.width(), image.height());
        for x in 0..image.width() {
            for y in 0..image.height() {
                let pixel = image.get_pixel(x, y);
                if pixel.0[1] <= TRANSPARENT_THRESHOLD {
                    out_image.set(x, y, false).unwrap();
                } else {
                    out_image.set(x, y, true).unwrap();
                }
            }
        }
        out_image
    }

    pub fn to_lumaa_image(&self, true_color: LumaA<u8>, false_color: LumaA<u8>) -> GrayAlphaImage {
        let mut out_image = GrayAlphaImage::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = match self.get(x, y).unwrap() {
                    true => true_color,
                    false => false_color,
                };
                out_image.put_pixel(x, y, pixel);
            }
        }
        out_image
    }
}
