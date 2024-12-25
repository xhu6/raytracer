use glam::DVec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, distance: f64) -> DVec3 {
        self.origin + self.direction * distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vec::approx_eq;

    use glam::dvec3;

    #[test]
    fn create_ray() {
        let ray = Ray::new(dvec3(1.0, 2.0, 3.0), dvec3(1.0, 0.0, 0.0));

        assert!(approx_eq(ray.origin, dvec3(1.0, 2.0, 3.0)));
        assert!(approx_eq(ray.direction, dvec3(1.0, 0.0, 0.0)));
    }

    #[test]
    fn advance_normal_ray() {
        let ray = Ray::new(dvec3(4.0, 5.0, 6.0), dvec3(-1.0, 0.0, 0.0));

        assert!(approx_eq(ray.at(4.0), dvec3(0.0, 5.0, 6.0)));
    }

    #[test]
    fn advance_unnormal_ray() {
        let ray = Ray::new(dvec3(4.0, 5.0, 6.0), dvec3(1.0, 1.0, 0.0));

        assert!(approx_eq(
            ray.at(2.0),
            dvec3(4.0 + 2.0f64.sqrt(), 5.0 + 2.0f64.sqrt(), 6.0),
        ));
    }
}
