
//! Helper functions for hue and saturation selection

use image::{
    GenericImage,
    Rgba,
    RgbaImage,
};

/// Selects color at coordinate in a rectangle show hue and saturation
pub fn select(rect: [u32, ..4], x: u32, y: u32) -> Rgba<u8> {
    let inv_rect_width = 1.0_f64 / rect[2] as f64;
    let half_inv_rect_height = 0.5_f64 / rect[3] as f64;
    let inv60 = 1.0_f64 / 60.0_f64;
    
    let fx = (x - rect[0]) as f64 * inv_rect_width;
    let hue = fx * 360.0_f64;
    // Saturation is 0.5 * fy, so by halving the height we save operations.
    let saturation = (y - rect[1]) as f64 * half_inv_rect_height;
    let hi = (hue * inv60) as int;
    let f = hue * inv60 - hi as f64;
    let hi = hi % 6;
    let v = 255; // use white as color without saturation.
    let p = (255.0 * (1.0 - saturation)) as u8;
    let q = (255.0 * (1.0 - f * saturation)) as u8;
    let t = (255.0 * (1.0 - (1.0 - f) * saturation)) as u8;
    Rgba([
        match hi {
            0 | 1 => p,
            2 => t,
            3 | 4 => v,
            _ => q
        },
        match hi {
            0 => t,
            1 | 2 => v,
            3 => q,
            _ => p
        },
        match hi {
            1 => q,
            2 | 3 => p,
            4 => t,
            _ => v
        },
        255
    ])
}

/// Sets colors to hue/saturation image.
/// White is used as color without saturation.
/// This is at the top of the image,
/// while saturated colors are at the bottom.
pub fn fill_image(
    image: &mut RgbaImage,
    rect: [u32, ..4]
) {
    let inv_rect_width = 1.0_f64 / rect[2] as f64;
    let half_inv_rect_height = 0.5_f64 / rect[3] as f64;
    let inv60 = 1.0_f64 / 60.0_f64;
    for x in range(rect[0], rect[0] + rect[2]) {
        for y in range(rect[1], rect[1] + rect[3]) {
            let fx = (x - rect[0]) as f64 * inv_rect_width;
            let hue = fx * 360.0_f64;
            // Saturation is 0.5 * fy, so by halving the height we save operations.
            let saturation = (y - rect[1]) as f64 * half_inv_rect_height;
            let hi = (hue * inv60) as int;
            let f = hue * inv60 - hi as f64;
            let hi = hi % 6;
            let v = 255; // use white as color without saturation.
            let p = (255.0 * (1.0 - saturation)) as u8;
            let q = (255.0 * (1.0 - f * saturation)) as u8;
            let t = (255.0 * (1.0 - (1.0 - f) * saturation)) as u8;
            *image.get_pixel_mut(x, y) = Rgba([
                match hi {
                    0 | 1 => p,
                    2 => t,
                    3 | 4 => v,
                    _ => q
                },
                match hi {
                    0 => t,
                    1 | 2 => v,
                    3 => q,
                    _ => p
                },
                match hi {
                    1 => q,
                    2 | 3 => p,
                    4 => t,
                    _ => v
                },
                255
            ]);
        }
    }
}
