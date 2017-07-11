use std::fmt;
use std::cmp::Eq;
use std::ops::Mul;
use math::Point;

/// A row-major, 4x4 Matrix for use with homogeneous coordinates.
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

    pub fn perspective(near: f32, far: f32, fov_radians: f32) -> Matrix4x4 {
        let inv_tan_half_fov = 1.0 / ((fov_radians / 2.0).tan());
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
}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "\n");
        for row in 0..4 {
            res = write!(
                f,
                "{:>8.4} {:>8.4} {:>8.4} {:>8.4}\n",
                self.m[row][0],
                self.m[row][1],
                self.m[row][2],
                self.m[row][3]
            );
            if res.is_err() {
                return res;
            }
        }
        res
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Matrix4x4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if (self.m[i][j] - other.m[i][j]).abs() > 1e-6 {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Matrix4x4 {}

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

// TODO: Perform the w divide.
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
        assert!(m_inv == m);
        assert!(m_inv * m == m * m_inv);
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
        assert!(m * m_inv == Matrix4x4::identity());
        assert!(m_inv * m == Matrix4x4::identity());

        let zero_matrix = Matrix4x4 { m: [[0.0; 4]; 4] };
        assert!(zero_matrix.inverse().is_none());
    }
}
