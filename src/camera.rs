use core::f64;
use fastrand;
use glam::{dvec3, DVec3};
use image::{Rgb, RgbImage};
use std::f64::consts::PI;

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
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

// NOTE: Shouldn't be here but will keep for now
fn to_rgb(data: DVec3) -> Rgb<u8> {
    Rgb(data
        .to_array()
        .map(|x| (x.clamp(0.0, 0.999) * 256.0).floor() as u8))
}

fn nothing() -> (f64, f64) {
    (0.0, 0.0)
}

fn random_square() -> (f64, f64) {
    (fastrand::f64() - 0.5, fastrand::f64() - 0.5)
}

fn random_unit_vector() -> DVec3 {
    // Randomly distribute along sphere surface
    let theta = fastrand::f64() * PI;
    let phi = fastrand::f64() * 2.0 * PI;
    dvec3(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    )
}

fn random_on_hemisphere(normal: &DVec3) -> DVec3 {
    let unit_vector = random_unit_vector();

    if normal.dot(unit_vector) > 0.0 {
        unit_vector
    } else {
        -unit_vector
    }
}

impl Camera {
    // aspect_ratio should match width and height
    // although some might want "stretched res"
    pub fn new(
        focal_length: f64,
        aspect_ratio: f64,
        vfov: f64,
        position: DVec3,
        width: u32,
        height: u32,
    ) -> Self {
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
            width,
            height,
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }

    // Values are between [0, 1]
    pub fn get_ray(&self, (u, v): (f64, f64)) -> Ray {
        let end = self.top_left + self.viewport_u * u + self.viewport_v * v;
        Ray::new(self.position, end - self.position)
    }

    pub fn sample(&self, world: &HittableList, ray: &Ray, depth: u32) -> DVec3 {
        // No light after depth exceeded
        if depth <= 0 {
            return DVec3::ZERO;
        }

        if let Some(hit) = world.hit(&ray, 1e-9, f64::MAX) {
            let direction = hit.normal + random_unit_vector();
            return 0.5 * self.sample(world, &Ray::new(hit.point, direction), depth - 1);
        }

        // Background
        let a = 0.5 * (ray.direction.y + 1.0);
        DVec3::ONE.lerp(dvec3(0.5, 0.7, 1.0), a)
    }

    pub fn get_uv(&self, x: u32, y: u32) -> (f64, f64) {
        let (dx, dy) = random_square();
        (
            (x as f64 + 0.5 + dx) / self.width as f64,
            (y as f64 + 0.5 + dy) / self.height as f64,
        )
    }

    pub fn render_pixel(&self, world: &HittableList, x: u32, y: u32) -> Rgb<u8> {
        let mut out = DVec3::ZERO;

        for _ in 0..self.samples_per_pixel {
            let uv = self.get_uv(x, y);
            let ray = self.get_ray(uv);
            out += self.sample(world, &ray, self.max_depth);
        }

        to_rgb(out / self.samples_per_pixel as f64)
    }

    pub fn render(&self, world: &HittableList) -> RgbImage {
        // Make RNG deterministic
        fastrand::seed(0);

        RgbImage::from_fn(self.width, self.height, |x, y| {
            self.render_pixel(world, x, y)
        })
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(1.0, 1.0, 90.0, DVec3::ZERO, 1920, 1080)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_random() {
        for _ in 0..100 {
            println!("{}", random_unit_vector().length());
        }
    }
}
