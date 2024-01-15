use std::{f64::consts::PI, sync::Arc};

use na::Vector3;
use nalgebra as na;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

use crate::{
    find_normal, function_approximation::ProbabilityDensityFunction, reflect,
    shape::IntersectionInfo, Ray, Shader,
};

#[derive(Clone)]
pub enum Material {
    Reflective {
        color: Arc<dyn Shader>,
        roughness: f64,
        transmission: f64,
        ior: f64,
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
    pub fn new_reflective<S: Shader + 'static>(
        color: S,
        roughness: f64,
        transmission: f64,
        ior: f64,
    ) -> Self {
        let pdf = ProbabilityDensityFunction::build(|x| ggx(x, roughness), 1000);
        Self::Reflective {
            color: Arc::new(color),
            roughness,
            transmission,
            ior,
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
            Self::new_reflective(color, roughness, 0., 1.)
        }
    }

    pub fn emission_color(&self) -> Vector3<f64> {
        match self {
            Material::Emissive { color } => *color,
            _ => Vector3::zeros(),
        }
    }

    pub fn absorption_color(&self, local_position: &Vector3<f64>) -> Vector3<f64> {
        match self {
            Material::Reflective { color, .. } => color.shade(local_position),
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
                pdf,
                transmission,
                ior,
                ..
            } => {
                if (normal.dot(incoming) < 0.) == (normal.dot(outgoing) > 0.) {
                    let mut sampled_normal = find_normal(incoming, outgoing);
                    if sampled_normal.dot(normal) < 0. {
                        sampled_normal *= -1.;
                    }
                    let angle_dot = sampled_normal.dot(normal);
                    pdf.likelihood(angle_dot) * (1. - transmission)
                } else {
                    let direction = if normal.dot(incoming) >= 0. {
                        normal.slerp(incoming, *ior)
                    } else {
                        (-normal).slerp(incoming, 1. / ior)
                    };

                    let angle_dot = direction.dot(outgoing);
                    pdf.likelihood(angle_dot) * transmission
                }
            }
            Material::Emissive { .. } => outgoing.dot(normal).max(0.),
        }
    }

    pub fn interact(&self, incoming: &Ray, intersection: &IntersectionInfo) -> SurfaceInteraction {
        let mut intersection = *intersection;

        // if intersection.normal.dot(&incoming.direction) > 0. {
        //     intersection.normal = -intersection.normal;
        // }

        match self {
            Material::Reflective {
                color,
                roughness,
                pdf,
                transmission,
                ior,
                ..
            } => {
                let mut rng = thread_rng();
                let transmitted = rng.gen_bool(*transmission);

                let (desired_angle, likelihood) = if *roughness == 0. {
                    (1., 1.)
                } else {
                    let sample = pdf.sample(&mut rng);
                    let likelihood = pdf.likelihood(sample);
                    (sample, likelihood)
                };
                if intersection.normal.dot(&incoming.direction) > 0. {
                    intersection.normal = -intersection.normal;
                }

                let specular_normal = &intersection.normal;
                let random_direction: Vector3<f64> =
                    Vector3::from_distribution(&StandardNormal, &mut rng).normalize();

                let perpendicular_vector = specular_normal.cross(&random_direction);

                let scatter_normal = perpendicular_vector.slerp(specular_normal, desired_angle);

                // if scatter_normal.dot(&intersection.normal) < 0. {
                //     scatter_normal = -scatter_normal;
                // }

                let (outgoing_direction, likelihood) = if transmitted {
                    if scatter_normal.dot(&incoming.direction) >= 0. {
                        let direction = scatter_normal.slerp(&incoming.direction, *ior);
                        (direction, likelihood * transmission)
                    } else {
                        let direction = (-scatter_normal).slerp(&incoming.direction, 1. / ior);
                        (direction, likelihood * transmission)
                    }
                } else {
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
                    color_filter: color.shade(&intersection.position.coords),
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
