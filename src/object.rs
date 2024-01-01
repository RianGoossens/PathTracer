use nalgebra::{Similarity3, Vector3};
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::{shape::IntersectionInfo, Material, Ray, Shape};

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub transform: Similarity3<f64>,
    pub inverse_transform: Similarity3<f64>,
    pub material: Material,
}

impl Object {
    pub fn new<S: Shape + 'static>(
        shape: S,
        transform: Similarity3<f64>,
        material: Material,
    ) -> Self {
        Self {
            shape: Box::new(shape),
            transform,
            material,
            inverse_transform: transform.inverse(),
        }
    }

    pub fn sample_emissive_ray(&self) -> Ray {
        let origin = self.shape.sample_random_point();
        let normal = self.shape.sample_normal(origin);

        let mut direction =
            Vector3::from_distribution(&StandardNormal, &mut thread_rng()).normalize();

        if direction.dot(&normal) < 0. {
            direction = -direction;
        }

        direction = normal.slerp(&direction, self.material.roughness);

        let local_ray = Ray { origin, direction };

        let mut global_ray = local_ray.transform_similarity(&self.transform);
        global_ray.direction.normalize_mut();

        global_ray.origin += global_ray.direction * 0.001;

        global_ray
    }
}

impl Object {
    pub fn local_intersection(&self, ray: &Ray) -> Option<IntersectionInfo> {
        let local_ray = ray.transform_similarity(&self.inverse_transform);

        self.shape.intersection(&local_ray)
    }

    pub fn area(&self) -> f64 {
        self.shape.area() * self.transform.scaling() * self.transform.scaling()
    }
}
