use nalgebra::{Vector2, Vector3};
use rand::{thread_rng, Rng};
use rand_distr::{num_traits::Pow, Normal};

use crate::Ray;

pub trait Aperture: Send + Sync {
    fn sample_offset(&self) -> Vector2<f64>;

    fn sample_ray(&self, Ray { origin, direction }: &Ray, focal_length: f64) -> Ray {
        let focal_point = direction * focal_length;

        let offset = self.sample_offset();
        let origin = origin + Vector3::new(offset.x, offset.y, 0.);
        let direction = (focal_point - origin.coords).normalize();

        Ray { origin, direction }
    }
}

pub struct PinholeAperture;

impl Aperture for PinholeAperture {
    fn sample_offset(&self) -> Vector2<f64> {
        todo!()
    }
    fn sample_ray(&self, ray: &Ray, _focal_length: f64) -> Ray {
        *ray
    }
}

pub struct GaussianAperture {
    pub std_dev: f64,
    distribution: Normal<f64>,
}

impl GaussianAperture {
    pub fn new(std_dev: f64) -> Self {
        Self {
            std_dev,
            distribution: Normal::new(0., std_dev).unwrap(),
        }
    }
}

impl Aperture for GaussianAperture {
    fn sample_offset(&self) -> Vector2<f64> {
        Vector2::from_distribution(&self.distribution, &mut thread_rng())
    }
}

pub struct RegularPolygonAperture {
    pub radius: f64,
    pub angles: u8,
}

impl RegularPolygonAperture {
    pub fn new(radius: f64, angles: u8) -> Self {
        Self { radius, angles }
    }
}

impl Aperture for RegularPolygonAperture {
    fn sample_offset(&self) -> Vector2<f64> {
        let mut rng = thread_rng();
        let section: u8 = rng.gen_range(0..self.angles);

        let angle_a = section as f64 * 2. * std::f64::consts::PI / self.angles as f64;
        let angle_b = (section as f64 + 1.) * 2. * std::f64::consts::PI / self.angles as f64;

        let weight = rng.gen::<f64>();
        let distance: f64 = rng.gen::<f64>().sqrt() * self.radius;

        let vector_a = Vector2::new(angle_a.cos(), angle_a.sin()) * distance;
        let vector_b = Vector2::new(angle_b.cos(), angle_b.sin()) * distance;

        vector_a.lerp(&vector_b, weight)
    }
}
