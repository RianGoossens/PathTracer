use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

use crate::Ray;

#[derive(Clone, Debug)]
pub struct Camera {
    pub transform: Perspective3<f64>,
    pub inverse_transform: Matrix4<f64>,
    pub width: u32,
    pub height: u32,
    pub aspect: f64,
    rays: Vec<Vec<Ray>>,
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

        let mut rays = vec![];

        for y_index in 0..height {
            let y = -2. * (y_index as f64 / (height - 1) as f64) + 1.;
            let mut row = Vec::with_capacity(width as usize);
            for x_index in 0..width {
                let x = 2. * (x_index as f64 / (width - 1) as f64) - 1.;
                let origin = Point3::new(x, y, -1.);
                let direction = Vector3::new(0., 0., 1.);
                let ray = Ray { origin, direction };
                row.push(ray.transform(&inverse_transform));
            }
            rays.push(row);
        }

        Self {
            transform,
            inverse_transform,
            width,
            height,
            aspect,
            rays,
        }
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Option<&Ray> {
        self.rays
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }
}
