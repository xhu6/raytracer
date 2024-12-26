use crate::{hittable::Hit, ray::Ray};

use glam::DVec3;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)>;
}
