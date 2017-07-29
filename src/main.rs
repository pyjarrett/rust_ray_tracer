#[macro_use]
extern crate approx;

extern crate image;
use std::fs::File;
use std::path::Path;

mod math;
use math::{Matrix4x4, Plane, Sphere, Vector};
use math::Axis;

mod scene;
use scene::*;

use scene::dimensions::Dimensions2;
use scene::material::*;
use scene::nonarea_light::*;

extern crate clap;
use clap::{App, SubCommand};

fn render_multiple_spheres() {
    // Set up the camera, film and the recording source.
    let film = Film::new(800, 600);
    let projection = Perspective::new(1.0, 1000.0, PlanarAngle::Degrees(90.0));
    let mut image = image::ImageBuffer::new(film.width() as u32, film.height() as u32);
    let camera = Camera::new(&film, &projection);

    // Build the scene.
    let mut scene = Scene::new();
    scene.add_light(Box::new(DirectionalLight::new(
        &Vector::new(0.0, 0.0, 1.0),
        &Vector::new(1.0, 1.0, 1.0),
    )));

    scene.add_light(Box::new(DirectionalLight::new(
        &Vector::new(0.0, -1.0, 0.0),
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

    scene.add_entity(
        Box::new(Plane::new(0.0, 1.0, 0.0, 1.5)),
        Box::new(LambertianMaterial::new(&Vector::new(0.5, 0.5, 0.5))),
        Matrix4x4::identity(),
    );

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

    // Write the scene.
    let scene_name = "scene.png";
    let ref mut fout = File::create(&Path::new(scene_name)).unwrap();
    let _ = image::ImageRgb8(image).save(fout, image::PNG);
}

extern crate rand;
use rand::distributions::{IndependentSample, Range};
use std::f32;
/// Single dimension Monte Carlo Integrator
///
/// # Arguments
/// * `a` - lower bound of integral
/// * `b` - upper bound of integral
/// * `n` - number of estimations
/// * `f` - function to integrate
fn monte_carlo_integrate<F>(a: f32, b: f32, n: u32, f: F) -> f32
where
    F: Fn(f32) -> f32,
{
    let between = Range::new(a, b);
    let mut rng = rand::thread_rng();

    let mut sum = 0.0_f32;
    let mut runs = 0;
    for _ in 0..n {
        sum += f(between.ind_sample(&mut rng));
        runs += 1;
    }
    println!("Runs {}", runs);
    return (b - a) / (n as f32) * sum;
}

fn monte_carlo_spike() {
    // Monte Carlo estimators give the expected value of the integration
    // F_N = (b-a)/N * sum(f(X_i) : from i=1 to N)
    // the PDF of the random variable X_i must be uniform and equal to 1/(b-a).
    // OR
    // the PDF is arbitrary, but must be nonzero where |f(x)| > 0
    println!(
        "integral of x*2, from 0->20: {}",
        monte_carlo_integrate(0.0, 20.0, 100, |x: f32| x * x)
    );
    println!(
        "integral of x*2, from 0->20: {}",
        monte_carlo_integrate(0.0, 20.0, 10000, |x: f32| x * x)
    );
    println!(
        "integral of x*2, from 0->20: {}",
        monte_carlo_integrate(0.0, 20.0, 100000, |x: f32| x * x)
    );
    println!(
        "integral of sin(x), from 0->pi: {}",
        monte_carlo_integrate(0.0, f32::consts::PI, 1000, |x: f32| x.sin())
    );
    println!(
        "integral of sin(x), from 0->pi: {}",
        monte_carlo_integrate(0.0, f32::consts::PI, 1000000, |x: f32| x.sin())
    );

    let estimate = monte_carlo_integrate(0.0, f32::consts::PI, 100, |x: f32| x.sin());
    let relative_error = (estimate - 2.0) / 2.0;
    println!(
        "integral of sin(x), 0->pi: {} {}% error",
        estimate,
        100.0 * relative_error
    );
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
        .subcommand(SubCommand::with_name("monte_carlo"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("scene") {
        render_multiple_spheres();
    } else if let Some(_) = matches.subcommand_matches("monte_carlo" /*prototype*/) {
        monte_carlo_spike();
    } else {
        println!("Unhandled render command.");
    }
}
