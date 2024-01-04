use std::f64::consts::TAU;

use nalgebra::{Point3, Vector3};
use rand::{thread_rng, Rng};

use crate::Shape;

pub struct Cylinder {
    pub radius: f64,
    pub height: f64,
}

impl Cylinder {
    pub fn new(radius: f64, height: f64) -> Self {
        Cylinder { radius, height }
    }
}

impl Shape for Cylinder {
    fn intersection_distance(&self, ray: &crate::Ray) -> Option<f64> {
        //(xo + xd * t) ** 2 + (yo + yd * t) ** 2 = radius ** 2
        //sum((o + d * t) ** 2) = radius ** 2
        //sum(o ** 2 + 2 * t * o * d + t ** 2 * d ** 2) = radius ** 2
        //2 * t * sum(o * d) + t ** 2 * d ** 2 = radius ** 2 - sum(o ** 2)

        //a = d ** 2
        //b = 2 * sum(o * d)
        //c = sum(o ** 2) - radius ** 2

        let origin = ray.origin.xy().coords;
        let direction = ray.direction.xy();

        let a = direction.dot(&direction);
        let b = 2. * origin.dot(&direction);
        let c = origin.dot(&origin) - self.radius.powi(2);

        let determinant = b * b - 4. * a * c;
        if determinant < 0. {
            None
        } else {
            let d_root = determinant.sqrt();
            let t1 = (-b - d_root) / (2. * a);
            let t2 = (-b + d_root) / (2. * a);

            let z1 = ray.origin.z + t1 * ray.direction.z;
            let z2 = ray.origin.z + t2 * ray.direction.z;

            let valid_1 = t1 >= 0. && z1 <= self.height / 2. && z1 >= -self.height / 2.;
            let valid_2 = t2 >= 0. && z2 <= self.height / 2. && z2 >= -self.height / 2.;

            if valid_1 {
                if valid_2 {
                    Some(t1.min(t2))
                } else {
                    Some(t1)
                }
            } else if valid_2 {
                Some(t2)
            } else {
                None
            }
        }
    }

    fn sample_normal(&self, position: nalgebra::Point3<f64>) -> nalgebra::Vector3<f64> {
        let mut rng = thread_rng();
        let xy = position.xy().coords.normalize();

        if rng.gen_bool(0.5) {
            Vector3::new(xy.x, xy.y, 0.)
        } else {
            Vector3::new(-xy.x, -xy.y, 0.)
        }
    }

    fn sample_random_point(&self) -> nalgebra::Point3<f64> {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0. ..TAU);
        let x = angle.cos() * self.radius;
        let z = angle.sin() * self.radius;
        let y = rng.gen_range(-self.height / 2.0..self.height / 2.);
        Point3::new(x, y, z)
    }

    fn area(&self) -> f64 {
        2. * TAU * self.height
    }
}
