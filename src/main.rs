use std::sync::Arc;
use std::time::Instant;

use glam::{dvec3, DVec3};
use raytracer::camera::{Camera, CameraParams};
use raytracer::hittable::{HittableList, Sphere};
use raytracer::material::{Dielectric, Lambertian, Material, Metal};
use raytracer::random::random_colour;

fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(dvec3(0.5, 0.5, 0.5)));

    world.add(Sphere::new(
        dvec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for x in -11..11 {
        for z in -11..11 {
            let mat = fastrand::f64();
            let position = dvec3(
                x as f64 + 0.9 * fastrand::f64(),
                0.2,
                z as f64 + 0.9 * fastrand::f64(),
            );

            if (position - dvec3(4.0, 0.2, 0.0)).length() < 0.9 {
                continue;
            }

            let material: Arc<dyn Material> = if mat < 0.8 {
                let albedo = random_colour() * random_colour();
                Arc::new(Lambertian::new(albedo))
            } else if mat < 0.95 {
                let albedo = random_colour() / 2.0 + 0.5;
                let fuzz = fastrand::f64();
                Arc::new(Metal::new(albedo, fuzz))
            } else {
                Arc::new(Dielectric::new(1.5))
            };

            world.add(Sphere::new(position, 0.2, material));
        }
    }

    world.add(Sphere::new(
        dvec3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    world.add(Sphere::new(
        dvec3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(dvec3(0.4, 0.2, 0.1))),
    ));

    world.add(Sphere::new(
        dvec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(dvec3(0.7, 0.6, 0.5), 0.0)),
    ));

    let mut params = CameraParams::default();

    params.width = 1920;
    params.height = 1080;
    params.samples_per_pixel = 512;
    params.vfov = 20.0;

    params.position = dvec3(13.0, 2.0, 3.0);
    params.forward = (DVec3::ZERO - params.position).normalize();

    params.defocus_angle = 0.6;
    params.focal_length = 10.0;

    let cam = Camera::from(&params);

    let time = Instant::now();
    let img = cam.render(&world);
    let duration = time.elapsed();

    println!("Took {duration:.2?}");

    img.save("out.png").unwrap();
}
