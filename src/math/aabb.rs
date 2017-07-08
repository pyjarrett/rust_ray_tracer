use math::{Point, Ray, XYZ};
use std::{f32, mem};

/// Axis-Aligned Bounding Box (AABB).
///
/// TODO: Checks to ensure that lower < upper
pub struct AABB {
    pub lower: Point,
    pub upper: Point,
}

impl AABB {
    pub fn intersection_time(&self, a_ray: Ray) -> Option<f32> {
        let mut r = a_ray;
        r.normalize().unwrap();

        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;

        // For each of the dimensions, X, Y, Z, check:
        for a in &XYZ {
            // Determine if the ray is parallel to the slab.
            // Check to see if the ray origin is in between the slabs.
            let dir = r.direction[*a];
            let left = self.lower[*a];
            let right = self.upper[*a];
            let pos = r.origin[*a];
            if dir == 0.0 {
                if pos < left || right < pos {
                    return None;
                }
            }
            let mut t1 = (left - pos) / dir;
            let mut t2 = (right - pos) / dir;

            // Swap to ensure t1 < t2.
            if t1 > t2 {
                mem::swap(&mut t1, &mut t2);
            }

            // Move t_near out, and t_far in
            if t1 > t_near {
                t_near = t1;
            }
            if t2 < t_far {
                t_far = t2;
            }

            if t_near > t_far || t_far < 0.0 {
                return None;
            }
        }
        return Some(t_near);
    }
}

#[cfg(test)]
mod test {
    use super::AABB;
    use math::{Point, Ray, Vector};
    use precision::assert_approx_eq;

    #[test]
    fn test_misses_box() {
        let bb = AABB {
            lower: Point::new(-1.0, 2.0, 1.0),
            upper: Point::new(3.0, 3.0, 3.0),
        };
        let r = Ray {
            origin: Point::new(0.0, 4.0, 2.0),
            direction: Vector::new(0.218, -0.436, 0.873),
        };
        let intersection_time = bb.intersection_time(r);
        assert!(intersection_time.is_none());
    }

    #[test]
    fn test_hits_box() {
        let bb = AABB {
            lower: Point::new(-1.0, -1.0, -1.0),
            upper: Point::new(1.0, 1.0, 1.0),
        };
        let r = Ray {
            origin: Point::new(0.0, 0.0, 50.0),
            direction: Vector::new(0.0, 0.0, -1.0),
        };
        let intersection_time = bb.intersection_time(r);
        assert!(intersection_time.is_some());
        assert_approx_eq(intersection_time.unwrap(), 49.0);
    }
}
