use super::traits::Material;

use crate::{hittable::Hit, random::random_unit_vector, ray::Ray, vec::approx_zero};

use glam::DVec3;

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
