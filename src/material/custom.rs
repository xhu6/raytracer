use super::traits::Material;

use crate::{hittable::Hit, ray::Ray};

use glam::{dvec3, DVec3};
use num_complex::Complex64;

pub struct Custom {}

impl Custom {
    pub fn new() -> Self {
        Self {}
    }
}

fn fractal(point: DVec3) -> DVec3 {
    let mut z = Complex64::new(point.x, point.z);
    let a = Complex64::new(1.0, 1.0);

    for _ in 0..32 {
        z -= (z * z - 1.0) / (2.0 * z) * a;
    }

    dvec3(z.re.abs(), z.im.abs(), z.im)
}

impl Material for Custom {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)> {
        Some((
            Ray::new(hit.point, ray.direction.reflect(hit.normal)),
            fractal(hit.point),
        ))
    }
}
