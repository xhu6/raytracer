use std::{sync::Arc, time::Instant};

use camera::CameraParams;
use core::f64;
use glam::{dvec3, DVec3};
use material::{Custom, Dielectric, Lambertian, Material, Metal};
use random::random_colour;

use crate::{camera::Camera, hittable::HittableList, sphere::Sphere};

mod camera;
mod hittable;
mod material;
mod random;
mod ray;
mod sphere;
mod vec;

fn main() {
    // Make RNG deterministic
    fastrand::seed(0);

    let glass = Arc::new(Dielectric::new(1.5));
    let ground = Arc::new(Lambertian::new(dvec3(0.9, 0.9, 0.9)));
    let surface = Arc::new(Lambertian::new(dvec3(0.0, 0.5, 0.5)));
    let custom = Arc::new(Custom::new());
    let diamond = Arc::new(Dielectric::new(2.417));

    let mut world = HittableList::new();

    world.add(Sphere::new(
        dvec3(0.0, -1000.0, 0.0),
        1000.0,
        custom.clone(),
    ));
    world.add(Sphere::new(dvec3(1.0, 1.0, 0.0), 1.0, diamond.clone()));
    world.add(Sphere::new(dvec3(-1.0, 1.0, 0.0), 1.0, surface.clone()));

    let mut params = CameraParams::default();

    params.width = 1920;
    params.height = 1080;
    params.aspect_ratio = params.width as f64 / params.height as f64;
    params.position = dvec3(0.0, 4.0, -4.0);
    params.forward = (dvec3(0.0, 0.0, 0.0) - params.position).normalize();
    params.samples_per_pixel = 500;

    let cam = Camera::from(&params);

    let time = Instant::now();
    let img = cam.render(&world);
    let duration = time.elapsed();

    println!("Took {:.2?}", duration);

    img.save("out.png").unwrap();
}
