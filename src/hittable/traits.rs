use super::hit::Hit;

use crate::ray::Ray;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit>;
}
