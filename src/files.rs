use crate::utils::*;
use csv;
use image::{self, DynamicImage, GenericImageView, Pixel, Rgba, imageops::FilterType::Nearest};
use std::io::ErrorKind;
use std::{
    collections::HashMap,
    fs,
    path::{self},
    u32,
};

const COLOR_IMAGES_PATH: &str = "./recolored_images";
const DOWNSCALED_IMAGES_PATH: &str = "./downscaled_images";

pub fn down_scale_image(img: &DynamicImage, new_resolution: &str, file_name: &str) -> DynamicImage {
    let res: Vec<&str> = new_resolution.split("x").collect();
    if res.len() != 2 {
        panic!("invalid downscale resolution, please use ex. 16x16")
    }
    let width = res[0]
        .parse::<u32>()
        .expect("downscale resolution has to consist of positive integers, ex. 16x16");
    let height = res[1]
        .parse::<u32>()
        .expect("downscale resolution has to consist of positive integers, ex. 16x16");

    let resized_image = img.resize(width, height, Nearest);

    if let Err(e) = fs::create_dir(DOWNSCALED_IMAGES_PATH) {
        match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::PermissionDenied => {
                panic!("did not have permission to create {DOWNSCALED_IMAGES_PATH}");
            }
            _ => {
                panic!("Unexpected error {:?}", e);
            }
        }
    };

    let new_file_path = path::PathBuf::from(format!(
        "{DOWNSCALED_IMAGES_PATH}/{new_resolution}-{file_name}"
    ));
    match resized_image.save(new_file_path) {
        Ok(_) => {}
        Err(msg) => println!("{msg}\nCould not create the new resolution image "),
    }

    return resized_image;
}

pub fn make_image(
    img: &DynamicImage,
    color_lookup: &HashMap<Rgba<u8>, Rgba<u8>>,
    amount: usize,
    file_name: &str,
) {
    let mut new_image =
        DynamicImage::new(img.width(), img.height(), image::ColorType::Rgba8).to_rgba8();
    for (x, y, color) in img.pixels() {
        if let [_, _, _, alpha] = color.channels() {
            // if fully transparent
            if *alpha == 0 {
                continue;
            }
        }
        new_image.put_pixel(x, y, *color_lookup.get(&color).unwrap());
    }

    if let Err(e) = fs::create_dir(COLOR_IMAGES_PATH) {
        match e.kind() {
            ErrorKind::AlreadyExists => (),
            ErrorKind::PermissionDenied => {
                panic!("did not have permission to create {COLOR_IMAGES_PATH}");
            }
            _ => {
                panic!("Unexpected error {:?}", e);
            }
        }
    };

    let path = path::PathBuf::from(format!("{COLOR_IMAGES_PATH}/{amount}-color-{file_name}"));
    if let Err(msg) = new_image.save(path) {
        println!("Could not make the reduced color image: {}", msg);
    };
}

pub fn make_csv_file(colors: HashMap<Rgba<u8>, (u32, u32, u32)>) {
    let mut writer = csv::Writer::from_path("color_data.csv").expect("failed to create csv file");

    const HEADER: [&str; 4] = ["color", "count", "edges", "3D-requirement"];
    writer.write_record(HEADER).unwrap();

    let mut total_count = 0;
    let mut total_slices = 0;
    let mut total_edges = 0;

    for (color, (count, edges, slices)) in colors {
        let hex = rgba_to_hex(&color);

        if hex == "#00000000" {
            continue;
        }

        total_count += count;
        total_slices += slices;
        total_edges += edges;

        let row = [
            hex,
            count.to_string(),
            edges.to_string(),
            slices.to_string(),
        ];

        writer.write_record(row).unwrap();
    }

    writer
        .write_record([
            "total".to_string(),
            total_count.to_string(),
            total_edges.to_string(),
            total_slices.to_string(),
        ])
        .unwrap();
}
