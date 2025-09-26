use glam::DVec3;

use crate::hittable::Hit;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Default)]
pub struct Rainbow {}

impl Rainbow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Rainbow {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(DVec3, Option<Ray>)> {
        Some((hit.point.map(f64::sin).normalize(), None))
    }
}
