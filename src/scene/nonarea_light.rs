use math::{Point, Vector};
use scene::Spectrum;


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

    /// Gives the vector pointing to the light.
    fn light_vector(&self, point: &Point) -> Vector;

    /// As the light if it lies before or after the given intersection point along the light
    /// vector.
    fn is_hidden_from(&self, point: &Point, first_hit_along_light_vector: Option<f32>) -> bool;
}

/// A light who supplies light from a specific direction.
pub struct DirectionalLight {
    direction: Vector,
    radiance: Spectrum,
}

impl DirectionalLight {
    pub fn new(direction: &Vector, radiance: &Spectrum) -> DirectionalLight {
        let mut d = *direction;
        d.normalize().expect(
            "Provide a direction vector which cannot be normalized for a directional light.",
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
        (-self.direction).dot(&normal).max(0.0) * self.radiance
    }

    #[allow(unused_variables)]
    fn light_vector(&self, point: &Point) -> Vector {
        -self.direction
    }

    /// The first object hit will shadow all further objects.
    #[allow(unused_variables)]
    fn is_hidden_from(&self, point: &Point, first_hit_along_light_vector: Option<f32>) -> bool {
        first_hit_along_light_vector.is_some()
    }
}

pub struct PointLight {
    position: Point,
    intensity: Spectrum,
}

impl PointLight {
    pub fn new(position: Point, intensity: Spectrum) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

impl NonAreaLight for PointLight {
    fn irradiance(&self, position: &Point, normal: &Vector) -> Spectrum {
        let distance = position.distance_to(self.position);
        let e = self.light_vector(position).dot(normal).max(0.0) * self.intensity /
            (distance * distance).min(1.0);

        //println!("light vector {}", light_vector);
        //println!("position {}", position);
        //println!("normal {}", normal);
        //println!("distance*distance {}", distance * distance);
        //println!("e={}", e);
        e
    }

    #[allow(unused_variables)]
    fn light_vector(&self, point: &Point) -> Vector {
        let mut light_vector = self.position - *point;
        light_vector.normalize().expect(
            "Cannot normalize light vector.",
        );
        light_vector
    }

    #[allow(unused_variables)]
    fn is_hidden_from(&self, point: &Point, first_hit_along_light_vector: Option<f32>) -> bool {
        let distance = self.position.distance_to(*point);
        match first_hit_along_light_vector {
            Some(hit) => hit < distance,
            None => false,
        }
    }
}
