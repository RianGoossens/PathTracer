use std::f64::consts::PI;

use na::Vector3;
use nalgebra as na;
use rand::thread_rng;
use rand_distr::StandardNormal;

use crate::{
    find_normal, function_approximation::ProbabilityDensityFunction, reflect,
    shape::IntersectionInfo, Ray,
};

#[derive(Clone)]
pub struct OldMaterial {
    pub color: Vector3<f64>,
    pub roughness: f64,
    pub emissive: bool,
    pub pdf: ProbabilityDensityFunction,
}

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

/*
fn roughness_pdf(x: f64, roughness: f64) -> f64 {
    let roughness = roughness + 0.0001;
    E.powf(-(x * x) / (roughness * roughness) / 2.) / (roughness * TAU.sqrt())
}*/

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
            Material::Reflective { pdf, .. } => {
                if (normal.dot(incoming) > 0.) != (normal.dot(outgoing) < 0.) {
                    0.
                } else {
                    let sampled_normal = find_normal(incoming, outgoing);
                    /*if sampled_normal.dot(normal) < 0. {
                        sampled_normal *= -1.;
                    }*/
                    let angle_dot = sampled_normal.dot(normal);
                    pdf.likelihood(angle_dot)
                }
            }
            Material::Emissive { .. } => 1.,
        }
        //ggx(angle_dot, self.roughness)
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
                ..
            } => {
                let specular_normal = intersection.normal;

                //let specular_outgoing = reflect(&incoming.direction, &specular_normal);

                let mut random_direction: Vector3<f64> =
                    Vector3::from_distribution(&StandardNormal, &mut thread_rng()).normalize();

                if random_direction.dot(&specular_normal) < 0. {
                    random_direction = reflect(&random_direction, &specular_normal);
                }
                let angle = random_direction.dot(&specular_normal).acos();
                let desired_angle = if *roughness == 0. {
                    0.
                } else {
                    pdf.sample(&mut thread_rng()).acos()
                };
                //println!("{angle} {desired_angle}");

                let scatter_normal =
                    specular_normal.slerp(&random_direction, desired_angle / angle);
                /*println!(
                    "{} {:?} {:?} {:?}",
                    self.roughness, specular_normal, random_direction, scatter_normal
                );*/

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

                SurfaceInteraction {
                    intersection,
                    color_filter: *color,
                    emission: Vector3::zeros(),
                    outgoing: Some(outgoing),
                }
            }
            Material::Emissive { color } => SurfaceInteraction {
                intersection,
                color_filter: Vector3::new(1., 1., 1.),
                emission: *color,
                outgoing: None,
            },
        }
    }
}
