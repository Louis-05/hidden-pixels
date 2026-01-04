use crate::gen_filter::gen_filter;
use crate::gen_images::gen_images;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod bool_image;
mod consts;
mod gen_filter;
mod gen_images;
mod img_processing;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    let args = CliArgs::parse();

    match args.command {
        Commands::GenFilter(_) => gen_filter(args),
        Commands::GenImgs => gen_images(args),
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    folder: PathBuf,

    #[arg(long)]
    height: u32,

    #[arg(long)]
    width: u32,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    GenFilter(FilterType),
    GenImgs,
}

#[derive(Subcommand)]
enum FilterType {
    Random,
    Grid,
}
