use std::rc::Rc;

use nalgebra::{Matrix4, Projective3};

use crate::{shape::IntersectionInfo, Material, Ray, Shape};

#[derive(Clone)]
pub struct Object {
    pub shape: Rc<dyn Shape>,
    pub transform: Projective3<f64>,
    pub inverse_transform: Matrix4<f64>,
    pub material: Material,
}

impl Object {
    pub fn new(shape: Rc<dyn Shape>, transform: Projective3<f64>, material: Material) -> Self {
        Self {
            shape,
            transform,
            material,
            inverse_transform: *transform.inverse().matrix(),
        }
    }
}

impl Object {
    pub fn local_intersection(&self, ray: &Ray) -> Option<IntersectionInfo> {
        let local_ray = ray.transform(&self.inverse_transform);

        self.shape.intersection(&local_ray)
    }
}
