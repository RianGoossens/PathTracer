use nalgebra::{Similarity3, Vector3};
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::{
    shape::{Empty, IntersectionInfo},
    Material, Ray, Shape,
};

pub struct ObjectDefinition {
    pub shape: Box<dyn Shape>,
    pub material: Material,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
    pub scale: f64,
}

impl Default for ObjectDefinition {
    fn default() -> Self {
        Self {
            shape: Box::new(Empty),
            material: Material::Emissive {
                color: Vector3::zeros(),
            },
            x: 0.,
            y: 0.,
            z: 0.,
            rx: 0.,
            ry: 0.,
            rz: 0.,
            scale: 1.,
        }
    }
}

pub struct Object {
    shape: Box<dyn Shape>,
    transform: Similarity3<f64>,
    inverse_transform: Similarity3<f64>,
    material: Material,
}

impl Object {
    pub fn new(definition: ObjectDefinition) -> Self {
        let transform = Similarity3::new(
            Vector3::new(definition.x, definition.y, definition.z),
            Vector3::new(definition.rx, definition.ry, definition.rz),
            definition.scale,
        );
        Self {
            shape: definition.shape,
            transform,
            inverse_transform: transform.inverse(),
            material: definition.material,
        }
    }

    pub fn new_old<S: Shape + 'static>(
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

    pub fn transform(&self) -> &Similarity3<f64> {
        &self.transform
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn sample_emissive_ray(&self) -> Ray {
        let origin = self.shape.sample_random_point();
        let normal = self.shape.sample_normal(origin);

        let mut direction =
            Vector3::from_distribution(&StandardNormal, &mut thread_rng()).normalize();

        if direction.dot(&normal) < 0. {
            direction = -direction;
        }

        let local_ray = Ray { origin, direction };

        let mut global_ray = local_ray.transform_similarity(&self.transform);
        global_ray.direction.normalize_mut();

        global_ray.origin += global_ray.direction * 0.001;

        global_ray
    }

    pub fn local_intersection(&self, ray: &Ray) -> Option<IntersectionInfo> {
        let local_ray = ray.transform_similarity(&self.inverse_transform);

        self.shape.intersection(&local_ray)
    }

    pub fn area(&self) -> f64 {
        self.shape.area() * self.transform.scaling() * self.transform.scaling()
    }
}
