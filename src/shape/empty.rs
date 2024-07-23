use nalgebra::{Point3, Vector3};

use crate::Shape;

pub struct Empty;

impl Shape for Empty {
    fn intersection_distance(&self, _ray: &crate::Ray) -> Option<f64> {
        None
    }

    fn sample_normal(&self, _position: nalgebra::Point3<f64>) -> nalgebra::Vector3<f64> {
        Vector3::zeros()
    }

    fn sample_random_point(&self) -> nalgebra::Point3<f64> {
        Point3::new(0., 0., 0.)
    }

    fn area(&self) -> f64 {
        0.
    }
}
