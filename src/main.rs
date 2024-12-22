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

fn main() {
    let mut world = HittableList::new();

    world.add(Sphere::new(dvec3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(dvec3(0.0, -100.5, -1.0), 100.0));

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let cam = Camera::new(1.0, aspect_ratio, 90.0, DVec3::ZERO);
    let img = cam.render(&world, WIDTH, HEIGHT);
    img.save("out.png").unwrap();
}
