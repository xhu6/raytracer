use glam::DVec3;

use crate::{hittable::Hit, random::random_unit_vector, ray::Ray, vec::approx_zero};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)>;
}

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
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)> {
        let reflected = ray.direction.reflect(hit.normal);
        let mut direction = reflected + self.fuzz * random_unit_vector();

        if approx_zero(direction) {
            direction = reflected;
        }

        Some((Ray::new(hit.point, direction), self.albedo))
    }
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
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)> {
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

        let direction = if eta * sin_theta > 1.0 {
            ray.direction.reflect(hit.normal)
        } else {
            ray.direction.refract(hit.normal, eta)
        };

        Some((Ray::new(hit.point, direction), DVec3::ONE))
    }
}
