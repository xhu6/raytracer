use crate::material::Material;

use glam::DVec3;
use std::sync::Arc;

pub struct Hit {
    pub point: DVec3,
    pub normal: DVec3,
    pub distance: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}
