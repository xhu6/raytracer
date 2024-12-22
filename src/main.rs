use core::f64;
use glam::{dvec3, DVec3};

use crate::{camera::Camera, hittable::HittableList, sphere::Sphere};

mod camera;
mod hittable;
mod ray;
mod sphere;
mod testing;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const VIEWPORT_WIDTH: f64 = 4.0;
const VIEWPORT_HEIGHT: f64 = VIEWPORT_WIDTH * HEIGHT as f64 / WIDTH as f64;

fn main() {
    let mut world = HittableList::new();

    world.add(Sphere::new(dvec3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(dvec3(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new(1.0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT, DVec3::ZERO);
    let img = cam.render(&world, WIDTH, HEIGHT);
    img.save("out.png").unwrap();
}
