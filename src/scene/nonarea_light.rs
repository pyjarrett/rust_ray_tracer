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
}
