use std::sync::Arc;

use glam::DVec3;

use crate::material::Material;

pub struct Hit {
    pub point: DVec3,
    pub normal: DVec3,
    pub distance: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl Hit {
    pub fn new(
        point: DVec3,
        normal: DVec3,
        distance: f64,
        front_face: bool,
        material: Arc<dyn Material>,
    ) -> Self {
        Hit {
            point,
            normal,
            distance,
            front_face,
            material,
        }
    }
}
