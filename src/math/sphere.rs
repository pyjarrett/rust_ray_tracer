use math::{Intersection, Point, Ray, Solid, Vector};

/// A sphere type centered at a specific origin.
pub struct Sphere {
    origin: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: Point, radius: f32) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
        }
    }

    /// Creates a sphere of given radius at the origin.
    pub fn new_with_radius(radius: f32) -> Sphere {
        assert!(radius >= 0.0);
        Sphere {
            origin: Point::new(0.0, 0.0, 0.0),
            radius: radius,
        }
    }

    pub fn intersection_time(&self, a_ray: Ray) -> Option<f32> {
        let mut r = a_ray;
        r.normalize().unwrap();

        // Set up basic relations.
        let origin_to_center = self.origin - r.origin;
        let sqrd_distance_to_center = origin_to_center.dot(origin_to_center);
        let sqrd_radius = self.radius * self.radius;

        // Find if ray is inside the sphere.
        let inside_sphere = sqrd_distance_to_center < sqrd_radius;

        let t_closest_approach = origin_to_center.dot(r.direction);

        // Ray closest approach is behind itself.
        if t_closest_approach <= 0.0 && !inside_sphere {
            return None;
        }

        // Determine half-chord distance.
        let t_sqrd_half_chord = sqrd_radius - sqrd_distance_to_center +
                                t_closest_approach * t_closest_approach;
        if t_sqrd_half_chord < 0.0 {
            return None;
        }
        if inside_sphere {
            return Some(t_closest_approach + t_sqrd_half_chord.sqrt());
        } else {
            return Some(t_closest_approach - t_sqrd_half_chord.sqrt());
        }
    }

    pub fn intersection_normal(&self, r: Ray) -> Option<Vector> {
        match self.intersection_time(r) {
            Some(t) => {
                let p = r.at(t);
                let mut normal = p - self.origin;
                normal.normalize().unwrap();
                Some(normal)
            }
            None => None,
        }
    }
}

impl Solid for Sphere {
    fn intersect(&self, r: &Ray) -> Option<Intersection> {
        debug_assert!(r.is_normalized());

        if let Some(time) = self.intersection_time(*r) {
            let point = r.at(time);
            let mut normal = point - self.origin;
            normal.normalize().unwrap();

            Some(Intersection {
                time: time,
                point: point,
                normal: normal,
            })
        } else {
            None
        }
    }
}


#[cfg(test)]
mod test {
    use super::Sphere;
    use math::{Point, Ray, Solid, Vector};
    use precision::*;

    #[test]
    pub fn test_intersection() {
        let s = Sphere {
            origin: Point::new(3.0, 0.0, 5.0),
            radius: 3.0,
        };

        let mut r = Ray {
            origin: Point::new(1.0, -2.0, -1.0),
            direction: Vector::new(1.0, 2.0, 4.0),
        };
        r.normalize().unwrap();

        let intersection = s.intersect(&r).unwrap();

        let expected_t = 3.744; //3.7434776; //3.744 per intro book.
        let expected_intersection_point = Point::new(1.816, -0.368, 2.269);
        let expected_normal = Vector::new(-0.395, -0.123, -0.91);

        let t_intersection = s.intersection_time(r).unwrap();
        assert_eq_eps(expected_t, t_intersection, 0.01);
        assert_eq_eps(expected_t, intersection.time, 0.01);

        // Use expected_t here to reduce impact of carried error.
        assert_eq_eps(expected_intersection_point.x, r.at(expected_t).x, 0.01);
        assert_eq_eps(expected_intersection_point.y, r.at(expected_t).y, 0.01);
        assert_eq_eps(expected_intersection_point.z, r.at(expected_t).z, 0.01);
        assert_eq_eps(expected_intersection_point.x, intersection.point.x, 0.01);
        assert_eq_eps(expected_intersection_point.y, intersection.point.y, 0.01);
        assert_eq_eps(expected_intersection_point.z, intersection.point.z, 0.01);

        // assert!(expected_intersection_point == r.at(expected_t));
        let intersection_normal = s.intersection_normal(r).unwrap();
        assert_eq_eps(expected_normal.x, intersection_normal.x, 0.01);
        assert_eq_eps(expected_normal.y, intersection_normal.y, 0.01);
        assert_eq_eps(expected_normal.z, intersection_normal.z, 0.01);

        assert!(intersection.normal.is_normalized());
        assert_eq_eps(expected_normal.x, intersection.normal.x, 0.01);
        assert_eq_eps(expected_normal.y, intersection.normal.y, 0.01);
        assert_eq_eps(expected_normal.z, intersection.normal.z, 0.01);
    }
}
