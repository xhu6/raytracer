use glam::DVec3;

use crate::ray::Ray;

pub struct Hit {
    pub point: DVec3,
    pub normal: DVec3,
    pub distance: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit>;
}
