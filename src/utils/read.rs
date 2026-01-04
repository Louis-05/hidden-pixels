use anyhow::Result;
use image::{GrayAlphaImage, ImageReader, RgbaImage};
use std::path::Path;

pub fn read_lumaa_img(path: impl AsRef<Path>) -> Result<GrayAlphaImage> {
    let img = ImageReader::open(path)?.decode()?.to_luma_alpha8();
    Ok(img)
}
