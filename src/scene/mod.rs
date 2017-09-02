//! Lights, camera, materials, and dimensions!
//!
//! # Coordinate spaces
//! ## Screen space
//! Screen space defined by the image plane of the viewing frustum
//! centered at (0, 0).
//!
//! On the more limiting dimension of width or height, screen space extends from -1 to 1.  The
//! larger dimension will extend from `-aspect_ratio` to `+aspect_ratio`.
//!
//! The near plane is a Z=0 and the far plane at Z=1.
//!
//! ## Raster Space
//! The coordinates for pixels (sample positions) on the image.
//!
//! X range is [0, width], and Y range is [0, height] with (0,0) in the top left
//! corner.  This matches the way images are represented.
//!
//! ## World space
//! A left-handed coordinate system with X to the right, Y is up, and Z is into the screen.
#![allow(dead_code)]
pub mod camera;
pub mod dimensions;
pub mod nonarea_light;
pub mod material;
pub use self::camera::{Camera, Film, Perspective, Projection};
use self::nonarea_light::NonAreaLight;
use self::material::Material;

use std::f32::INFINITY;
use math::{Intersection, Matrix4x4, Ray, Solid, Vector};

// TODO: Define some set of units for this.
pub type Spectrum = Vector;

/// Some thing with a shape, and material properties.
struct Entity {
    solid: Box<Solid>,
    material: Box<Material>,

    // Transform into and out of this entity's coordinate space.
    transform: Transform,
}

impl Solid for Entity {
    fn intersect(&self, r: &Ray) -> Option<Intersection> {
        let local_ray = self.transform.to_local * (*r);

        if let Some(intersection) = self.solid.intersect(&local_ray) {
            // Convert the intersection back into the world coordinate system.
            return Some(self.transform.to_world * intersection);
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

/// An intersection which occurred on the scene.
struct SceneIntersection<'a> {
    pub entity: &'a Box<Entity>,
    pub intersection: Intersection,
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

    /// Creates an entity with given properties.
    ///
    /// # Arguments
    /// * `solid` - the intersection bounds of the object to create
    /// * `material` - material to apply to the object
    /// * `transform` - converts world coordinates to local coordinates
    pub fn add_entity(&mut self, solid: Box<Solid>, material: Box<Material>, transform: Matrix4x4) {
        self.entities.push(Box::new(Entity {
            solid: solid,
            material: material,
            transform: Transform {
                to_local: transform.inverse().expect(
                    "Uninvertible transform used for an entity.",
                ),
                to_world: transform,
            },
        }));
    }

    /// Called to determine the radiance returning along this ray in the opposite direction it was
    /// cast from.  This makes this used for backward ray casting.
    ///
    /// # Argument
    /// * `ray` - a ray emanating from the camera from the viewer, along which the radiance
    /// should be determined.
    ///
    /// # Returns
    /// * `Spectrum` - the radiance along this ray in the opposite direction.
    pub fn trace(&self, ray: &Ray) -> Spectrum {
        self.bounce(ray, 8)
    }

    fn bounce(&self, ray: &Ray, bounces_left: u32) -> Spectrum {
        // If no entity was intersected, return black.
        // This might be changed to account for other types of ambient light.
        match self.intersect(ray) {
            Some(si) => {
                if bounces_left == 0 {
                    return Vector::new(0.0, 0.0, 0.0);
                }
                let min_surface_distance = 0.01;
                let new_direction = ray.direction.reflect(&si.intersection.normal);
                let new_origin = si.intersection.point + min_surface_distance * new_direction;
                let next_ray = Ray {
                    origin: new_origin,
                    direction: new_direction,
                };
                return self.radiance_from(ray, si.entity, &si.intersection) +
                    self.bounce(&next_ray, bounces_left - 1);
            }
            None => Vector::new(0.0, 0.0, 0.),
        }
    }

    /// Finds the object and intersection point if a ray hits something.
    fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        let mut closest_object: Option<&Box<Entity>> = None;
        let mut closest_intersection: Option<Intersection> = None;
        let mut best_time: f32 = INFINITY;

        for ref obj in self.entities.iter() {
            if let Some(intersection) = obj.intersect(&ray) {
                //println!("Intersection at {}", intersection.point);
                if intersection.time < best_time && intersection.time > 0.0 {
                    best_time = intersection.time;
                    closest_intersection = Some(intersection);
                    closest_object = Some(*obj);
                }
            }
        }

        match closest_object {
            Some(object) => Some(SceneIntersection {
                entity: object,
                intersection: closest_intersection.unwrap(),
            }),
            None => None,
        }
    }

    /// Determine the total amount of radiance coming from a specific
    /// object along a given ray.
    fn radiance_from(
        &self,
        ray: &Ray,
        entity: &Box<Entity>,
        intersection: &Intersection,
    ) -> Spectrum {
        // Sum the contributions from all lights.
        let mut radiance = Vector::new(0.0, 0.0, 0.0);
        for ref light in self.lights.iter() {
            const PREVENT_SELF_INTERSECTION_RANGE: f32 = 0.01;
            let light_vector = light.light_vector(&intersection.point);
            let shadow_intersection = self.intersect(&Ray {
                origin: intersection.point + (PREVENT_SELF_INTERSECTION_RANGE * light_vector),
                direction: light_vector,
            });
            let light_hidden = match shadow_intersection {
                Some(si) => light.is_hidden_from(&intersection.point, Some(si.intersection.time)),
                None => light.is_hidden_from(&intersection.point, None),
            };

            if !light_hidden {
                // Determine if we can even see this light from the intersection point.
                // FIXME: this is wrong, and is just a guess-timate, and not physically accurate.
                radiance += entity.material.f(
                    // TODO: get direction to light.
                    &-light_vector,
                    &-ray.direction,
                ) *
                    light.irradiance(&intersection.point, &intersection.normal)
            } else {
                //radiance += Vector::new(0.0, 0.0, 0.5);
            }
        }
        radiance
    }
}
