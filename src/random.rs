use std::f64::consts::PI;

use glam::{dvec3, DVec3};

pub fn nothing() -> (f64, f64) {
    (0.0, 0.0)
}

pub fn random_square() -> (f64, f64) {
    // Random between [-0.5, 0.5]
    (fastrand::f64() - 0.5, fastrand::f64() - 0.5)
}

pub fn random_on_disc() -> (f64, f64) {
    // Randomly uniformly over unit disc
    let theta = fastrand::f64() * 2.0 * PI;
    let radius = fastrand::f64().sqrt();

    (radius * theta.sin(), radius * theta.cos())
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

pub fn random_colour() -> DVec3 {
    DVec3::new(fastrand::f64(), fastrand::f64(), fastrand::f64())
}
