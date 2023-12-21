use std::f64::consts::PI;

use nalgebra as na;

use na::{Point3, Vector3};
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::{Ray, Shape};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Sphere {
    fn intersection_distances(&self, ray: &Ray) -> Option<f64> {
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.origin.coords.dot(&ray.direction);
        let c = ray.origin.coords.dot(&ray.origin.coords) - self.radius;

        let determinant = b * b - 4. * a * c;
        if determinant < 0. {
            None
        } else {
            let d_root = determinant.sqrt();
            let t1 = (-b - d_root) / (2. * a);
            let t2 = (-b + d_root) / (2. * a);
            if t1 < 0. {
                if t2 < 0. {
                    None
                } else {
                    Some(t2)
                }
            } else if t2 < 0. {
                Some(t1)
            } else {
                Some(t1.min(t2))
            }
        }
    }

    fn area(&self) -> f64 {
        4. * PI * self.radius * self.radius
    }

    fn sample_random_point(&self) -> Point3<f64> {
        let origin: Point3<f64> = Vector3::from_distribution(&StandardNormal, &mut thread_rng())
            .normalize()
            .into();

        origin
    }

    fn sample_normal(&self, position: Point3<f64>) -> Vector3<f64> {
        position.coords.normalize()
    }
}
