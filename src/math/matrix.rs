extern crate approx;

use approx::ApproxEq;
use math::{Point, Ray, Vector};
use std::f32;
use std::fmt;
use std::ops::Mul;

/// A row-major, 4x4 Matrix for use with homogeneous coordinate transforms.
///
/// # Remarks
/// Using a row-major memory ordering of matrix elements allows convenient indexing of the matrix
/// using `m[row][col]`.  However, matrix element memory ordering is independent of the notation to
/// specify multiplications.  Note that the convention maintained is to treat vectors and points as
/// columns, not rows so we use the post-multiply convention `M * v` rather than the pre-multiply
/// convention `v * M`.
///
/// The columns of this matrix show how the three basis vectors and the origin point for the
/// current coordinate system will be transformed.
///
#[derive(Copy, Clone)]
pub struct Matrix4x4 {
    m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Generates a possibly non-uniform scale.
    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Generates a perspective transform, in a coordinate system with X+ going to the right,
    /// Y+ going up, and Z+ going into the screen.
    ///
    /// Along the X and Y axes, the inverse tangent of the half the field of view (FOV) gives the
    /// ratio of Z per unit in that direction.  The shorter dimension will scale to [-1, 1], and
    /// the large dimension according to the aspect ratio.  The smaller side of either X or Y will
    /// be the side with the given FOV.  Note this is slightly different than the typical realtime
    /// graphics perspective transform which uses the appropriate FOV for each X and Y axis based
    /// on the desired frustum size from the frustum top->bottom and left->right sizes.
    ///
    /// The lower half of the matrix transforms the range of Z values from [near, far]
    /// to [0, 1].
    ///
    /// # Arguments
    /// * `near` - Z value of the near plane
    /// * `far` - Z value of far plane
    /// * `fov_degrees` - field of view in degrees
    ///
    /// # Preconditions
    /// * `0 <= near < far`
    /// # `0 < fov_degrees < 180 degrees`
    pub fn perspective(near: f32, far: f32, fov_degrees: f32) -> Matrix4x4 {
        assert!(
            0.0 <= near,
            "The distance to the near plane cannot be negative."
        );
        assert!(near < far, "The near plane must be behind the far plane.");
        let inv_tan_half_fov = 1.0 / ((fov_degrees.to_radians() / 2.0).tan());
        assert!(
            inv_tan_half_fov > 0.0,
            "Invalid field of view: {} degrees",
            fov_degrees
        );
        Matrix4x4 {
            m: [
                [inv_tan_half_fov, 0.0, 0.0, 0.0],
                [0.0, inv_tan_half_fov, 0.0, 0.0],
                [0.0, 0.0, far / (far - near), -(far * near) / (far - near)],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }
    }

    /// Use Gauss-Jordan elimination to find the matrix inverse.
    /// After elimination is complete, reduce the left side to an identity matrix to get the result.
    pub fn inverse(&self) -> Option<Matrix4x4> {
        let mut inv: Matrix4x4 = self.clone();

        // Create an augmented matrix with identity matrix on the right hand side.
        let mut aug: [[f32; 8]; 4] = [[0.0; 8]; 4];
        for i in 0..4 {
            for j in 0..4 {
                aug[i][j] = inv.m[i][j];
            }
        }
        aug[0][4] = 1.0;
        aug[1][5] = 1.0;
        aug[2][6] = 1.0;
        aug[3][7] = 1.0;

        for k in 0..4 {
            // Find the row with the maximum first element.
            let mut i_max = k;
            let mut max_val = aug[i_max][k].abs();
            for i in k..4 {
                if max_val < aug[i][k].abs() {
                    max_val = aug[i][k].abs();
                    i_max = i;
                }
            }

            // Guard against singular matrices
            if aug[i_max][k] == 0.0 {
                return None;
            }

            // Swap rows k and i_max
            if i_max != k {
                for idx in 0..8 {
                    let tmp = aug[i_max][idx];
                    aug[i_max][idx] = aug[k][idx];
                    aug[k][idx] = tmp
                }
            }

            // Normalize the row.
            let normalizer = aug[k][k];
            for idx in 0..8 {
                aug[k][idx] = aug[k][idx] / normalizer;
            }

            // Reduce all elements of following rows.
            for i in (k + 1)..4 {
                let f = aug[i][k];
                for j in (k + 1)..8 {
                    aug[i][j] = aug[i][j] - aug[k][j] * f;
                }
                // Fill lower triangle with zeros.
                aug[i][k] = 0.0;
            }
        }

        // Back substitute
        for i in 1..4 {
            for j in 0..i {
                if aug[j][i] != 0.0 {
                    // substitute to reduce the row
                    let f = aug[j][i] / aug[i][i];
                    for k in i..8 {
                        aug[j][k] = aug[j][k] - aug[i][k] * f;
                    }
                }
            }
        }

        // Copy results out.
        for i in 0..4 {
            for j in 0..4 {
                inv.m[i][j] = aug[i][j + 4];
            }
        }
        Some(inv)
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut n: [[f32; 4]; 4] = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                n[i][j] = self.m[j][i];
            }
        }
        Matrix4x4 { m: n }
    }
}

