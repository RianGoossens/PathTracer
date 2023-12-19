use nalgebra::Similarity3;

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
        let local_ray = self.shape.sample_emissive_ray();
        local_ray.transform_similarity(&self.transform)
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
