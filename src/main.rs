use csv;
use image::{self, DynamicImage, GenericImageView, Pixel, Rgba, imageops::FilterType::Nearest};
use std::{collections::HashMap, env, path};

fn rgba_to_hex(rgba: Rgba<u8>) -> String {
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

fn hex_to_rbga(hex: &str) -> Rgba<u8> {
    let red = hex[0..2]
        .parse::<u8>()
        .expect("could not parse hex to rbga (invalid red)");

    let green = hex[2..4]
        .parse::<u8>()
        .expect("could not parse hex to rbga (invalid green)");

    let blue = hex[4..6]
        .parse::<u8>()
        .expect("could not parse hex to rbga (invalid blue)");

    let alpha = hex[6..8]
        .parse::<u8>()
        .expect("could not parse hex to rbga (invalid alpha)");

    return Rgba([red, green, blue, alpha]);
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

fn get_pixel_info(img: &DynamicImage) -> HashMap<Rgba<u8>, (u32, u32, u32)> {
    let mut colors: HashMap<Rgba<u8>, (u32, u32, u32)> = HashMap::new();
    // 6 for the cube and 2 for the back and front flaps
    let cube_requirement: u32 = 8;

    for (x, y, pixel_color) in img.pixels() {
        let edges = edges_of_pixel(x, y, &img);
        let rectangle = edges + cube_requirement;

        if colors.contains_key(&pixel_color) {
            let (count, edge, rectancles) = colors.get_mut(&pixel_color).unwrap();
            *count += 1;
            *edge += edges;
            *rectancles += rectangle;
        } else {
            colors.insert(pixel_color, (1, edges, rectangle));
        }
    }

    return colors;
}

fn down_scale_image(img: &DynamicImage, new_resolution: &str) -> DynamicImage {
    let res: Vec<&str> = new_resolution.split("x").collect();
    if res.len() != 2 {
        panic!("invalid downscale resolution, please use ex. 16x16")
    }

    let width = res[0]
        .parse::<u32>()
        .expect("downscale resolution has to consist of integers");
    let height = res[1]
        .parse::<u32>()
        .expect("downscale resolution has to consist of integers");

    let resized_image = img.resize(width, height, Nearest);

    let new_file_path =
        path::PathBuf::from(format!("./downscaled_images/{}-image.png", new_resolution));
    match resized_image.save(new_file_path) {
        Ok(_) => {}
        Err(msg) => println!("{msg}, BUUUUU"),
    }

    return resized_image;
}

fn reduce_colors(map: &HashMap<Rgba<u8>, (u32, u32, u32)>, amount: u32) {}

fn compare_colors(color1: Rgba<u8>, color2: Rgba<u8>) -> u32 {
    let [red1, green1, blue1] = color1.channels() else {
        panic!("rgba did not have 4 u8 values")
    };

    let [red2, green2, blue2] = color2.channels() else {
        panic!("rgba did not have 4 u8 values")
    };

    // can range from 0 to 195075
    let difference = (red1 - red2) as u32 * (red1 - red2) as u32
        + (green1 - green2) as u32 * (green1 - green2) as u32
        + (blue1 - blue2) as u32 * (blue1 - blue2) as u32;

    return difference;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut max_colors = 0;

    if args.len() < 2 {
        panic!("too few command line arguments, a image file path is required")
    }

    let image_path = path::PathBuf::from(args[1].as_str());
    let mut img = image::ImageReader::open(image_path)
        .unwrap()
        .decode()
        .unwrap();

    for i in (2..args.len()).step_by(2) {
        match args[i].as_str() {
            "-r" | "--resolution" => img = down_scale_image(&img, &args[i + 1]),
            "-m" | "--max" => {
                max_colors = args[i + 1]
                    .parse()
                    .expect("invalid input after -m or --max, has to be a positive int");
            }
            rest => panic!("Invalid command line argument, could not find '{rest}'"),
        }
    }

    let colors = get_pixel_info(&img);

    if max_colors != 0 {
        reduce_colors(&colors, max_colors);
    }

    let mut writer = csv::Writer::from_path("color_data.csv").expect("failed to create csv file");

    let header = &["color", "count", "edges", "3D-requirement"];
    writer.write_record(header).unwrap();

    let mut total_count: u32 = 0;
    let mut total_slices: u32 = 0;
    let mut total_edges: u32 = 0;

    for (color, (count, edges, slices)) in colors {
        let hex = rgba_to_hex(color);

        if hex == "#00000000" {
            continue;
        }

        total_count += count;
        total_slices += slices;
        total_edges += edges;

        let str_count = format!("{count}");
        let str_slices = format!("{slices}");
        let str_edges = format!("{edges}");

        let row = &[hex, str_count, str_edges, str_slices];

        writer.write_record(row).unwrap();
    }

    writer
        .write_record(&[
            "total",
            format!("{total_count}").as_str(),
            format!("{total_edges}").as_str(),
            format!("{total_slices}").as_str(),
        ])
        .unwrap();
}
