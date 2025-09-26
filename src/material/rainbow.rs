use glam::DVec3;

use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::Hit;

pub struct Rainbow {}

impl Rainbow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Rainbow {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(DVec3, Option<Ray>)> {
        Some((hit.point.map(|x| x.sin()).normalize(), None))
    }
}