impl fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "\n┏ {:35} ┓\n", "");
        if res.is_err() {
            return res;
        }
        for row in 0..4 {
            res =
                write!(
                f,
                "┃ {:>8.4} {:>8.4} {:>8.4} {:>8.4} ┃\n",
                self.m[row][0],
                self.m[row][1],
                self.m[row][2],
                self.m[row][3],
            );
            if res.is_err() {
                return res;
            }
        }
        res = write!(f, "┗ {:35} ┛\n", "");
        res
    }
}

impl ApproxEq for Matrix4x4 {
    type Epsilon = <f32 as ApproxEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    // ulps = Units in Last Place
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !f32::relative_eq(&self.m[i][j], &other.m[i][j], epsilon, max_relative) {
                    return false;
                }
            }
        }
        true
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !f32::ulps_eq(&self.m[i][j], &other.m[i][j], epsilon, max_ulps) {
                    return false;
                }
            }
        }
        true
    }
}


impl Mul for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        &self * &rhs
    }
}


impl<'a, 'b> Mul<&'a Matrix4x4> for &'b Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Self::Output {
        Matrix4x4 {
            m: [
                // first row.
                [
                    self.m[0][0] * rhs.m[0][0] + self.m[0][1] * rhs.m[1][0] +
                        self.m[0][2] * rhs.m[2][0] + self.m[0][3] * rhs.m[3][0],
                    self.m[0][0] * rhs.m[0][1] + self.m[0][1] * rhs.m[1][1] +
                        self.m[0][2] * rhs.m[2][1] + self.m[0][3] * rhs.m[3][1],
                    self.m[0][0] * rhs.m[0][2] + self.m[0][1] * rhs.m[1][2] +
                        self.m[0][2] * rhs.m[2][2] + self.m[0][3] * rhs.m[3][2],
                    self.m[0][0] * rhs.m[0][3] + self.m[0][1] * rhs.m[1][3] +
                        self.m[0][2] * rhs.m[2][3] + self.m[0][3] * rhs.m[3][3],
                ],

                // second row
                [
                    self.m[1][0] * rhs.m[0][0] + self.m[1][1] * rhs.m[1][0] +
                        self.m[1][2] * rhs.m[2][0] + self.m[1][3] * rhs.m[3][0],
                    self.m[1][0] * rhs.m[0][1] + self.m[1][1] * rhs.m[1][1] +
                        self.m[1][2] * rhs.m[2][1] + self.m[1][3] * rhs.m[3][1],
                    self.m[1][0] * rhs.m[0][2] + self.m[1][1] * rhs.m[1][2] +
                        self.m[1][2] * rhs.m[2][2] + self.m[1][3] * rhs.m[3][2],
                    self.m[1][0] * rhs.m[0][3] + self.m[1][1] * rhs.m[1][3] +
                        self.m[1][2] * rhs.m[2][3] + self.m[1][3] * rhs.m[3][3],
                ],

                // etc...
                [
                    self.m[2][0] * rhs.m[0][0] + self.m[2][1] * rhs.m[1][0] +
                        self.m[2][2] * rhs.m[2][0] + self.m[2][3] * rhs.m[3][0],
                    self.m[2][0] * rhs.m[0][1] + self.m[2][1] * rhs.m[1][1] +
                        self.m[2][2] * rhs.m[2][1] + self.m[2][3] * rhs.m[3][1],
                    self.m[2][0] * rhs.m[0][2] + self.m[2][1] * rhs.m[1][2] +
                        self.m[2][2] * rhs.m[2][2] + self.m[2][3] * rhs.m[3][2],
                    self.m[2][0] * rhs.m[0][3] + self.m[2][1] * rhs.m[1][3] +
                        self.m[2][2] * rhs.m[2][3] + self.m[2][3] * rhs.m[3][3],
                ],

                [
                    self.m[3][0] * rhs.m[0][0] + self.m[3][1] * rhs.m[1][0] +
                        self.m[3][2] * rhs.m[2][0] + self.m[3][3] * rhs.m[3][0],
                    self.m[3][0] * rhs.m[0][1] + self.m[3][1] * rhs.m[1][1] +
                        self.m[3][2] * rhs.m[2][1] + self.m[3][3] * rhs.m[3][1],
                    self.m[3][0] * rhs.m[0][2] + self.m[3][1] * rhs.m[1][2] +
                        self.m[3][2] * rhs.m[2][2] + self.m[3][3] * rhs.m[3][2],
                    self.m[3][0] * rhs.m[0][3] + self.m[3][1] * rhs.m[1][3] +
                        self.m[3][2] * rhs.m[2][3] + self.m[3][3] * rhs.m[3][3],
                ],
            ],
        }
    }
}

