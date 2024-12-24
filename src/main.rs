use std::{sync::Arc, time::Instant};

use core::f64;
use glam::{dvec3, DVec3};
use material::{Dielectric, Lambertian, Material, Metal};
use random::random_colour;

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

    let ground_material = Arc::new(Lambertian::new(dvec3(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        dvec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let position = dvec3(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );

            if (position - dvec3(4.0, 0.2, 0.0)).length() < 0.9 {
                continue;
            }

            let choose_mat = fastrand::f64();
            let material: Arc<dyn Material> = if choose_mat < 0.8 {
                let albedo = random_colour() * random_colour();
                Arc::new(Lambertian::new(albedo))
            } else if choose_mat < 0.95 {
                let albedo = random_colour() / 2.0 + DVec3::splat(0.5);
                let fuzz = fastrand::f64();
                Arc::new(Metal::new(albedo, fuzz))
            } else {
                Arc::new(Dielectric::new(1.5))
            };

            world.add(Sphere::new(position, 0.2, material));
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(dvec3(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(dvec3(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere::new(dvec3(0.0, 1.0, 0.0), 1.0, material1));
    world.add(Sphere::new(dvec3(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(Sphere::new(dvec3(4.0, 1.0, 0.0), 1.0, material3));

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let cam = Camera::new(
        aspect_ratio,
        20.0,
        0.6,
        10.0,
        dvec3(13.0, 2.0, 3.0),
        dvec3(0.0, 0.0, 0.0),
        dvec3(0.0, 1.0, 0.0),
        WIDTH,
        HEIGHT,
    );

    let time = Instant::now();
    let img = cam.render(&world);
    let duration = time.elapsed();

    println!("Took {:.2?}", duration);

    img.save("out.png").unwrap();
}
