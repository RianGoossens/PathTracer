use nalgebra as na;

use na::{Point3, Vector3};
use rand::{thread_rng, Rng};

use crate::{Ray, Shape};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    width: f64,
    height: f64,
}

impl Plane {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Plane {
    fn intersection_distances(&self, Ray { origin, direction }: &Ray) -> Option<f64> {
        if direction.z == 0. {
            None
        } else {
            let t = -origin.z / direction.z;
            if t >= 0. {
                let x = origin.x + direction.x * t;
                let y = origin.y + direction.y * t;

                if x < -self.width / 2.
                    || x > self.width / 2.
                    || y < -self.height / 2.
                    || y > self.height / 2.
                {
                    None
                } else {
                    Some(t)
                }
            } else {
                None
            }
        }
    }

    fn sample_random_point(&self) -> Point3<f64> {
        let mut rng = thread_rng();

        let x = rng.gen_range((-self.width / 2.)..self.width / 2.);
        let y = rng.gen_range((-self.height / 2.)..self.height / 2.);

        Point3::new(x, y, 0.)
    }

    fn area(&self) -> f64 {
        2. * self.width * self.height
    }

    fn sample_normal(&self, _position: Point3<f64>) -> Vector3<f64> {
        let mut rng = thread_rng();
        if rng.gen_bool(0.5) {
            Vector3::new(0., 0., 1.)
        } else {
            Vector3::new(0., 0., -1.)
        }
    }
}
