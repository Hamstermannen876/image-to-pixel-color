use image::{self, DynamicImage, GenericImageView, Rgba};
use std::collections::HashMap;

/// Returns how many edges a pixel has
/// An edge is when the pixel next to it is:
/// 1. Outside the image
/// 2. has the rgba value (0, 0, 0, 0) = fully transparent png pixel
pub fn edges_of_pixel(x: u32, y: u32, img: &DynamicImage) -> u32 {
    let mut edges: u32 = 0;

    let conditions = [
        x == 0 || x == img.width() - 1,
        y == 0 || y == img.height() - 1,
        x != img.width() - 1 && img.get_pixel(x + 1, y) == Rgba([0, 0, 0, 0]),
        x != 0 && img.get_pixel(x - 1, y) == Rgba([0, 0, 0, 0]),
        y != img.height() - 1 && img.get_pixel(x, y + 1) == Rgba([0, 0, 0, 0]),
        y != 0 && img.get_pixel(x, y - 1) == Rgba([0, 0, 0, 0]),
    ];

    for condition in conditions {
        if condition {
            edges += 1;
        }
    }

    return edges;
}

pub fn get_pixel_info(img: &DynamicImage) -> HashMap<Rgba<u8>, (u32, u32, u32)> {
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
