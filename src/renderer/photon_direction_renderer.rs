
use crate::{
    bsptree::{BSPElement, BSPTree},
    octree::{BoundingBox, HasCoordinate}, Ray, RenderBuffer, Renderer, Scene,
};

use na::{Point3, Vector4};
use nalgebra::{self as na, Vector3};
use rand::{seq::IteratorRandom, thread_rng, Rng};

pub struct PhotonDirectionRenderer {
    pub max_bounces: u8,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Copy, Clone)]
pub struct PhotonDirection {
    pub position: Point3<f64>,
    pub origin: Point3<f64>,
}
impl HasCoordinate for PhotonDirection {
    fn coordinate(&self) -> Point3<f64> {
        self.position
    }
}

impl PhotonDirectionRenderer {
    pub fn new(max_bounces: u8) -> Self {
        Self {
            max_bounces,
            bounding_box: BoundingBox::new(-10., 10., -10., 10., -10., 10.),
        }
    }
    fn sample_light_direction(
        &self,
        scene: &Scene,
        output_buffer: &mut Vec<BSPElement<Point3<f64>>>,
    ) {
        let max_bounces = self.max_bounces; //thread_rng().gen_range(1..self.max_bounces);
        let light = scene.random_light();

        let mut current_ray = light.sample_emissive_ray();

        for _ in 0..max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                if object.material().roughness() > 0.1 {
                    output_buffer.push(BSPElement {
                        element: current_ray.origin,
                        position: intersection.position,
                    });
                }

                let interaction = object.material().interact(&current_ray, &intersection);
                if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                }
            } else {
                break;
            }
        }
    }

    fn sample_color<'a>(
        &self,
        ray: &Ray,
        scene: &Scene,
        light_directions: &'a BSPTree<Point3<f64>>,
        relevant_points: &mut Vec<&'a BSPElement<Point3<f64>>>,
    ) -> Vector3<f64> {
        let mut current_color_filter = Vector3::new(1., 1., 1.);
        let mut current_emission = Vector3::zeros();
        let mut current_ray = *ray;

        let mut path_pdf = 1.;
        let path_distance = 0.;

        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let interaction = object.material().interact(&current_ray, &intersection);

                current_emission += interaction.emission.component_mul(&current_color_filter);
                current_color_filter.component_mul_assign(&interaction.filter);
                //path_distance += na::distance(&current_ray.origin, &intersection.position);

                const RADIUS: f64 = 0.01;
                relevant_points.clear();

                light_directions.samples_in_sphere(&intersection.position, RADIUS, relevant_points);

                let photon_pdf = if relevant_points.is_empty() {
                    0.
                } else {
                    1. / relevant_points.len() as f64
                };
                if object.material().roughness() > 0.1 && thread_rng().gen_bool(0.5) {
                    if let Some(chosen_point) = relevant_points.iter().choose(&mut thread_rng()) {
                        let direction = (chosen_point.element - intersection.position).normalize();

                        let brdf_pdf = object
                            .material()
                            .likelihood(&current_ray.direction, &direction, &intersection.normal)
                            .min(2.);
                        if brdf_pdf > 0. {
                            current_ray =
                                Ray::new(intersection.position + direction * 0.001, direction);

                            let mis_pdf = (brdf_pdf + photon_pdf) / 2.;

                            path_pdf *= mis_pdf;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                    let brdf_pdf = interaction.pdf.min(2.);
                    let mis_pdf = (brdf_pdf + photon_pdf) / 2.;
                    path_pdf *= mis_pdf;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if path_pdf > 0.01 {
            current_emission / path_pdf // * (1. / path_distance.max(1.)).powi(2) / path_pdf
        } else {
            Vector3::zeros()
        }
    }

    fn sample_color_test(
        &self,
        ray: &Ray,
        scene: &Scene,
        light_directions: &BSPTree<Point3<f64>>,
    ) -> Vector3<f64> {
        let mut current_ray = *ray;

        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let interaction = object.material().interact(&current_ray, &intersection);

                if object.material().roughness() > 0. {
                    if let Some((light_origin, distance)) =
                        light_directions.closest(&intersection.position)
                    {
                        if distance <= 0.1 {
                            return Vector3::new(1., 1., 1.) * (0.1 - distance) * 10.;
                        } else {
                            return Vector3::zeros();
                        }
                    }
                }
                if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Vector3::new(1., 0., 0.)
    }
}

impl Renderer for PhotonDirectionRenderer {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        let num_photons: usize = (width * height) as usize;

        let mut photons = Vec::with_capacity(num_photons);
        for _ in 0..num_photons {
            self.sample_light_direction(scene, &mut photons);
        }
        println!("Shot {} succesful photons", photons.len());
        let bsp_tree = BSPTree::build(photons);

        let mut render_buffer = RenderBuffer::new(width, height);

        let mut relevant_points = vec![];

        for x in 0..width {
            for y in 0..height {
                let ray = scene.camera.get_ray(x, y);

                let color = self.sample_color(&ray, scene, &bsp_tree, &mut relevant_points);
                render_buffer[(x, y)] += Vector4::new(color.x, color.y, color.z, 1.);
            }
        }

        render_buffer
    }
}
