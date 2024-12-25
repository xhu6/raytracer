use glam::DVec3;

use crate::{
    hittable::Hit, material::traits::Material, random::random_unit_vector, ray::Ray,
    vec::approx_zero,
};

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
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)> {
        let mut direction = hit.normal + random_unit_vector();

        // Avoid problems with normalising later
        if approx_zero(direction) {
            direction = hit.normal;
        }

        Some((Ray::new(hit.point, direction), self.albedo))
    }
}
