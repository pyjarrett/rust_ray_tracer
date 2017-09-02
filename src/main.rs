#[macro_use]
extern crate approx;

extern crate image;
use std::fs::File;
use std::path::Path;

mod math;
use math::{Axis, Matrix4x4, PlanarAngle, Plane, Point, Sphere, Vector};

mod scene;
use scene::*;

use scene::dimensions::Dimensions2;
use scene::material::*;
use scene::nonarea_light::*;

extern crate clap;
use clap::{App, SubCommand};

type ColorImage = image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;

fn render_multiple_spheres() {
    let film = Film::new(800, 600);
    //let film = Film::new(3840, 2160); (4K)
    let mut image = ColorImage::new(film.width() as u32, film.height() as u32);

    ray_cast(create_default_camera(&film), build_scene(), &mut image);

    write_image(image, "scene.png");
}

fn print_view_frustum_corners(film: &Film, camera: &Camera, near: f32, far: f32) {
    // Print the corners of the view frustum
    println!("Rendered world outer points.");
    let corners = [
        (0, film.width() as u32),
        (0, 0),
        (film.height() as u32, 0),
        (film.height() as u32, film.width() as u32),
    ];

    for &pair in corners.iter() {
        let ray = camera.generate_ray(pair.0, pair.1);
        println!("{} {} {}", pair.0, pair.1, ray.at(near));
        println!("{} {} {}", pair.0, pair.1, ray.at(far));
    }
}

fn create_default_camera(film: &Film) -> Camera {
    let near = 1.0;
    let far = 1000.0;
    let projection = Perspective::new(near, far, PlanarAngle::Degrees(90.0));
    let c = Camera::new(film, &projection);

    print_view_frustum_corners(film, &c, near, far);
    c
}

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // LIGHTS!
    scene.add_light(Box::new(DirectionalLight::new(
        &Vector::new(0.0, -1.0, 0.0),
        &Vector::new(1.0, 1.0, 1.0),
    )));

    scene.add_light(Box::new(PointLight::new(
        Point::new(0.0, 20.0, 30.0),
        1.0 * Vector::new(1.0, 1.0, 1.0),
    )));

    // OBJECTS
    scene.add_entity(
        Box::new(Sphere::new_with_radius(5.0)),
        Box::new(LambertianMaterial::new(&Vector::new(1.0, 1.0, 1.0))),
        Matrix4x4::translate(0.0, 0.0, 30.0),
    );

    scene.add_entity(
        Box::new(Sphere::new_with_radius(5.0)),
        Box::new(LambertianMaterial::new(&Vector::new(1.0, 1.0, 1.0))),
        Matrix4x4::translate(0.0, 10.0, 30.0),
    );
    scene.add_entity(
        Box::new(Sphere::new_with_radius(5.0)),
        Box::new(LambertianMaterial::new(&Vector::new(1.0, 1.0, 1.0))),
        Matrix4x4::translate(10.0, 0.0, 30.0),
    );

    scene.add_entity(
        Box::new(Plane::from_normal_and_point(
            &Vector::new(0.0, 1.0, 0.0),
            &Point::new(0.0, -5.0, 30.0),
        )),
        Box::new(LambertianMaterial::new(&Vector::new(0.2, 0.2, 0.2))),
        Matrix4x4::identity(),
    );

    scene
}

fn ray_cast(camera: Camera, scene: Scene, image: &mut ColorImage) {
    // Generates samples for all film points.
    // (0, 0) is the top left corner.
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let ray = camera.generate_ray(x, y);

        let shade = scene.trace(&ray);
        *pixel = image::Rgb(
            [
                (shade[Axis::X] * 255.0).min(255.0) as u8,
                (shade[Axis::Y] * 255.0).min(255.0) as u8,
                (shade[Axis::Z] * 255.0).min(255.0) as u8,
            ],
        );
    }
}

fn write_image(image: ColorImage, file_name: &str) {
    let ref mut fout = File::create(&Path::new(file_name)).unwrap();
    let _ = image::ImageRgb8(image).save(fout, image::PNG);
}

fn main() {
    let matches = App::new("Rust Ray Tracer")
        .version("1.0")
        .about("Basic ray tracing renderer, written in Rust.")
        .subcommand(SubCommand::with_name("basic_sphere").about(
            "Render simple sphere",
        ))
        .subcommand(SubCommand::with_name("scene").about(
            "Render a simple directional light and multiple spheres!",
        ))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("scene") {
        render_multiple_spheres();
    } else {
        println!("Unhandled render command.");
    }
}
