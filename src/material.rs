use std::f64::consts::{E, PI, TAU};

use na::Vector3;
use nalgebra as na;
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::{
    find_normal, function_approximation::ProbabilityDensityFunction, reflect,
    shape::IntersectionInfo, Ray,
};

//#[derive(Clone)]
pub struct Material {
    pub color: Vector3<f64>,
    pub roughness: f64,
    pub emissive: bool,
    pub pdf: ProbabilityDensityFunction,
}

fn roughness_pdf(x: f64, roughness: f64) -> f64 {
    let roughness = roughness + 0.0001;
    E.powf(-(x * x) / (roughness * roughness) / 2.) / (roughness * TAU.sqrt())
}

fn ggx(x: f64, roughness: f64) -> f64 {
    let roughness = roughness + 0.00001;
    roughness.powi(2) / (PI * (x.powi(2) * (roughness.powi(2) - 1.) + 1.).powi(2))
}

#[derive(Clone, Copy, Debug)]
pub struct SurfaceInteraction {
    pub intersection: IntersectionInfo,
    pub color_filter: Vector3<f64>,
    pub emission: Vector3<f64>,
    pub outgoing: Ray,
}

impl Material {
    pub fn new(color: Vector3<f64>, roughness: f64, emissive: bool) -> Self {
        let pdf = ProbabilityDensityFunction::build(|x| ggx(x, roughness), 1000);
        Self {
            color,
            roughness,
            emissive,
            pdf,
        }
    }

    pub fn likelihood(
        &self,
        incoming: &Vector3<f64>,
        outgoing: &Vector3<f64>,
        normal: &Vector3<f64>,
    ) -> f64 {
        if normal.dot(&-incoming) < 0. || normal.dot(outgoing) < 0. {
            0.
        } else {
            let sampled_normal = find_normal(incoming, outgoing);
            /*if sampled_normal.dot(normal) < 0. {
                sampled_normal *= -1.;
            }*/
            let angle_dot = sampled_normal.dot(normal);
            self.pdf.likelihood(angle_dot)
        }
    }

    pub fn interact(&self, incoming: &Ray, intersection: &IntersectionInfo) -> SurfaceInteraction {
        let mut specular_normal = intersection.normal;

        if specular_normal.dot(&-incoming.direction) < 0. {
            specular_normal = -intersection.normal;
        }

        let color_filter = self.color;

        //let specular_outgoing = reflect(&incoming.direction, &specular_normal);

        let mut random_direction: Vector3<f64> =
            Vector3::from_distribution(&StandardNormal, &mut thread_rng()).normalize();

        if random_direction.dot(&specular_normal) < 0. {
            random_direction = reflect(&random_direction, &specular_normal);
        }

        let angle = random_direction.dot(&specular_normal).acos();
        let desired_angle = self.pdf.sample(&mut thread_rng()).acos();

        let scatter_normal = specular_normal.slerp(&random_direction, desired_angle / angle);

        //let mut outgoing_direction = specular_outgoing.slerp(&random_direction, self.roughness);
        let mut outgoing_direction = reflect(&incoming.direction, &scatter_normal); //

        /* */
        if outgoing_direction.dot(&specular_normal) < 0. {
            outgoing_direction = reflect(&outgoing_direction, &specular_normal);
        }

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
            outgoing,
        }
    }
}
