use na::Vector3;
use nalgebra as na;
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::{find_normal, reflect, shape::IntersectionInfo, Ray};

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
    pub fn can_connect(&self, lhs: Vector3<f64>, rhs: Vector3<f64>, normal: Vector3<f64>) -> bool {
        //let sampled_normal = (lhs + rhs).normalize();
        let sampled_normal = find_normal(&lhs, &-rhs);
        let required_roughness = 1. - sampled_normal.dot(&normal);
        required_roughness < self.roughness
    }

    pub fn interact(&self, incoming: &Ray, intersection: &IntersectionInfo) -> SurfaceInteraction {
        if false {
            SurfaceInteraction {
                intersection: *intersection,
                color_filter: Vector3::new(1., 1., 1.),
                emission: self.color,
                outgoing: None,
            }
        } else {
            let specular_normal = &intersection.normal;

            let color_filter = self.color;

            let specular_outgoing = reflect(&incoming.direction, specular_normal);

            let mut scatter_direction: Vector3<f64> =
                Vector3::from_distribution(&StandardNormal, &mut thread_rng()).normalize();
            if scatter_direction.dot(specular_normal) < 0. {
                scatter_direction = reflect(&scatter_direction, specular_normal);
            }

            let outgoing_direction = specular_outgoing.slerp(&scatter_direction, self.roughness);

            let outgoing = Ray {
                direction: outgoing_direction,
                origin: intersection.position + outgoing_direction * 0.001,
            };

            let color_filter = if self.emissive {
                Vector3::new(1., 1., 1.)
            } else {
                color_filter
            };

            let emission = if self.emissive {
                self.color
            } else {
                Vector3::zeros()
            };

            SurfaceInteraction {
                intersection: *intersection,
                color_filter,
                emission,
                outgoing: Some(outgoing),
            }
        }
    }
}
