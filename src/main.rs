extern crate image;
use std::fs::File;
use std::f32;
use std::path::Path;

mod math;
use math::{Intersection, Point, Solid, Sphere, Vector};

mod scene;
use scene::{AngleUnit, Camera, Film, Perspective, Rectangle};

/// Converts a float value in range [-1, 1] to [0, 255].
fn float_to_hue(f: f32) -> u8 {
    assert!(f >= -1.0);
    assert!(f <= 1.0);
    (((f + 1.0) / 2.0) * 255.0) as u8
}

/// Converts a normal to an array to assign as a color.
fn unit_vector_as_color(v: Vector) -> [u8; 3] {
    [float_to_hue(v.x), float_to_hue(v.y), float_to_hue(v.z)]
}

fn main() {
    let film = Film::new(400, 300);
    let projection = Perspective::new(1.0, 1000.0, (45.0, AngleUnit::Degrees));

    let mut image = image::ImageBuffer::new(film.width() as u32, film.height() as u32);
    let camera = Camera::new(&film, &projection);

    let sphere = Sphere::new(Point::new(0.0, 0.0, 10.0), 2.0);
    let origin = Point::new(0.0, 0.0, 0.0);

    let light_direction = Vector::unit(1.0, 1.0, -1.0).unwrap();

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let ray = camera.generate_ray(x, y);
        if let Some(intersection) = sphere.intersect(&ray) {
            let normal = intersection.normal;
            assert!(normal.is_normalized());
            // Dot between normal and light position to get basic lambertian shading.
            let shade = normal.dot(light_direction);
            if shade > 0.0 {
                *pixel = image::Rgb([(shade * 255.0) as u8,
                                     (shade * 255.0) as u8,
                                     (shade * 255.0) as u8]);
            } else {
                image::Rgb([0, 0, 0]);
            }
            // Color like a normal map.
            // *pixel = image::Rgb(unit_vector_as_color(intersection.normal));
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    let ref mut fout = File::create(&Path::new("sphere.png")).unwrap();
    let _ = image::ImageRgb8(image).save(fout, image::PNG);
}
