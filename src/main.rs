use core::f64;
use glam::{dvec3, DVec3};
use image::{Rgb, RgbImage};

use crate::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    ray::Ray,
    sphere::Sphere,
};

mod camera;
mod hittable;
mod ray;
mod sphere;
mod testing;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const VIEWPORT_WIDTH: f64 = 4.0;
const VIEWPORT_HEIGHT: f64 = VIEWPORT_WIDTH * HEIGHT as f64 / WIDTH as f64;

fn to_rgb(data: DVec3) -> Rgb<u8> {
    Rgb(data.to_array().map(|x| (x * 255.999).floor() as u8))
}

fn get_colour(world: &HittableList, ray: Ray) -> Rgb<u8> {
    if let Some(hit) = world.hit(&ray, 0.0, f64::MAX) {
        return to_rgb(hit.normal.map(|x| 0.5 * (x + 1.0)));
    }

    let a = 0.5 * (ray.direction.y + 1.0);
    let result = (1.0 - a) * DVec3::ONE + a * dvec3(0.5, 0.7, 1.0);
    to_rgb(result)
}

fn f(world: &HittableList, cam: &Camera, x: u32, y: u32) -> Rgb<u8> {
    let ray = cam.sample(
        (x as f64 + 0.5) / WIDTH as f64,
        (y as f64 + 0.5) / HEIGHT as f64,
    );
    get_colour(world, ray)
}

fn main() {
    let mut world = HittableList::new();

    world.add(Sphere::new(dvec3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(dvec3(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new(1.0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT, DVec3::ZERO);
    let img = RgbImage::from_fn(WIDTH, HEIGHT, |x, y| f(&world, &cam, x, y));
    img.save("out.png").unwrap();
}
