use glam::{dvec3, DVec3};
use image::{Rgb, RgbImage};

use crate::{
    hittable::{Hittable, HittableList},
    ray::Ray,
};

#[derive(Debug)]
pub struct Camera {
    position: DVec3,
    viewport_u: DVec3,
    viewport_v: DVec3,
    top_left: DVec3,
}

// NOTE: Shouldn't be here but will keep for now
fn to_rgb(data: DVec3) -> Rgb<u8> {
    Rgb(data
        .to_array()
        .map(|x| (x.clamp(0.0, 0.999) * 256.0).floor() as u8))
}

impl Camera {
    pub fn new(focal_length: f64, aspect_ratio: f64, vfov: f64, position: DVec3) -> Self {
        let viewport_height = (vfov.to_radians() / 2.0).tan() * 2.0;
        let viewport_width = viewport_height * aspect_ratio;

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
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let end = self.top_left + self.viewport_u * u + self.viewport_v * v;
        Ray::new(self.position, end - self.position)
    }

    pub fn get_pixel(&self, world: &HittableList, ray: &Ray) -> Rgb<u8> {
        if let Some(hit) = world.hit(&ray, 0.0, f64::MAX) {
            return to_rgb(hit.normal.map(|x| 0.5 * (x + 1.0)));
        }

        let a = 0.5 * (ray.direction.y + 1.0);
        let result = (1.0 - a) * DVec3::ONE + a * dvec3(0.5, 0.7, 1.0);
        to_rgb(result)
    }

    pub fn render(&self, world: &HittableList, width: u32, height: u32) -> RgbImage {
        RgbImage::from_fn(width, height, |x, y| {
            let ray = self.get_ray(
                (x as f64 + 0.5) / width as f64,
                (y as f64 + 0.5) / height as f64,
            );
            self.get_pixel(world, &ray)
        })
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(1.0, 1.0, 1.0, DVec3::ZERO)
    }
}
