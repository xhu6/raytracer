use core::f64;

use glam::{dvec3, DVec3};
use image::{Rgb, RgbImage};

use crate::hittable::{Hittable, HittableList};
use crate::random::{random_on_disc, random_square};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    position: DVec3,
    viewport_u: DVec3,
    viewport_v: DVec3,
    defocus_u: DVec3,
    defocus_v: DVec3,
    top_left: DVec3,

    // Image dimensions
    width: u32,
    height: u32,

    // Anti-aliasing
    samples_per_pixel: u32,

    // Ray bounce limit
    max_depth: u32,

    // Gamma
    gamma: f64,
}

fn linear_to_gamma(data: f64, gamma: f64) -> f64 {
    if data > 0.0 {
        data.powf(1.0 / gamma)
    } else {
        0.0
    }
}

fn to_rgb(data: DVec3, gamma: f64) -> Rgb<u8> {
    Rgb(data
        .to_array()
        .map(|x| (linear_to_gamma(x, gamma).clamp(0.0, 0.999) * 256.0).floor() as u8))
}

fn sample(world: &HittableList, ray: &Ray, depth: u32) -> DVec3 {
    let ambient = DVec3::ZERO;

    // No light after depth exceeded
    if depth == 0 {
        return ambient;
    }

    // Avoid intersecting same object by using a small value
    if let Some(hit) = world.hit(ray, 1e-9, f64::MAX) {
        if let Some((attenuation, potential_ray)) = hit.material.scatter(ray, &hit) {
            if let Some(new_ray) = potential_ray {
                return attenuation * sample(world, &new_ray, depth - 1);
            }

            return attenuation;
        }

        return ambient;
    }

    // Background
    let a = 0.5 * (ray.direction.y + 1.0);
    DVec3::ONE.lerp(dvec3(0.5, 0.7, 1.0), a)
    // DVec3::ZERO
}

impl Camera {
    pub fn new(
        &CameraParams {
            vfov,
            defocus_angle,
            focal_length,
            position,
            forward,
            up,
            width,
            height,
            samples_per_pixel,
            max_depth,
            gamma,
        }: &CameraParams,
    ) -> Self {
        let aspect_ratio = width as f64 / height as f64;
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
            gamma,
        }
    }

    fn sample_defocus_disk(&self) -> DVec3 {
        let scale = random_on_disc();
        self.position + scale.0 * self.defocus_u + scale.1 * self.defocus_v
    }

    // Values are between [0, 1]
    fn get_ray(&self, (u, v): (f64, f64)) -> Ray {
        let end = self.top_left + self.viewport_u * u + self.viewport_v * v;
        let start = self.sample_defocus_disk();
        Ray::new(start, end - start)
    }

    fn get_uv(&self, x: u32, y: u32) -> (f64, f64) {
        let (dx, dy) = random_square();
        (
            (x as f64 + 0.5 + dx) / self.width as f64,
            (y as f64 + 0.5 + dy) / self.height as f64,
        )
    }

    fn render_pixel(&self, world: &HittableList, x: u32, y: u32) -> Rgb<u8> {
        let mut out = DVec3::ZERO;

        for _ in 0..self.samples_per_pixel {
            let uv = self.get_uv(x, y);
            let ray = self.get_ray(uv);
            out += sample(world, &ray, self.max_depth);
        }

        to_rgb(out / self.samples_per_pixel as f64, self.gamma)
    }

    pub fn render(&self, world: &HittableList) -> RgbImage {
        RgbImage::from_par_fn(self.width, self.height, |x, y| {
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
    pub gamma: f64,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
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
            gamma: 2.0,
        }
    }
}

impl From<&CameraParams> for Camera {
    fn from(value: &CameraParams) -> Self {
        Camera::new(value)
    }
}
