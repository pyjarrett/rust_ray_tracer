use math::{Intersection, Point, Ray, Solid, Vector};

/// An infinitely stretching plane defined by a normal, and the distance from the coordinate system
/// origin to the plane.
///
/// The surface normal is defined by `(a, b, c)`.
pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Plane {
        assert_relative_eq!(a * a + b * b + c * c, 1.0);
        Plane { a, b, c, d }
    }
}

impl Solid for Plane {
    fn intersect(&self, r: &Ray) -> Option<Intersection> {
        // Intersection time is determined by the t needed for the ray's point at that t to be
        let normal = Vector::new(self.a, self.b, self.c);
        let v0 = -(normal.dot(&Vector::from(r.origin)) + self.d);

        // Ray is parallel to the plane.
        if v0 == 0.0 {
            return None;
        }

        let vd = normal.dot(&r.direction);
        let t = v0 / vd;

        Some(Intersection {
            time: t,
            point: r.at(t),
            normal: normal,
        })
    }
}


mod tests {
    use math::{Intersection, Point, Ray, Solid, Vector};
    use super::Plane;

    #[test]
    fn test_intersection() {
        let x_at_7 = Plane::new(1.0, 0.0, 0.0, -7.0);
        let origin = Point::new(2.0, 3.0, 4.0);
        let direction = Vector::new(0.577, 0.577, 0.577);
        let ray = Ray { origin, direction };

        let expected_t = 8.665511;
        let expected_p = Point::new(7.0, 8.0, 9.0);

        if let Some(intersection) = x_at_7.intersect(&ray) {
            assert_relative_eq!(intersection.time, expected_t);
            assert_relative_eq!(intersection.point.x, expected_p.x);
            assert_relative_eq!(intersection.point.y, expected_p.y);
            assert_relative_eq!(intersection.point.z, expected_p.z);
        } else {
            panic!("Couldn't find intersection of ray and plane!");
        }
    }
}
