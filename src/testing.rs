use glam::DVec3;

// Good enough
const ERROR: f64 = 1e-9;

pub fn assert_approx_eq_error(a: DVec3, b: DVec3, error: f64) {
    assert!((a - b).length() < error);
}

pub fn assert_approx_eq(a: DVec3, b: DVec3) {
    assert_approx_eq_error(a, b, ERROR);
}
