use core::panic;
use image::{self};
use std::{env, path};

mod files;
mod pixels;
mod utils;

use files::*;
use pixels::*;
use utils::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut max_colors = 0;

    if args.len() < 2 {
        panic!("too few command line arguments, a image file path is required")
    }

    let image_path = path::PathBuf::from(args[1].as_str());
    let image_name = match image_path.file_name() {
        Some(name) => name.to_str().unwrap(),
        None => panic!("could not find a valid image file name"),
    };

    let img_reader = match image::ImageReader::open(&image_path) {
        Err(_) => panic!("could not find an image at the path {:?}", image_path),
        Ok(reader) => reader,
    };

    if let Ok(mut img) = img_reader.decode() {
        for i in (2..args.len()).step_by(2) {
            match args[i].as_str() {
                "-r" | "--resolution" => img = down_scale_image(&img, &args[i + 1], image_name),
                "-m" | "--max" => {
                    max_colors = args[i + 1]
                        .parse()
                        .expect("invalid input after -m or --max, has to be a positive int");
                }
                rest => panic!("Invalid command line argument, could not find '{rest}'"),
            }
        }

        let mut colors = get_pixel_info(&img);

        if max_colors != 0 {
            colors = reduce_colors(&colors, max_colors, &img, image_name);
        }

        make_csv_file(colors);
    } else {
        panic!("could not decode file at {:?}", image_path)
    }
}
