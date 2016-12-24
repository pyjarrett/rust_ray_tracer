use math::{Point, Ray, Vector};

/// Provides intersection information for use by the renderer.
/// In general, intersections need to provide the time, point, and surface normal.
/// This information gets reported in the local coordinate system.
pub struct Intersection {
    pub time: f32,
    pub point: Point,
    pub normal: Vector,
}


pub trait Solid {
    /// Provides intersection reporting against a ray.
    fn intersect(&self, r: &Ray) -> Option<Intersection>;
}
