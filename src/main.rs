use image::ImageBuffer;

fn main() {

    let width = 800;
    let height = 800;
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r =( 0.3 * x as f32) as u8;
        let g = 0;
        let b =( 0.3 * y as f32) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("image.png").unwrap();
    println!("Image has been saved");
}