impl Mul<Point> for Matrix4x4 {
    type Output = Point;
    fn mul(self, p: Point) -> Self::Output {
        let r = Point::new(
            self.m[0][0] * p.x + self.m[0][1] * p.y + self.m[0][2] * p.z + self.m[0][3] * 1.0,
            self.m[1][0] * p.x + self.m[1][1] * p.y + self.m[1][2] * p.z + self.m[1][3] * 1.0,
            self.m[2][0] * p.x + self.m[2][1] * p.y + self.m[2][2] * p.z + self.m[2][3] * 1.0,
        );
        let w = self.m[3][0] * p.x + self.m[3][1] * p.y + self.m[3][2] * p.z + self.m[3][3];
        1.0 / w * r
    }
}

impl Mul<Vector> for Matrix4x4 {
    type Output = Vector;
    fn mul(self, p: Vector) -> Self::Output {
        Vector::new(
            self.m[0][0] * p.x + self.m[0][1] * p.y + self.m[0][2] * p.z,
            self.m[1][0] * p.x + self.m[1][1] * p.y + self.m[1][2] * p.z,
            self.m[2][0] * p.x + self.m[2][1] * p.y + self.m[2][2] * p.z,
        )
    }
}

impl Mul<Ray> for Matrix4x4 {
    type Output = Ray;
    fn mul(self, r: Ray) -> Self::Output {
        let mut new_direction = self * r.direction;
        new_direction.normalize().unwrap();

        Ray {
            origin: self * r.origin,
            direction: new_direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix4x4;
    use math::Point;

    #[test]
    pub fn test_identity() {
        let p = Point::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::identity();
        let m_inv = m.inverse().unwrap();
        let same_point = m * p;
        assert_relative_eq!(same_point.x, p.x);
        assert_relative_eq!(same_point.y, p.y);
        assert_relative_eq!(same_point.z, p.z);
        assert_relative_eq!(m_inv, m);
        assert_relative_eq!(m_inv * m, m * m_inv);
    }

    #[test]
    pub fn test_translate_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::translate(7.0, 8.0, 9.0);
        let translated = m * p;
        assert_relative_eq!(translated.x, 8.0);
        assert_relative_eq!(translated.y, 10.0);
        assert_relative_eq!(translated.z, 12.0);
    }

    #[test]
    pub fn test_inverse_translate() {
        let p = Point::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::translate(7.0, 8.0, 9.0);
        let m_inv = m.inverse().unwrap();
        let translated = m_inv * m * p;
        assert_relative_eq!(translated.x, p.x);
        assert_relative_eq!(translated.y, p.y);
        assert_relative_eq!(translated.z, p.z);
    }

    #[test]
    #[should_panic]
    pub fn test_perspective_near_closer_than_far() {
        Matrix4x4::perspective(100.0, 10.0, 45.0_f32);
    }

    #[test]
    #[should_panic]
    pub fn test_perspective_near_greater_than_0() {
        Matrix4x4::perspective(-100.0, 10.0, 45.0_f32);
    }

    #[test]
    #[should_panic]
    pub fn test_perspective_fov_less_than_180() {
        Matrix4x4::perspective(0.0, 100.0, 190.0_f32);
    }

    #[test]
    pub fn test_perspective_frustrum_points() {
        let near = 10.0;
        let far = 100.0;
        let p = Matrix4x4::perspective(near, far, 60.0_f32);

        let center_near = Point::new(0.0, 0.0, near);
        let center_far = Point::new(0.0, 0.0, far);

        assert_relative_eq!((p * center_near).z, 0.0);
        assert_relative_eq!((p * center_far).z, 1.0);
    }

    #[test]
    pub fn test_scale() {
        let p = Point::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::scale(2.5, 4.0, 8.0);
        let scaled = m * p;
        assert_relative_eq!(scaled.x, 2.5);
        assert_relative_eq!(scaled.y, 8.0);
        assert_relative_eq!(scaled.z, 24.0);
    }

    #[test]
    pub fn test_inverse() {
        let m = Matrix4x4::scale(2.5, 4.0, 8.0) * Matrix4x4::translate(1.0, 3.0, 5.0);
        let m_inv = m.inverse().unwrap();
        assert_relative_eq!(m * m_inv, Matrix4x4::identity());
        assert_relative_eq!(m_inv * m, Matrix4x4::identity());

        let zero_matrix = Matrix4x4 { m: [[0.0; 4]; 4] };
        assert!(zero_matrix.inverse().is_none());
    }
}
