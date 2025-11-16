use image::{self, GenericImageView, Pixel, Rgba};
use std::{collections::HashMap, env, path};
use csv;

fn rgb_to_hex (rgba: Rgba<u8>) -> String {
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


fn get_pixel_colors (image_path: path::PathBuf) -> HashMap<Rgba<u8>, u32> {
    let img = image::ImageReader::open(image_path).unwrap().decode().unwrap();

    let mut colors: HashMap<Rgba<u8>, u32> = HashMap::new();

    for (_, _, pixel_color) in img.pixels() {
        if colors.contains_key(&pixel_color) {
            *colors.get_mut(&pixel_color).unwrap() += 1;
        } else {
            colors.insert(pixel_color, 1);
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

    let colors = get_pixel_colors(image_path); 

    let mut writer = csv::Writer::from_path("color_data.csv").expect("failed to create csv file");

    let header = &["color", "count"];
    writer.write_record(header).unwrap();


    for (color, count) in colors {
        let hex = rgb_to_hex(color);
        let str_count = format!("{count}");

        let row = &[hex, str_count];

        writer.write_record(row).unwrap();
    }

}
