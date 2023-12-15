use nalgebra::{Point3, Vector3};

pub mod camera;
pub mod object;
pub mod ray;
pub mod scene;
pub mod shape;

pub use camera::Camera;
pub use object::Object;
pub use ray::Ray;
pub use scene::Scene;
pub use shape::{Shape, Sphere};

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Vector3<f64>,
    pub roughness: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub position: Point3<f64>,
    pub color: Vector3<f64>,
}
