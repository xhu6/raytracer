use glam::DVec3;

// Good enough
const ERROR: f64 = 1e-9;

pub fn approx_zero_with_error(a: DVec3, error: f64) -> bool {
    a.length() < error
}

pub fn approx_eq_with_error(a: DVec3, b: DVec3, error: f64) -> bool {
    approx_zero_with_error(a - b, error)
}

pub fn approx_zero(a: DVec3) -> bool {
    approx_zero_with_error(a, ERROR)
}

pub fn approx_eq(a:DVec3, b: DVec3) -> bool {
    approx_eq_with_error(a, b, ERROR)
}
