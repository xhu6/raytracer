use glam::DVec3;

use crate::hittable::Hit;
use crate::ray::Ray;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(DVec3, Option<Ray>)>;
}
