use std::f64::consts::PI;

use na::Vector3;
use nalgebra as na;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

use crate::{
    find_normal, function_approximation::ProbabilityDensityFunction, reflect,
    shape::IntersectionInfo, Ray,
};

#[derive(Clone)]
pub enum Material {
    Reflective {
        color: Vector3<f64>,
        roughness: f64,
        transmission: f64,
        pdf: ProbabilityDensityFunction,
    },
    Emissive {
        color: Vector3<f64>,
    },
}

// fn ggx(x: f64, roughness: f64) -> f64 {
//     let roughness = roughness.powf(2.) + 0.0001;
//     let x = 1. - x;
//     E.powf(-(x * x) / (roughness * roughness) / 2.) / (roughness * TAU.sqrt())
// }

fn ggx(x: f64, roughness: f64) -> f64 {
    if roughness < 0.001 {
        if x >= 0.9999 {
            1.
        } else {
            0.
        }
    } else {
        roughness.powi(2) / (PI * (x.powi(2) * (roughness.powi(2) - 1.) + 1.).powi(2))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SurfaceInteraction {
    pub intersection: IntersectionInfo,
    pub color_filter: Vector3<f64>,
    pub emission: Vector3<f64>,
    pub outgoing: Option<Ray>,
    pub likelihood: f64,
}

impl Material {
    pub fn new_reflective(color: Vector3<f64>, roughness: f64, transmission: f64) -> Self {
        let pdf = ProbabilityDensityFunction::build(|x| ggx(x, roughness), 1000);
        Self::Reflective {
            color,
            roughness,
            transmission,
            pdf,
        }
    }

    pub fn new_emissive(color: Vector3<f64>) -> Self {
        Self::Emissive { color }
    }

    pub fn new(color: Vector3<f64>, roughness: f64, emissive: bool) -> Self {
        if emissive {
            Self::new_emissive(color)
        } else {
            Self::new_reflective(color, roughness, 0.)
        }
    }

    pub fn emission_color(&self) -> Vector3<f64> {
        match self {
            Material::Emissive { color } => *color,
            _ => Vector3::zeros(),
        }
    }

    pub fn absorption_color(&self) -> Vector3<f64> {
        match self {
            Material::Reflective { color, .. } => *color,
            _ => Vector3::new(1., 1., 1.),
        }
    }

    pub fn likelihood(
        &self,
        incoming: &Vector3<f64>,
        outgoing: &Vector3<f64>,
        normal: &Vector3<f64>,
    ) -> f64 {
        match self {
            Material::Reflective {
                pdf, transmission, ..
            } => {
                if (normal.dot(incoming) >= 0.) != (normal.dot(outgoing) <= 0.) {
                    let angle_dot = incoming.dot(outgoing);
                    if angle_dot > 0.9999 {
                        *transmission
                    } else {
                        0.
                    }
                } else {
                    let sampled_normal = find_normal(incoming, outgoing);
                    let angle_dot = sampled_normal.dot(normal);
                    pdf.likelihood(angle_dot) * (1. - transmission)
                }
            }
            Material::Emissive { .. } => 1.,
        }
    }

    pub fn interact(&self, incoming: &Ray, intersection: &IntersectionInfo) -> SurfaceInteraction {
        let mut intersection = *intersection;

        if intersection.normal.dot(&incoming.direction) > 0. {
            intersection.normal = -intersection.normal;
        }

        match self {
            Material::Reflective {
                color,
                roughness,
                pdf,
                transmission,
                ..
            } => {
                let mut rng = thread_rng();
                let transmitted = rng.gen_bool(*transmission);
                let (outgoing_direction, likelihood) = if transmitted {
                    (incoming.direction, *transmission)
                } else {
                    let (desired_angle, likelihood) = if *roughness == 0. {
                        (1., 1.)
                    } else {
                        let sample = pdf.sample(&mut rng);
                        let likelihood = pdf.likelihood(sample);
                        (sample, likelihood)
                    };

                    let specular_normal = &intersection.normal;
                    let random_direction: Vector3<f64> =
                        Vector3::from_distribution(&StandardNormal, &mut rng).normalize();

                    let perpendicular_vector = specular_normal.cross(&random_direction);

                    let scatter_normal = perpendicular_vector.slerp(specular_normal, desired_angle);

                    let mut outgoing_direction = reflect(&incoming.direction, &scatter_normal);

                    if outgoing_direction.dot(specular_normal) < 0. {
                        outgoing_direction = reflect(&outgoing_direction, specular_normal);
                    }

                    (outgoing_direction, likelihood * (1.0 - transmission))
                };

                let outgoing = Ray {
                    direction: outgoing_direction,
                    origin: intersection.position + outgoing_direction * 0.001,
                };

                SurfaceInteraction {
                    intersection,
                    color_filter: *color,
                    emission: Vector3::zeros(),
                    outgoing: Some(outgoing),
                    likelihood,
                }
            }
            Material::Emissive { color } => SurfaceInteraction {
                intersection,
                color_filter: Vector3::new(1., 1., 1.),
                emission: *color,
                outgoing: None,
                likelihood: 1.,
            },
        }
    }
}
