use nalgebra as na;

use na::{Matrix4, Point3, Vector3};
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::Ray;

#[derive(Debug, Clone, Copy)]
pub struct IntersectionInfo {
    pub distance: f64,
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
}

impl IntersectionInfo {
    pub fn transform(&self, matrix: &Matrix4<f64>, inverse: &Matrix4<f64>) -> Self {
        let new_position = matrix.transform_point(&self.position);
        let new_normal = inverse
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

    fn sample_intersection_info(&self, ray: &Ray, distance: f64) -> IntersectionInfo;

    fn sample_emissive_ray(&self) -> Ray;

    fn intersection(&self, ray: &Ray) -> Option<IntersectionInfo> {
        self.intersection_distances(ray)
            .map(|distance| self.sample_intersection_info(ray, distance))
    }

    fn blocks(&self, ray: &Ray) -> bool {
        self.intersection_distances(ray).is_some()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere;

impl Shape for Sphere {
    fn intersection_distances(&self, ray: &Ray) -> Option<f64> {
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.origin.coords.dot(&ray.direction);
        let c = ray.origin.coords.dot(&ray.origin.coords) - 1.;

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

    fn sample_intersection_info(&self, ray: &Ray, distance: f64) -> IntersectionInfo {
        let position = ray.origin + distance * ray.direction;
        let normal = position.coords;
        IntersectionInfo {
            distance,
            position,
            normal,
        }
    }

    fn sample_emissive_ray(&self) -> Ray {
        let random_vector =
            Vector3::from_distribution(&StandardNormal, &mut thread_rng()).normalize();

        Ray {
            origin: random_vector.into(),
            direction: random_vector,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Inverted<S: Shape>(pub S);

impl<S: Shape> Shape for Inverted<S> {
    fn intersection_distances(&self, ray: &Ray) -> Option<f64> {
        self.0.intersection_distances(ray)
    }

    fn sample_intersection_info(&self, ray: &Ray, distance: f64) -> IntersectionInfo {
        let mut intersection_info = self.0.sample_intersection_info(ray, distance);
        intersection_info.normal *= -1.;
        intersection_info
    }

    fn sample_emissive_ray(&self) -> Ray {
        let mut ray = self.0.sample_emissive_ray();
        ray.direction *= -1.;

        ray
    }
}
