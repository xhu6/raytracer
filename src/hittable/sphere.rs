use super::hit::Hit;
use super::traits::Hittable;

use crate::{material::Material, ray::Ray};

use glam::DVec3;
use std::sync::Arc;

pub struct Sphere {
    position: DVec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(position: DVec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            position,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit> {
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
            .map(|x| {
                let point = ray.at(x);
                let outward_normal = (point - self.position).normalize();
                let front_face = outward_normal.dot(ray.direction) < 0.0;
                Hit {
                    point,
                    distance: x,
                    normal: if front_face {
                        outward_normal
                    } else {
                        -outward_normal
                    },
                    front_face,
                    material: self.material.clone(),
                }
            })
    }
}
