// Borrowed from old rust source.
// Won't export correctly for some reason.
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!((*a - *b).abs() < 1.0e-6,
                "{} is not approximately equal to {}", *a, *b);
    })
}

pub fn assert_approx_eq(a: f32, b: f32) {
    assert!((a - b).abs() < 1.0e-6,
            "{} is not approximately equal to {}",
            a,
            b);
}
