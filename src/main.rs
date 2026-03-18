#![allow(dead_code)]
mod conv;


use image::{ImageBuffer, ImageReader};

fn main() {
    //generate_gradient_image();
    load_images();
}

fn load_images() {
    let img = ImageReader::open("zebra.jpg").unwrap().decode().unwrap();
    let mut img = img.to_rgb8();

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        let r = pixel[0].saturating_sub(50);
        let g = pixel[1].saturating_sub(50);
        let b = pixel[2].saturating_sub(50);
        *pixel = image::Rgb([r, g, b]);
    }

    img.save("zebra_darker.jpg").unwrap();
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
