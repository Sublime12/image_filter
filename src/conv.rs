#![allow(dead_code)]

use image::Rgb32FImage;

pub fn correlate(img: &Rgb32FImage, kernel: &[Vec<f32>], out_img: &mut Rgb32FImage) {
    for (x, y, pixel) in out_img.enumerate_pixels_mut() {
        let mut r = 0f32;
        let mut g = 0f32;
        let mut b = 0f32;

        let center_x = kernel.len() / 2;
        let center_y = kernel[0].len() / 2;

        for (i, k_row) in kernel.iter().enumerate() {
            for (j, kernel_pixel) in k_row.iter().enumerate() {
                let offset_i = i as i32 - center_x as i32;
                let offset_j = j as i32 - center_y as i32;

                let pixel = img.get_pixel_checked(
                    (x as i32 + offset_i) as u32,
                    ((y as i32) + offset_j) as u32,
                );
                if let Some(pixel) = pixel {
                    r += pixel[0] * kernel_pixel;
                    g += pixel[1] * kernel_pixel;
                    b += pixel[2] * kernel_pixel;
                }
            }
        }

        let value = image::Rgb([r, g, b]);
        *pixel = value;
    }
}
