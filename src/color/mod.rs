#![allow(dead_code)]

use math::Vector;

/// Converts a float value in range [-1, 1] to [0, 255].
pub fn float_to_hue(f: f32) -> u8 {
    assert!(f >= -1.0);
    assert!(f <= 1.0);
    (((f + 1.0) / 2.0) * 255.0) as u8
}

/// Converts a normal to an array to assign as a color.
pub fn unit_vector_as_color(v: Vector) -> [u8; 3] {
    [float_to_hue(v.x), float_to_hue(v.y), float_to_hue(v.z)]
}

