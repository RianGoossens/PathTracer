use crate::{Aperture, Ray};

use rand::{thread_rng, Rng};

use nalgebra as na;

use na::{Isometry3, Perspective3, Point3, Vector3};

pub struct Camera {
    pub perspective: Perspective3<f64>,
    pub translation_and_rotation: Isometry3<f64>,
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
    fn new(perspective: &Perspective3<f64>) -> Self {
        let inverse_transform = perspective.inverse();

        let near_point = inverse_transform.transform_point(&Point3::new(1., 1., -1.));
        let far_point = inverse_transform.transform_point(&Point3::new(1., 1., 1.));

        FrustrumData {
            zfar: perspective.zfar(),
            znear: perspective.znear(),
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

pub struct CameraSettings {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
    pub width: u32,
    pub height: u32,
    pub fov_degrees: f64,
    pub znear: f64,
    pub zfar: f64,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            rx: Default::default(),
            ry: Default::default(),
            rz: Default::default(),
            width: 100,
            height: 100,
            fov_degrees: 90.,
            znear: 1.,
            zfar: 100.,
        }
    }
}

impl Camera {
    pub fn new<Ap: Aperture + Sync + 'static>(
        settings: CameraSettings,
        aperture: Ap,
        focal_distance: f64,
    ) -> Self {
        let aspect = settings.width as f64 / settings.height as f64;
        let transform = Perspective3::new(
            aspect,
            settings.fov_degrees / 180. * std::f64::consts::PI,
            settings.znear,
            settings.zfar,
        );
        let frustrum_data = FrustrumData::new(&transform);

        let translation_and_rotation = Isometry3::new(
            Vector3::new(settings.x, settings.y, settings.z),
            Vector3::new(settings.rx, settings.ry, settings.rz),
        );
        Self {
            perspective: transform,
            width: settings.width,
            height: settings.height,
            translation_and_rotation,
            aspect,
            aperture: Box::new(aperture),
            focal_length: focal_distance,
            frustrum_data,
        }
    }
    pub fn new_at_origin<Ap: Aperture + Sync + 'static>(
        width: u32,
        height: u32,
        fov_degrees: f64,
        znear: f64,
        zfar: f64,
        aperture: Ap,
        focal_distance: f64,
    ) -> Self {
        Self::new(
            CameraSettings {
                width,
                height,
                fov_degrees,
                znear,
                zfar,
                ..Default::default()
            },
            aperture,
            focal_distance,
        )
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

        let local_ray = self.aperture.sample_ray(&source_ray, self.focal_length);

        local_ray.transform_isometry(&self.translation_and_rotation)
    }
}
