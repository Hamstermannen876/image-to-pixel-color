use image::{self, GenericImageView, Rgba};
use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid command line argument count, only input image file");
    }

    let image_path = args[1].as_str();

    let img = image::ImageReader::open(image_path).unwrap().decode().unwrap();

    let mut colors: HashMap<Rgba<u8>, u32> = HashMap::new();

    for (_, _, pixel) in img.pixels() {
        if colors.contains_key(&pixel) {
            *colors.get_mut(&pixel).unwrap() += 1;
        } else {
            colors.insert(pixel, 1);
        }
    }

    for value in colors {
        println!("{:?}", value);
    }
}
