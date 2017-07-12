#![allow(dead_code)]
pub mod camera;
pub use self::camera::{AngleUnit, Camera, Film, Rectangle, Perspective, Projection};

use std::f32::INFINITY;
use math::{Intersection, Matrix4x4, Point, Ray, Solid, Vector};

// TODO: Define some set of units for this.
type Spectrum = Vector;

/// Materials determine the next ray direction of travel, as well as the describing the surface
/// properties of the object.
pub trait Material {
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
pub struct LambertianMaterial {
    diffuse: Spectrum,
}

impl LambertianMaterial {
    pub fn new(diffuse: &Spectrum) -> LambertianMaterial {
        LambertianMaterial {
            diffuse: *diffuse
        }
    }
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
    transform: Transform,
}

impl Solid for Entity {
    fn intersect(&self, r: &Ray) -> Option<Intersection> {
        let local_ray = self.transform.to_local * (*r);

        if let Some(intersection) = self.shape.intersect(&local_ray) {
            // Convert the intersection back into the world coordinate system.
            return Some(self.transform.to_world * intersection)
        }
        None
    }
}

/// Store to and from the transforms into and out of a given local coordinate space.
///
/// Since these inversions can be expensive if calculated, but not if the inversion
/// gets created based on the original transform (e.g. rotating -30 degrees about X
/// has the inverse of rotating 30 degrees, but calculating the inversion is expensive).
pub struct Transform {
    pub to_local: Matrix4x4,
    pub to_world: Matrix4x4,
}

pub trait NonAreaLight {
    /// We use a simplified version of the BRDF in this case, so here use irradiance instead of
    /// radiance.
    ///
    /// # Arguments
    /// * `position` - point to illuminate with the light
    /// * `normal` - the surface normal being illuminated.
    ///
    /// # Returns
    /// * `Spectrum` - the irradiance measured at the surface
    fn irradiance(&self, position: &Point, normal: &Vector) -> Spectrum;
}

pub struct DirectionalLight {
    direction: Vector,
    radiance: Spectrum,
}

impl DirectionalLight {
    pub fn new(direction: &Vector, radiance: &Spectrum) -> DirectionalLight {
        let mut d = *direction;
        d.normalize().expect(
            "Provide a direction vector which cannot be normalized for a directional light."
        );
        DirectionalLight {
            direction: d,
            radiance: *radiance,
        }
    }
}

impl NonAreaLight for DirectionalLight {
    #[allow(unused_variables)]
    fn irradiance(&self, position: &Point, normal: &Vector) -> Spectrum {
        (-self.direction.dot(&normal)).max(0.0) * self.radiance
    }
}

pub struct Scene {
    lights: Vec<Box<NonAreaLight>>,
    entities: Vec<Box<Entity>>,
}

impl Scene {
    /// Creates a new and empty scene.
    pub fn new() -> Scene {
        Scene {
            lights: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: Box<NonAreaLight>) {
        self.lights.push(light);
    }

    /// TODO: Merge terminology of "shape" and "solid".
    pub fn add_entity(&mut self, shape: Box<Solid>, material: Box<Material>, transform: Matrix4x4)
    {
        self.entities.push(Box::new(Entity {
            shape: shape,
            material: material,
            transform: Transform {
                to_local: transform,
                to_world: transform.inverse().expect("Uninvertible trasnform used for an entity."),
            }
        }));
    }

    /// Called to determine the radiance returning along this ray in the opposite direction it was
    /// cast from.  This makes this used for backward ray casting.
    ///
    /// # Argument
    /// * `ray` - a ray eminating from the camera from the viewer, along which the radiance
    /// should be determined.
    ///
    /// # Return
    /// * `Spectrum` - the radiance along this ray in the opposite direction.
    pub fn trace(&self, ray: &Ray) -> Spectrum {
        // Find the closest entity being intersected.
        let mut closest_object : Option<&Box<Entity>> = None;
        let mut closest_intersection: Option<Intersection> = None;
        let mut best_time: f32 = INFINITY;

        for ref obj in self.entities.iter() {
            // TODO: Transform the ray into the local coordinate space of the object.
            if let Some(intersection) = obj.intersect(&ray) {
                if intersection.time < best_time {
                    best_time = intersection.time;
                    closest_intersection = Some(intersection);
                    closest_object = Some(*obj);
                }
            }
        }

        // If no entity was intersected, return black.
        match closest_object {
            None => Vector::new(0.0, 0.0, 0.),
            Some(obj) => self.radiance_from(ray, obj, &closest_intersection.unwrap()),
        }
    }

    /// Determine the total amount of radiance coming from a specific
    /// object along a given ray.
    fn radiance_from(&self, ray: &Ray, entity: &Box<Entity>, intersection: &Intersection) -> Spectrum {
        // Sum the contributions from all lights.
        let mut radiance = Vector::new(0.0, 0.0, 0.0);

        for ref light in self.lights.iter() {
            radiance += entity.material.f(&intersection.normal, &intersection.normal) * light.irradiance(&intersection.point, &intersection.normal)
        }
        radiance
    }
}
