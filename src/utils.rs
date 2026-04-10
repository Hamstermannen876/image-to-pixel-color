use crate::files::*;

use image::{DynamicImage, Pixel, Rgba};
use std::{collections::HashMap, u32};

/// Uses the k-mean clustering algorithm.
/// https://en.wikipedia.org/wiki/Cluster_analysis
pub fn reduce_colors(
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

pub fn compare_colors(color1: &Rgba<u8>, color2: &Rgba<u8>) -> u32 {
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

#[allow(unused)]
pub fn rgba_to_hex(rgba: &Rgba<u8>) -> String {
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
pub fn hex_to_rbga(hex: &str) -> Rgba<u8> {
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
