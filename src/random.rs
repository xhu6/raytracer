use glam::{dvec3, DVec3};
use std::f64::consts::PI;

pub fn nothing() -> (f64, f64) {
    (0.0, 0.0)
}

pub fn random_square() -> (f64, f64) {
    (fastrand::f64() - 0.5, fastrand::f64() - 0.5)
}

pub fn random_unit_vector() -> DVec3 {
    // Randomly distribute along sphere surface
    let theta = fastrand::f64() * PI;
    let phi = fastrand::f64() * 2.0 * PI;
    dvec3(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    )
}

pub fn random_on_hemisphere(normal: &DVec3) -> DVec3 {
    let unit_vector = random_unit_vector();

    if normal.dot(unit_vector) > 0.0 {
        unit_vector
    } else {
        -unit_vector
    }
}
