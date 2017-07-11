#![allow(dead_code)]
pub mod camera;
pub use self::camera::{AngleUnit, Camera, Film, Rectangle, Perspective, Projection};

use math::{Matrix4x4, Point, Solid, Vector};

type Spectrum = Vector;

/// Materials determine the next ray direction of travel, as well as the describing the surface
/// properties of the object.
trait Material {
    /// Materials are perfectly reflective by default.
    fn next_ray_direction(&self, incident: &Vector, normal: &Vector) -> Vector {
        2.0 * (normal.dot(incident) * (*normal))
    }

    /// BRDF function giving ratio of differential outgoing radiance (dependent upon the view
    /// vector) to differential irradiance, dependent upon the light direction.
    ///
    /// # Arguments
    /// * `light` - light vector, points to the light
    /// * `view` - view vector, points to the viewer.
    fn f(&self, light: &Vector, view: &Vector) -> Spectrum;
}

/// Lambertian material consisting of a single diffuse color.
struct LambertianMaterial {
    diffuse: Spectrum,
}

impl Material for LambertianMaterial {
    #[allow(unused_variables)]
    fn f(&self, light: &Vector, view: &Vector) -> Spectrum {
        self.diffuse
    }
}

/// Some thing with a shape, and material properties.
struct Entity {
    shape: Box<Solid>,
    material: Box<Material>,

    // Transform into and out of this entity's coordinate space.
    to_local: Matrix4x4,
    to_world: Matrix4x4,
}

impl Entity {}


pub trait NonAreaLight {
    /// We use a simplified version of the BRDF in this case, so here use irradiance instead of
    /// radiance.
    fn irradiance(&self, position: &Point, light: &Vector) -> Spectrum;
}

pub struct DirectionalLight {
    direction: Vector,
    color: Spectrum,
}

impl DirectionalLight {
    pub fn new(direction: &Vector, color: &Spectrum) -> DirectionalLight {
        let mut d = *direction;
        d.normalize().expect(
            "Provide a direction vector which cannot be normalized for a directional light."
        );
        DirectionalLight {
            direction: d,
            color: *color,
        }
    }
}

impl NonAreaLight for DirectionalLight {
    #[allow(unused_variables)]
    fn irradiance(&self, position: &Point, normal: &Vector) -> Spectrum {
        (-self.direction.dot(&normal)).max(0.0) * self.color
    }
}

/*
impl Solid for Entity {
    fn intersect(&self, r: &Ray) -> Option<Intersection> {
        let local_ray = to_local * r;

        if let Some(intersection) = self.shape.intersect(local_ray) {
            // Convert the intersection back into the world coordinate system.
            Some(to_world * intersection)
        }
        None
    }
}
*/
