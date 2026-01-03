use anyhow::Result;
use image::{ImageReader, RgbaImage};
use std::path::Path;

pub fn read_img(path: impl AsRef<Path>) -> Result<RgbaImage> {
    let img = ImageReader::open(path)?.decode()?.to_rgba8();
    Ok(img)
}
