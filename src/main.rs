#![allow(dead_code)]
mod conv;


use image::{DynamicImage, ImageBuffer, ImageReader};

use crate::conv::correlate;

fn main() {
    //generate_gradient_image();
    load_images();
}

fn load_images() {
    let img = ImageReader::open("zebra.jpg").unwrap().decode().unwrap();
    let mut img = img.to_rgb32f();

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        let r = (pixel[0] - 0.2).clamp(0., 1.);
        let g = (pixel[1] - 0.2).clamp(0., 1.);
        let b = (pixel[2] - 0.2).clamp(0., 1.);
        *pixel = image::Rgb([r, g, b]);
    }

    let kernel: Vec<Vec<f32>> = [
        [-1., 0. , 1.].to_vec(),
        [-2., 0. , 2.].to_vec(),
        [-1., 0. , 1.].to_vec(),
    ].to_vec();

    let mut output_img = ImageBuffer::new(img.width(), img.height());
    correlate(&img, &kernel, &mut output_img);

    let output_img = DynamicImage::ImageRgb32F(output_img).to_rgb8();
    output_img.save("zebra_sobel.jpg").unwrap();
}

fn generate_gradient_image() {
    let width = 800;
    let height = 800;
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let g = 0;
        let b = (0.3 * y as f32) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("image.png").unwrap();
    println!("Image has been saved");
}
