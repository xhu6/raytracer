use std::sync::Arc;

use glam::{dvec3, DVec3};

use super::hit::Hit;
use super::traits::Hittable;
use crate::material::{Material, Rainbow};
use crate::ray::Ray;

pub struct Mandelbulb {
    material: Arc<dyn Material>,
    power: f64,
}

impl Mandelbulb {
    pub fn new(power: f64) -> Self {
        Mandelbulb {
            material: Arc::new(Rainbow::new()),
            power,
        }
    }

    fn distance_equation(&self, c: DVec3) -> f64 {
        let mut z = c;
        let mut dr = 1.0;
        let mut r = 0.0;

        for _ in 0..32 {
            r = z.length();

            if r > 10.0 {
                break;
            }

            let mut theta = (z.z / r).acos();
            let mut phi = (z.y / z.x).atan();
            dr = r.powf(self.power - 1.0) * self.power * dr + 1.0;

            let zr = r.powf(self.power);
            theta *= self.power;
            phi *= self.power;

            z = zr
                * dvec3(
                    theta.sin() * phi.cos(),
                    theta.sin() * phi.sin(),
                    theta.cos(),
                );
            z += c;
        }

        0.5 * r.ln() * r / dr
    }
}

impl Hittable for Mandelbulb {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit> {
        let mut distance = min;
        let max_iters = 128;

        for n in 0..max_iters {
            let delta = self.distance_equation(ray.at(distance));
            distance += delta;

            if delta > 0.0002 {
                continue;
            }

            if !(min..max).contains(&distance) {
                continue;
            }

            return Some(Hit::new(
                ray.at(distance),
                ray.direction,
                n as f64 / max_iters as f64,
                false,
                self.material.clone(),
            ));
        }

        None
    }
}
