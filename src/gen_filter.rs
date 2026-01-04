use std::path::Path;
use crate::Commands::GenFilter;
use crate::img_processing::filters::filter_full_rng;
use crate::{
    CliArgs,
    consts::{BLACK_COLOR, FILTER_FILE, TRANSPARENT_COLOR},
    img_processing::filters::filter_grid_rng,
};

use anyhow::Result;

pub fn gen_filter(args: CliArgs) -> Result<()> {
    let main_folder = args.folder;

    let GenFilter(ftype) = args.command else {
        panic!()
    };



    let filter_img = filter_full_rng(args.width, args.height, 0.5f64);
    let out_img = filter_img.to_lumaa_image(BLACK_COLOR, TRANSPARENT_COLOR);

    out_img.save_with_format(
        main_folder.join(Path::new(FILTER_FILE)),
        image::ImageFormat::Png,
    )?;
    println!(
        "Saved filter img at {}",
        main_folder.join(Path::new(FILTER_FILE)).to_string_lossy()
    );
    Ok(())
}
