use math::Vector;
use scene::Spectrum;

/// Materials determine the next ray direction of travel, as well as the describing the surface
/// properties of the object.
pub trait Material {
    /// Materials are perfectly reflective by default.
    ///
    /// # Arguments
    /// * `incident` - vector pointing into the material whose next direction must be determined.
    /// * `normal` - vector perpendicular to the surface
    ///
    /// # Return
    /// Either a reflected or refracted vector pointing in the new direction.
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
        LambertianMaterial { diffuse: *diffuse }
    }
}

impl Material for LambertianMaterial {
    #[allow(unused_variables)]
    fn f(&self, light: &Vector, view: &Vector) -> Spectrum {
        self.diffuse
    }
}
