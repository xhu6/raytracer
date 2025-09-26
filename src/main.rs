use std::time::Instant;

use glam::{dvec3, DVec3};
use raytracer::camera::{Camera, CameraParams};
use raytracer::hittable::{HittableList, Mandelbulb};

fn main() {
    let mut world = HittableList::new();

    world.add(Mandelbulb::new(5.0));

    let mut params = CameraParams::default();

    params.width = 1920;
    params.height = 1080;
    params.position = dvec3(1.0, 0.0, 2.0);
    params.forward = (DVec3::ZERO - params.position).normalize();
    params.samples_per_pixel = 1;

    let cam = Camera::from(&params);

    let time = Instant::now();
    let img = cam.render(&world);
    let duration = time.elapsed();

    println!("Took {duration:.2?}");

    img.save("out.png").unwrap();
}
