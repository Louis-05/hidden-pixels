use image::LumaA;

pub const FILTER_FILE: &str = "filter.png";
pub const INPUT_FOLDER: &str = "inputs";
pub const OUTPUT_FOLDER: &str = "outputs";
pub const DEMO_FOLDER: &str = "demos";

pub const BLACK_COLOR: LumaA<u8> = LumaA([0u8, 255u8]);
pub const WHITE_COLOR: LumaA<u8> = LumaA([255u8, 255u8]);
pub const TRANSPARENT_COLOR: LumaA<u8> = LumaA([0u8, 0u8]);
pub const TRANSPARENT_THRESHOLD: u8 = 64;
