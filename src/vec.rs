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

pub fn approx_eq(a: DVec3, b: DVec3) -> bool {
    approx_eq_with_error(a, b, ERROR)
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::f64;
    use glam::dvec3;
    use std::f64::consts::PI;

    #[test]
    fn zero_approx_zero() {
        assert!(approx_zero(DVec3::ZERO));
    }

    #[test]
    fn almost_zero_approx_zero() {
        assert!(approx_zero(dvec3(f64::EPSILON, 0.0, 0.0)));
    }

    #[test]
    fn zero_approx_eq_zero() {
        assert!(approx_eq(DVec3::ZERO, DVec3::ZERO));
    }

    #[test]
    fn almost_zero_approx_eq_zero() {
        assert!(approx_eq(dvec3(f64::EPSILON, 0.0, 0.0), DVec3::ZERO));
    }

    #[test]
    fn numbers_approx_eq() {
        let a = dvec3(0.1 + 0.2, 0.0, 0.0);
        let b = dvec3(0.3, 0.0, 0.0);

        // To show a approx b but a != b
        assert!(a != b);
        assert!(approx_eq(a, b));
    }

    // Should return false
    #[test]
    fn one_not_approx_zero() {
        assert!(!approx_zero(DVec3::ONE));
    }

    #[test]
    fn inf_not_approx_zero() {
        assert!(!approx_zero(DVec3::INFINITY));
    }

    #[test]
    fn nan_not_approx_zero() {
        assert!(!approx_zero(DVec3::NAN));
    }

    #[test]
    fn small_not_approx_zero() {
        assert!(!approx_zero(dvec3(0.001, 0.0, 0.0)));
    }

    #[test]
    fn one_not_approx_eq_zero() {
        assert!(!approx_eq(DVec3::ONE, DVec3::ZERO));
    }

    #[test]
    fn pi_not_approx_eq_frac() {
        let a = DVec3::splat(PI);
        let b = DVec3::splat(22.0 / 7.0);
        assert!(!approx_eq(a, b));
    }
}
