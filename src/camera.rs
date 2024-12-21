use glam::{dvec3, DVec3};

use crate::ray::Ray;

pub struct Camera {
    position: DVec3,
    viewport_u: DVec3,
    viewport_v: DVec3,
    top_left: DVec3,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        viewport_height: f64,
        viewport_width: f64,
        position: DVec3,
    ) -> Self {
        // These must all be perpendicular to each other.
        // u: right
        // v: down
        let viewport_u = dvec3(viewport_width, 0.0, 0.0);
        let viewport_v = dvec3(0.0, -viewport_height, 0.0);
        let forward = dvec3(0.0, 0.0, -1.0);

        let top_left = position + forward * focal_length - viewport_u / 2.0 - viewport_v / 2.0;

        Camera {
            position,
            viewport_u,
            viewport_v,
            top_left,
        }
    }

    // x and y are in [0, 1]
    pub fn sample(&self, u: f64, v: f64) -> Ray {
        let end = self.top_left + self.viewport_u * u + self.viewport_v * v;
        Ray::new(self.position, end - self.position)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(1.0, 1.0, 1.0, DVec3::ZERO)
    }
}
