use glam::DVec3;

use crate::{hittable::Hit, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)>;
}
