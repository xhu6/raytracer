use glam::DVec3;

use super::traits::Material;
use crate::hittable::Hit;
use crate::random::random_unit_vector;
use crate::ray::Ray;
use crate::vec::approx_zero;

pub struct Lambertian {
    albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    // Doesn't use input ray
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(DVec3, Option<Ray>)> {
        let mut direction = hit.normal + random_unit_vector();

        // Avoid problems with normalising later
        if approx_zero(direction) {
            direction = hit.normal;
        }

        Some((self.albedo, Some(Ray::new(hit.point, direction))))
    }
}
