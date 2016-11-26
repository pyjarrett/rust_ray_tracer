extern crate image;
use std::fs::File;
use std::path::Path;

mod math;


fn main() {
    let mut image = image::ImageBuffer::new(400, 200);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if x % 20 == 0 || y % 20 == 0 {
            *pixel = image::Rgb([255, 255, 255]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    let ref mut fout = File::create(&Path::new("sample.png")).unwrap();
    let _ = image::ImageRgb8(image).save(fout, image::PNG);
}
