use std::ops::Mul;

use math::{Matrix4x4, Point, Ray, Vector};

/// Provides intersection information for use by the renderer.
/// In general, intersections need to provide the time, point, and surface normal.
/// This information gets reported in the local coordinate system.
pub struct Intersection {
    pub time: f32,
    pub point: Point,
    pub normal: Vector,
}


impl Mul<Intersection> for Matrix4x4 {
    type Output = Intersection;
    fn mul(self, i: Intersection) -> Self::Output {
        Intersection {
            time: i.time,
            point: self * i.point,
            normal: self * i.normal,
        }
    }
}

pub trait Solid {
    /// Provides intersection reporting against a ray.
    fn intersect(&self, r: &Ray) -> Option<Intersection>;
}
