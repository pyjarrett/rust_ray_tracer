use std::fmt;

/// Provides a convenient mechanism to refer to values in `Point`s and `Vector`s by index without
/// resorting to arbitrary numeric indices (e.g. 0, 1, 2).
#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub const XYZ: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Axis::X => write!(f, "X"),
            Axis::Y => write!(f, "Y"),
            Axis::Z => write!(f, "Z"),
        }
    }
}
