use super::hit::Hit;
use super::traits::Hittable;

use crate::ray::Ray;

use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn from(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Arc::new(object));
    }

    pub fn add_shared(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit> {
        let mut closest = max;
        let mut out = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, min, closest) {
                closest = hit.distance;
                out = Some(hit);
            }
        }

        out
    }
}
