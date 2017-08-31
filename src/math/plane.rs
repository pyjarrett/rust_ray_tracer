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
        Plane { a, b, c, d }
    }

    /// Plane defined using `x0` from `n.dot(x - x0) == 0`.
    /// Thus, `d = -x0.dot(n)`
    pub fn from_normal_and_point(normal: &Vector, point: &Point) -> Plane {
        assert!(normal.is_normalized());

        Plane::new(
            normal.x,
            normal.y,
            normal.z,
            -normal.dot(&Vector::from(*point)),
        )
    }

    pub fn normal(&self) -> Vector {
        let mut n = Vector::new(self.a, self.b, self.c);
        n.normalize().expect(
            "Cannot create a normal vector for the plane",
        );
        n
    }

    pub fn distance_to_point(&self, point: &Point) -> f32 {
        ((self.a * point.x + self.b * point.y + self.c * point.z + self.d) /
             f32::sqrt(self.a * self.a + self.b * self.b + self.c * self.c)).abs()
    }
}

impl Solid for Plane {
    fn intersect(&self, r: &Ray) -> Option<Intersection> {
        // Intersection time is determined by the t needed for the ray's point at that t to be
        let mut normal = Vector::new(self.a, self.b, self.c);
        normal.normalize().expect("Plane has no normal...?");
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

#[cfg(test)]
mod tests {
    use math::{Ray, Point, Solid, Vector};
    use super::Plane;

    #[test]
    fn test_normal() {
        let px = Plane::new(4.0, 0.0, 0.0, 0.0);
        assert_relative_eq!(px.normal(), Vector::new(1.0, 0.0, 0.0));

        let py = Plane::new(0.0, -4.0, 0.0, 0.0);
        assert_relative_eq!(py.normal(), Vector::new(0.0, -1.0, 0.0));
    }

    #[test]
    fn test_from_normal_and_point() {
        let normal = Vector::new(1.0, 0.0, 0.0);
        let point = Point::new(4.0, 0.0, 0.0);
        let p = Plane::from_normal_and_point(&normal, &point);
        assert_relative_eq!(p.normal(), normal);
        assert_relative_eq!(p.distance_to_point(&Point::new(4.0, 15.0, 20.0)), 0.0);
        assert_relative_eq!(p.distance_to_point(&Point::new(4.0, 20.0, 50.0)), 0.0);
        assert_relative_eq!(p.distance_to_point(&Point::new(3.0, 20.0, 50.0)), 1.0);
    }

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
            assert_relative_eq!(intersection.point, expected_p);
        } else {
            panic!("Couldn't find intersection of ray and plane!");
        }
    }
}
