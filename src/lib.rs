use nalgebra::{Point3, Vector3};

pub mod aperture;
pub mod camera;
pub mod function_approximation;
pub mod material;
pub mod object;
pub mod ray;
pub mod render_buffer;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod shape;

pub use aperture::Aperture;
pub use camera::Camera;
pub use material::Material;
pub use object::Object;
pub use ray::Ray;
pub use render_buffer::RenderBuffer;
pub use renderer::{BackwardRenderer, Renderer};
pub use scene::Scene;
pub use shader::Shader;
pub use shape::{Inverted, Shape, Sphere};

pub fn reflect(incoming: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    incoming - 2. * normal.dot(incoming) * normal
}

pub fn find_normal(incoming: &Vector3<f64>, outgoing: &Vector3<f64>) -> Vector3<f64> {
    (outgoing - incoming).normalize()
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
}

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub position: Point3<f64>,
    pub color: Vector3<f64>,
}
