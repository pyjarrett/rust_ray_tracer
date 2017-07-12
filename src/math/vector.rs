use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg, Index};
use std::fmt;
use math::Axis;

const MIN_LENGTH_FOR_NORMALIZATION: f32 = 1e-6;
const NORMALIZED_EPS: f32 = 1e-6;

/// 3D type for vectors and points.
#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    /// Creates a vector and then normalizes it.
    /// Returns an error if the vector cannot assume unit length.
    pub fn unit(x: f32, y: f32, z: f32) -> Result<Vector, ()> {
        let mut v = Vector::new(x, y, z);
        match v.normalize() {
            Ok(_) => Ok(v),
            Err(_) => Err(()),
        }
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Possibly normalizes the vector.
    pub fn normalize(&mut self) -> Result<f32, ()> {
        let len = self.length();
        let inv_len = 1.0 / len;
        if len > MIN_LENGTH_FOR_NORMALIZATION {
            self.x = self.x * inv_len;
            self.y = self.y * inv_len;
            self.z = self.z * inv_len;
            return Ok(self.length());
        }
        return Err(());
    }

    pub fn is_normalized(&self) -> bool {
        (self.length() - 1.0).abs() < NORMALIZED_EPS
    }

    pub fn dot(&self, v: &Vector) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// The angle between two vectors in degrees in the range [0, 180].
    pub fn angle_with_in_degrees(&self, v: Vector) -> f32 {
        let mut a = self.clone();
        let mut b = v.clone();
        a.normalize().unwrap();
        b.normalize().unwrap();
        a.dot(&b).acos().to_degrees()
    }
}


impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Vector {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

/// Allows indexing for X, Y, and Z components.
impl Index<Axis> for Vector {
    type Output = f32;
    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:>7.4} {:>7.4} {:>7.4} ]", self.x, self.y, self.z)
    }
}


#[cfg(test)]
mod test {
    use super::Vector;

    #[test]
    fn test_vector_length() {
        assert_relative_eq!(Vector::new(0.0, 0.0, 0.0).length(), 0.0);
        assert_relative_eq!(Vector::new(1.0, 0.0, 0.0).length(), 1.0);
        assert_relative_eq!(Vector::new(4.0, 0.0, 0.0).length(), 4.0);
        assert_relative_eq!(Vector::new(0.0, 3.0, 4.0).length(), 5.0);
        assert_relative_eq!(Vector::new(0.0, -3.0, -4.0).length(), 5.0);
    }

    #[test]
    fn test_vector_normalization() {
        let mut v1 = Vector::new(3.0, 4.0, 5.0);
        let new_length = v1.normalize().unwrap();
        assert_relative_eq!(new_length, 1.0);
        assert_relative_eq!(new_length, v1.length());

        let mut zero_vector = Vector::new(0.0, 0.0, 0.0);
        assert!(zero_vector.normalize().is_err());
    }

    #[test]
    fn test_vector_fails_normalization() {
        let mut v1 = Vector::new(0.0, 0.0, 0.0);
        assert!(v1.normalize().is_err());
    }

    #[test]
    fn test_angle_in_degrees() {
        let x = Vector::new(1.0, 0.0, 0.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        let z = Vector::new(0.0, 0.0, 1.0);

        assert_relative_eq!(x.angle_with_in_degrees(x), 0.0, max_relative = 0.001);
        assert_relative_eq!(x.angle_with_in_degrees(-x), 180.0, max_relative = 0.001);
        assert_relative_eq!((-x).angle_with_in_degrees(x), 180.0, max_relative = 0.001);

        assert_relative_eq!(y.angle_with_in_degrees(y), 0.0, max_relative = 0.001);
        assert_relative_eq!(y.angle_with_in_degrees(-y), 180.0, max_relative = 0.001);
        assert_relative_eq!((-y).angle_with_in_degrees(y), 180.0, max_relative = 0.001);

        assert_relative_eq!(z.angle_with_in_degrees(z), 0.0, max_relative = 0.001);
        assert_relative_eq!(z.angle_with_in_degrees(-z), 180.0, max_relative = 0.001);
        assert_relative_eq!((-z).angle_with_in_degrees(z), 180.0, max_relative = 0.001);

        assert_relative_eq!(x.angle_with_in_degrees(y), 90.0, max_relative = 0.001);
        assert_relative_eq!(x.angle_with_in_degrees(-y), 90.0, max_relative = 0.001);

        assert_relative_eq!(y.angle_with_in_degrees(x), 90.0, max_relative = 0.001);
        assert_relative_eq!(y.angle_with_in_degrees(-x), 90.0, max_relative = 0.001);

        assert_relative_eq!(z.angle_with_in_degrees(y), 90.0, max_relative = 0.001);
        assert_relative_eq!(z.angle_with_in_degrees(-y), 90.0, max_relative = 0.001);

        assert_relative_eq!(z.angle_with_in_degrees(x), 90.0, max_relative = 0.001);
        assert_relative_eq!(z.angle_with_in_degrees(-x), 90.0, max_relative = 0.001);
    }

    #[test]
    fn test_vector_add() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_relative_eq!(v3.x, 5.0);
        assert_relative_eq!(v3.y, 7.0);
        assert_relative_eq!(v3.z, 9.0);
    }

    #[test]
    fn test_vector_sub() {
        let v1 = Vector::new(4.0, 5.0, 6.0);
        let v2 = Vector::new(3.0, 2.0, 1.0);
        let v3 = v1 - v2;
        assert_relative_eq!(v3.x, 1.0);
        assert_relative_eq!(v3.y, 3.0);
        assert_relative_eq!(v3.z, 5.0);
    }

    #[test]
    fn test_vector_mul() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = 7.0 * v1;
        assert_relative_eq!(v2.x, 7.0);
        assert_relative_eq!(v2.y, 14.0);
        assert_relative_eq!(v2.z, 21.0);
    }

    #[test]
    fn test_vector_div() {
        let v1 = Vector::new(10.0, 12.0, 0.0);
        let v2 = v1 / 2.0;
        assert_relative_eq!(v2.x, 5.0);
        assert_relative_eq!(v2.y, 6.0);
        assert_relative_eq!(v2.z, 0.0);
    }

    #[test]
    fn test_vector_neg() {
        let v1 = Vector::new(1.0, -2.0, 0.0);
        let v2 = -v1;
        assert_relative_eq!(v1.x, 1.0);
        assert_relative_eq!(v1.y, -2.0);
        assert_relative_eq!(v1.z, 0.0);
        assert_relative_eq!(v2.x, -1.0);
        assert_relative_eq!(v2.y, 2.0);
        assert_relative_eq!(v2.z, 0.0);
    }
}
