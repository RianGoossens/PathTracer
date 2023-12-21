use nalgebra as na;

use na::{Point3, Similarity3, Vector3};

use crate::Ray;

mod cuboid;
mod plane;
mod sphere;

pub use cuboid::Cuboid;
pub use plane::Plane;
pub use sphere::Sphere;

#[derive(Debug, Clone, Copy)]
pub struct IntersectionInfo {
    pub distance: f64,
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
}

impl IntersectionInfo {
    pub fn transform_similarity(&self, matrix: &Similarity3<f64>) -> Self {
        let new_position = matrix.transform_point(&self.position);
        let new_normal = matrix
            .inverse()
            .to_homogeneous()
            .transpose()
            .transform_vector(&self.normal)
            .normalize();
        IntersectionInfo {
            position: new_position,
            normal: new_normal,
            ..*self
        }
    }
}

pub trait Shape: Send + Sync {
    fn intersection_distances(&self, ray: &Ray) -> Option<f64>;

    fn sample_normal(&self, position: Point3<f64>) -> Vector3<f64>;

    fn sample_random_point(&self) -> Point3<f64>;

    fn area(&self) -> f64;

    fn intersection(&self, ray: &Ray) -> Option<IntersectionInfo> {
        self.intersection_distances(ray).map(|distance| {
            let position = ray.origin + distance * ray.direction;
            let normal = self.sample_normal(position);
            IntersectionInfo {
                distance,
                position,
                normal,
            }
        })
    }

    fn blocks(&self, ray: &Ray) -> bool {
        self.intersection_distances(ray).is_some()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Inverted<S: Shape>(pub S);

impl<S: Shape> Shape for Inverted<S> {
    fn intersection_distances(&self, ray: &Ray) -> Option<f64> {
        self.0.intersection_distances(ray)
    }

    fn area(&self) -> f64 {
        self.0.area()
    }

    fn sample_random_point(&self) -> Point3<f64> {
        self.0.sample_random_point()
    }

    fn sample_normal(&self, position: Point3<f64>) -> Vector3<f64> {
        -self.0.sample_normal(position)
    }
}
