use image::{self, GenericImageView, Rgba};
use std::{collections::HashMap, env, path};
use csv;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid command line argument count, only input image file");
    }

    let image_path = path::PathBuf::from(args[1].as_str());

    let img = image::ImageReader::open(image_path).unwrap().decode().unwrap();

    let mut colors: HashMap<Rgba<u8>, u32> = HashMap::new();

    for (_, _, pixel) in img.pixels() {
        if colors.contains_key(&pixel) {
            *colors.get_mut(&pixel).unwrap() += 1;
        } else {
            colors.insert(pixel, 1);
        }
    }

    let mut writer = csv::Writer::from_path("color_data.csv").expect("failed to create csv file");
    writer.write_record(&["color", "count"]).unwrap();

    for (color, count) in colors {
        writer.write_record(&[format!("{:?}", color), format!("{count}")]).unwrap();
    }

}
