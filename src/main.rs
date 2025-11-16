use csv;
use image::{self, DynamicImage, GenericImageView, Pixel, Rgba};
use std::{collections::HashMap, env, path};

fn rgb_to_hex(rgba: Rgba<u8>) -> String {
    println!("{:?}", rgba);

    let mut hex = String::from("#");

    for value in rgba.channels() {
        if value < &16u8 {
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

fn down_scale_image(image: DynamicImage, new_resolution: String) {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => panic!("too few command line arguments, a image file path is required"),
        2 => {},
        _ => {
            for i in (2..args.len()).step_by(2) {
                match args[i].as_str() {
                    "-s" => {},
                    "--size" => {},
                    "-a" => {},
                    "--accuracy" => {},
                    _ => panic!("Invalid command line argument"),
                }
            }
        }
    }

    let image_path = path::PathBuf::from(args[1].as_str());

    let colors = get_pixel_info(image_path);

    let mut writer = csv::Writer::from_path("color_data.csv").expect("failed to create csv file");

    let header = &["color", "count", "3D-requirement"];
    writer.write_record(header).unwrap();

    let mut total_count: u32 = 0;
    let mut total_slices: u32 = 0;

    for (color, (count, slices)) in colors {
        let hex = rgb_to_hex(color);

        if hex == "#00000000" {
            continue;
        }

        total_count += count;
        total_slices += slices;

        let str_count = format!("{count}");
        let str_slices = format!("{slices}");

        let row = &[hex, str_count, str_slices];

        writer.write_record(row).unwrap();
    }

    writer.write_record(&["total", format!("{total_count}").as_str(), format!("{total_slices}").as_str()]).unwrap();
}
