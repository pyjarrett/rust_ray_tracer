use std::cmp::Eq;
use std::fmt;
use std::convert::From;
use std::ops::{Add, Sub, Mul, Index};
use math::{Axis, Vector};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z }
    }

    pub fn distance_to(self, p: Point) -> f32 {
        (self - p).length()
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, v: Vector) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, p: Point) -> Vector {
        Vector {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, p: Point) -> Point {
        Point {
            x: self * p.x,
            y: self * p.y,
            z: self * p.z,
        }
    }
}

impl Index<Axis> for Point {
    type Output = f32;
    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}


impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (relative_eq!(other.x, self.x) && relative_eq!(other.y, self.y) &&
             relative_eq!(other.z, self.z))
    }
}

impl Eq for Point {}


impl From<Point> for Vector {
    fn from(p: Point) -> Self {
        Vector::new(p.x, p.y, p.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:>7.4} {:>7.4} {:>7.4} ]", self.x, self.y, self.z)
    }
}


#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_equality() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(3.0, 5.0, 3.0);
        let p3 = Point::new(1.0, 2.0, -3.0);
        let p1_eq = Point::new(1.0, 2.0, 3.0);

        assert!(p1 != p2);
        assert!(p1 != p3);
        assert!(p1 == p1_eq);
    }

    #[test]
    fn test_distance_to() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 0.0);
        assert_relative_eq!(p1.distance_to(p2), 5.0);
        assert_relative_eq!(p2.distance_to(p1), 5.0);
    }
}
