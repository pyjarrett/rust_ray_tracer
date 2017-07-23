#[macro_use]
extern crate approx;

extern crate image;
use std::fs::File;
use std::path::Path;

mod math;
use math::{Matrix4x4, Point, Sphere, Vector};
use math::Axis;

mod scene;
use scene::*;

extern crate clap;
use clap::{App, SubCommand};

fn render_multiple_spheres() {
    // Set up the camera, film and the recording source.
    let film = Film::new(400, 300);
    let projection = Perspective::new(1.0, 1000.0, PlanarAngle::Degrees(90.0));
    let mut image = image::ImageBuffer::new(film.width() as u32, film.height() as u32);
    let camera = Camera::new(&film, &projection);

    // Build the scene.
    let mut scene = Scene::new();
    scene.add_light(Box::new(DirectionalLight::new(
        &Vector::new(0.0, 0.0, 1.0),
        &Vector::new(1.0, 1.0, 1.0),
    )));

    scene.add_entity(
        Box::new(Sphere::new_with_radius(0.1)),
        Box::new(LambertianMaterial::new(&Vector::new(0.0, 0.0, 1.0))),
        Matrix4x4::translate(0.0, 0.0, 30.0),
    );

    scene.add_entity(
        Box::new(Sphere::new_with_radius(0.1)),
        Box::new(LambertianMaterial::new(&Vector::new(1.0, 0.0, 0.0))),
        Matrix4x4::translate(0.2, 0.4, 30.0),
    );

    scene.add_entity(
        Box::new(Sphere::new_with_radius(0.1)),
        Box::new(LambertianMaterial::new(&Vector::new(1.0, 0.0, 0.0))),
        Matrix4x4::translate(0.2, 0.0, 30.0),
    );

    // Generates samples for all film points.
    // (0, 0) is the top left corner.
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let ray = camera.generate_ray(x, y);

        let shade = scene.trace(&ray);
        *pixel = image::Rgb(
            [
                (shade[Axis::X] * 255.0) as u8,
                (shade[Axis::Y] * 255.0) as u8,
                (shade[Axis::Z] * 255.0) as u8,
            ],
        );
    }

    // Write the scene.
    let scene_name = "scene.png";
    let ref mut fout = File::create(&Path::new(scene_name)).unwrap();
    let _ = image::ImageRgb8(image).save(fout, image::PNG);
}


fn main() {
    let matches = App::new("Rust Ray Tracer")
        .version("1.0")
        .about("Basic ray tracing renderer, written in Rust.")
        .subcommand(SubCommand::with_name("basic_sphere").about(
            "Render simple sphere",
        ))
        .subcommand(SubCommand::with_name("directional_light").about(
            "Render a simple directional light and sphere",
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
