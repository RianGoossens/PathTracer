use nalgebra::{Perspective3, Point3};
use rand::{thread_rng, Rng};

use crate::{Aperture, Ray};

pub struct Camera {
    pub transform: Perspective3<f64>,
    pub width: u32,
    pub height: u32,
    pub aspect: f64,
    pub aperture: Box<dyn Aperture>,
    pub focal_length: f64,
    frustrum_data: FrustrumData,
}

#[derive(Clone, Copy, Debug)]
struct FrustrumData {
    znear: f64,
    zfar: f64,
    near_half_width: f64,
    near_half_height: f64,
    far_half_width: f64,
    far_half_height: f64,
}

impl FrustrumData {
    fn new(transform: &Perspective3<f64>) -> Self {
        let inverse_transform = transform.inverse();

        let near_point = inverse_transform.transform_point(&Point3::new(1., 1., -1.));
        let far_point = inverse_transform.transform_point(&Point3::new(1., 1., 1.));

        FrustrumData {
            zfar: transform.zfar(),
            znear: transform.znear(),
            far_half_height: far_point.y,
            far_half_width: far_point.x,
            near_half_height: near_point.y,
            near_half_width: near_point.x,
        }
    }

    fn get_ray_from_normalized_coordinates(&self, x: f64, y: f64) -> Ray {
        let origin = Point3::new(
            x * self.near_half_width,
            y * self.near_half_height,
            -self.znear,
        );

        let far_point = Point3::new(
            x * self.far_half_width,
            y * self.far_half_height,
            -self.zfar,
        );

        let direction = (far_point - origin).normalize();

        Ray { origin, direction }
    }
}

impl Camera {
    pub fn new<Ap: Aperture + 'static>(
        width: u32,
        height: u32,
        fov_degrees: f64,
        znear: f64,
        zfar: f64,
        aperture: Ap,
        focal_distance: f64,
    ) -> Self {
        let aspect = width as f64 / height as f64;
        let transform = Perspective3::new(
            aspect,
            fov_degrees / 180. * std::f64::consts::PI,
            znear,
            zfar,
        );
        let frustrum_data = FrustrumData::new(&transform);

        Self {
            transform,
            width,
            height,
            aspect,
            aperture: Box::new(aperture),
            focal_length: focal_distance,
            frustrum_data,
        }
    }

    pub fn get_ray(&self, x_index: u32, y_index: u32) -> Ray {
        let mut rng = thread_rng();
        //Sample randomly from the area of the pixel for anti-aliasing
        let x: f64 = x_index as f64 + rng.gen::<f64>() - 0.5;
        let y: f64 = y_index as f64 + rng.gen::<f64>() - 0.5;

        //Normalize the coordinates
        let x = 2. * (x / (self.width - 1) as f64) - 1.;
        let y = -2. * (y / (self.height - 1) as f64) + 1.;

        let source_ray = self.frustrum_data.get_ray_from_normalized_coordinates(x, y);

        self.aperture.sample_ray(&source_ray, self.focal_length)
    }
}
