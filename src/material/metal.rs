use glam::DVec3;

use super::traits::Material;
use crate::hittable::Hit;
use crate::random::random_unit_vector;
use crate::ray::Ray;
use crate::vec::approx_zero;

pub struct Metal {
    albedo: DVec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(DVec3, Option<Ray>)> {
        let reflected = ray.direction.reflect(hit.normal);
        let mut direction = reflected + self.fuzz * random_unit_vector();

        if approx_zero(direction) {
            direction = reflected;
        }

        Some((self.albedo, Some(Ray::new(hit.point, direction))))
    }
}
