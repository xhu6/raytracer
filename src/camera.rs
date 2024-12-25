use core::f64;
use glam::{dvec3, DVec3};
use image::{Rgb, RgbImage};

use crate::{
    hittable::{Hittable, HittableList},
    random::{random_on_disc, random_square},
    ray::Ray,
};

#[derive(Debug)]
pub struct Camera {
    position: DVec3,
    viewport_u: DVec3,
    viewport_v: DVec3,
    defocus_u: DVec3,
    defocus_v: DVec3,
    top_left: DVec3,
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

fn linear_to_gamma(data: f64) -> f64 {
    let gamma = 2.0;

    if data > 0.0 {
        data.powf(1.0 / gamma)
    } else {
        0.0
    }
}

// NOTE: Shouldn't be here but will keep for now
fn to_rgb(data: DVec3) -> Rgb<u8> {
    Rgb(data
        .to_array()
        .map(|x| (linear_to_gamma(x).clamp(0.0, 0.999) * 256.0).floor() as u8))
}

impl Camera {
    // aspect_ratio should match width and height
    // although some might want "stretched res"
    pub fn new(
        aspect_ratio: f64,
        vfov: f64,
        defocus_angle: f64,
        focal_length: f64,
        position: DVec3,
        forward: DVec3,
        up: DVec3,
        width: u32,
        height: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let viewport_height = (vfov.to_radians() / 2.0).tan() * 2.0 * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        // These must all be perpendicular to each other.
        // u: right, v: up, w: forward
        let w = forward.normalize();
        let u = w.cross(up).normalize();
        let v = u.cross(w).normalize();

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let top_left = position + w * focal_length - viewport_u / 2.0 - viewport_v / 2.0;

        let defocus_radius = focal_length * (defocus_angle / 2.0).to_radians().tan();
        let defocus_u = u * defocus_radius;
        let defocus_v = v * defocus_radius;

        Camera {
            position,
            viewport_u,
            viewport_v,
            defocus_u,
            defocus_v,
            top_left,
            width,
            height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn from(params: &CameraParams) -> Self {
        Camera::new(
            params.aspect_ratio,
            params.vfov,
            params.defocus_angle,
            params.focal_length,
            params.position,
            params.forward,
            params.up,
            params.width,
            params.height,
            params.samples_per_pixel,
            params.max_depth,
        )
    }

    pub fn sample_defocus_disk(&self) -> DVec3 {
        let scale = random_on_disc();
        self.position + scale.0 * self.defocus_u + scale.1 * self.defocus_v
    }

    // Values are between [0, 1]
    pub fn get_ray(&self, (u, v): (f64, f64)) -> Ray {
        let end = self.top_left + self.viewport_u * u + self.viewport_v * v;
        let start = self.sample_defocus_disk();
        Ray::new(start, end - start)
    }

    pub fn sample(&self, world: &HittableList, ray: &Ray, depth: u32) -> DVec3 {
        // No light after depth exceeded
        if depth <= 0 {
            return DVec3::ZERO;
        }

        if let Some(hit) = world.hit(ray, 1e-9, f64::MAX) {
            if let Some((new_ray, attenuation)) = hit.material.scatter(ray, &hit) {
                return attenuation * self.sample(world, &new_ray, depth - 1);
            }

            return DVec3::ZERO;
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
        RgbImage::from_fn(self.width, self.height, |x, y| {
            self.render_pixel(world, x, y)
        })
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::from(&CameraParams::default())
    }
}

pub struct CameraParams {
    pub aspect_ratio: f64,
    pub vfov: f64,
    pub defocus_angle: f64,
    pub focal_length: f64,
    pub position: DVec3,
    pub forward: DVec3,
    pub up: DVec3,
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            vfov: 90.0,
            defocus_angle: 0.0,
            focal_length: 1.0,
            position: DVec3::ZERO,
            forward: dvec3(0.0, 0.0, 1.0),
            up: dvec3(0.0, 1.0, 0.0),
            width: 1024,
            height: 1024,
            samples_per_pixel: 16,
            max_depth: 64,
        }
    }
}

impl CameraParams {
    pub fn to_camera(&self) -> Camera {
        Camera::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
