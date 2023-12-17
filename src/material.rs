use na::Vector3;
use nalgebra as na;
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::{reflect, shape::IntersectionInfo, Ray};

#[derive(Default, Clone, Copy, Debug)]
pub struct Material {
    pub color: Vector3<f64>,
    pub roughness: f64,
    pub emissive: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct SurfaceInteraction {
    pub intersection: IntersectionInfo,
    pub color_filter: Vector3<f64>,
    pub emission: Vector3<f64>,
    pub outgoing: Option<Ray>,
}

impl Material {
    pub fn interact(&self, incoming: &Ray, intersection: &IntersectionInfo) -> SurfaceInteraction {
        if self.emissive {
            SurfaceInteraction {
                color_filter: Vector3::new(1., 1., 1.),
                emission: self.color,
                intersection: *intersection,
                outgoing: None,
            }
        } else {
            let specular_normal = &intersection.normal;
            let mut random_normal: Vector3<f64> =
                Vector3::from_distribution(&StandardNormal, &mut thread_rng()).normalize();
            if random_normal.dot(specular_normal) < 0. {
                random_normal = reflect(&random_normal, specular_normal);
            }

            let sampled_normal = specular_normal.slerp(&random_normal, self.roughness);

            let color_filter = self.color;

            let outgoing_direction = reflect(&incoming.direction, &sampled_normal);

            let outgoing = Ray {
                direction: outgoing_direction,
                origin: intersection.position + specular_normal * 0.001,
            };

            SurfaceInteraction {
                color_filter,
                intersection: *intersection,
                outgoing: Some(outgoing),
                emission: Vector3::zeros(),
            }
        }
    }
}
