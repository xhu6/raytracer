use glam::{dvec3, DVec3};
use image::{Rgb, RgbImage};
use sphere::Sphere;

use crate::camera::Camera;
use crate::ray::Ray;

mod camera;
mod ray;
mod sphere;
mod testing;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn to_rgb(data: DVec3) -> Rgb<u8> {
    Rgb(data.to_array().map(|x| (x * 255.999).floor() as u8))
}

fn get_colour(ray: Ray) -> Rgb<u8> {
    let sphere = Sphere::new(dvec3(0.0, 0.0, -1.0), 0.5);
    if sphere.hit(&ray) {
        return Rgb([255, 0, 0]);
    }

    let a = 0.5 * (ray.direction.y + 1.0);
    let result = (1.0 - a) * DVec3::ONE + a * dvec3(0.5, 0.7, 1.0);
    to_rgb(result)
}

fn f(cam: &Camera, x: u32, y: u32) -> Rgb<u8> {
    let ray = cam.sample(
        (x as f64 + 0.5) / WIDTH as f64,
        (y as f64 + 0.5) / HEIGHT as f64,
    );
    get_colour(ray)
}

fn main() {
    let cam = Camera::new(1.0, 2.0, 2.0, DVec3::ZERO);
    let img = RgbImage::from_fn(WIDTH, HEIGHT, |x, y| f(&cam, x, y));
    img.save("out.png").unwrap();
}
