use core::panic;
use csv;
use image::{self, DynamicImage, GenericImageView, Pixel, Rgba, imageops::FilterType::Nearest};
use std::{collections::HashMap, env, path, u32};

fn rgba_to_hex(rgba: &Rgba<u8>) -> String {
    let mut hex = String::from("#");

    for value in rgba.channels() {
        if value < &16u8 {
            hex.push('0');
        }

        hex.push_str(format!("{:x}", value).as_str());
    }

    return hex;
}

#[allow(unused)]
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
    // 6 for the cube and 2 for the back and front flaps.
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
        .expect("downscale resolution has to consist of positive integers, ex. 16x16");
    let height = res[1]
        .parse::<u32>()
        .expect("downscale resolution has to consist of positive integers, ex. 16x16");

    let resized_image = img.resize(width, height, Nearest);

    let new_file_path =
        path::PathBuf::from(format!("./downscaled_images/{}-image.png", new_resolution));
    match resized_image.save(new_file_path) {
        Ok(_) => {}
        Err(msg) => println!("{msg}, BUUUUU"),
    }

    return resized_image;
}

/// Uses the k-mean clustering algorithm.
/// https://en.wikipedia.org/wiki/Cluster_analysis
fn reduce_colors(
    colors: &HashMap<Rgba<u8>, (u32, u32, u32)>,
    amount: usize,
    img: &DynamicImage,
) -> HashMap<Rgba<u8>, (u32, u32, u32)> {
    let length = colors.len();
    if amount >= length {
        return HashMap::new();
    }

    let mut clusters = HashMap::<Rgba<u8>, Vec<Rgba<u8>>>::new();

    let mut most_occurances = 0u32;
    let mut most_of_color = Rgba([0, 0, 0, 0]);
    for (color, (count, _, _)) in colors {
        if most_occurances < *count {
            most_occurances = *count;
            most_of_color = *color;
        }
    }

    clusters.insert(most_of_color, Vec::new());

    // Making cluster main colors.
    for _ in 0..amount - 1 {
        let mut greatest_min_distance = 0u32;
        let mut new_color = Rgba([0, 0, 0, 0]);

        for color in colors.keys() {
            if clusters.contains_key(color) {
                continue;
            };

            let mut min_distance = u32::MAX;
            for main in clusters.keys() {
                let diff = compare_colors(color, main);
                if min_distance > diff {
                    min_distance = diff
                }
            }

            if min_distance >= greatest_min_distance {
                greatest_min_distance = min_distance;
                new_color = *color;
            }
        }

        clusters.insert(new_color, Vec::new());
    }

    // Filling cluster vecs.
    for color in colors.keys() {
        let mut smallest_difference = u32::MAX;
        let mut cluster = Rgba::<u8>([0, 0, 0, 0]);

        for main in clusters.keys() {
            let difference = compare_colors(color, main);
            if smallest_difference > difference {
                cluster = *main;
                smallest_difference = difference;
            }
        }

        if let Some(cluster_vec) = clusters.get_mut(&cluster) {
            cluster_vec.push(*color);
        }
    }

    // A hashmap with the old color as key and the new reduced color as value
    let mut color_lookup = HashMap::<Rgba<u8>, Rgba<u8>>::new();

    for cluster in clusters.values() {
        let mut total_color = [0u32, 0u32, 0u32];
        let mut total = 0;
        for color in cluster {
            let [red, green, blue, _] = color.channels() else {
                panic!("could not parse color to 4 u8s");
            };

            if let Some((count, _, _)) = colors.get(color) {
                total_color[0] += *red as u32 * *count;
                total_color[1] += *green as u32 * *count;
                total_color[2] += *blue as u32 * *count;

                total += *count;
            };
        }

        total_color.iter_mut().for_each(|v| {
            *v /= total;
        });

        for color in cluster {
            color_lookup.insert(
                *color,
                Rgba([
                    total_color[0] as u8,
                    total_color[1] as u8,
                    total_color[2] as u8,
                    255u8,
                ]),
            );
        }
    }

    make_image(img, &color_lookup, amount);

    let mut return_map = HashMap::<Rgba<u8>, (u32, u32, u32)>::new();

    for (old_color, new_color) in color_lookup {
        if let Some((new_count, new_edges, new_req)) = return_map.get_mut(&new_color) {
            let (count, edges, req) = *colors.get(&old_color).unwrap();
            *new_count += count;
            *new_edges += edges;
            *new_req += req;
        } else {
            return_map.insert(new_color, *colors.get(&old_color).unwrap());
        };
    }

    return return_map;
}

fn make_image(img: &DynamicImage, color_lookup: &HashMap<Rgba<u8>, Rgba<u8>>, amount: usize) {
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

    let path = path::PathBuf::from(format!("./recolored_images/{}-color-image.png", amount));
    if let Err(msg) = new_image.save(path) {
        println!("Could not make the reduced color image: {}", msg);
    };
}

fn compare_colors(color1: &Rgba<u8>, color2: &Rgba<u8>) -> u32 {
    let [red1, green1, blue1, _] = color1.channels() else {
        panic!("rgba did not have 4 u8 values")
    };

    let [red2, green2, blue2, _] = color2.channels() else {
        panic!("rgba did not have 4 u8 values")
    };

    let dr = *red1 as i32 - *red2 as i32;
    let dg = *green1 as i32 - *green2 as i32;
    let db = *blue1 as i32 - *blue2 as i32;

    // can range from 0 to 195075
    return (dr * dr + dg * dg + db * db) as u32;
}

fn make_csv_file(colors: HashMap<Rgba<u8>, (u32, u32, u32)>) {
    let mut writer = csv::Writer::from_path("color_data.csv").expect("failed to create csv file");

    let header = &["color", "count", "edges", "3D-requirement"];
    writer.write_record(header).unwrap();

    let mut total_count: u32 = 0;
    let mut total_slices: u32 = 0;
    let mut total_edges: u32 = 0;

    for (color, (count, edges, slices)) in colors {
        let hex = rgba_to_hex(&color);

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut max_colors = 0;

    if args.len() < 2 {
        panic!("too few command line arguments, a image file path is required")
    }

    let image_path = path::PathBuf::from(args[1].as_str());
    let img_reader = match image::ImageReader::open(&image_path) {
        Err(_) => panic!("could not find an image at the path {:?}", image_path),
        Ok(reader) => reader,
    };

    if let Ok(mut img) = img_reader.decode() {
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

        let mut colors = get_pixel_info(&img);

        if max_colors != 0 {
            colors = reduce_colors(&colors, max_colors, &img);
        }

        make_csv_file(colors);
    } else {
        panic!("could not decode file at {:?}", image_path)
    }
}
