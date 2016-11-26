#![macro_use]

use std::ops::{Add, Sub, Mul, Neg};

#[derive(Clone,Copy)]
/// 3D type for vectors and points.
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    /// Possibly normalizes the vector.
    pub fn normalize(&mut self) -> Result<f32, ()> {
        let len = self.length();
        let inv_len = 1.0 / len;
        if len > 1e-6 {
            self.x = self.x * inv_len;
            self.y = self.y * inv_len;
            self.z = self.z * inv_len;
            return Ok(self.length());
        }
        return Err(());
    }
}


impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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

impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}


mod test {
    use super::Vector;
    use math::util::assert_approx_eq;

    #[test]
    fn test_vector_length() {
        assert_approx_eq(Vector::new(0.0, 0.0, 0.0).length(), 0.0);
        assert_approx_eq(Vector::new(1.0, 0.0, 0.0).length(), 1.0);
        assert_approx_eq(Vector::new(4.0, 0.0, 0.0).length(), 4.0);
        assert_approx_eq(Vector::new(0.0, 3.0, 4.0).length(), 5.0);
        assert_approx_eq(Vector::new(0.0, -3.0, -4.0).length(), 5.0);
    }

    #[test]
    fn test_vector_normalization() {
        let mut v1 = Vector::new(3.0, 4.0, 5.0);
        let new_length = v1.normalize().unwrap();
        assert_approx_eq(new_length, 1.0);
        assert_approx_eq(new_length, v1.length());

        let mut zero_vector = Vector::new(0.0, 0.0, 0.0);
        assert!(zero_vector.normalize().is_err());
    }

    #[test]
    fn test_vector_add() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_approx_eq(v3.x, 5.0);
        assert_approx_eq(v3.y, 7.0);
        assert_approx_eq(v3.z, 9.0);
    }

    #[test]
    fn test_vector_sub() {
        let v1 = Vector::new(4.0, 5.0, 6.0);
        let v2 = Vector::new(3.0, 2.0, 1.0);
        let v3 = v1 - v2;
        assert_approx_eq(v3.x, 1.0);
        assert_approx_eq(v3.y, 3.0);
        assert_approx_eq(v3.z, 5.0);
    }


    #[test]
    fn test_vector_mul() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = 7.0 * v1;
        assert_approx_eq(v2.x, 7.0);
        assert_approx_eq(v2.y, 14.0);
        assert_approx_eq(v2.z, 21.0);
    }

    #[test]
    fn test_vector_neg() {
        let v1 = Vector::new(1.0, -2.0, 0.0);
        let v2 = -v1;
        assert_approx_eq(v2.x, -1.0);
        assert_approx_eq(v2.y, 2.0);
        assert_approx_eq(v2.z, 0.0);
    }
}
