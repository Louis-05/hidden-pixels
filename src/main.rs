use std::path::{Path, PathBuf};

use clap::Parser;
use image::{Rgba, RgbaImage};

use crate::{
    consts::*,
    img_processing::{
        apply_colors::apply_colors, apply_contrast::apply_contrast, apply_filter::apply_filter,
        gen_demo::gen_demo, gen_filter::gen_filter_full_rng,
    },
    utils::{path::gen_name, read::read_img},
};
mod consts;
mod img_processing;
mod utils;

fn main() {
    println!("Hello, world!");
    let args = CliArgs::parse();
    let main_folder = args.folder;
    let filter_img = gen_filter_full_rng(args.width, args.height, 0.5f64);

    let rgba_filter_img = apply_colors(
        &filter_img,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        Rgba([0u8, 0u8, 0u8, 0u8]),
    )
    .unwrap();

    rgba_filter_img
        .save_with_format(
            main_folder.join(Path::new(FILTER_FILE)),
            image::ImageFormat::Png,
        )
        .unwrap();
    println!(
        "Saved filter img at {}",
        main_folder.join(Path::new(FILTER_FILE)).to_string_lossy()
    );

    for file in main_folder.join(INPUT_FOLDER).read_dir().unwrap() {
        let input_file = file.unwrap();
        let img: RgbaImage = read_img(&input_file.path()).unwrap();
        let img = apply_contrast(&img);
        let filtered_img = apply_filter(&img, &filter_img).unwrap();
        let demo_img = gen_demo(
            &filtered_img,
            &filter_img,
            Rgba([0u8, 0u8, 0u8, 255u8]),
            Rgba([255u8, 255u8, 255u8, 255u8]),
        )
        .unwrap();

        let demo_img_path = gen_name(
            &main_folder.join(DEMO_FOLDER),
            input_file.file_name().to_string_lossy(),
            "demo_",
        );
        demo_img
            .save_with_format(demo_img_path, image::ImageFormat::Png)
            .unwrap();

        let out_img = apply_colors(
            &filtered_img,
            Rgba([0u8, 0u8, 0u8, 255u8]),
            Rgba([255u8, 255u8, 255u8, 255u8]),
        )
        .unwrap();
let out_img_path = gen_name(
            &main_folder.join(OUTPUT_FOLDER),
            input_file.file_name().to_string_lossy(),
            "out_",
        );
         out_img
            .save_with_format(out_img_path, image::ImageFormat::Png)
            .unwrap();


    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    folder: PathBuf,

    #[arg(long)]
    height: u32,

    #[arg(long)]
    width: u32,
}
