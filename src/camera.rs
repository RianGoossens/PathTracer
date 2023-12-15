use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

use crate::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub transform: Perspective3<f64>,
    pub inverse_transform: Matrix4<f64>,
    pub width: u32,
    pub height: u32,
    pub aspect: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov_degrees: f64, znear: f64, zfar: f64) -> Self {
        let aspect = width as f64 / height as f64;
        let transform = Perspective3::new(
            aspect,
            fov_degrees / 180. * std::f64::consts::PI,
            znear,
            zfar,
        );
        let inverse_transform = transform.inverse();
        Self {
            transform,
            inverse_transform,
            width,
            height,
            aspect,
        }
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let x = 2. * (x as f64 / (self.width - 1) as f64) - 1.;
        let y = -2. * (y as f64 / (self.height - 1) as f64) + 1.;
        let origin = Point3::new(x, y, -1.);
        let direction = Vector3::new(0., 0., 1.);
        Ray { origin, direction }
    }
}
