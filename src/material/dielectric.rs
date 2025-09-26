use glam::DVec3;

use super::traits::Material;
use crate::hittable::Hit;
use crate::ray::Ray;

fn reflectance(cos_theta: f64, eta: f64) -> f64 {
    let tmp = (1.0 - eta) / (1.0 + eta);
    let r_0 = tmp * tmp;
    r_0 + (1.0 - r_0) * (1.0 - cos_theta).powi(5)
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(DVec3, Option<Ray>)> {
        // Assume surrounding medium is 1.0 (air)
        // eta is the ratio of the refractive indexes
        let eta = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        // Set upper limit to 1.0 due to rounding errors
        let cos_theta = (-ray.direction.dot(hit.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if eta * sin_theta > 1.0 || reflectance(cos_theta, eta) > fastrand::f64() {
            ray.direction.reflect(hit.normal)
        } else {
            ray.direction.refract(hit.normal, eta)
        };

        Some((DVec3::ONE, Some(Ray::new(hit.point, direction))))
    }
}
