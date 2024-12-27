mod camera;
mod hittable;
mod material;
mod random;
mod ray;
mod vec;

use crate::{camera::Camera, hittable::HittableList};

use camera::CameraParams;
use glam::{dvec3, DVec3};
use hittable::Mandelbulb;
use std::time::Instant;

fn main() {
    let mut world = HittableList::new();

    world.add(Mandelbulb::new());

    let mut params = CameraParams::default();

    params.width = 1920;
    params.height = 1080;
    params.position = dvec3(1.0, 0.0, 2.0);
    params.forward = (DVec3::ZERO - params.position).normalize();
    params.samples_per_pixel = 8;

    let cam = Camera::from(&params);

    let time = Instant::now();
    let img = cam.render(&world);
    let duration = time.elapsed();

    println!("Took {:.2?}", duration);

    img.save("out.png").unwrap();
}
