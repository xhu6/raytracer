use glam::DVec3;

use crate::ray::Ray;

pub struct Sphere {
    position: DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(position: DVec3, radius: f64) -> Self {
        Sphere { position, radius }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let tmp = self.position - ray.origin;

        let a = ray.direction.length_squared();
        let b = -2.0 * ray.direction.dot(tmp);
        let c = tmp.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}
