//! Does A equal B?
//! 
//! Module dealing with precision and accuracy.
//!
//! Provides functions dealing with closeness of solutions to "true" 
//! solutions, especially when dealing with floating point and 
//! scales of several magnitudes.

pub fn assert_approx_eq(a: f32, b: f32) {
    assert!((a - b).abs() < 1e-6,
            "{} is not approximately equal to {}",
            a,
            b);
}

pub fn assert_eq_eps(a: f32, b: f32, eps: f32) {
    assert!((a - b).abs() < eps,
            "{} is not approximately equal to {}",
            a,
            b);
}
