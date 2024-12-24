use std::{sync::Arc, time::Instant};

use core::f64;
use glam::{dvec3, DVec3};
use material::{Dielectric, Lambertian, Metal};

use crate::{camera::Camera, hittable::HittableList, sphere::Sphere};

mod camera;
mod hittable;
mod material;
mod random;
mod ray;
mod sphere;
mod vec;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    // Make RNG deterministic
    fastrand::seed(0);

    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(dvec3(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(dvec3(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_left_bubble = Arc::new(Dielectric::new(1.0/1.5));
    let material_right = Arc::new(Metal::new(dvec3(0.8, 0.6, 0.2), 1.0));

    world.add(Sphere::new(
        dvec3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));

    world.add(Sphere::new(dvec3(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(dvec3(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(
        dvec3(-1.0, 0.0, -1.0),
        0.4,
        material_left_bubble,
    ));
    world.add(Sphere::new(dvec3(1.0, 0.0, -1.0), 0.5, material_right));

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let cam = Camera::new(aspect_ratio, 20.0, dvec3(-2.0, 2.0, 1.0), dvec3(0.0, 0.0, -1.0), dvec3(0.0, 1.0, 0.0), WIDTH, HEIGHT);

    let time = Instant::now();
    let img = cam.render(&world);
    let duration = time.elapsed();

    println!("Took {:.2?}", duration);

    img.save("out.png").unwrap();
}
