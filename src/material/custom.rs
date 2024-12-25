use glam::{dvec2, dvec3, DVec2, DVec3, Vec3Swizzles};

use crate::{hittable::Hit, material::Material, ray::Ray};

pub struct Custom {}

impl Custom {
    pub fn new() -> Self {
        Self {}
    }
}

fn cmul(a: DVec2, b: DVec2) -> DVec2 {
    dvec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x)
}

fn conjugate(a: DVec2) -> DVec2 {
    dvec2(a.x, -a.y)
}

fn cinv(a: DVec2) -> DVec2 {
    conjugate(a) / a.length_squared()
}

fn cdiv(a: DVec2, b: DVec2) -> DVec2 {
    cmul(a, cinv(b))
}

fn fractal(point: DVec3) -> DVec3 {
    let mut z = point.xz() / 16.0;

    for _ in 0..32 {
        z -= cmul(cdiv(cmul(z, z) - dvec2(1.0, 0.0), 2.0 * z), DVec2::ONE);
    }

    dvec3(z.x.abs(), z.y.abs(), z.y)
}

impl Material for Custom {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, DVec3)> {
        Some((
            Ray::new(hit.point, ray.direction.reflect(hit.normal)),
            fractal(hit.point),
        ))
    }
}
