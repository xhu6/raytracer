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

    pub fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<f64> {
        let tmp = self.position - ray.origin;

        // The quadratic equation but 2 is factored out
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(tmp);
        let c = tmp.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        // No solutions
        if discriminant < 0.0 {
            return None;
        }

        // 1st root will **always** be smaller
        let sqrtd = discriminant.sqrt();
        let roots = [(h - sqrtd) / a, (h + sqrtd) / a];

        roots
            .iter()
            .filter(|x| (min..max).contains(x))
            .next()
            .copied()
    }
}
