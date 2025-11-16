use csv;
use image::{self, DynamicImage, GenericImageView, Pixel, Rgba};
use std::{collections::HashMap, env, path};

fn rgb_to_hex(rgba: Rgba<u8>) -> String {
    println!("{:?}", rgba);

    let mut hex = String::from("#");

    for value in rgba.channels() {
        if value < &10u8 {
            hex.push('0');
        }

        hex.push_str(format!("{:x}", value).as_str());
    }

    return hex;
}

fn edges_of_pixel(x: u32, y: u32, img: &DynamicImage) -> u32 {
    let mut edges: u32 = 0;

    if x == 0 || x == img.width() - 1 {
        edges += 1;
    }

    if y == 0 || y == img.height() - 1 {
        edges += 1;
    }

    if x != img.width() - 1 && img.get_pixel(x + 1, y) == Rgba([0, 0, 0, 0]) {
        edges += 1;
    }

    if x != 0 && img.get_pixel(x - 1, y) == Rgba([0, 0, 0, 0]) {
        edges += 1;
    }

    if y != img.height() - 1 && img.get_pixel(x, y + 1) == Rgba([0, 0, 0, 0]) {
        edges += 1;
    }

    if y != 0 && img.get_pixel(x, y - 1) == Rgba([0, 0, 0, 0]) {
        edges += 1;
    }

    return edges;
}

fn get_pixel_info(image_path: path::PathBuf) -> HashMap<Rgba<u8>, (u32, u32)> {
    let img = image::ImageReader::open(image_path)
        .unwrap()
        .decode()
        .unwrap();

    let mut colors: HashMap<Rgba<u8>, (u32, u32)> = HashMap::new();
    let cube_requirement: u32 = 8;

    for (x, y, pixel_color) in img.pixels() {
        let edges = edges_of_pixel(x, y, &img);
        let rectangle = edges + cube_requirement;

        if colors.contains_key(&pixel_color) {
            let (count, rectancles) = colors.get_mut(&pixel_color).unwrap();
            *count += 1;
            *rectancles += rectangle;
        } else {
            colors.insert(pixel_color, (1, rectangle));
        }
    }

    return colors;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid command line argument count, only input image file");
    }

    let image_path = path::PathBuf::from(args[1].as_str());

    let colors = get_pixel_info(image_path);

    let mut writer = csv::Writer::from_path("color_data.csv").expect("failed to create csv file");

    let header = &["color", "count", "3D-requirement"];
    writer.write_record(header).unwrap();

    for (color, (count, edges)) in colors {
        let hex = rgb_to_hex(color);

        if hex == "#00000000" {
            continue;
        }

        let str_count = format!("{count}");
        let str_edges = format!("{edges}");

        let row = &[hex, str_count, str_edges];

        writer.write_record(row).unwrap();
    }
}
