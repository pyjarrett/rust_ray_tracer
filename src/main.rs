#[macro_use]
extern crate approx;

extern crate image;
use std::fs::File;
use std::path::Path;

mod math;
use math::{Matrix4x4, Point, Solid, Sphere, Vector};
use math::Axis;

mod scene;
use scene::*;

extern crate clap;
use clap::{App, SubCommand};

fn render_sphere() {
    // Set up the camera, film and the recording source.
    let film = Film::new(400, 300);
    let projection = Perspective::new(1.0, 1000.0, (45.0, AngleUnit::Degrees));
    let mut image = image::ImageBuffer::new(film.width() as u32, film.height() as u32);
    let camera = Camera::new(&film, &projection);

    // Build the scene.
    let origin = Point::new(0.0, 0.0, 10.0);
    let sphere = Sphere::new(origin, 2.0);
    let light_direction = Vector::unit(1.0, 1.0, -1.0).unwrap();

    // Generates samples for all film points.
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let ray = camera.generate_ray(x, y);

        // Test for collision against the scene.
        if let Some(intersection) = sphere.intersect(&ray) {
            let normal = intersection.normal;
            assert!(normal.is_normalized());
            // Dot between normal and light position to get basic lambertian shading.
            let shade = normal.dot(&light_direction);
            if shade > 0.0 {
                *pixel = image::Rgb(
                    [
                        (shade * 255.0) as u8,
                        (shade * 255.0) as u8,
                        (shade * 255.0) as u8,
                    ],
                );
            } else {
                image::Rgb([0, 0, 0]);
            }
        // Color like a normal map.
        // *pixel = image::Rgb(unit_vector_as_color(intersection.normal));
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    // Write the scene.
    let scene_name = "sphere.png";
    let ref mut fout = File::create(&Path::new(scene_name)).unwrap();
    let _ = image::ImageRgb8(image).save(fout, image::PNG);
}

fn render_with_light() {
    // Set up the camera, film and the recording source.
    let film = Film::new(400, 300);
    let projection = Perspective::new(1.0, 1000.0, (45.0, AngleUnit::Degrees));
    let mut image = image::ImageBuffer::new(film.width() as u32, film.height() as u32);
    let camera = Camera::new(&film, &projection);

    // Build the scene.
    let origin = Point::new(0.0, 0.0, 10.0);
    let sphere = Sphere::new(origin, 2.0);
    let light = DirectionalLight::new(&Vector::new(0.0, 1.0, 1.0), &Vector::new(1.0, 1.0, 1.0));

    // Generates samples for all film points.
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let ray = camera.generate_ray(x, y);

        // Test for collision against the scene.
        if let Some(intersection) = sphere.intersect(&ray) {
            let normal = intersection.normal;
            assert!(normal.is_normalized());
            // Dot between normal and light position to get basic lambertian shading.
            let shade = light.irradiance(&intersection.point, &intersection.normal);
            *pixel = image::Rgb(
                [
                    (shade[Axis::X] * 255.0) as u8,
                    (shade[Axis::Y] * 255.0) as u8,
                    (shade[Axis::Z] * 255.0) as u8,
                ],
            );
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    // Write the scene.
    let scene_name = "sphere.png";
    let ref mut fout = File::create(&Path::new(scene_name)).unwrap();
    let _ = image::ImageRgb8(image).save(fout, image::PNG);
}

fn render_multiple_spheres() {
    // Set up the camera, film and the recording source.
    let film = Film::new(400, 300);
    let projection = Perspective::new(1.0, 1000.0, (45.0, AngleUnit::Degrees));
    let mut image = image::ImageBuffer::new(film.width() as u32, film.height() as u32);
    let camera = Camera::new(&film, &projection);

    // Build the scene.
    let mut scene = Scene::new();
    scene.add_light(Box::new(DirectionalLight::new(
        &Vector::new(0.0, 1.0, 0.0),
        &Vector::new(1.0, 1.0, 1.0),
    )));

    scene.add_light(Box::new(DirectionalLight::new(
        &Vector::new(0.0, -1.0, 0.0),
        &Vector::new(0.0, 1.0, 1.0),
    )));

    scene.add_entity(
        Box::new(Sphere::new_with_radius(2.0)),
        Box::new(LambertianMaterial::new(&Vector::new(0.0, 0.0, 1.0))),
        Matrix4x4::translate(0.0, 0.0, -10.0),
    );

    scene.add_entity(
        Box::new(Sphere::new_with_radius(1.0)),
        Box::new(LambertianMaterial::new(&Vector::new(1.0, 0.0, 0.0))),
        Matrix4x4::translate(-3.0, 0.0, -10.0),
    );

    scene.add_entity(
        Box::new(Sphere::new_with_radius(1.0)),
        Box::new(LambertianMaterial::new(&Vector::new(1.0, 0.0, 0.0))),
        Matrix4x4::translate(4.0, 0.0, -10.0),
    );

    // Generates samples for all film points.
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


trait Renderer {
    fn render();
}

/// A basic renderer which does not support area lights.
struct BasicRenderer {}
impl BasicRenderer {
    pub fn render() {}
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

    if let Some(_) = matches.subcommand_matches("basic_sphere") {
        render_sphere();
    } else if let Some(_) = matches.subcommand_matches("directional_light") {
        render_with_light();
    } else if let Some(_) = matches.subcommand_matches("scene") {
        render_multiple_spheres();
    }
}
