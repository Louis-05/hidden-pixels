use std::path::Path;

use image::GrayAlphaImage;

use crate::{
    CliArgs,
    bool_image::BoolImage,
    consts::{BLACK_COLOR, DEMO_FOLDER, FILTER_FILE, INPUT_FOLDER, OUTPUT_FOLDER, WHITE_COLOR},
    utils::{path::gen_name, read::read_lumaa_img},
};

use anyhow::Result;
pub fn gen_images(args: CliArgs) -> Result<()> {
    let main_folder = args.folder;

    let filter_img = read_lumaa_img(main_folder.join(Path::new(FILTER_FILE)))?;
    let filter_img = BoolImage::from_lumaa_image(&filter_img);

    for file in main_folder.join(INPUT_FOLDER).read_dir()? {
        let input_file = file?;
        let input_img: GrayAlphaImage = read_lumaa_img(&input_file.path())?;
        let input_img = BoolImage::from_lumaa_image(&input_img);

        let filtered_img = filter_img.xor(&input_img)?;

        let demo_img = filtered_img.or(&filter_img)?;

        let demo_img_path = gen_name(
            &main_folder.join(DEMO_FOLDER),
            input_file.file_name().to_string_lossy(),
            "demo_",
        );

        let demo_img = demo_img.to_lumaa_image(BLACK_COLOR, WHITE_COLOR);
        demo_img.save_with_format(demo_img_path, image::ImageFormat::Png)?;

        let out_img = filtered_img.to_lumaa_image(BLACK_COLOR, WHITE_COLOR);
        let out_img_path = gen_name(
            &main_folder.join(OUTPUT_FOLDER),
            input_file.file_name().to_string_lossy(),
            "out_",
        );
        out_img.save_with_format(out_img_path, image::ImageFormat::Png)?;
    }

    Ok(())
}
